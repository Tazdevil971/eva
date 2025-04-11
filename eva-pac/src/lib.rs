#![no_std]

#[cfg(target_chip_family = "stm32")]
include!("stm32/chips.rs");
