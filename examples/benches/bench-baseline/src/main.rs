#![no_std]
#![no_main]

extern crate eva_bsp_stm32f767;

use core::arch::asm;
use core::panic::PanicInfo;
use core::sync::atomic::{Ordering, compiler_fence};

use eva_kernel::kprintln;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eva_kernel::rt::abort();
    kprintln!("{}", info);

    loop {}
}

fn init() {
    // Enable PB0 and PB7
    unsafe {
        eva_pac::RCC
            .ahb1enr()
            .update(|ahb1enr| ahb1enr.set_gpioben(true).set_gpiocen(true));

        compiler_fence(Ordering::SeqCst);
        asm!("dsb", options(nostack, preserves_flags));
        compiler_fence(Ordering::SeqCst);

        eva_pac::RCC.cfgr().update(|cfgr| {
            use eva_pac::rcc::*;
            cfgr.set_mco2sel(Mco2selVal::Sys)
                .set_mco2pre(McopreVal::Div1)
        });

        eva_pac::GPIOB.moder().update(|moder| {
            use eva_pac::gpio::*;
            moder
                .set_moder(0, ModerVal::Output)
                .set_moder(7, ModerVal::Output)
        });

        eva_pac::GPIOB.otyper().update(|otyper| {
            use eva_pac::gpio::*;
            otyper.set_ot(0, OtVal::PushPull).set_ot(7, OtVal::PushPull)
        });

        eva_pac::GPIOB.ospeedr().update(|ospeedr| {
            use eva_pac::gpio::*;
            ospeedr
                .set_ospeedr(0, OspeedrVal::VeryHighSpeed)
                .set_ospeedr(7, OspeedrVal::VeryHighSpeed)
        });

        eva_pac::GPIOB.pupdr().update(|pupdr| {
            use eva_pac::gpio::*;
            pupdr
                .set_pupdr(0, PupdrVal::Floating)
                .set_pupdr(7, PupdrVal::Floating)
        });

        eva_pac::GPIOC.moder().update(|moder| {
            use eva_pac::gpio::*;
            moder.set_moder(9, ModerVal::Alternate)
        });

        eva_pac::GPIOC.otyper().update(|otyper| {
            use eva_pac::gpio::*;
            otyper.set_ot(9, OtVal::PushPull)
        });

        eva_pac::GPIOC.ospeedr().update(|ospeedr| {
            use eva_pac::gpio::*;
            ospeedr.set_ospeedr(9, OspeedrVal::VeryHighSpeed)
        });

        eva_pac::GPIOC.pupdr().update(|pupdr| {
            use eva_pac::gpio::*;
            pupdr.set_pupdr(9, PupdrVal::Floating)
        });

        eva_pac::GPIOC.afr(0).update(|afr| afr.set_afr(0, 0));
    }
}

fn set_pin() {
    unsafe {
        eva_pac::GPIOB.bsrr().write({
            use eva_pac::gpio::*;
            BsrrBits::default().set_bs(0, true).set_bs(7, true)
        });
    }
}

fn reset_pin() {
    unsafe {
        eva_pac::GPIOB.bsrr().write({
            use eva_pac::gpio::*;
            BsrrBits::default().set_br(0, true).set_br(7, true)
        });
    }
}

eva_kernel::kmain!(main);

fn main() {
    init();
    reset_pin();

    set_pin();
    unsafe {
        asm!(
            "
            mov r0, #0x10000
            1:
            subs r0, r0, #1
            bne 1b
            "
        );
    }
    reset_pin();
}
