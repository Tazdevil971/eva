#![no_std]
#![no_main]

extern crate eva_bsp_stm32f767;

use core::panic::PanicInfo;
use core::ptr;
use core::arch::asm;

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
    gpio::enable_irq(gpio::Port::B, 1, gpio::Trigger::RisingEdge, 1);
    
    let thread = rt::spawn(16 * 1024, 1, other, c"Other", ptr::null_mut()).unwrap();
    unsafe { THREAD = Some(thread); }
    kdbg!(1);
    yield_now();
    kdbg!(3);
    gpio::high(gpio::Port::B, 1);
    kdbg!(5);
}

extern "C" fn other(_: *mut ()) {
    kdbg!(2);
    rt::suspend_and_yield();
    kdbg!(4);
    gpio::low(gpio::Port::B, 1);
}
