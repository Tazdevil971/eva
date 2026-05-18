#![no_std]
#![no_main]

extern crate eva_bsp_stm32f767;

use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr;

use eva_bsp_stm32f767::gpio;
use eva_kernel::rt::yield_now;
use eva_kernel::{kprintln, rt};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eva_kernel::rt::abort();
    kprintln!("{}", info);

    loop {}
}

#[cfg(debug_assertions)]
use eva_kernel::io::kdbg;
#[cfg(not(debug_assertions))]
macro_rules! kdbg {
    ($($tt:tt)*) => {};
}

eva_kernel::kmain!(main);

static mut THREAD: Option<rt::Thread> = None;

#[unsafe(no_mangle)]
unsafe extern "C" fn EXTI1() {
    gpio::reset_irq(1);

    let thread = unsafe { THREAD };
    if let Some(thread) = thread {
        unsafe {
            rt::resume_irq_unchecked(thread);
        }
    }

    unsafe {
        eva_pac::SCB
            .icsr()
            .write(eva_pac::scb::IcsrBits::default().set_pendsvset(true));

        // This is required, to immediately flush the pending interrupt
        asm!("dsb", options(nostack, preserves_flags));
    }
}

// PB14 TIM1_CH2N

fn configure_timer() {
    use eva_pac::timer::*;

    unsafe {
        eva_pac::TIM1.psc().write(10);
        eva_pac::TIM1.arr().update(|reg| reg.set_arr(256));
        eva_pac::TIM1
            .egr()
            .write(EgrAdvBits::default().set_ug(true));

        eva_pac::TIM1.ccmr_output(0).update(|reg| {
            reg.set_ccs(1, CcmrOutputCcsVal::Output)
                .set_ocm(1, OcmVal::ForceActive)
        });
        eva_pac::TIM1.ccer().update(|reg| reg.set_cce(1, true).set_ccne(1, true));
        eva_pac::TIM1.ccr(1).update(|reg| reg.set_ccr(128));

        // Enable the timer
        eva_pac::TIM1.cr1().update(|reg| reg.set_cen(true));
        eva_pac::TIM1.bdtr().update(|reg| reg.set_moe(true));
    }
}

fn main() {
    // gpio::enable_mco2_out(eva_pac::rcc::McopreVal::Div5);
    // gpio::set_mode(
    //     gpio::Port::B,
    //     1,
    //     gpio::Mode::Output(gpio::OutputType::PushPull),
    //     gpio::Pull::None,
    //     gpio::Speed::VeryHigh,
    // );
    // gpio::low(gpio::Port::B, 1);
    // gpio::enable_irq(gpio::Port::B, 1, gpio::Trigger::RisingEdge, 1);
    configure_timer();
    gpio::set_mode(
        gpio::Port::B,
        14,
        gpio::Mode::Alternate(1),
        gpio::Pull::None,
        gpio::Speed::VeryHigh,
    );
}
