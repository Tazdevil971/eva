#![no_std]
#![no_main]

#[cfg(target_arch = "x86_64")]
extern crate eva_bsp_linux;
#[cfg(target_arch = "arm")]
extern crate eva_bsp_stm32f767;

extern crate eva_libc;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    eva_kernel::rt::abort();
    loop {}
}
