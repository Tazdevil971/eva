#[allow(unused_imports)]
use super::utils;
#[path = "../modules/adc_v2.rs"]
pub mod adc;
#[path = "../modules/adccommon_v2.rs"]
pub mod adccommon;
#[path = "../modules/can_bxcan.rs"]
pub mod can;
#[path = "../modules/crc_v1.rs"]
pub mod crc;
#[path = "../modules/cryp_v2.rs"]
pub mod cryp;
#[path = "../modules/dac_v2.rs"]
pub mod dac;
#[path = "../modules/dbgmcu_f4.rs"]
pub mod dbgmcu;
#[path = "../modules/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../modules/dma_v2.rs"]
pub mod dma;
#[path = "../modules/dma2d_v1.rs"]
pub mod dma2d;
#[path = "../modules/dsihost_v1.rs"]
pub mod dsihost;
#[path = "../modules/eth_v1b.rs"]
pub mod eth;
#[path = "../modules/exti_v1.rs"]
pub mod exti;
#[path = "../modules/flash_f4.rs"]
pub mod flash;
#[path = "../modules/gpio_v2.rs"]
pub mod gpio;
#[path = "../modules/hash_v2.rs"]
pub mod hash;
#[path = "../modules/i2c_v1.rs"]
pub mod i2c;
#[path = "../modules/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../modules/ltdc_v1.rs"]
pub mod ltdc;
#[path = "../modules/otg_v1.rs"]
pub mod otg;
#[path = "../modules/pwr_f4.rs"]
pub mod pwr;
#[path = "../modules/quadspi_v1.rs"]
pub mod quadspi;
#[path = "../modules/rcc_f4.rs"]
pub mod rcc;
#[path = "../modules/rng_v1.rs"]
pub mod rng;
#[path = "../modules/rtc_v2f4.rs"]
pub mod rtc;
#[path = "../modules/sai_v2.rs"]
pub mod sai;
#[path = "../modules/sdmmc_v1.rs"]
pub mod sdmmc;
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
pub const TIM2: timer::TimGp32 = unsafe { <timer::TimGp32>::from_addr(0x40000000) };
pub const TIM3: timer::TimGp16 = unsafe { <timer::TimGp16>::from_addr(0x40000400) };
pub const TIM4: timer::TimGp16 = unsafe { <timer::TimGp16>::from_addr(0x40000800) };
pub const TIM5: timer::TimGp32 = unsafe { <timer::TimGp32>::from_addr(0x40000c00) };
pub const TIM6: timer::TimBasic = unsafe { <timer::TimBasic>::from_addr(0x40001000) };
pub const TIM7: timer::TimBasic = unsafe { <timer::TimBasic>::from_addr(0x40001400) };
pub const TIM12: timer::Tim2ch = unsafe { <timer::Tim2ch>::from_addr(0x40001800) };
pub const TIM13: timer::Tim1ch = unsafe { <timer::Tim1ch>::from_addr(0x40001c00) };
pub const TIM14: timer::Tim1ch = unsafe { <timer::Tim1ch>::from_addr(0x40002000) };
pub const RTC: rtc::Rtc = unsafe { <rtc::Rtc>::from_addr(0x40002800) };
pub const WWDG: wwdg::Wwdg = unsafe { <wwdg::Wwdg>::from_addr(0x40002c00) };
pub const IWDG: iwdg::Iwdg = unsafe { <iwdg::Iwdg>::from_addr(0x40003000) };
pub const SPI2: spi::Spi = unsafe { <spi::Spi>::from_addr(0x40003800) };
pub const SPI3: spi::Spi = unsafe { <spi::Spi>::from_addr(0x40003c00) };
pub const USART2: usart::Usart = unsafe { <usart::Usart>::from_addr(0x40004400) };
pub const USART3: usart::Usart = unsafe { <usart::Usart>::from_addr(0x40004800) };
pub const UART4: usart::Usart = unsafe { <usart::Usart>::from_addr(0x40004c00) };
pub const UART5: usart::Usart = unsafe { <usart::Usart>::from_addr(0x40005000) };
pub const I2C1: i2c::I2c = unsafe { <i2c::I2c>::from_addr(0x40005400) };
pub const I2C2: i2c::I2c = unsafe { <i2c::I2c>::from_addr(0x40005800) };
pub const I2C3: i2c::I2c = unsafe { <i2c::I2c>::from_addr(0x40005c00) };
pub const CAN1: can::Can = unsafe { <can::Can>::from_addr(0x40006400) };
pub const CAN2: can::Can = unsafe { <can::Can>::from_addr(0x40006800) };
pub const PWR: pwr::Pwr = unsafe { <pwr::Pwr>::from_addr(0x40007000) };
pub const DAC1: dac::Dac = unsafe { <dac::Dac>::from_addr(0x40007400) };
pub const UART7: usart::Usart = unsafe { <usart::Usart>::from_addr(0x40007800) };
pub const UART8: usart::Usart = unsafe { <usart::Usart>::from_addr(0x40007c00) };
pub const TIM1: timer::TimAdv = unsafe { <timer::TimAdv>::from_addr(0x40010000) };
pub const TIM8: timer::TimAdv = unsafe { <timer::TimAdv>::from_addr(0x40010400) };
pub const USART1: usart::Usart = unsafe { <usart::Usart>::from_addr(0x40011000) };
pub const USART6: usart::Usart = unsafe { <usart::Usart>::from_addr(0x40011400) };
pub const ADC1: adc::Adc = unsafe { <adc::Adc>::from_addr(0x40012000) };
pub const ADC2: adc::Adc = unsafe { <adc::Adc>::from_addr(0x40012100) };
pub const ADC3: adc::Adc = unsafe { <adc::Adc>::from_addr(0x40012200) };
pub const ADC123_COMMON: adccommon::AdcCommon =
    unsafe { <adccommon::AdcCommon>::from_addr(0x40012300) };
pub const SDIO: sdmmc::Sdmmc = unsafe { <sdmmc::Sdmmc>::from_addr(0x40012c00) };
pub const SPI1: spi::Spi = unsafe { <spi::Spi>::from_addr(0x40013000) };
pub const SPI4: spi::Spi = unsafe { <spi::Spi>::from_addr(0x40013400) };
pub const SYSCFG: syscfg::Syscfg = unsafe { <syscfg::Syscfg>::from_addr(0x40013800) };
pub const EXTI: exti::Exti = unsafe { <exti::Exti>::from_addr(0x40013c00) };
pub const TIM9: timer::Tim2ch = unsafe { <timer::Tim2ch>::from_addr(0x40014000) };
pub const TIM10: timer::Tim1ch = unsafe { <timer::Tim1ch>::from_addr(0x40014400) };
pub const TIM11: timer::Tim1ch = unsafe { <timer::Tim1ch>::from_addr(0x40014800) };
pub const SPI5: spi::Spi = unsafe { <spi::Spi>::from_addr(0x40015000) };
pub const SPI6: spi::Spi = unsafe { <spi::Spi>::from_addr(0x40015400) };
pub const SAI1: sai::Sai = unsafe { <sai::Sai>::from_addr(0x40015800) };
pub const LTDC: ltdc::Ltdc = unsafe { <ltdc::Ltdc>::from_addr(0x40016800) };
pub const DSIHOST: dsihost::Dsihost = unsafe { <dsihost::Dsihost>::from_addr(0x40016c00) };
pub const GPIOA: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40020000) };
pub const GPIOB: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40020400) };
pub const GPIOC: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40020800) };
pub const GPIOD: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40020c00) };
pub const GPIOE: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40021000) };
pub const GPIOF: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40021400) };
pub const GPIOG: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40021800) };
pub const GPIOH: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40021c00) };
pub const GPIOI: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40022000) };
pub const GPIOJ: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40022400) };
pub const GPIOK: gpio::Gpio = unsafe { <gpio::Gpio>::from_addr(0x40022800) };
pub const CRC: crc::Crc = unsafe { <crc::Crc>::from_addr(0x40023000) };
pub const RCC: rcc::Rcc = unsafe { <rcc::Rcc>::from_addr(0x40023800) };
pub const FLASH: flash::Flash = unsafe { <flash::Flash>::from_addr(0x40023c00) };
pub const DMA1: dma::Dma = unsafe { <dma::Dma>::from_addr(0x40026000) };
pub const DMA2: dma::Dma = unsafe { <dma::Dma>::from_addr(0x40026400) };
pub const ETH: eth::Eth = unsafe { <eth::Eth>::from_addr(0x40028000) };
pub const DMA2D: dma2d::Dma2d = unsafe { <dma2d::Dma2d>::from_addr(0x4002b000) };
pub const USB_OTG_HS: otg::Otg = unsafe { <otg::Otg>::from_addr(0x40040000) };
pub const USB_OTG_FS: otg::Otg = unsafe { <otg::Otg>::from_addr(0x50000000) };
pub const DCMI: dcmi::Dcmi = unsafe { <dcmi::Dcmi>::from_addr(0x50050000) };
pub const CRYP: cryp::Cryp = unsafe { <cryp::Cryp>::from_addr(0x50060000) };
pub const HASH: hash::Hash = unsafe { <hash::Hash>::from_addr(0x50060400) };
pub const RNG: rng::Rng = unsafe { <rng::Rng>::from_addr(0x50060800) };
pub const QUADSPI: quadspi::Quadspi = unsafe { <quadspi::Quadspi>::from_addr(0xa0001000) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { <dbgmcu::Dbgmcu>::from_addr(0xe0042000) };
