#![no_std]
#![no_main]

extern crate eva_bsp_linux;

use core::panic::PanicInfo;

use eva_kernel::kprintln;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eva_kernel::rt::abort();
    kprintln!("{}", info);

    loop {}
}

eva_kernel::kmain!(main);

fn main() {
    kprintln!("Hello, world!");
}
