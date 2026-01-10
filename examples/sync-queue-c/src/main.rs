#![no_std]
#![no_main]

extern crate eva_bsp_linux;
extern crate eva_kernel;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    eva_kernel::rt::abort();
    loop {}
}
