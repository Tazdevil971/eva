#![no_std]
use core::alloc::Layout;
use core::arch::{asm, global_asm};
use core::fmt;
use core::mem::size_of;
use core::ptr::{self, addr_of_mut};
use core::sync::atomic::{AtomicU32, Ordering};
use core::time::Duration;

use eva_kernel::scheduler::thread::{self, Priority};
use eva_kernel::{allocator, kdbg, kprint, kprintln, portability, scheduler};

unsafe extern "C" {
    unsafe fn Reset();
    unsafe fn SVCall();
}

#[repr(C)]
union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[unsafe(link_section = ".isr_vector.exceptions")]
#[unsafe(no_mangle)]
static __EXCEPTIONS: [Vector; 15] = [
    Vector { handler: Reset },
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

global_asm!(
    "
    .global Reset
    .type Reset,%function
    .thumb_func
    Reset:
    ",
    // Disable interrupts right out of reset
    "
    cpsid i
    ",
    // Initialize .bss
    "
    ldr r0, =__sbss
    ldr r1, =__ebss
    movs r2, #0
    0:
    cmp r1, r0
    beq 1f
    stm r0!, {{r2}}
    b 0b
    1:
    ",
    // Initialize .data
    "
    ldr r0, =__sdata
    ldr r1, =__edata
    ldr r2, =__sidata
    0:
    cmp r1, r0
    beq 1f
    ldm r2!, {{r3}}
    stm r0!, {{r3}}
    b 0b
    1:
    ",
    // Enable full CP10/CP11 (FPU) access
    "
    ldr r0, =0xe000ed88
    ldr r1, [r0]
    orr r1, r1, #(0b1111 << 20)
    str r1, [r0]
    dsb
    isb
    ",
    // Perform early system initialization
    "
    bl {init_stage0}
    ",
    // Perform kernel initialization
    "
    bl {init_stage1}
    ",
    // Jump to the first thread
    "
    ldr r4, ={switchctx}
    ldr r4, [r4]
    ldr r4, [r4]
    
    ldmia r4, {{r0-r3}}
    ldr r12, [r4, #20]
    ldr lr, [r4, #24]

    msr psp, r4
    movs r4, #2
    msr control, r4
    isb
    cpsie i
    blx lr
    bkpt
    ",
    switchctx = sym SWITCHCTX,
    init_stage0 = sym init_stage0,
    init_stage1 = sym init_stage1
);

unsafe extern "C" fn init_stage0() {
    unsafe {
        // Enable fault handlers
        eva_pac::SCB.shcrs().update(|bits| {
            bits.set_usgfaultena(true)
                .set_busfaultena(true)
                .set_memfaultena(true)
        });
    }

    unsafe {
        // Enable HSI
        eva_pac::RCC.cr().update(|reg| reg.set_hsion(true));

        // Zero out CFGR register
        eva_pac::RCC.cfgr().write(Default::default());

        // Disable HSE/CSS/PLL
        eva_pac::RCC
            .cr()
            .update(|reg| reg.set_hseon(false).set_csson(false).set_pllon(false));

        // Set PLL configuration to its default value
        eva_pac::RCC.pllcfgr().write({
            use eva_pac::rcc::*;

            // Default value: 0x24003010
            PllcfgrBits::default()
                .set_pllm(PllmVal::Div16)
                .set_plln(PllnVal::Mul192)
                .set_pllp(PllpVal::Div2)
                .set_pllsrc(PllsrcVal::Hsi)
                .set_pllq(PllqVal::Div4)
                .set_pllr(PllrVal::Div2)
        });

        // Reset HSE bypass
        eva_pac::RCC.cr().update(|reg| reg.set_hsebyp(false));

        // Reset to zero all interrupts
        eva_pac::RCC.cir().write(Default::default());
    }
}

mod interrupt {
    use core::arch::asm;
    use core::sync::atomic::{Ordering, compiler_fence};

    pub fn disable() {
        unsafe {
            asm!("cpsid i", options(nomem, nostack, preserves_flags));
        }

        // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
        compiler_fence(Ordering::SeqCst);
    }

    pub unsafe fn enable() {
        // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
        compiler_fence(Ordering::SeqCst);

        unsafe {
            asm!("cpsie i", options(nomem, nostack, preserves_flags));
        }
    }

    pub unsafe fn free<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let primask = unsafe { eva_pac::primask::read().primask() };

        struct EnableGuard;

        impl Drop for EnableGuard {
            fn drop(&mut self) {
                unsafe {
                    enable();
                }
            }
        }

        disable();
        let _guard = if !primask { Some(EnableGuard) } else { None };

        f()
    }
}

fn kprint_fmt(args: fmt::Arguments) {
    use core::fmt::Write as _;

    unsafe {
        // SAFETY: We are not doing any yielding
        interrupt::free(|| {
            let mut channel = {
                // SAFETY: We are running in an interrupt free section!
                rtt_target::UpChannel::conjure(0).unwrap()
            };

            let _ = channel.write_fmt(args);
        })
    }
}

unsafe extern "C" fn init_stage1() {
    rtt_target::rtt_init! {
        up: {
            0: {
                size: 1024,
                mode: rtt_target::ChannelMode::NoBlockSkip,
                name: "Terminal"
            }
        }
    };

    kprintln!("--- EVA BOOTLOG ---");
    kprintln!("-> EVA logger    [online]");

    // Initialize allocator
    {
        unsafe extern "C" {
            unsafe static mut __heap_start: u8;
            unsafe static mut __heap_end: u8;
        }

        unsafe {
            allocator::init(addr_of_mut!(__heap_start), addr_of_mut!(__heap_end));
        }

        kprintln!("-> EVA allocator [online]");
    }

    // Initialize scheduling interrupts
    {
        // Set PendSV priority to minimum
        unsafe {
            eva_pac::SCB.shpr3().update(|reg| reg.set_pri_14(255));
        }
    }

    {
        unsafe {
            // Spawn first thread
            thread::spawn(4096, Priority::MIN, init_stage2, 0 as _);

            // Launch the scheduler
            scheduler::init();
        }
    }
}

unsafe extern "C" fn init_stage2(_: *mut ()) {
    // Yay this is the first thread!
    kprintln!("-> EVA scheduler [online]");

    // Now that the scheduler is fully online, we can safely enable preemption
    {
        unsafe {
            // Retrieve the reload value for tenms
            let tenms = eva_pac::SYST.calib().read().tenms();

            // Setup reload value
            eva_pac::SYST
                .rvr()
                .write(eva_pac::syst::RvrBits::default().set_reload(tenms / 10));

            // Final configuration and timer start
            eva_pac::SYST.csr().write({
                use eva_pac::syst::*;
                CsrBits::default()
                    .set_countflag(false)
                    .set_clksource(ClksourceVal::Processor)
                    .set_tickint(true)
                    .set_enable(true)
            });
        }
    }

    kprintln!("Pivoting control to user code, good luck!");
    kprintln!("--- EVA BOOTLOG ---");

    eva_kernel::kmain::invoke();
}

#[macro_export]
macro_rules! eva_main {
    ($name:expr) => {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn __eva_start() {
            const MAIN: fn() = $name;
            (MAIN)();
        }
    };
}

#[unsafe(no_mangle)]
unsafe extern "C" fn NMI() {
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn HardFault() {
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn MemManage() {
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn BusFault() {
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn UsageFault() {
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn DefaultHandler() {
    loop {}
}

// TODO: This should really not be here
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", info);

    loop {}
}

#[repr(C)]
#[derive(Debug)]
struct SwitchCtx {
    sp: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    handler_lr: u32,
}

#[repr(C)]
struct ArmIrqStack {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
    s: [u32; 16],
    fpscr: u32,
    reserved: u32,
}

struct PortabilityImpl;

impl portability::Impl for PortabilityImpl {
    fn switchctx_layout() -> Layout {
        Layout::new::<SwitchCtx>()
    }

    unsafe fn init_switchctx(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
        _stack_size: usize,
        entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
    ) {
        unsafe {
            let stack_ptr = stack_ptr.sub(size_of::<ArmIrqStack>());

            stack_ptr.cast::<ArmIrqStack>().write(ArmIrqStack {
                r0: arg1 as _,
                r1: arg2 as _,
                r2: 0,
                r3: 0,
                r12: 0,
                lr: 0,
                pc: entry as _,
                xpsr: 0x01000000,
                s: [0; 16],
                fpscr: 0,
                reserved: 0,
            });

            switchctx_ptr.cast::<SwitchCtx>().write(SwitchCtx {
                sp: stack_ptr as _,
                r4: 0,
                r5: 0,
                r6: 0,
                r7: 0,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
                handler_lr: 0xffff_fffd,
            });
        }
    }

    unsafe fn set_global_switchctx(switchctx_ptr: *mut u8) {
        unsafe {
            SWITCHCTX = switchctx_ptr;
        }
    }

    fn yield_now() {
        unsafe {
            eva_pac::SCB
                .icsr()
                .write(eva_pac::scb::IcsrBits::default().set_pendsvset(true));
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Acquire);
    }

    fn kprint_fmt(args: fmt::Arguments) {
        kprint_fmt(args);
    }

    fn get_time() -> Duration {
        Duration::from_millis(TICKS.load(Ordering::SeqCst) as _)
    }
}

eva_kernel::set_global_portability_impl!(PortabilityImpl);

unsafe extern "C" {
    unsafe fn PendSV();
}

static mut SWITCHCTX: *mut u8 = ptr::null_mut();

global_asm!(
    "
    .global PendSV
    .type PendSV,%function
    .thumb_func
    PendSV:
    ",
    // Save current context
    "
    ldr r0, ={switchctx}
    ldr r0, [r0]
    mrs r1, psp
    stmia r0, {{r1,r4-r11,lr}}
    ",
    // Call into the scheduler
    "
    bl {schedule}
    ",
    // Restore new context
    "
    ldr r0, ={switchctx}
    ldr r0, [r0]
    ldmia r0, {{r1,r4-r11,lr}}
    msr psp, r1
    bx lr
    ",
    switchctx = sym SWITCHCTX,
    schedule = sym scheduler_tick
);

unsafe extern "C" fn scheduler_tick() {
    unsafe {
        scheduler::tick();
    }
}

static TICKS: AtomicU32 = AtomicU32::new(0);

#[unsafe(no_mangle)]
unsafe extern "C" fn SysTick() {
    TICKS.fetch_add(1, Ordering::SeqCst);
}
