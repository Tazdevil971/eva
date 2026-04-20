#![no_std]
#![no_main]

extern crate eva_bsp_stm32f767;

use core::panic::PanicInfo;
use core::ptr;
use core::time::Duration;

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

fn main() {
    // gpio::enable_mco2_out(eva_pac::rcc::McopreVal::Div5);
    gpio::set_mode(
        gpio::Port::B,
        1,
        gpio::Mode::Output(gpio::OutputType::PushPull),
        gpio::Pull::None,
        gpio::Speed::VeryHigh,
    );
    gpio::low(gpio::Port::B, 1);
    
    let _thread = rt::spawn(16 * 1024, 0, other, c"Other", ptr::null_mut());
    kdbg!(1);
    yield_now();
    kdbg!(3);
    gpio::high(gpio::Port::B, 1);
    kdbg!(4);
    yield_now();
    kdbg!(6);
}

extern "C" fn other(_: *mut ()) {
    kdbg!(2);
    yield_now();
    kdbg!(5);
    gpio::low(gpio::Port::B, 1);
}
