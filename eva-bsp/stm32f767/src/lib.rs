#![no_std]
use core::alloc::Layout;
use core::arch::{asm, naked_asm};
use core::mem::size_of;
use core::ptr::{self, addr_of_mut};
use core::sync::atomic::{AtomicU32, Ordering, compiler_fence};
use core::time::Duration;

use eva_kernel::{allocator, kprintln, port, rt};

pub mod gpio;

unsafe extern "C" {
    unsafe fn SVCall();
    unsafe fn WWDG();
    unsafe fn PVD();
    unsafe fn TAMP_STAMP();
    unsafe fn RTC_WKUP();
    unsafe fn FLASH();
    unsafe fn RCC();
    unsafe fn EXTI0();
    unsafe fn EXTI1();
    unsafe fn EXTI2();
    unsafe fn EXTI3();
    unsafe fn EXTI4();
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "C" fn Reset() {
    naked_asm!(
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
}

#[repr(C)]
union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[unsafe(link_section = ".isr_vector.exceptions")]
#[unsafe(no_mangle)]
static __EXCEPTIONS: [Vector; 26] = [
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
    Vector { handler: WWDG },
    Vector { handler: PVD },
    Vector { handler: TAMP_STAMP },
    Vector { handler: RTC_WKUP },
    Vector { handler: FLASH },
    Vector { handler: RCC },
    Vector { handler: EXTI0 },
    Vector { handler: EXTI1 },
    Vector { handler: EXTI2 },
    Vector { handler: EXTI3 },
    Vector { handler: EXTI4 }
];

unsafe extern "C" fn init_stage0() {
    unsafe {
        // Enable fault handlers
        eva_pac::SCB.shcrs().update(|bits| {
            bits.set_usgfaultena(true)
                .set_busfaultena(true)
                .set_memfaultena(true)
        });
    }

    // Initial clock configuration
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
    
    // 216MHz SysClock setup
    unsafe {
        // Enable Power Control clock
        eva_pac::RCC.apb1enr().update(|reg| reg.set_pwren(true));
        // Config Voltage Scale 1
        eva_pac::PWR.cr1().update(|reg| {
            use eva_pac::pwr::*;
            reg.set_vos(VosVal::Scale1)
        });
        // Enable HSE
        eva_pac::RCC.cr().update(|reg| reg.set_hseon(true));

        // Wait till HSE is ready
        while !eva_pac::RCC.cr().read().hserdy() {}

        // Enable Power Control clock
        eva_pac::RCC.apb1enr().update(|reg| reg.set_pwren(true));
        // Config Voltage Scale 1
        eva_pac::PWR.cr1().update(|reg| {
            use eva_pac::pwr::*;
            reg.set_vos(VosVal::Scale1)
        });
        
        // Enable Over Drive to reach the 216MHz frequency
        // Enable ODEN
        eva_pac::PWR.cr1().update(|reg| reg.set_oden(true));
        
        // Wait till ODR is ready
        while !eva_pac::PWR.csr1().read().odrdy() {}
        
        // Enable DSW
        eva_pac::PWR.cr1().update(|reg| reg.set_odswen(true));
        // Wait till ODSW is ready
        while !eva_pac::PWR.csr1().read().odswrdy() {}
        
        // HCLK = SYSCLK / 1
        eva_pac::RCC.cfgr().update(|reg| {
            use eva_pac::rcc::*;
            reg.set_hpre(HpreVal::Div1)
        });
        // PLCLK2 = HCLK / 2
        eva_pac::RCC.cfgr().update(|reg| {
            use eva_pac::rcc::*;
            reg.set_ppre2(PpreVal::Div2)
        });
        // PLCLK1 = HCLK / 4
        eva_pac::RCC.cfgr().update(|reg| {
            use eva_pac::rcc::*;
            reg.set_ppre1(PpreVal::Div4)
        });
        
        // Configure the main PLL
        eva_pac::RCC.pllcfgr().write({
            use eva_pac::rcc::*;
            PllcfgrBits::default()
                .set_pllsrc(PllsrcVal::Hse)
                .set_pllm(PllmVal::Div25)
                .set_pllq(PllqVal::Div9)
                .set_pllr(PllrVal::Div7)
                .set_plln(PllnVal::Mul432)
                .set_pllp(PllpVal::Div2)
        });
        
        // Enable the main PLL
        eva_pac::RCC.cr().update(|reg| reg.set_pllon(true));
        
        // Wait till the PLL is ready
        while !eva_pac::RCC.cr().read().pllrdy() {}
    
        // Configure flash prefetch, instruction cache, data cache and wait state
        eva_pac::FLASH.acr().write({
            use eva_pac::flash::*;
            AcrBits::default().set_latency(LatencyVal::Ws7)
        });
        
        eva_pac::RCC.cfgr().update(|reg| {
            use eva_pac::rcc::*;
            reg.set_sw(SwVal::Pll1P)
        });
        
        {
            use eva_pac::rcc::*;
            while eva_pac::RCC.cfgr().read().sws() != SwVal::Pll1P {}
        }
    }

    // Initialize caches
    unsafe {
        // icache setup
        asm!("dsb", options(nostack, preserves_flags));
        asm!("isb", options(nostack, preserves_flags));
        eva_pac::SCB.iciallu().write(0);
        asm!("dsb", options(nostack, preserves_flags));
        asm!("isb", options(nostack, preserves_flags));
        eva_pac::SCB.ccr().update(|reg| {
            reg.set_ic(true)
        });
        asm!("dsb", options(nostack, preserves_flags));
        asm!("isb", options(nostack, preserves_flags));

        // dcache setup
        eva_pac::SCB.csselr().write({
            use eva_pac::scb::CsselrBits;
            CsselrBits::default()
        });
        asm!("dsb", options(nostack, preserves_flags));
        
        let ccsidr = eva_pac::SCB.ccsidr().read();
        for set in 0..=ccsidr.num_set() {
            for way in 0..=ccsidr.assoc() {
                eva_pac::SCB.dcisw().write({
                    use eva_pac::scb::DcswBits;
                    DcswBits::default()
                        .set_way(way as _)
                        .set_set(set as _)
                });
            }
        }
        asm!("dsb", options(nostack, preserves_flags));
        
        eva_pac::SCB.ccr().update(|reg| {
            reg.set_dc(true)
        });
        asm!("dsb", options(nostack, preserves_flags));
        asm!("isb", options(nostack, preserves_flags));
    }
}

unsafe extern "C" fn init_stage1() {
    rtt_target::rtt_init! {
        up: {
            0: {
                size: 1024,
                mode: rtt_target::ChannelMode::BlockIfFull,
                name: "Terminal"
            }
        }
        down: {
            0: {
                size: 128,
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

    // Initialize preemption, as the kernel is offline
    {
        unsafe {
            // Retrieve the reload value for tenms
            let tenms = eva_pac::SYST.calib().read().tenms();

            // Setup reload value
            eva_pac::SYST
                .rvr()
                .write(eva_pac::syst::RvrBits::default().set_reload(tenms));

            // Final configuration and timer start
            eva_pac::SYST.csr().write({
                use eva_pac::syst::*;
                CsrBits::default()
                    .set_countflag(false)
                    .set_clksource(ClksourceVal::External)
                    .set_tickint(true)
                    .set_enable(true)
            });
        }
    }

    {
        // Spawn first thread
        rt::spawn(64 * 1024, 0, init_stage2, c"Main", ptr::null_mut());

        unsafe {
            // Launch the scheduler
            rt::init();
        }
    }
}

extern "C" fn init_stage2(_: *mut ()) {
    // Yay this is the first thread!

    kprintln!("-> EVA scheduler [online]");
    kprintln!("Pivoting control to user code, good luck!");
    kprintln!("--- EVA BOOTLOG ---");

    // Now perform extra needed configuration
    unsafe {
        // Enable power to all GPIO banks
        eva_pac::RCC.ahb1enr().update(|reg| {
            reg.set_gpioaen(true)
                .set_gpioben(true)
                .set_gpiocen(true)
                .set_gpioden(true)
                .set_gpioeen(true)
                .set_gpiofen(true)
                .set_gpiogen(true)
                .set_gpiohen(true)
                .set_gpioien(true)
                .set_gpiojen(true)
                .set_gpioken(true)
        });
        
        eva_pac::RCC.apb2enr().update(|reg| {
            reg.set_syscfgen(true)
        });
    }

    eva_kernel::kmain::invoke();
}

pub fn irq_disable() {
    unsafe {
        asm!("cpsid i", options(nomem, nostack, preserves_flags));
    }

    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);
}

pub unsafe fn irq_enable() {
    // Ensure no preceding memory accesses are reordered to after interrupts are enabled.
    compiler_fence(Ordering::SeqCst);

    unsafe {
        asm!("cpsie i", options(nomem, nostack, preserves_flags));
    }
}

pub unsafe fn irq_free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let primask = unsafe { eva_pac::primask::read().primask() };

    struct EnableGuard;

    impl Drop for EnableGuard {
        fn drop(&mut self) {
            unsafe {
                irq_enable();
            }
        }
    }

    irq_disable();
    let _guard = if !primask { Some(EnableGuard) } else { None };

    f()
}

struct Cs;

unsafe impl critical_section::Impl for Cs {
    unsafe fn acquire() -> bool {
        let primask = unsafe { eva_pac::primask::read().primask() };
        irq_disable();

        primask
    }

    unsafe fn release(restore_state: bool) {
        if !restore_state {
            unsafe {
                irq_enable();
            }
        }
    }
}

critical_section::set_impl!(Cs);

#[unsafe(no_mangle)]
unsafe extern "C" fn NMI() {
    // kprintln!("NMI");
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn HardFault() {
    kprintln!("HardFault");
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn MemManage() {
    kprintln!("MemManage");
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn BusFault() {
    kprintln!("BusFault");
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn UsageFault() {
    kprintln!("UsageFault");
    loop {}
}

#[unsafe(no_mangle)]
unsafe extern "C" fn DefaultHandler() {
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
    s: [u32; 16],
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

impl port::Impl for PortabilityImpl {
    fn switchctx_layout() -> Layout {
        Layout::new::<SwitchCtx>()
    }

    unsafe fn init_switchctx(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
        _stack_size: usize,
        entry: unsafe extern "C" fn(*mut (), *mut (), *mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
        arg3: *mut (),
        arg4: *mut (),
    ) {
        unsafe {
            let stack_ptr = stack_ptr.sub(size_of::<ArmIrqStack>());

            stack_ptr.cast::<ArmIrqStack>().write(ArmIrqStack {
                r0: arg1 as _,
                r1: arg2 as _,
                r2: arg3 as _,
                r3: arg4 as _,
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
                s: [0; 16],
            });
        }
    }

    unsafe fn drop_switchctx(switchctx_ptr: *mut u8) {
        // Actually a NO-OP, included for correctness
        unsafe {
            switchctx_ptr.cast::<SwitchCtx>().drop_in_place();
        }
    }

    unsafe fn set_global_switchctx(switchctx_ptr: *mut u8) {
        unsafe {
            SWITCHCTX = switchctx_ptr as _;
        }
    }

    fn yield_now() {
        compiler_fence(Ordering::SeqCst);
        unsafe {
            eva_pac::SCB
                .icsr()
                .write(eva_pac::scb::IcsrBits::default().set_pendsvset(true));

            // This is required, to immediately flush the pending interrupt
            asm!("dsb", options(nostack, preserves_flags));
        }
        compiler_fence(Ordering::SeqCst);
    }

    fn kwrite(data: &[u8]) -> usize {
        unsafe {
            // SAFETY: We are not doing any yielding
            irq_free(|| {
                let mut channel = {
                    // SAFETY: We are running in an interrupt free section!
                    rtt_target::UpChannel::conjure(0).unwrap()
                };

                channel.write(data)
            })
        }
    }

    fn kread(data: &mut [u8]) -> usize {
        0
        // unsafe {
        //     // SAFETY: We are not doing any yielding
        //     irq_free(|| {
        //         let mut channel = {
        //             // SAFETY: We are running in an interrupt free section!
        //             rtt_target::DownChannel::new(0).unwrap()
        //         };
        //
        //         channel.read(data)
        //     })
        // }
    }

    fn get_time() -> Duration {
        // TODO: This could be more accurate!
        Duration::from_millis(MS.load(Ordering::Relaxed) as _)
    }
}

eva_kernel::set_global_portability_impl!(PortabilityImpl);

static mut NULL_SWITCHCTX: SwitchCtx = SwitchCtx {
    sp: 0,
    r4: 0,
    r5: 0,
    r6: 0,
    r7: 0,
    r8: 0,
    r9: 0,
    r10: 0,
    r11: 0,
    handler_lr: 0,
    s: [0; 16],
};

static mut SWITCHCTX: *mut SwitchCtx = addr_of_mut!(NULL_SWITCHCTX);

#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn PendSV() {
    naked_asm!(
        // FIXME: Workaround for llvm-project#98673 and llvm-project#97685
        // Essentially LLVM forgets about what target it is compiling and we need to specify it here too
        "
        .cpu cortex-m7
        ",
        // Save current context
        "
        ldr r0, ={switchctx}
        ldr r0, [r0]
        mrs r1, psp
        stmia r0, {{r1,r4-r11,lr}}
        tst lr, #0x10
        it eq
        vstmiaeq r0, {{s16-s31}}
        dmb
        ",
        // Call into the scheduler
        "
        bl {scheduler_tick}
        ",
        // Restore new context
        "
        ldr r0, ={switchctx}
        ldr r0, [r0]
        ldmia r0, {{r1,r4-r11,lr}}
        tst lr, #0x10
        it eq
        vldmiaeq r0, {{s16-s31}}
        msr psp, r1
        bx lr
        ",
        switchctx = sym SWITCHCTX,
        scheduler_tick = sym scheduler_tick
    );
}

unsafe extern "C" fn scheduler_tick() {
    unsafe {
        rt::tick();
    }
}

static MS: AtomicU32 = AtomicU32::new(0);

#[unsafe(no_mangle)]
unsafe extern "C" fn SysTick() {
    MS.fetch_add(10, Ordering::Relaxed);

    // Pend a yield
    // unsafe {
    //     eva_pac::SCB
    //         .icsr()
    //         .write(eva_pac::scb::IcsrBits::default().set_pendsvset(true));
    // }
}
