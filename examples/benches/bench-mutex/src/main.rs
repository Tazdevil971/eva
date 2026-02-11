#![no_std]
#![no_main]

extern crate eva_bsp_stm32f767;

use core::panic::PanicInfo;
use core::ptr;
use core::sync::atomic::{compiler_fence, Ordering};
use core::arch::asm;

use eva_kernel::rt::sync::Mutex;
use eva_kernel::rt::yield_now;
use eva_kernel::{kprintln, rt};

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
            .update(|ahb1enr| ahb1enr.set_gpioben(true));

        compiler_fence(Ordering::SeqCst);
        asm!("dsb", options(nostack, preserves_flags));
        compiler_fence(Ordering::SeqCst);

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

#[cfg(debug_assertions)]
use eva_kernel::io::kdbg;
#[cfg(not(debug_assertions))]
macro_rules! kdbg {
    ($($tt:tt)*) => {};
}

static MUTEX: Mutex<()> = Mutex::new(());

fn main() {
    init();

    set_pin();
    {
        let _guard = MUTEX.lock();
    }
    reset_pin();

    let _thread = rt::spawn(16 * 1024, 0, other, c"Other", ptr::null_mut());
    kdbg!(1);
    yield_now();
    kdbg!(4);
    set_pin();    
    kdbg!(5);
    MUTEX.lock();
    kdbg!(9);
    set_pin();
}

extern "C" fn other(_: *mut ()) {
    kdbg!(2);
    let guard = MUTEX.lock();
    kdbg!(3);
    yield_now();
    kdbg!(6);
    reset_pin();
    kdbg!(7);
    core::mem::drop(guard);
    kdbg!(8);
    yield_now();
}
