#[allow(unused_imports)]
use super::utils;
#[path = "../modules/adc_v2.rs"]
pub mod adc;
#[path = "../modules/adccommon_v2.rs"]
pub mod adccommon;
#[path = "../modules/crc_v1.rs"]
pub mod crc;
#[path = "../modules/dac_v2.rs"]
pub mod dac;
#[path = "../modules/dbgmcu_f4.rs"]
pub mod dbgmcu;
#[path = "../modules/dma_v2.rs"]
pub mod dma;
#[path = "../modules/exti_v1.rs"]
pub mod exti;
#[path = "../modules/flash_f4.rs"]
pub mod flash;
#[path = "../modules/fmpi2c_v2.rs"]
pub mod fmpi2c;
#[path = "../modules/gpio_v2.rs"]
pub mod gpio;
#[path = "../modules/i2c_v1.rs"]
pub mod i2c;
#[path = "../modules/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../modules/lptim_v1a.rs"]
pub mod lptim;
#[path = "../modules/pwr_f4.rs"]
pub mod pwr;
#[path = "../modules/rcc_f410.rs"]
pub mod rcc;
#[path = "../modules/rng_v1.rs"]
pub mod rng;
#[path = "../modules/rtc_v2f4.rs"]
pub mod rtc;
#[path = "../modules/spi_v1.rs"]
pub mod spi;
#[path = "../modules/syscfg_f4.rs"]
pub mod syscfg;
#[path = "../modules/timer_v1.rs"]
pub mod timer;
#[path = "../modules/uid_v1.rs"]
pub mod uid;
#[path = "../modules/usart_v2.rs"]
pub mod usart;
#[path = "../modules/wwdg_v1.rs"]
pub mod wwdg;
pub const UID: uid::Uid = unsafe { <uid::Uid>::from_addr(0x1fff7a10) };
pub const TIM5: timer::TimGp32 = unsafe { <timer::TimGp32>::from_addr(0x40000c00) };
pub const TIM6: timer::TimBasic = unsafe { <timer::TimBasic>::from_addr(0x40001000) };
pub const LPTIM1: lptim::Lptim = unsafe { <lptim::Lptim>::from_addr(0x40002400) };
pub const RTC: rtc::Rtc = unsafe { <rtc::Rtc>::from_addr(0x40002800) };
pub const WWDG: wwdg::Wwdg = unsafe { <wwdg::Wwdg>::from_addr(0x40002c00) };
pub const IWDG: iwdg::Iwdg = unsafe { <iwdg::Iwdg>::from_addr(0x40003000) };
pub const USART2: usart::Usart = unsafe { <usart::Usart>::from_addr(0x40004400) };
pub const I2C1: i2c::I2c = unsafe { <i2c::I2c>::from_addr(0x40005400) };
pub const I2C2: i2c::I2c = unsafe { <i2c::I2c>::from_addr(0x40005800) };
pub const FMPI2C1: fmpi2c::Fmpi2c = unsafe { <fmpi2c::Fmpi2c>::from_addr(0x40006000) };
pub const PWR: pwr::Pwr = unsafe { <pwr::Pwr>::from_addr(0x40007000) };
pub const DAC1: dac::Dac = unsafe { <dac::Dac>::from_addr(0x40007400) };
pub const TIM1: timer::TimAdv = unsafe { <timer::TimAdv>::from_addr(0x40010000) };
pub const USART1: usart::Usart = unsafe { <usart::Usart>::from_addr(0x40011000) };
pub const ADC1: adc::Adc = unsafe { <adc::Adc>::from_addr(0x40012000) };
pub const ADC1_COMMON: adccommon::AdcCommon =
    unsafe { <adccommon::AdcCommon>::from_addr(0x40012300) };
pub const SPI1: spi::Spi = unsafe { <spi::Spi>::from_addr(0x40013000) };
pub const SYSCFG: syscfg::Syscfg = unsafe { <syscfg::Syscfg>::from_addr(0x40013800) };
pub const EXTI: exti::Exti = unsafe { <exti::Exti>::from_addr(0x40013c00) };
pub const TIM9: timer::Tim2ch = unsafe { <timer::Tim2ch>::from_addr(0x40014000) };
pub const TIM11: timer::Tim1ch = unsafe { <timer::Tim1ch>::from_addr(0x40014800) };
pub const GPIOA: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40020000) };
pub const GPIOB: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40020400) };
pub const GPIOC: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40020800) };
pub const GPIOH: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40021c00) };
pub const CRC: crc::Crc = unsafe { <crc::Crc>::from_addr(0x40023000) };
pub const RCC: rcc::Rcc = unsafe { <rcc::Rcc>::from_addr(0x40023800) };
pub const FLASH: flash::Flash = unsafe { <flash::Flash>::from_addr(0x40023c00) };
pub const DMA1: dma::Dma = unsafe { <dma::Dma>::from_addr(0x40026000) };
pub const DMA2: dma::Dma = unsafe { <dma::Dma>::from_addr(0x40026400) };
pub const RNG: rng::Rng = unsafe { <rng::Rng>::from_addr(0x40080000) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { <dbgmcu::Dbgmcu>::from_addr(0xe0042000) };
