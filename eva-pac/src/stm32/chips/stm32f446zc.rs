#[allow(unused_imports)]
use super::utils;
#[path = "../modules/adc_v2.rs"]
pub mod adc;
#[path = "../modules/adccommon_v2.rs"]
pub mod adccommon;
#[path = "../modules/can_bxcan.rs"]
pub mod can;
#[path = "../modules/cec_v2.rs"]
pub mod cec;
#[path = "../modules/crc_v1.rs"]
pub mod crc;
#[path = "../modules/dac_v2.rs"]
pub mod dac;
#[path = "../modules/dbgmcu_f4.rs"]
pub mod dbgmcu;
#[path = "../modules/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../modules/dma_v2.rs"]
pub mod dma;
#[path = "../modules/exti_v1.rs"]
pub mod exti;
#[path = "../modules/flash_f4.rs"]
pub mod flash;
#[path = "../modules/fmc_v2x1.rs"]
pub mod fmc;
#[path = "../modules/fmpi2c_v2.rs"]
pub mod fmpi2c;
#[path = "../modules/gpio_v2.rs"]
pub mod gpio;
#[path = "../modules/i2c_v1.rs"]
pub mod i2c;
#[path = "../modules/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../modules/otg_v1.rs"]
pub mod otg;
#[path = "../modules/pwr_f4.rs"]
pub mod pwr;
#[path = "../modules/quadspi_v1.rs"]
pub mod quadspi;
#[path = "../modules/rcc_f4.rs"]
pub mod rcc;
#[path = "../modules/rtc_v2f4.rs"]
pub mod rtc;
#[path = "../modules/sai_v2.rs"]
pub mod sai;
#[path = "../modules/sdmmc_v1.rs"]
pub mod sdmmc;
#[path = "../modules/spdifrx_v1.rs"]
pub mod spdifrx;
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
pub const UID: uid::Uid = unsafe { <uid::Uid>::from_ptr(0x1fff7a10u64 as _) };
pub const TIM2: timer::TimGp32 = unsafe { <timer::TimGp32>::from_ptr(0x40000000u64 as _) };
pub const TIM3: timer::TimGp16 = unsafe { <timer::TimGp16>::from_ptr(0x40000400u64 as _) };
pub const TIM4: timer::TimGp16 = unsafe { <timer::TimGp16>::from_ptr(0x40000800u64 as _) };
pub const TIM5: timer::TimGp32 = unsafe { <timer::TimGp32>::from_ptr(0x40000c00u64 as _) };
pub const TIM6: timer::TimBasic = unsafe { <timer::TimBasic>::from_ptr(0x40001000u64 as _) };
pub const TIM7: timer::TimBasic = unsafe { <timer::TimBasic>::from_ptr(0x40001400u64 as _) };
pub const TIM12: timer::Tim2ch = unsafe { <timer::Tim2ch>::from_ptr(0x40001800u64 as _) };
pub const TIM13: timer::Tim1ch = unsafe { <timer::Tim1ch>::from_ptr(0x40001c00u64 as _) };
pub const TIM14: timer::Tim1ch = unsafe { <timer::Tim1ch>::from_ptr(0x40002000u64 as _) };
pub const RTC: rtc::Rtc = unsafe { <rtc::Rtc>::from_ptr(0x40002800u64 as _) };
pub const WWDG: wwdg::Wwdg = unsafe { <wwdg::Wwdg>::from_ptr(0x40002c00u64 as _) };
pub const IWDG: iwdg::Iwdg = unsafe { <iwdg::Iwdg>::from_ptr(0x40003000u64 as _) };
pub const SPI2: spi::Spi = unsafe { <spi::Spi>::from_ptr(0x40003800u64 as _) };
pub const SPI3: spi::Spi = unsafe { <spi::Spi>::from_ptr(0x40003c00u64 as _) };
pub const SPDIFRX1: spdifrx::Spdifrx = unsafe { <spdifrx::Spdifrx>::from_ptr(0x40004000u64 as _) };
pub const USART2: usart::Usart = unsafe { <usart::Usart>::from_ptr(0x40004400u64 as _) };
pub const USART3: usart::Usart = unsafe { <usart::Usart>::from_ptr(0x40004800u64 as _) };
pub const UART4: usart::Usart = unsafe { <usart::Usart>::from_ptr(0x40004c00u64 as _) };
pub const UART5: usart::Usart = unsafe { <usart::Usart>::from_ptr(0x40005000u64 as _) };
pub const I2C1: i2c::I2c = unsafe { <i2c::I2c>::from_ptr(0x40005400u64 as _) };
pub const I2C2: i2c::I2c = unsafe { <i2c::I2c>::from_ptr(0x40005800u64 as _) };
pub const I2C3: i2c::I2c = unsafe { <i2c::I2c>::from_ptr(0x40005c00u64 as _) };
pub const FMPI2C1: fmpi2c::Fmpi2c = unsafe { <fmpi2c::Fmpi2c>::from_ptr(0x40006000u64 as _) };
pub const CAN1: can::Can = unsafe { <can::Can>::from_ptr(0x40006400u64 as _) };
pub const CAN2: can::Can = unsafe { <can::Can>::from_ptr(0x40006800u64 as _) };
pub const CEC: cec::Cec = unsafe { <cec::Cec>::from_ptr(0x40006c00u64 as _) };
pub const PWR: pwr::Pwr = unsafe { <pwr::Pwr>::from_ptr(0x40007000u64 as _) };
pub const DAC1: dac::Dac = unsafe { <dac::Dac>::from_ptr(0x40007400u64 as _) };
pub const TIM1: timer::TimAdv = unsafe { <timer::TimAdv>::from_ptr(0x40010000u64 as _) };
pub const TIM8: timer::TimAdv = unsafe { <timer::TimAdv>::from_ptr(0x40010400u64 as _) };
pub const USART1: usart::Usart = unsafe { <usart::Usart>::from_ptr(0x40011000u64 as _) };
pub const USART6: usart::Usart = unsafe { <usart::Usart>::from_ptr(0x40011400u64 as _) };
pub const ADC1: adc::Adc = unsafe { <adc::Adc>::from_ptr(0x40012000u64 as _) };
pub const ADC2: adc::Adc = unsafe { <adc::Adc>::from_ptr(0x40012100u64 as _) };
pub const ADC3: adc::Adc = unsafe { <adc::Adc>::from_ptr(0x40012200u64 as _) };
pub const ADC123_COMMON: adccommon::AdcCommon =
    unsafe { <adccommon::AdcCommon>::from_ptr(0x40012300u64 as _) };
pub const SDIO: sdmmc::Sdmmc = unsafe { <sdmmc::Sdmmc>::from_ptr(0x40012c00u64 as _) };
pub const SPI1: spi::Spi = unsafe { <spi::Spi>::from_ptr(0x40013000u64 as _) };
pub const SPI4: spi::Spi = unsafe { <spi::Spi>::from_ptr(0x40013400u64 as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { <syscfg::Syscfg>::from_ptr(0x40013800u64 as _) };
pub const EXTI: exti::Exti = unsafe { <exti::Exti>::from_ptr(0x40013c00u64 as _) };
pub const TIM9: timer::Tim2ch = unsafe { <timer::Tim2ch>::from_ptr(0x40014000u64 as _) };
pub const TIM10: timer::Tim1ch = unsafe { <timer::Tim1ch>::from_ptr(0x40014400u64 as _) };
pub const TIM11: timer::Tim1ch = unsafe { <timer::Tim1ch>::from_ptr(0x40014800u64 as _) };
pub const SAI1: sai::Sai = unsafe { <sai::Sai>::from_ptr(0x40015800u64 as _) };
pub const SAI2: sai::Sai = unsafe { <sai::Sai>::from_ptr(0x40015c00u64 as _) };
pub const GPIOA: gpio::Gpio = unsafe { <gpio::Gpio>::from_ptr(0x40020000u64 as _) };
pub const GPIOB: gpio::Gpio = unsafe { <gpio::Gpio>::from_ptr(0x40020400u64 as _) };
pub const GPIOC: gpio::Gpio = unsafe { <gpio::Gpio>::from_ptr(0x40020800u64 as _) };
pub const GPIOD: gpio::Gpio = unsafe { <gpio::Gpio>::from_ptr(0x40020c00u64 as _) };
pub const GPIOE: gpio::Gpio = unsafe { <gpio::Gpio>::from_ptr(0x40021000u64 as _) };
pub const GPIOF: gpio::Gpio = unsafe { <gpio::Gpio>::from_ptr(0x40021400u64 as _) };
pub const GPIOG: gpio::Gpio = unsafe { <gpio::Gpio>::from_ptr(0x40021800u64 as _) };
pub const GPIOH: gpio::Gpio = unsafe { <gpio::Gpio>::from_ptr(0x40021c00u64 as _) };
pub const CRC: crc::Crc = unsafe { <crc::Crc>::from_ptr(0x40023000u64 as _) };
pub const RCC: rcc::Rcc = unsafe { <rcc::Rcc>::from_ptr(0x40023800u64 as _) };
pub const FLASH: flash::Flash = unsafe { <flash::Flash>::from_ptr(0x40023c00u64 as _) };
pub const DMA1: dma::Dma = unsafe { <dma::Dma>::from_ptr(0x40026000u64 as _) };
pub const DMA2: dma::Dma = unsafe { <dma::Dma>::from_ptr(0x40026400u64 as _) };
pub const USB_OTG_HS: otg::Otg = unsafe { <otg::Otg>::from_ptr(0x40040000u64 as _) };
pub const USB_OTG_FS: otg::Otg = unsafe { <otg::Otg>::from_ptr(0x50000000u64 as _) };
pub const DCMI: dcmi::Dcmi = unsafe { <dcmi::Dcmi>::from_ptr(0x50050000u64 as _) };
pub const FMC: fmc::Fmc = unsafe { <fmc::Fmc>::from_ptr(0xa0000000u64 as _) };
pub const QUADSPI: quadspi::Quadspi = unsafe { <quadspi::Quadspi>::from_ptr(0xa0001000u64 as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { <dbgmcu::Dbgmcu>::from_ptr(0xe0042000u64 as _) };
