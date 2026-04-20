#![no_std]
#![no_main]

extern crate eva_bsp_stm32f767;

use core::panic::PanicInfo;
use core::ptr;

use eva_bsp_stm32f767::gpio;
use eva_kernel::rt::yield_now;
use eva_kernel::{kprintln, rt};
use eva_kernel::rt::sync::Mutex;

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

static MUTEX: Mutex<()> = Mutex::new(());

eva_kernel::kmain!(main);

fn main() {
    gpio::enable_mco2_out(eva_pac::rcc::McopreVal::Div5);
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
    kdbg!(4);
    gpio::high(gpio::Port::B, 1);
    kdbg!(5);
    let _ = MUTEX.lock();
}

extern "C" fn other(_: *mut ()) {
    kdbg!(2);
    let _guard = MUTEX.lock();
    kdbg!(3);
    yield_now();
    kdbg!(6);
    gpio::low(gpio::Port::B, 1);
}
