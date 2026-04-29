#![feature(restricted_std)]

#[cfg(target_arch = "x86_64")]
extern crate eva_bsp_linux;
#[cfg(target_arch = "arm")]
extern crate eva_bsp_stm32f767;

fn main() {    
    println!("Hello, world!");
}
