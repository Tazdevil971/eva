#![no_std]
use core::alloc::Layout;
use core::arch::{asm, global_asm};
use core::mem::size_of;
use core::ptr::{self, addr_of_mut};
use core::sync::atomic::{AtomicU32, Ordering};
use core::time::Duration;

use eva_scheduler::raw_thread::{self, Priority};

unsafe extern "C" {
    unsafe fn Reset();
    unsafe fn NMI();
    unsafe fn HardFault();
    unsafe fn MemManage();
    unsafe fn BusFault();
    unsafe fn UsageFault();
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
    bl {system_init}
    ",
    // Perform kernel initialization
    "
    bl {kernel_init}
    ",
    system_init = sym system_init,
    kernel_init = sym kernel_init
);

unsafe extern "C" fn system_init() {
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
            use eva_pac::rcc::fields::Pllcfgr;
            use eva_pac::rcc::vals::*;

            // Default value: 0x24003010
            Pllcfgr::default()
                .set_pllm(Pllm::Div16)
                .set_plln(Plln::Mul192)
                .set_pllp(Pllp::Div2)
                .set_pllsrc(Pllsrc::Hsi)
                .set_pllq(Pllq::Div4)
                .set_pllr(Pllr::Div2)
        });

        // Reset HSE bypass
        eva_pac::RCC.cr().update(|reg| reg.set_hsebyp(false));

        // Reset to zero all interrupts
        eva_pac::RCC.cir().write(Default::default());
    }
}

unsafe extern "C" fn kernel_init() {
    rtt_target::rtt_init_print!();
    rtt_target::rprintln!("--- EVA BOOTLOG ---");
    rtt_target::rprintln!("-> EVA logger    [online]");

    // Initialize allocator
    {
        unsafe extern "C" {
            unsafe static mut __heap_start: u8;
            unsafe static mut __heap_end: u8;
        }

        rtt_target::rprint!("-> EVA allocator ");
        unsafe {
            eva_alloc::init(addr_of_mut!(__heap_start), addr_of_mut!(__heap_end));
        }
        rtt_target::rprintln!("[online]");
    }

    // Initialize scheduling interrupts
    {
        // Set PendSV priority to minimum
        unsafe {
            eva_pac::SCB.shpr3().update(|reg| reg.set_pri_14(255));
        }
    }

    // Initialize systick
    {
        unsafe {
            // Retrieve the reload value for tenms
            let tenms = eva_pac::SYST.calib().read().tenms();

            // Setup reload value
            eva_pac::SYST.rvr().write({
                use eva_pac::syst::fields::Rvr;

                Rvr::default().set_reload(tenms / 10)
            });

            // Final configuration and timer start
            eva_pac::SYST.csr().write({
                use eva_pac::syst::fields::Csr;
                use eva_pac::syst::vals::Clksource;

                Csr::default()
                    .set_countflag(false)
                    .set_clksource(Clksource::Processor)
                    .set_tickint(true)
                    .set_enable(true)
            });
        }
    }

    {
        unsafe extern "C" fn entrypoint(_: *mut ()) {
            unsafe extern "C" {
                unsafe fn __eva_start();
            }

            rtt_target::rprintln!("[online]");
            rtt_target::rprintln!("Pivoting control to user code, good luck!");
            rtt_target::rprintln!("--- EVA BOOTLOG ---");

            unsafe { __eva_start() }
        }

        rtt_target::rprint!("-> EVA scheduler ");

        unsafe {
            // Spawn first thread
            raw_thread::spawn(4096, Priority::MIN, entrypoint, 0 as _);

            // Launch the scheduler
            eva_scheduler::portability::enter();
        }
    }
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
unsafe extern "C" fn DefaultHandler() {}

// TODO: This should really not be here
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rtt_target::rprintln!("{}", info);

    loop {}
}

mod _critical_section {
    use core::arch::asm;

    struct CmCriticalSection;

    unsafe impl critical_section::Impl for CmCriticalSection {
        unsafe fn acquire() -> bool {
            if unsafe { eva_pac::primask::read().primask() } {
                // Interrupts already disabled
                false
            } else {
                unsafe { asm!("cpsid i", options(nomem, nostack, preserves_flags)) }
                true
            }
        }

        unsafe fn release(state: bool) {
            if state {
                unsafe { asm!("cpsie i", options(nomem, nostack, preserves_flags)) }
            }
        }
    }

    critical_section::set_impl!(CmCriticalSection);
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

struct SchedulerPortabilityImpl;

unsafe impl eva_scheduler::portability::Impl for SchedulerPortabilityImpl {
    fn switchctx_layout() -> Layout {
        Layout::new::<SwitchCtx>()
    }

    unsafe fn init_switchctx(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
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

    unsafe fn enter_first_thread(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
        entry: unsafe extern "C" fn() -> !,
    ) -> ! {
        unsafe {
            asm!(
                // Install thread switchctx
                "
                ldr r3, ={switchctx}
                str r0, [r3]
                ",
                // Clear the whole stack used so far
                "
                ldr r0, =__irq_stack_top
                msr msp, r0
                ",
                // Install new thread stack
                "
                msr psp, r1
                movs r0, #2
                msr control, r0
                isb
                ",
                // Invoke entrypoint
                "
                cpsie i
                blx r2
                bkpt
                ",
                switchctx = sym SWITCHCTX,
                in("r0") switchctx_ptr,
                in("r1") stack_ptr,
                in("r2") entry,
                options(noreturn)
            );
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
                .write(eva_pac::scb::fields::Icsr::default().set_pendsvset(true));
        }
    }

    fn get_time() -> Duration {
        Duration::from_millis(TICKS.load(Ordering::SeqCst) as _)
    }
}

eva_scheduler::set_portability_impl!(SchedulerPortabilityImpl);

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
        eva_scheduler::portability::scheduler_tick();
    }
}

static TICKS: AtomicU32 = AtomicU32::new(0);

#[unsafe(no_mangle)]
unsafe extern "C" fn SysTick() {
    TICKS.fetch_add(1, Ordering::SeqCst);
}
