#![no_std]
#![no_main]

#[cfg(target_arch = "x86_64")]
extern crate eva_bsp_linux;
#[cfg(target_arch = "arm")]
extern crate eva_bsp_stm32f767;

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
