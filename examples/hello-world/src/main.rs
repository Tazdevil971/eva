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
    unsafe {
        eva_pac::RCC.cfgr().update(|reg| {
            use eva_pac::rcc::*;
            reg.set_mco2sel(Mco2selVal::Sys)
                .set_mco2pre(McopreVal::Div4)
        });
        
        eva_pac::RCC.ahb1enr().update(|reg| {
            reg.set_gpioaen(true)
                .set_gpioben(true)
                .set_gpiocen(true)
        });
        
        eva_pac::GPIOC.moder().update(|reg| {
            use eva_pac::gpio::*;
            reg.set_moder(9, ModerVal::Alternate)
        });
        
        eva_pac::GPIOC.ospeedr().update(|reg| {
            use eva_pac::gpio::*;
            reg.set_ospeedr(9, OspeedrVal::VeryHighSpeed)
        });
    }
    
    
    kprintln!("Hello, world!");
}
