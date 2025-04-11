
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Reset and clock control"]
pub struct Rcc {
    ptr: *mut u8,
}
impl Rcc {
    #[inline(always)]
    pub const unsafe fn from_addr(addr: usize) -> Self {
        unsafe { Self::from_ptr(addr as _) }
    }
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "clock control register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "PLL configuration register"]
    pub const fn pllcfgr(&self) -> utils::Reg<PllcfgrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<PllcfgrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "clock configuration register"]
    pub const fn cfgr(&self) -> utils::Reg<CfgrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<CfgrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "clock interrupt register"]
    pub const fn cir(&self) -> utils::Reg<CirBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<CirBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB1 peripheral reset register"]
    pub const fn ahb1rstr(&self) -> utils::Reg<Ahb1rstrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<Ahb1rstrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB2 peripheral reset register"]
    pub const fn ahb2rstr(&self) -> utils::Reg<Ahb2rstrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<Ahb2rstrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB3 peripheral reset register"]
    pub const fn ahb3rstr(&self) -> utils::Reg<Ahb3rstrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<Ahb3rstrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "APB1 peripheral reset register"]
    pub const fn apb1rstr(&self) -> utils::Reg<Apb1rstrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<Apb1rstrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "APB2 peripheral reset register"]
    pub const fn apb2rstr(&self) -> utils::Reg<Apb2rstrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<Apb2rstrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB1 peripheral clock register"]
    pub const fn ahb1enr(&self) -> utils::Reg<Ahb1enrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<Ahb1enrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB2 peripheral clock enable register"]
    pub const fn ahb2enr(&self) -> utils::Reg<Ahb2enrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<Ahb2enrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB3 peripheral clock enable register"]
    pub const fn ahb3enr(&self) -> utils::Reg<Ahb3enrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<Ahb3enrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "APB1 peripheral clock enable register"]
    pub const fn apb1enr(&self) -> utils::Reg<Apb1enrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<Apb1enrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "APB2 peripheral clock enable register"]
    pub const fn apb2enr(&self) -> utils::Reg<Apb2enrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<Apb2enrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB1 peripheral clock enable in low power mode register"]
    pub const fn ahb1lpenr(&self) -> utils::Reg<Ahb1lpenrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<Ahb1lpenrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB2 peripheral clock enable in low power mode register"]
    pub const fn ahb2lpenr(&self) -> utils::Reg<Ahb2lpenrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x54);
            <utils::Reg<Ahb2lpenrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB3 peripheral clock enable in low power mode register"]
    pub const fn ahb3lpenr(&self) -> utils::Reg<Ahb3lpenrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x58);
            <utils::Reg<Ahb3lpenrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "APB1 peripheral clock enable in low power mode register"]
    pub const fn apb1lpenr(&self) -> utils::Reg<Apb1lpenrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<Apb1lpenrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "APB2 peripheral clock enabled in low power mode register"]
    pub const fn apb2lpenr(&self) -> utils::Reg<Apb2lpenrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x64);
            <utils::Reg<Apb2lpenrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Backup domain control register"]
    pub const fn bdcr(&self) -> utils::Reg<BdcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x70);
            <utils::Reg<BdcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "clock control & status register"]
    pub const fn csr(&self) -> utils::Reg<CsrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x74);
            <utils::Reg<CsrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "spread spectrum clock generation register"]
    pub const fn sscgr(&self) -> utils::Reg<SscgrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x80);
            <utils::Reg<SscgrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "PLLI2S configuration register"]
    pub const fn plli2scfgr(&self) -> utils::Reg<Plli2scfgrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x84);
            <utils::Reg<Plli2scfgrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "PLL configuration register"]
    pub const fn pllsaicfgr(&self) -> utils::Reg<PllsaicfgrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x88);
            <utils::Reg<PllsaicfgrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "dedicated clocks configuration register"]
    pub const fn dckcfgr1(&self) -> utils::Reg<Dckcfgr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8c);
            <utils::Reg<Dckcfgr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "dedicated clocks configuration register"]
    pub const fn dckcfgr2(&self) -> utils::Reg<Dckcfgr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x90);
            <utils::Reg<Dckcfgr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB1 peripheral clock register"]
pub struct Ahb1enrBits {
    bits: u32,
}
impl Default for Ahb1enrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ahb1enrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "IO port A clock enable"]
    pub const fn set_gpioaen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port A clock enable"]
    pub const fn gpioaen(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port B clock enable"]
    pub const fn set_gpioben(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port B clock enable"]
    pub const fn gpioben(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port C clock enable"]
    pub const fn set_gpiocen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port C clock enable"]
    pub const fn gpiocen(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port D clock enable"]
    pub const fn set_gpioden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port D clock enable"]
    pub const fn gpioden(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port E clock enable"]
    pub const fn set_gpioeen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port E clock enable"]
    pub const fn gpioeen(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port F clock enable"]
    pub const fn set_gpiofen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port F clock enable"]
    pub const fn gpiofen(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port G clock enable"]
    pub const fn set_gpiogen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port G clock enable"]
    pub const fn gpiogen(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port H clock enable"]
    pub const fn set_gpiohen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port H clock enable"]
    pub const fn gpiohen(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port I clock enable"]
    pub const fn set_gpioien(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port I clock enable"]
    pub const fn gpioien(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port J clock enable"]
    pub const fn set_gpiojen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port J clock enable"]
    pub const fn gpiojen(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port K clock enable"]
    pub const fn set_gpioken(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port K clock enable"]
    pub const fn gpioken(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CRC clock enable"]
    pub const fn set_crcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CRC clock enable"]
    pub const fn crcen(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Backup SRAM interface clock enable"]
    pub const fn set_bkpsramen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Backup SRAM interface clock enable"]
    pub const fn bkpsramen(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CCM data RAM clock enable"]
    pub const fn set_dtcmramen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CCM data RAM clock enable"]
    pub const fn dtcmramen(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA1 clock enable"]
    pub const fn set_dma1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA1 clock enable"]
    pub const fn dma1en(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA2 clock enable"]
    pub const fn set_dma2en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA2 clock enable"]
    pub const fn dma2en(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA2D clock enable"]
    pub const fn set_dma2den(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA2D clock enable"]
    pub const fn dma2den(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Ethernet MAC clock enable"]
    pub const fn set_ethen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Ethernet MAC clock enable"]
    pub const fn ethen(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Ethernet Transmission clock enable"]
    pub const fn set_ethtxen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Ethernet Transmission clock enable"]
    pub const fn ethtxen(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Ethernet Reception clock enable"]
    pub const fn set_ethrxen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Ethernet Reception clock enable"]
    pub const fn ethrxen(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Ethernet PTP clock enable"]
    pub const fn set_ethptpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Ethernet PTP clock enable"]
    pub const fn ethptpen(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB OTG HS clock enable"]
    pub const fn set_usb_otg_hsen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB OTG HS clock enable"]
    pub const fn usb_otg_hsen(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB OTG HSULPI clock enable"]
    pub const fn set_usb_otg_hsulpien(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB OTG HSULPI clock enable"]
    pub const fn usb_otg_hsulpien(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB1 peripheral clock enable in low power mode register"]
pub struct Ahb1lpenrBits {
    bits: u32,
}
impl Default for Ahb1lpenrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ahb1lpenrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "IO port A clock enable during sleep mode"]
    pub const fn set_gpioalpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port A clock enable during sleep mode"]
    pub const fn gpioalpen(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port B clock enable during Sleep mode"]
    pub const fn set_gpioblpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port B clock enable during Sleep mode"]
    pub const fn gpioblpen(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port C clock enable during Sleep mode"]
    pub const fn set_gpioclpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port C clock enable during Sleep mode"]
    pub const fn gpioclpen(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port D clock enable during Sleep mode"]
    pub const fn set_gpiodlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port D clock enable during Sleep mode"]
    pub const fn gpiodlpen(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port E clock enable during Sleep mode"]
    pub const fn set_gpioelpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port E clock enable during Sleep mode"]
    pub const fn gpioelpen(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port F clock enable during Sleep mode"]
    pub const fn set_gpioflpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port F clock enable during Sleep mode"]
    pub const fn gpioflpen(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port G clock enable during Sleep mode"]
    pub const fn set_gpioglpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port G clock enable during Sleep mode"]
    pub const fn gpioglpen(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port H clock enable during Sleep mode"]
    pub const fn set_gpiohlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port H clock enable during Sleep mode"]
    pub const fn gpiohlpen(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port I clock enable during Sleep mode"]
    pub const fn set_gpioilpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port I clock enable during Sleep mode"]
    pub const fn gpioilpen(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port J clock enable during Sleep mode"]
    pub const fn set_gpiojlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port J clock enable during Sleep mode"]
    pub const fn gpiojlpen(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port K clock enable during Sleep mode"]
    pub const fn set_gpioklpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port K clock enable during Sleep mode"]
    pub const fn gpioklpen(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CRC clock enable during Sleep mode"]
    pub const fn set_crclpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CRC clock enable during Sleep mode"]
    pub const fn crclpen(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "AXI to AHB bridge clock enable during Sleep mode"]
    pub const fn set_axilpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "AXI to AHB bridge clock enable during Sleep mode"]
    pub const fn axilpen(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Flash interface clock enable during Sleep mode"]
    pub const fn set_flashlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Flash interface clock enable during Sleep mode"]
    pub const fn flashlpen(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SRAM 1interface clock enable during Sleep mode"]
    pub const fn set_sram1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SRAM 1interface clock enable during Sleep mode"]
    pub const fn sram1lpen(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SRAM 2 interface clock enable during Sleep mode"]
    pub const fn set_sram2lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SRAM 2 interface clock enable during Sleep mode"]
    pub const fn sram2lpen(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Backup SRAM interface clock enable during Sleep mode"]
    pub const fn set_bkpsramlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Backup SRAM interface clock enable during Sleep mode"]
    pub const fn bkpsramlpen(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SRAM 3 interface clock enable during Sleep mode"]
    pub const fn set_sram3lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SRAM 3 interface clock enable during Sleep mode"]
    pub const fn sram3lpen(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DTCM RAM interface clock enable during Sleep mode"]
    pub const fn set_dtcmlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DTCM RAM interface clock enable during Sleep mode"]
    pub const fn dtcmlpen(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA1 clock enable during Sleep mode"]
    pub const fn set_dma1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA1 clock enable during Sleep mode"]
    pub const fn dma1lpen(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA2 clock enable during Sleep mode"]
    pub const fn set_dma2lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA2 clock enable during Sleep mode"]
    pub const fn dma2lpen(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA2D clock enable during Sleep mode"]
    pub const fn set_dma2dlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA2D clock enable during Sleep mode"]
    pub const fn dma2dlpen(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Ethernet MAC clock enable during Sleep mode"]
    pub const fn set_ethlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Ethernet MAC clock enable during Sleep mode"]
    pub const fn ethlpen(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Ethernet transmission clock enable during Sleep mode"]
    pub const fn set_ethtxlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Ethernet transmission clock enable during Sleep mode"]
    pub const fn ethtxlpen(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Ethernet reception clock enable during Sleep mode"]
    pub const fn set_ethrxlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Ethernet reception clock enable during Sleep mode"]
    pub const fn ethrxlpen(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Ethernet PTP clock enable during Sleep mode"]
    pub const fn set_ethptplpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Ethernet PTP clock enable during Sleep mode"]
    pub const fn ethptplpen(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB OTG HS clock enable during Sleep mode"]
    pub const fn set_usb_otg_hslpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB OTG HS clock enable during Sleep mode"]
    pub const fn usb_otg_hslpen(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB OTG HS ULPI clock enable during Sleep mode"]
    pub const fn set_usb_otg_hsulpilpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB OTG HS ULPI clock enable during Sleep mode"]
    pub const fn usb_otg_hsulpilpen(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB1 peripheral reset register"]
pub struct Ahb1rstrBits {
    bits: u32,
}
impl Default for Ahb1rstrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ahb1rstrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "IO port A reset"]
    pub const fn set_gpioarst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port A reset"]
    pub const fn gpioarst(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port B reset"]
    pub const fn set_gpiobrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port B reset"]
    pub const fn gpiobrst(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port C reset"]
    pub const fn set_gpiocrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port C reset"]
    pub const fn gpiocrst(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port D reset"]
    pub const fn set_gpiodrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port D reset"]
    pub const fn gpiodrst(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port E reset"]
    pub const fn set_gpioerst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port E reset"]
    pub const fn gpioerst(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port F reset"]
    pub const fn set_gpiofrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port F reset"]
    pub const fn gpiofrst(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port G reset"]
    pub const fn set_gpiogrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port G reset"]
    pub const fn gpiogrst(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port H reset"]
    pub const fn set_gpiohrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port H reset"]
    pub const fn gpiohrst(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port I reset"]
    pub const fn set_gpioirst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port I reset"]
    pub const fn gpioirst(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port J reset"]
    pub const fn set_gpiojrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port J reset"]
    pub const fn gpiojrst(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IO port K reset"]
    pub const fn set_gpiokrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IO port K reset"]
    pub const fn gpiokrst(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CRC reset"]
    pub const fn set_crcrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CRC reset"]
    pub const fn crcrst(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA2 reset"]
    pub const fn set_dma1rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA2 reset"]
    pub const fn dma1rst(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA2 reset"]
    pub const fn set_dma2rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA2 reset"]
    pub const fn dma2rst(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA2D reset"]
    pub const fn set_dma2drst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA2D reset"]
    pub const fn dma2drst(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Ethernet MAC reset"]
    pub const fn set_ethrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Ethernet MAC reset"]
    pub const fn ethrst(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB OTG HS module reset"]
    pub const fn set_usb_otg_hsrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB OTG HS module reset"]
    pub const fn usb_otg_hsrst(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB2 peripheral clock enable register"]
pub struct Ahb2enrBits {
    bits: u32,
}
impl Default for Ahb2enrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ahb2enrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Camera interface enable"]
    pub const fn set_dcmien(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Camera interface enable"]
    pub const fn dcmien(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "JPEG enable"]
    pub const fn set_jpegen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "JPEG enable"]
    pub const fn jpegen(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "AES module clock enable"]
    pub const fn set_aesen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "AES module clock enable"]
    pub const fn aesen(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Cryptographic modules clock enable"]
    pub const fn set_crypen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Cryptographic modules clock enable"]
    pub const fn crypen(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Hash modules clock enable"]
    pub const fn set_hashen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Hash modules clock enable"]
    pub const fn hashen(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Random number generator clock enable"]
    pub const fn set_rngen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Random number generator clock enable"]
    pub const fn rngen(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB OTG FS clock enable"]
    pub const fn set_usb_otg_fsen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB OTG FS clock enable"]
    pub const fn usb_otg_fsen(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB2 peripheral clock enable in low power mode register"]
pub struct Ahb2lpenrBits {
    bits: u32,
}
impl Default for Ahb2lpenrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ahb2lpenrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Camera interface enable during Sleep mode"]
    pub const fn set_dcmilpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Camera interface enable during Sleep mode"]
    pub const fn dcmilpen(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "JPEG module enabled during Sleep mode"]
    pub const fn set_jpeglpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "JPEG module enabled during Sleep mode"]
    pub const fn jpeglpen(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "AES module clock enable during Sleep mode"]
    pub const fn set_aeslpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "AES module clock enable during Sleep mode"]
    pub const fn aeslpen(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Cryptography modules clock enable during Sleep mode"]
    pub const fn set_cryplpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Cryptography modules clock enable during Sleep mode"]
    pub const fn cryplpen(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Hash modules clock enable during Sleep mode"]
    pub const fn set_hashlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Hash modules clock enable during Sleep mode"]
    pub const fn hashlpen(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Random number generator clock enable during Sleep mode"]
    pub const fn set_rnglpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Random number generator clock enable during Sleep mode"]
    pub const fn rnglpen(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB OTG FS clock enable during Sleep mode"]
    pub const fn set_usb_otg_fslpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB OTG FS clock enable during Sleep mode"]
    pub const fn usb_otg_fslpen(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB2 peripheral reset register"]
pub struct Ahb2rstrBits {
    bits: u32,
}
impl Default for Ahb2rstrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ahb2rstrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Camera interface reset"]
    pub const fn set_dcmirst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Camera interface reset"]
    pub const fn dcmirst(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "AES module reset"]
    pub const fn set_aesrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "AES module reset"]
    pub const fn aesrst(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Cryptographic module reset"]
    pub const fn set_cryprst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Cryptographic module reset"]
    pub const fn cryprst(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Hash module reset"]
    pub const fn set_hsahrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Hash module reset"]
    pub const fn hsahrst(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Random number generator module reset"]
    pub const fn set_rngrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Random number generator module reset"]
    pub const fn rngrst(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB OTG FS module reset"]
    pub const fn set_usb_otg_fsrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB OTG FS module reset"]
    pub const fn usb_otg_fsrst(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB3 peripheral clock enable register"]
pub struct Ahb3enrBits {
    bits: u32,
}
impl Default for Ahb3enrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ahb3enrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Flexible memory controller module clock enable"]
    pub const fn set_fmcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Flexible memory controller module clock enable"]
    pub const fn fmcen(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Quad SPI memory controller clock enable"]
    pub const fn set_quadspien(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Quad SPI memory controller clock enable"]
    pub const fn quadspien(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB3 peripheral clock enable in low power mode register"]
pub struct Ahb3lpenrBits {
    bits: u32,
}
impl Default for Ahb3lpenrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ahb3lpenrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Flexible memory controller module clock enable during Sleep mode"]
    pub const fn set_fmclpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Flexible memory controller module clock enable during Sleep mode"]
    pub const fn fmclpen(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Quand SPI memory controller clock enable during Sleep mode"]
    pub const fn set_quadspilpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Quand SPI memory controller clock enable during Sleep mode"]
    pub const fn quadspilpen(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB3 peripheral reset register"]
pub struct Ahb3rstrBits {
    bits: u32,
}
impl Default for Ahb3rstrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ahb3rstrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Flexible memory controller module reset"]
    pub const fn set_fmcrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Flexible memory controller module reset"]
    pub const fn fmcrst(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Quad SPI memory controller reset"]
    pub const fn set_quadspirst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Quad SPI memory controller reset"]
    pub const fn quadspirst(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "APB1 peripheral clock enable register"]
pub struct Apb1enrBits {
    bits: u32,
}
impl Default for Apb1enrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Apb1enrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TIM2 clock enable"]
    pub const fn set_tim2en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM2 clock enable"]
    pub const fn tim2en(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM3 clock enable"]
    pub const fn set_tim3en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM3 clock enable"]
    pub const fn tim3en(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM4 clock enable"]
    pub const fn set_tim4en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM4 clock enable"]
    pub const fn tim4en(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM5 clock enable"]
    pub const fn set_tim5en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM5 clock enable"]
    pub const fn tim5en(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM6 clock enable"]
    pub const fn set_tim6en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM6 clock enable"]
    pub const fn tim6en(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM7 clock enable"]
    pub const fn set_tim7en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM7 clock enable"]
    pub const fn tim7en(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM12 clock enable"]
    pub const fn set_tim12en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM12 clock enable"]
    pub const fn tim12en(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM13 clock enable"]
    pub const fn set_tim13en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM13 clock enable"]
    pub const fn tim13en(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM14 clock enable"]
    pub const fn set_tim14en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM14 clock enable"]
    pub const fn tim14en(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low power timer 1 clock enable"]
    pub const fn set_lptim1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low power timer 1 clock enable"]
    pub const fn lptim1en(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "RTCAPB clock enable"]
    pub const fn set_rtcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RTCAPB clock enable"]
    pub const fn rtcen(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Window watchdog clock enable"]
    pub const fn set_wwdgen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Window watchdog clock enable"]
    pub const fn wwdgen(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN 3 enable"]
    pub const fn set_can3en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN 3 enable"]
    pub const fn can3en(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI2 clock enable"]
    pub const fn set_spi2en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI2 clock enable"]
    pub const fn spi2en(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI3 clock enable"]
    pub const fn set_spi3en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI3 clock enable"]
    pub const fn spi3en(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPDIF-RX clock enable"]
    pub const fn set_spdifrxen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPDIF-RX clock enable"]
    pub const fn spdifrxen(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART 2 clock enable"]
    pub const fn set_usart2en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART 2 clock enable"]
    pub const fn usart2en(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART3 clock enable"]
    pub const fn set_usart3en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART3 clock enable"]
    pub const fn usart3en(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UART4 clock enable"]
    pub const fn set_uart4en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UART4 clock enable"]
    pub const fn uart4en(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UART5 clock enable"]
    pub const fn set_uart5en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UART5 clock enable"]
    pub const fn uart5en(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C1 clock enable"]
    pub const fn set_i2c1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C1 clock enable"]
    pub const fn i2c1en(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C2 clock enable"]
    pub const fn set_i2c2en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C2 clock enable"]
    pub const fn i2c2en(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C3 clock enable"]
    pub const fn set_i2c3en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C3 clock enable"]
    pub const fn i2c3en(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C4 clock enable"]
    pub const fn set_i2c4en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C4 clock enable"]
    pub const fn i2c4en(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN 1 clock enable"]
    pub const fn set_can1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN 1 clock enable"]
    pub const fn can1en(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN 2 clock enable"]
    pub const fn set_can2en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN 2 clock enable"]
    pub const fn can2en(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HDMI-CEN clock enable"]
    pub const fn set_cecen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HDMI-CEN clock enable"]
    pub const fn cecen(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Power interface clock enable"]
    pub const fn set_pwren(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Power interface clock enable"]
    pub const fn pwren(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DAC interface clock enable"]
    pub const fn set_dacen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DAC interface clock enable"]
    pub const fn dacen(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UART7 clock enable"]
    pub const fn set_uart7en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UART7 clock enable"]
    pub const fn uart7en(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UART8 clock enable"]
    pub const fn set_uart8en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UART8 clock enable"]
    pub const fn uart8en(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub struct Apb1lpenrBits {
    bits: u32,
}
impl Default for Apb1lpenrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Apb1lpenrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TIM2 clock enable during Sleep mode"]
    pub const fn set_tim2lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM2 clock enable during Sleep mode"]
    pub const fn tim2lpen(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM3 clock enable during Sleep mode"]
    pub const fn set_tim3lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM3 clock enable during Sleep mode"]
    pub const fn tim3lpen(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM4 clock enable during Sleep mode"]
    pub const fn set_tim4lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM4 clock enable during Sleep mode"]
    pub const fn tim4lpen(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM5 clock enable during Sleep mode"]
    pub const fn set_tim5lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM5 clock enable during Sleep mode"]
    pub const fn tim5lpen(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM6 clock enable during Sleep mode"]
    pub const fn set_tim6lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM6 clock enable during Sleep mode"]
    pub const fn tim6lpen(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM7 clock enable during Sleep mode"]
    pub const fn set_tim7lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM7 clock enable during Sleep mode"]
    pub const fn tim7lpen(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM12 clock enable during Sleep mode"]
    pub const fn set_tim12lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM12 clock enable during Sleep mode"]
    pub const fn tim12lpen(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM13 clock enable during Sleep mode"]
    pub const fn set_tim13lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM13 clock enable during Sleep mode"]
    pub const fn tim13lpen(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM14 clock enable during Sleep mode"]
    pub const fn set_tim14lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM14 clock enable during Sleep mode"]
    pub const fn tim14lpen(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "low power timer 1 clock enable during Sleep mode"]
    pub const fn set_lptim1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "low power timer 1 clock enable during Sleep mode"]
    pub const fn lptim1lpen(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "RTCAPB clock enable during Sleep mode"]
    pub const fn set_rtclpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RTCAPB clock enable during Sleep mode"]
    pub const fn rtclpen(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Window watchdog clock enable during Sleep mode"]
    pub const fn set_wwdglpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Window watchdog clock enable during Sleep mode"]
    pub const fn wwdglpen(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN 3 clock enable during Sleep mode"]
    pub const fn set_can3lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN 3 clock enable during Sleep mode"]
    pub const fn can3lpen(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI2 clock enable during Sleep mode"]
    pub const fn set_spi2lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI2 clock enable during Sleep mode"]
    pub const fn spi2lpen(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI3 clock enable during Sleep mode"]
    pub const fn set_spi3lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI3 clock enable during Sleep mode"]
    pub const fn spi3lpen(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPDIF-RX clock enable during sleep mode"]
    pub const fn set_spdifrxlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPDIF-RX clock enable during sleep mode"]
    pub const fn spdifrxlpen(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART2 clock enable during Sleep mode"]
    pub const fn set_usart2lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART2 clock enable during Sleep mode"]
    pub const fn usart2lpen(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART3 clock enable during Sleep mode"]
    pub const fn set_usart3lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART3 clock enable during Sleep mode"]
    pub const fn usart3lpen(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UART4 clock enable during Sleep mode"]
    pub const fn set_uart4lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UART4 clock enable during Sleep mode"]
    pub const fn uart4lpen(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UART5 clock enable during Sleep mode"]
    pub const fn set_uart5lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UART5 clock enable during Sleep mode"]
    pub const fn uart5lpen(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C1 clock enable during Sleep mode"]
    pub const fn set_i2c1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C1 clock enable during Sleep mode"]
    pub const fn i2c1lpen(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C2 clock enable during Sleep mode"]
    pub const fn set_i2c2lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C2 clock enable during Sleep mode"]
    pub const fn i2c2lpen(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C3 clock enable during Sleep mode"]
    pub const fn set_i2c3lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C3 clock enable during Sleep mode"]
    pub const fn i2c3lpen(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C4 clock enable during Sleep mode"]
    pub const fn set_i2c4lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C4 clock enable during Sleep mode"]
    pub const fn i2c4lpen(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN 1 clock enable during Sleep mode"]
    pub const fn set_can1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN 1 clock enable during Sleep mode"]
    pub const fn can1lpen(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN 2 clock enable during Sleep mode"]
    pub const fn set_can2lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN 2 clock enable during Sleep mode"]
    pub const fn can2lpen(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HDMI-CEN clock enable during Sleep mode"]
    pub const fn set_ceclpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HDMI-CEN clock enable during Sleep mode"]
    pub const fn ceclpen(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Power interface clock enable during Sleep mode"]
    pub const fn set_pwrlpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Power interface clock enable during Sleep mode"]
    pub const fn pwrlpen(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DAC interface clock enable during Sleep mode"]
    pub const fn set_daclpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DAC interface clock enable during Sleep mode"]
    pub const fn daclpen(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UART7 clock enable during Sleep mode"]
    pub const fn set_uart7lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UART7 clock enable during Sleep mode"]
    pub const fn uart7lpen(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UART8 clock enable during Sleep mode"]
    pub const fn set_uart8lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UART8 clock enable during Sleep mode"]
    pub const fn uart8lpen(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "APB1 peripheral reset register"]
pub struct Apb1rstrBits {
    bits: u32,
}
impl Default for Apb1rstrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Apb1rstrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TIM2 reset"]
    pub const fn set_tim2rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM2 reset"]
    pub const fn tim2rst(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM3 reset"]
    pub const fn set_tim3rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM3 reset"]
    pub const fn tim3rst(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM4 reset"]
    pub const fn set_tim4rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM4 reset"]
    pub const fn tim4rst(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM5 reset"]
    pub const fn set_tim5rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM5 reset"]
    pub const fn tim5rst(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM6 reset"]
    pub const fn set_tim6rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM6 reset"]
    pub const fn tim6rst(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM7 reset"]
    pub const fn set_tim7rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM7 reset"]
    pub const fn tim7rst(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM12 reset"]
    pub const fn set_tim12rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM12 reset"]
    pub const fn tim12rst(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM13 reset"]
    pub const fn set_tim13rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM13 reset"]
    pub const fn tim13rst(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM14 reset"]
    pub const fn set_tim14rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM14 reset"]
    pub const fn tim14rst(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low power timer 1 reset"]
    pub const fn set_lptim1rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low power timer 1 reset"]
    pub const fn lptim1rst(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Window watchdog reset"]
    pub const fn set_wwdgrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Window watchdog reset"]
    pub const fn wwdgrst(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN 3 reset"]
    pub const fn set_can3rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN 3 reset"]
    pub const fn can3rst(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI 2 reset"]
    pub const fn set_spi2rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI 2 reset"]
    pub const fn spi2rst(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI 3 reset"]
    pub const fn set_spi3rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI 3 reset"]
    pub const fn spi3rst(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPDIF-RX reset"]
    pub const fn set_spdifrxrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPDIF-RX reset"]
    pub const fn spdifrxrst(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART 2 reset"]
    pub const fn set_usart2rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART 2 reset"]
    pub const fn usart2rst(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART 3 reset"]
    pub const fn set_usart3rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART 3 reset"]
    pub const fn usart3rst(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART 4 reset"]
    pub const fn set_uart4rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART 4 reset"]
    pub const fn uart4rst(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART 5 reset"]
    pub const fn set_uart5rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART 5 reset"]
    pub const fn uart5rst(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C 1 reset"]
    pub const fn set_i2c1rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C 1 reset"]
    pub const fn i2c1rst(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C 2 reset"]
    pub const fn set_i2c2rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C 2 reset"]
    pub const fn i2c2rst(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C3 reset"]
    pub const fn set_i2c3rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C3 reset"]
    pub const fn i2c3rst(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C 4 reset"]
    pub const fn set_i2c4rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C 4 reset"]
    pub const fn i2c4rst(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN1 reset"]
    pub const fn set_can1rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN1 reset"]
    pub const fn can1rst(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN2 reset"]
    pub const fn set_can2rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN2 reset"]
    pub const fn can2rst(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HDMI-CEC reset"]
    pub const fn set_cecrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HDMI-CEC reset"]
    pub const fn cecrst(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Power interface reset"]
    pub const fn set_pwrrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Power interface reset"]
    pub const fn pwrrst(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DAC reset"]
    pub const fn set_dacrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DAC reset"]
    pub const fn dacrst(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UART7 reset"]
    pub const fn set_uart7rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UART7 reset"]
    pub const fn uart7rst(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UART8 reset"]
    pub const fn set_uart8rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UART8 reset"]
    pub const fn uart8rst(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "APB2 peripheral clock enable register"]
pub struct Apb2enrBits {
    bits: u32,
}
impl Default for Apb2enrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Apb2enrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TIM1 clock enable"]
    pub const fn set_tim1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM1 clock enable"]
    pub const fn tim1en(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM8 clock enable"]
    pub const fn set_tim8en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM8 clock enable"]
    pub const fn tim8en(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART1 clock enable"]
    pub const fn set_usart1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART1 clock enable"]
    pub const fn usart1en(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART6 clock enable"]
    pub const fn set_usart6en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART6 clock enable"]
    pub const fn usart6en(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SDMMC2 clock enable"]
    pub const fn set_sdmmc2en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDMMC2 clock enable"]
    pub const fn sdmmc2en(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADC1 clock enable"]
    pub const fn set_adc1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADC1 clock enable"]
    pub const fn adc1en(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADC2 clock enable"]
    pub const fn set_adc2en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADC2 clock enable"]
    pub const fn adc2en(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADC3 clock enable"]
    pub const fn set_adc3en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADC3 clock enable"]
    pub const fn adc3en(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SDMMC1 clock enable"]
    pub const fn set_sdmmc1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDMMC1 clock enable"]
    pub const fn sdmmc1en(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI1 clock enable"]
    pub const fn set_spi1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI1 clock enable"]
    pub const fn spi1en(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI4 clock enable"]
    pub const fn set_spi4en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI4 clock enable"]
    pub const fn spi4en(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "System configuration controller clock enable"]
    pub const fn set_syscfgen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "System configuration controller clock enable"]
    pub const fn syscfgen(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM9 clock enable"]
    pub const fn set_tim9en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM9 clock enable"]
    pub const fn tim9en(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM10 clock enable"]
    pub const fn set_tim10en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM10 clock enable"]
    pub const fn tim10en(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM11 clock enable"]
    pub const fn set_tim11en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM11 clock enable"]
    pub const fn tim11en(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI5 clock enable"]
    pub const fn set_spi5en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI5 clock enable"]
    pub const fn spi5en(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI6 clock enable"]
    pub const fn set_spi6en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI6 clock enable"]
    pub const fn spi6en(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SAI1 clock enable"]
    pub const fn set_sai1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SAI1 clock enable"]
    pub const fn sai1en(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SAI2 clock enable"]
    pub const fn set_sai2en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SAI2 clock enable"]
    pub const fn sai2en(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LTDC clock enable"]
    pub const fn set_ltdcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LTDC clock enable"]
    pub const fn ltdcen(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DSI clock enable"]
    pub const fn set_dsien(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DSI clock enable"]
    pub const fn dsien(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DFSDM1 clock enable"]
    pub const fn set_dfsdm1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DFSDM1 clock enable"]
    pub const fn dfsdm1en(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MDIO clock enable"]
    pub const fn set_mdiosen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MDIO clock enable"]
    pub const fn mdiosen(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB OTG HS PHY controller clock enable"]
    pub const fn set_usbphycen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB OTG HS PHY controller clock enable"]
    pub const fn usbphycen(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "APB2 peripheral clock enabled in low power mode register"]
pub struct Apb2lpenrBits {
    bits: u32,
}
impl Default for Apb2lpenrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Apb2lpenrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TIM1 clock enable during Sleep mode"]
    pub const fn set_tim1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM1 clock enable during Sleep mode"]
    pub const fn tim1lpen(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM8 clock enable during Sleep mode"]
    pub const fn set_tim8lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM8 clock enable during Sleep mode"]
    pub const fn tim8lpen(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART1 clock enable during Sleep mode"]
    pub const fn set_usart1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART1 clock enable during Sleep mode"]
    pub const fn usart1lpen(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART6 clock enable during Sleep mode"]
    pub const fn set_usart6lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART6 clock enable during Sleep mode"]
    pub const fn usart6lpen(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SDMMC2 clock enable during Sleep mode"]
    pub const fn set_sdmmc2lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDMMC2 clock enable during Sleep mode"]
    pub const fn sdmmc2lpen(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADC1 clock enable during Sleep mode"]
    pub const fn set_adc1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADC1 clock enable during Sleep mode"]
    pub const fn adc1lpen(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADC2 clock enable during Sleep mode"]
    pub const fn set_adc2lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADC2 clock enable during Sleep mode"]
    pub const fn adc2lpen(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADC 3 clock enable during Sleep mode"]
    pub const fn set_adc3lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADC 3 clock enable during Sleep mode"]
    pub const fn adc3lpen(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SDMMC1 clock enable during Sleep mode"]
    pub const fn set_sdmmc1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDMMC1 clock enable during Sleep mode"]
    pub const fn sdmmc1lpen(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI 1 clock enable during Sleep mode"]
    pub const fn set_spi1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI 1 clock enable during Sleep mode"]
    pub const fn spi1lpen(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI 4 clock enable during Sleep mode"]
    pub const fn set_spi4lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI 4 clock enable during Sleep mode"]
    pub const fn spi4lpen(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "System configuration controller clock enable during Sleep mode"]
    pub const fn set_syscfglpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "System configuration controller clock enable during Sleep mode"]
    pub const fn syscfglpen(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM9 clock enable during sleep mode"]
    pub const fn set_tim9lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM9 clock enable during sleep mode"]
    pub const fn tim9lpen(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM10 clock enable during Sleep mode"]
    pub const fn set_tim10lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM10 clock enable during Sleep mode"]
    pub const fn tim10lpen(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM11 clock enable during Sleep mode"]
    pub const fn set_tim11lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM11 clock enable during Sleep mode"]
    pub const fn tim11lpen(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI 5 clock enable during Sleep mode"]
    pub const fn set_spi5lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI 5 clock enable during Sleep mode"]
    pub const fn spi5lpen(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI 6 clock enable during Sleep mode"]
    pub const fn set_spi6lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI 6 clock enable during Sleep mode"]
    pub const fn spi6lpen(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SAI1 clock enable during sleep mode"]
    pub const fn set_sai1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SAI1 clock enable during sleep mode"]
    pub const fn sai1lpen(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SAI2 clock enable during sleep mode"]
    pub const fn set_sai2lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SAI2 clock enable during sleep mode"]
    pub const fn sai2lpen(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LTDC clock enable during sleep mode"]
    pub const fn set_ltdclpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LTDC clock enable during sleep mode"]
    pub const fn ltdclpen(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DSI clock enable during Sleep mode"]
    pub const fn set_dsilpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DSI clock enable during Sleep mode"]
    pub const fn dsilpen(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DFSDM1 clock enable during Sleep mode"]
    pub const fn set_dfsdm1lpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DFSDM1 clock enable during Sleep mode"]
    pub const fn dfsdm1lpen(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MDIO clock enable during Sleep mode"]
    pub const fn set_mdioslpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MDIO clock enable during Sleep mode"]
    pub const fn mdioslpen(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "APB2 peripheral reset register"]
pub struct Apb2rstrBits {
    bits: u32,
}
impl Default for Apb2rstrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Apb2rstrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TIM1 reset"]
    pub const fn set_tim1rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM1 reset"]
    pub const fn tim1rst(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM8 reset"]
    pub const fn set_tim8rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM8 reset"]
    pub const fn tim8rst(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART1 reset"]
    pub const fn set_usart1rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART1 reset"]
    pub const fn usart1rst(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USART6 reset"]
    pub const fn set_usart6rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USART6 reset"]
    pub const fn usart6rst(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SDMMC2 reset"]
    pub const fn set_sdmmc2rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDMMC2 reset"]
    pub const fn sdmmc2rst(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADC interface reset (common to all ADCs)"]
    pub const fn set_adcrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADC interface reset (common to all ADCs)"]
    pub const fn adcrst(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SDMMC1 reset"]
    pub const fn set_sdmmc1rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDMMC1 reset"]
    pub const fn sdmmc1rst(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI 1 reset"]
    pub const fn set_spi1rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI 1 reset"]
    pub const fn spi1rst(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI4 reset"]
    pub const fn set_spi4rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI4 reset"]
    pub const fn spi4rst(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "System configuration controller reset"]
    pub const fn set_syscfgrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "System configuration controller reset"]
    pub const fn syscfgrst(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM9 reset"]
    pub const fn set_tim9rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM9 reset"]
    pub const fn tim9rst(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM10 reset"]
    pub const fn set_tim10rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM10 reset"]
    pub const fn tim10rst(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM11 reset"]
    pub const fn set_tim11rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM11 reset"]
    pub const fn tim11rst(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI5 reset"]
    pub const fn set_spi5rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI5 reset"]
    pub const fn spi5rst(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SPI6 reset"]
    pub const fn set_spi6rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SPI6 reset"]
    pub const fn spi6rst(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SAI1 reset"]
    pub const fn set_sai1rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SAI1 reset"]
    pub const fn sai1rst(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SAI2 reset"]
    pub const fn set_sai2rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SAI2 reset"]
    pub const fn sai2rst(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LTDC reset"]
    pub const fn set_ltdcrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LTDC reset"]
    pub const fn ltdcrst(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DSI reset"]
    pub const fn set_dsirst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DSI reset"]
    pub const fn dsirst(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DFSDM 1 reset"]
    pub const fn set_dfsdm1rst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DFSDM 1 reset"]
    pub const fn dfsdm1rst(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MDIOS reset"]
    pub const fn set_mdiosrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MDIOS reset"]
    pub const fn mdiosrst(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB OTG HS PHY controller reset"]
    pub const fn set_usbphycrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB OTG HS PHY controller reset"]
    pub const fn usbphycrst(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Backup domain control register"]
pub struct BdcrBits {
    bits: u32,
}
impl Default for BdcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BdcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "External low-speed oscillator enable"]
    pub const fn set_lseon(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "External low-speed oscillator enable"]
    pub const fn lseon(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "External low-speed oscillator ready"]
    pub const fn set_lserdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "External low-speed oscillator ready"]
    pub const fn lserdy(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "External low-speed oscillator bypass"]
    pub const fn set_lsebyp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "External low-speed oscillator bypass"]
    pub const fn lsebyp(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LSE oscillator drive capability"]
    pub const fn set_lsedrv(mut self, val: LsedrvVal) -> Self {
        self.bits &= !(0x3 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "LSE oscillator drive capability"]
    pub const fn lsedrv(self) -> LsedrvVal {
        let val = ((self.bits >> 0x3) & 0x3) as _;
        unsafe { LsedrvVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "RTC clock source selection"]
    pub const fn set_rtcsel(mut self, val: RtcselVal) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "RTC clock source selection"]
    pub const fn rtcsel(self) -> RtcselVal {
        let val = ((self.bits >> 0x8) & 0x3) as _;
        unsafe { RtcselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "RTC clock enable"]
    pub const fn set_rtcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RTC clock enable"]
    pub const fn rtcen(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Backup domain software reset"]
    pub const fn set_bdrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Backup domain software reset"]
    pub const fn bdrst(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "clock configuration register"]
pub struct CfgrBits {
    bits: u32,
}
impl Default for CfgrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CfgrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "System clock switch"]
    pub const fn set_sw(mut self, val: SwVal) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "System clock switch"]
    pub const fn sw(self) -> SwVal {
        let val = ((self.bits >> 0x0) & 0x3) as _;
        unsafe { SwVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "System clock switch status"]
    pub const fn set_sws(mut self, val: SwVal) -> Self {
        self.bits &= !(0x3 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "System clock switch status"]
    pub const fn sws(self) -> SwVal {
        let val = ((self.bits >> 0x2) & 0x3) as _;
        unsafe { SwVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "AHB prescaler"]
    pub const fn set_hpre(mut self, val: HpreVal) -> Self {
        self.bits &= !(0xf << 0x4);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "AHB prescaler"]
    pub const fn hpre(self) -> HpreVal {
        let val = ((self.bits >> 0x4) & 0xf) as _;
        unsafe { HpreVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "APB Low speed prescaler (APB1)"]
    pub const fn set_ppre1(mut self, val: PpreVal) -> Self {
        self.bits &= !(0x7 << 0xa);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "APB Low speed prescaler (APB1)"]
    pub const fn ppre1(self) -> PpreVal {
        let val = ((self.bits >> 0xa) & 0x7) as _;
        unsafe { PpreVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "APB high-speed prescaler (APB2)"]
    pub const fn set_ppre2(mut self, val: PpreVal) -> Self {
        self.bits &= !(0x7 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "APB high-speed prescaler (APB2)"]
    pub const fn ppre2(self) -> PpreVal {
        let val = ((self.bits >> 0xd) & 0x7) as _;
        unsafe { PpreVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "HSE division factor for RTC clock"]
    pub const fn set_rtcpre(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0x10);
        self.bits |= (val as u32 & 0x1f) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "HSE division factor for RTC clock"]
    pub const fn rtcpre(self) -> u8 {
        ((self.bits >> 0x10) & 0x1f) as _
    }
    #[inline(always)]
    #[doc = "Microcontroller clock output 1"]
    pub const fn set_mco1sel(mut self, val: Mco1selVal) -> Self {
        self.bits &= !(0x3 << 0x15);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x15;
        self
    }
    #[inline(always)]
    #[doc = "Microcontroller clock output 1"]
    pub const fn mco1sel(self) -> Mco1selVal {
        let val = ((self.bits >> 0x15) & 0x3) as _;
        unsafe { Mco1selVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "I2S clock selection"]
    pub const fn set_i2ssrc(mut self, val: IssrcVal) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x17;
        self
    }
    #[inline(always)]
    #[doc = "I2S clock selection"]
    pub const fn i2ssrc(self) -> IssrcVal {
        let val = ((self.bits >> 0x17) & 0x1) as _;
        unsafe { IssrcVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "MCO1 prescaler"]
    pub const fn set_mco1pre(mut self, val: McopreVal) -> Self {
        self.bits &= !(0x7 << 0x18);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "MCO1 prescaler"]
    pub const fn mco1pre(self) -> McopreVal {
        let val = ((self.bits >> 0x18) & 0x7) as _;
        unsafe { McopreVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "MCO2 prescaler"]
    pub const fn set_mco2pre(mut self, val: McopreVal) -> Self {
        self.bits &= !(0x7 << 0x1b);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x1b;
        self
    }
    #[inline(always)]
    #[doc = "MCO2 prescaler"]
    pub const fn mco2pre(self) -> McopreVal {
        let val = ((self.bits >> 0x1b) & 0x7) as _;
        unsafe { McopreVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Microcontroller clock output 2"]
    pub const fn set_mco2sel(mut self, val: Mco2selVal) -> Self {
        self.bits &= !(0x3 << 0x1e);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x1e;
        self
    }
    #[inline(always)]
    #[doc = "Microcontroller clock output 2"]
    pub const fn mco2sel(self) -> Mco2selVal {
        let val = ((self.bits >> 0x1e) & 0x3) as _;
        unsafe { Mco2selVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "clock interrupt register"]
pub struct CirBits {
    bits: u32,
}
impl Default for CirBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CirBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "LSI ready interrupt flag"]
    pub const fn set_lsirdyf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LSI ready interrupt flag"]
    pub const fn lsirdyf(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LSE ready interrupt flag"]
    pub const fn set_lserdyf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LSE ready interrupt flag"]
    pub const fn lserdyf(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HSI ready interrupt flag"]
    pub const fn set_hsirdyf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSI ready interrupt flag"]
    pub const fn hsirdyf(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HSE ready interrupt flag"]
    pub const fn set_hserdyf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSE ready interrupt flag"]
    pub const fn hserdyf(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLL ready interrupt flag"]
    pub const fn set_pllrdyf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLL ready interrupt flag"]
    pub const fn pllrdyf(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLLI2S ready interrupt flag"]
    pub const fn set_plli2srdyf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLLI2S ready interrupt flag"]
    pub const fn plli2srdyf(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLLSAI ready interrupt flag"]
    pub const fn set_pllsairdyf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLLSAI ready interrupt flag"]
    pub const fn pllsairdyf(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock security system interrupt flag"]
    pub const fn set_cssf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock security system interrupt flag"]
    pub const fn cssf(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LSI ready interrupt enable"]
    pub const fn set_lsirdyie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LSI ready interrupt enable"]
    pub const fn lsirdyie(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LSE ready interrupt enable"]
    pub const fn set_lserdyie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LSE ready interrupt enable"]
    pub const fn lserdyie(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HSI ready interrupt enable"]
    pub const fn set_hsirdyie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSI ready interrupt enable"]
    pub const fn hsirdyie(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HSE ready interrupt enable"]
    pub const fn set_hserdyie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSE ready interrupt enable"]
    pub const fn hserdyie(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLL ready interrupt enable"]
    pub const fn set_pllrdyie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLL ready interrupt enable"]
    pub const fn pllrdyie(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLLI2S ready interrupt enable"]
    pub const fn set_plli2srdyie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLLI2S ready interrupt enable"]
    pub const fn plli2srdyie(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLLSAI Ready Interrupt Enable"]
    pub const fn set_pllsairdyie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLLSAI Ready Interrupt Enable"]
    pub const fn pllsairdyie(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LSI ready interrupt clear"]
    pub const fn set_lsirdyc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LSI ready interrupt clear"]
    pub const fn lsirdyc(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LSE ready interrupt clear"]
    pub const fn set_lserdyc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LSE ready interrupt clear"]
    pub const fn lserdyc(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HSI ready interrupt clear"]
    pub const fn set_hsirdyc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSI ready interrupt clear"]
    pub const fn hsirdyc(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HSE ready interrupt clear"]
    pub const fn set_hserdyc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSE ready interrupt clear"]
    pub const fn hserdyc(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Main PLL(PLL) ready interrupt clear"]
    pub const fn set_pllrdyc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Main PLL(PLL) ready interrupt clear"]
    pub const fn pllrdyc(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLLI2S ready interrupt clear"]
    pub const fn set_plli2srdyc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLLI2S ready interrupt clear"]
    pub const fn plli2srdyc(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLLSAI Ready Interrupt Clear"]
    pub const fn set_pllsairdyc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLLSAI Ready Interrupt Clear"]
    pub const fn pllsairdyc(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock security system interrupt clear"]
    pub const fn set_cssc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock security system interrupt clear"]
    pub const fn cssc(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "clock control register"]
pub struct CrBits {
    bits: u32,
}
impl Default for CrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Internal high-speed clock enable"]
    pub const fn set_hsion(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Internal high-speed clock enable"]
    pub const fn hsion(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Internal high-speed clock ready flag"]
    pub const fn set_hsirdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Internal high-speed clock ready flag"]
    pub const fn hsirdy(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Internal high-speed clock trimming"]
    pub const fn set_hsitrim(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0x3);
        self.bits |= (val as u32 & 0x1f) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Internal high-speed clock trimming"]
    pub const fn hsitrim(self) -> u8 {
        ((self.bits >> 0x3) & 0x1f) as _
    }
    #[inline(always)]
    #[doc = "Internal high-speed clock calibration"]
    pub const fn set_hsical(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Internal high-speed clock calibration"]
    pub const fn hsical(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "HSE clock enable"]
    pub const fn set_hseon(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSE clock enable"]
    pub const fn hseon(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HSE clock ready flag"]
    pub const fn set_hserdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSE clock ready flag"]
    pub const fn hserdy(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HSE clock bypass"]
    pub const fn set_hsebyp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSE clock bypass"]
    pub const fn hsebyp(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock security system enable"]
    pub const fn set_csson(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock security system enable"]
    pub const fn csson(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLL enable"]
    pub const fn set_pllon(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLL enable"]
    pub const fn pllon(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLL clock ready flag"]
    pub const fn set_pllrdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLL clock ready flag"]
    pub const fn pllrdy(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLLI2S enable"]
    pub const fn set_plli2son(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLLI2S enable"]
    pub const fn plli2son(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLLI2S clock ready flag"]
    pub const fn set_plli2srdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLLI2S clock ready flag"]
    pub const fn plli2srdy(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLLSAI enable"]
    pub const fn set_pllsaion(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLLSAI enable"]
    pub const fn pllsaion(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLLSAI clock ready flag"]
    pub const fn set_pllsairdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLLSAI clock ready flag"]
    pub const fn pllsairdy(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "clock control & status register"]
pub struct CsrBits {
    bits: u32,
}
impl Default for CsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Internal low-speed oscillator enable"]
    pub const fn set_lsion(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Internal low-speed oscillator enable"]
    pub const fn lsion(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Internal low-speed oscillator ready"]
    pub const fn set_lsirdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Internal low-speed oscillator ready"]
    pub const fn lsirdy(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Remove reset flag"]
    pub const fn set_rmvf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Remove reset flag"]
    pub const fn rmvf(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "BOR reset flag"]
    pub const fn set_borrstf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "BOR reset flag"]
    pub const fn borrstf(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PIN reset flag"]
    pub const fn set_padrstf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PIN reset flag"]
    pub const fn padrstf(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "POR/PDR reset flag"]
    pub const fn set_porrstf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "POR/PDR reset flag"]
    pub const fn porrstf(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Software reset flag"]
    pub const fn set_sftrstf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Software reset flag"]
    pub const fn sftrstf(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Independent watchdog reset flag"]
    pub const fn set_wdgrstf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Independent watchdog reset flag"]
    pub const fn wdgrstf(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Window watchdog reset flag"]
    pub const fn set_wwdgrstf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Window watchdog reset flag"]
    pub const fn wwdgrstf(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-power reset flag"]
    pub const fn set_lpwrrstf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-power reset flag"]
    pub const fn lpwrrstf(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "dedicated clocks configuration register"]
pub struct Dckcfgr1Bits {
    bits: u32,
}
impl Default for Dckcfgr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dckcfgr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "PLLI2S division factor for SAI1 clock"]
    pub const fn set_plli2sdivq(mut self, val: Plli2sdivqVal) -> Self {
        self.bits &= !(0x1f << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "PLLI2S division factor for SAI1 clock"]
    pub const fn plli2sdivq(self) -> Plli2sdivqVal {
        let val = ((self.bits >> 0x0) & 0x1f) as _;
        unsafe { Plli2sdivqVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLLSAI division factor for SAI1 clock"]
    pub const fn set_pllsaidivq(mut self, val: PllsaidivqVal) -> Self {
        self.bits &= !(0x1f << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x1f) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "PLLSAI division factor for SAI1 clock"]
    pub const fn pllsaidivq(self) -> PllsaidivqVal {
        let val = ((self.bits >> 0x8) & 0x1f) as _;
        unsafe { PllsaidivqVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "division factor for LCD_CLK"]
    pub const fn set_pllsaidivr(mut self, val: PllsaidivrVal) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "division factor for LCD_CLK"]
    pub const fn pllsaidivr(self) -> PllsaidivrVal {
        let val = ((self.bits >> 0x10) & 0x3) as _;
        unsafe { PllsaidivrVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "SAI1 clock source selection"]
    pub const fn set_sai1sel(mut self, val: SaiselVal) -> Self {
        self.bits &= !(0x3 << 0x14);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "SAI1 clock source selection"]
    pub const fn sai1sel(self) -> SaiselVal {
        let val = ((self.bits >> 0x14) & 0x3) as _;
        unsafe { SaiselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "SAI2 clock source selection"]
    pub const fn set_sai2sel(mut self, val: SaiselVal) -> Self {
        self.bits &= !(0x3 << 0x16);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x16;
        self
    }
    #[inline(always)]
    #[doc = "SAI2 clock source selection"]
    pub const fn sai2sel(self) -> SaiselVal {
        let val = ((self.bits >> 0x16) & 0x3) as _;
        unsafe { SaiselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Timers clocks prescalers selection"]
    pub const fn set_timpre(mut self, val: TimpreVal) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Timers clocks prescalers selection"]
    pub const fn timpre(self) -> TimpreVal {
        let val = ((self.bits >> 0x18) & 0x1) as _;
        unsafe { TimpreVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "DFSDM1 clock source selection"]
    pub const fn set_dfsdm1sel(mut self, val: DfsdmselVal) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x19;
        self
    }
    #[inline(always)]
    #[doc = "DFSDM1 clock source selection"]
    pub const fn dfsdm1sel(self) -> DfsdmselVal {
        let val = ((self.bits >> 0x19) & 0x1) as _;
        unsafe { DfsdmselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "DFSDM1 AUDIO clock source selection"]
    pub const fn set_adfsdm1sel(mut self, val: AdfsdmselVal) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1a;
        self
    }
    #[inline(always)]
    #[doc = "DFSDM1 AUDIO clock source selection"]
    pub const fn adfsdm1sel(self) -> AdfsdmselVal {
        let val = ((self.bits >> 0x1a) & 0x1) as _;
        unsafe { AdfsdmselVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "dedicated clocks configuration register"]
pub struct Dckcfgr2Bits {
    bits: u32,
}
impl Default for Dckcfgr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dckcfgr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "USART 1 clock source selection"]
    pub const fn set_usart1sel(mut self, val: Usart1selVal) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "USART 1 clock source selection"]
    pub const fn usart1sel(self) -> Usart1selVal {
        let val = ((self.bits >> 0x0) & 0x3) as _;
        unsafe { Usart1selVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "USART 2 clock source selection"]
    pub const fn set_usart2sel(mut self, val: Usart2selVal) -> Self {
        self.bits &= !(0x3 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "USART 2 clock source selection"]
    pub const fn usart2sel(self) -> Usart2selVal {
        let val = ((self.bits >> 0x2) & 0x3) as _;
        unsafe { Usart2selVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "USART 3 clock source selection"]
    pub const fn set_usart3sel(mut self, val: Usart2selVal) -> Self {
        self.bits &= !(0x3 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "USART 3 clock source selection"]
    pub const fn usart3sel(self) -> Usart2selVal {
        let val = ((self.bits >> 0x4) & 0x3) as _;
        unsafe { Usart2selVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "UART 4 clock source selection"]
    pub const fn set_uart4sel(mut self, val: Usart2selVal) -> Self {
        self.bits &= !(0x3 << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "UART 4 clock source selection"]
    pub const fn uart4sel(self) -> Usart2selVal {
        let val = ((self.bits >> 0x6) & 0x3) as _;
        unsafe { Usart2selVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "UART 5 clock source selection"]
    pub const fn set_uart5sel(mut self, val: Usart2selVal) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "UART 5 clock source selection"]
    pub const fn uart5sel(self) -> Usart2selVal {
        let val = ((self.bits >> 0x8) & 0x3) as _;
        unsafe { Usart2selVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "USART 6 clock source selection"]
    pub const fn set_usart6sel(mut self, val: Usart1selVal) -> Self {
        self.bits &= !(0x3 << 0xa);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "USART 6 clock source selection"]
    pub const fn usart6sel(self) -> Usart1selVal {
        let val = ((self.bits >> 0xa) & 0x3) as _;
        unsafe { Usart1selVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "UART 7 clock source selection"]
    pub const fn set_uart7sel(mut self, val: Usart2selVal) -> Self {
        self.bits &= !(0x3 << 0xc);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "UART 7 clock source selection"]
    pub const fn uart7sel(self) -> Usart2selVal {
        let val = ((self.bits >> 0xc) & 0x3) as _;
        unsafe { Usart2selVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "UART 8 clock source selection"]
    pub const fn set_uart8sel(mut self, val: Usart2selVal) -> Self {
        self.bits &= !(0x3 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "UART 8 clock source selection"]
    pub const fn uart8sel(self) -> Usart2selVal {
        let val = ((self.bits >> 0xe) & 0x3) as _;
        unsafe { Usart2selVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "I2C1 clock source selection"]
    pub const fn set_i2c1sel(mut self, val: I2cselVal) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "I2C1 clock source selection"]
    pub const fn i2c1sel(self) -> I2cselVal {
        let val = ((self.bits >> 0x10) & 0x3) as _;
        unsafe { I2cselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "I2C2 clock source selection"]
    pub const fn set_i2c2sel(mut self, val: I2cselVal) -> Self {
        self.bits &= !(0x3 << 0x12);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x12;
        self
    }
    #[inline(always)]
    #[doc = "I2C2 clock source selection"]
    pub const fn i2c2sel(self) -> I2cselVal {
        let val = ((self.bits >> 0x12) & 0x3) as _;
        unsafe { I2cselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "I2C3 clock source selection"]
    pub const fn set_i2c3sel(mut self, val: I2cselVal) -> Self {
        self.bits &= !(0x3 << 0x14);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "I2C3 clock source selection"]
    pub const fn i2c3sel(self) -> I2cselVal {
        let val = ((self.bits >> 0x14) & 0x3) as _;
        unsafe { I2cselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "I2C4 clock source selection"]
    pub const fn set_i2c4sel(mut self, val: I2cselVal) -> Self {
        self.bits &= !(0x3 << 0x16);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x16;
        self
    }
    #[inline(always)]
    #[doc = "I2C4 clock source selection"]
    pub const fn i2c4sel(self) -> I2cselVal {
        let val = ((self.bits >> 0x16) & 0x3) as _;
        unsafe { I2cselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Low power timer 1 clock source selection"]
    pub const fn set_lptim1sel(mut self, val: LptimselVal) -> Self {
        self.bits &= !(0x3 << 0x18);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Low power timer 1 clock source selection"]
    pub const fn lptim1sel(self) -> LptimselVal {
        let val = ((self.bits >> 0x18) & 0x3) as _;
        unsafe { LptimselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "HDMI-CEC clock source selection"]
    pub const fn set_cecsel(mut self, val: CecselVal) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1a;
        self
    }
    #[inline(always)]
    #[doc = "HDMI-CEC clock source selection"]
    pub const fn cecsel(self) -> CecselVal {
        let val = ((self.bits >> 0x1a) & 0x1) as _;
        unsafe { CecselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "48MHz clock source selection"]
    pub const fn set_clk48sel(mut self, val: Clk48selVal) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1b;
        self
    }
    #[inline(always)]
    #[doc = "48MHz clock source selection"]
    pub const fn clk48sel(self) -> Clk48selVal {
        let val = ((self.bits >> 0x1b) & 0x1) as _;
        unsafe { Clk48selVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "SDMMC1 clock source selection"]
    pub const fn set_sdmmc1sel(mut self, val: SdmmcselVal) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "SDMMC1 clock source selection"]
    pub const fn sdmmc1sel(self) -> SdmmcselVal {
        let val = ((self.bits >> 0x1c) & 0x1) as _;
        unsafe { SdmmcselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "SDMMC2 clock source selection"]
    pub const fn set_sdmmc2sel(mut self, val: SdmmcselVal) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1d;
        self
    }
    #[inline(always)]
    #[doc = "SDMMC2 clock source selection"]
    pub const fn sdmmc2sel(self) -> SdmmcselVal {
        let val = ((self.bits >> 0x1d) & 0x1) as _;
        unsafe { SdmmcselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "DSI clock source selection"]
    pub const fn set_dsisel(mut self, val: DsiselVal) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1e;
        self
    }
    #[inline(always)]
    #[doc = "DSI clock source selection"]
    pub const fn dsisel(self) -> DsiselVal {
        let val = ((self.bits >> 0x1e) & 0x1) as _;
        unsafe { DsiselVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "PLL configuration register"]
pub struct PllcfgrBits {
    bits: u32,
}
impl Default for PllcfgrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PllcfgrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Division factor for the PLL and audio PLL (PLLI2S) input clock"]
    pub const fn set_pllm(mut self, val: PllmVal) -> Self {
        self.bits &= !(0x3f << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x3f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Division factor for the PLL and audio PLL (PLLI2S) input clock"]
    pub const fn pllm(self) -> PllmVal {
        let val = ((self.bits >> 0x0) & 0x3f) as _;
        unsafe { PllmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL multiplication factor for VCO"]
    pub const fn set_plln(mut self, val: PllnVal) -> Self {
        self.bits &= !(0x1ff << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x1ff) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "PLL multiplication factor for VCO"]
    pub const fn plln(self) -> PllnVal {
        let val = ((self.bits >> 0x6) & 0x1ff) as _;
        unsafe { PllnVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL division factor for main system clock"]
    pub const fn set_pllp(mut self, val: PllpVal) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "PLL division factor for main system clock"]
    pub const fn pllp(self) -> PllpVal {
        let val = ((self.bits >> 0x10) & 0x3) as _;
        unsafe { PllpVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL and audio PLL (PLLI2S, PLLSAI) entry clock source"]
    pub const fn set_pllsrc(mut self, val: PllsrcVal) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x16;
        self
    }
    #[inline(always)]
    #[doc = "PLL and audio PLL (PLLI2S, PLLSAI) entry clock source"]
    pub const fn pllsrc(self) -> PllsrcVal {
        let val = ((self.bits >> 0x16) & 0x1) as _;
        unsafe { PllsrcVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
    pub const fn set_pllq(mut self, val: PllqVal) -> Self {
        self.bits &= !(0xf << 0x18);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
    pub const fn pllq(self) -> PllqVal {
        let val = ((self.bits >> 0x18) & 0xf) as _;
        unsafe { PllqVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL division factor for DSI clock"]
    pub const fn set_pllr(mut self, val: PllrVal) -> Self {
        self.bits &= !(0x7 << 0x1c);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "PLL division factor for DSI clock"]
    pub const fn pllr(self) -> PllrVal {
        let val = ((self.bits >> 0x1c) & 0x7) as _;
        unsafe { PllrVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "PLLI2S configuration register"]
pub struct Plli2scfgrBits {
    bits: u32,
}
impl Default for Plli2scfgrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Plli2scfgrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "PLL multiplication factor for VCO"]
    pub const fn set_plln(mut self, val: PllnVal) -> Self {
        self.bits &= !(0x1ff << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x1ff) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "PLL multiplication factor for VCO"]
    pub const fn plln(self) -> PllnVal {
        let val = ((self.bits >> 0x6) & 0x1ff) as _;
        unsafe { PllnVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL division factor for main system clock"]
    pub const fn set_pllp(mut self, val: PllpVal) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "PLL division factor for main system clock"]
    pub const fn pllp(self) -> PllpVal {
        let val = ((self.bits >> 0x10) & 0x3) as _;
        unsafe { PllpVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
    pub const fn set_pllq(mut self, val: PllqVal) -> Self {
        self.bits &= !(0xf << 0x18);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
    pub const fn pllq(self) -> PllqVal {
        let val = ((self.bits >> 0x18) & 0xf) as _;
        unsafe { PllqVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL division factor for DSI clock"]
    pub const fn set_pllr(mut self, val: PllrVal) -> Self {
        self.bits &= !(0x7 << 0x1c);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "PLL division factor for DSI clock"]
    pub const fn pllr(self) -> PllrVal {
        let val = ((self.bits >> 0x1c) & 0x7) as _;
        unsafe { PllrVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "PLL configuration register"]
pub struct PllsaicfgrBits {
    bits: u32,
}
impl Default for PllsaicfgrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PllsaicfgrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "PLL multiplication factor for VCO"]
    pub const fn set_plln(mut self, val: PllnVal) -> Self {
        self.bits &= !(0x1ff << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x1ff) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "PLL multiplication factor for VCO"]
    pub const fn plln(self) -> PllnVal {
        let val = ((self.bits >> 0x6) & 0x1ff) as _;
        unsafe { PllnVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL division factor for main system clock"]
    pub const fn set_pllp(mut self, val: PllpVal) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "PLL division factor for main system clock"]
    pub const fn pllp(self) -> PllpVal {
        let val = ((self.bits >> 0x10) & 0x3) as _;
        unsafe { PllpVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
    pub const fn set_pllq(mut self, val: PllqVal) -> Self {
        self.bits &= !(0xf << 0x18);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
    pub const fn pllq(self) -> PllqVal {
        let val = ((self.bits >> 0x18) & 0xf) as _;
        unsafe { PllqVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PLL division factor for DSI clock"]
    pub const fn set_pllr(mut self, val: PllrVal) -> Self {
        self.bits &= !(0x7 << 0x1c);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "PLL division factor for DSI clock"]
    pub const fn pllr(self) -> PllrVal {
        let val = ((self.bits >> 0x1c) & 0x7) as _;
        unsafe { PllrVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "spread spectrum clock generation register"]
pub struct SscgrBits {
    bits: u32,
}
impl Default for SscgrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SscgrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Modulation period"]
    pub const fn set_modper(mut self, val: u16) -> Self {
        self.bits &= !(0x1fff << 0x0);
        self.bits |= (val as u32 & 0x1fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Modulation period"]
    pub const fn modper(self) -> u16 {
        ((self.bits >> 0x0) & 0x1fff) as _
    }
    #[inline(always)]
    #[doc = "Incrementation step"]
    pub const fn set_incstep(mut self, val: u16) -> Self {
        self.bits &= !(0x7fff << 0xd);
        self.bits |= (val as u32 & 0x7fff) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Incrementation step"]
    pub const fn incstep(self) -> u16 {
        ((self.bits >> 0xd) & 0x7fff) as _
    }
    #[inline(always)]
    #[doc = "Spread Select"]
    pub const fn set_spreadsel(mut self, val: SpreadselVal) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1e;
        self
    }
    #[inline(always)]
    #[doc = "Spread Select"]
    pub const fn spreadsel(self) -> SpreadselVal {
        let val = ((self.bits >> 0x1e) & 0x1) as _;
        unsafe { SpreadselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Spread spectrum modulation enable"]
    pub const fn set_sscgen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Spread spectrum modulation enable"]
    pub const fn sscgen(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AdfsdmselVal {
    #[doc = "SAI1 clock selected as DFSDM1 Audio clock source"]
    Sai1 = 0x0,
    #[doc = "SAI2 clock selected as DFSDM1 Audio clock source"]
    Sai2 = 0x1,
}
impl AdfsdmselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CecselVal {
    #[doc = "LSE clock is selected as HDMI-CEC clock"]
    Lse = 0x0,
    #[doc = "HSI divided by 488 clock is selected as HDMI-CEC clock"]
    HsiDiv488 = 0x1,
}
impl CecselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Clk48selVal {
    #[doc = "48MHz clock from PLL is selected"]
    Pll1Q = 0x0,
    #[doc = "48MHz clock from PLLSAI is selected"]
    Pllsai1P = 0x1,
}
impl Clk48selVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DfsdmselVal {
    #[doc = "APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source"]
    Pclk2 = 0x0,
    #[doc = "System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source"]
    Sys = 0x1,
}
impl DfsdmselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DsiselVal {
    #[doc = "DSI-PHY used as DSI byte lane clock source (usual case)"]
    DsiPhy = 0x0,
    #[doc = "PLLR used as DSI byte lane clock source, used in case DSI PLL and DSI-PHY are off (low power mode)"]
    Pll1R = 0x1,
}
impl DsiselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HpreVal {
    #[doc = "SYSCLK not divided"]
    Div1 = 0x0,
    #[doc = "SYSCLK divided by 2"]
    Div2 = 0x8,
    #[doc = "SYSCLK divided by 4"]
    Div4 = 0x9,
    #[doc = "SYSCLK divided by 8"]
    Div8 = 0xa,
    #[doc = "SYSCLK divided by 16"]
    Div16 = 0xb,
    #[doc = "SYSCLK divided by 64"]
    Div64 = 0xc,
    #[doc = "SYSCLK divided by 128"]
    Div128 = 0xd,
    #[doc = "SYSCLK divided by 256"]
    Div256 = 0xe,
    #[doc = "SYSCLK divided by 512"]
    Div512 = 0xf,
}
impl HpreVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum I2cselVal {
    #[doc = "APB clock selected as I2C clock"]
    Pclk1 = 0x0,
    #[doc = "System clock selected as I2C clock"]
    Sys = 0x1,
    #[doc = "HSI clock selected as I2C clock"]
    Hsi = 0x2,
}
impl I2cselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IssrcVal {
    #[doc = "PLLI2S clock used as I2S clock source"]
    Plli2s = 0x0,
    #[doc = "External clock mapped on the I2S_CKIN pin used as I2S clock source"]
    Ckin = 0x1,
}
impl IssrcVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LptimselVal {
    #[doc = "APB1 clock (PCLK1) selected as LPTILM1 clock"]
    Pclk1 = 0x0,
    #[doc = "LSI clock is selected as LPTILM1 clock"]
    Lsi = 0x1,
    #[doc = "HSI clock is selected as LPTILM1 clock"]
    Hsi = 0x2,
    #[doc = "LSE clock is selected as LPTILM1 clock"]
    Lse = 0x3,
}
impl LptimselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LsedrvVal {
    #[doc = "Low driving capability"]
    Low = 0x0,
    #[doc = "Medium high driving capability"]
    MediumHigh = 0x1,
    #[doc = "Medium low driving capability"]
    MediumLow = 0x2,
    #[doc = "High driving capability"]
    High = 0x3,
}
impl LsedrvVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Mco1selVal {
    #[doc = "HSI clock selected"]
    Hsi = 0x0,
    #[doc = "LSE oscillator selected"]
    Lse = 0x1,
    #[doc = "HSE oscillator clock selected"]
    Hse = 0x2,
    #[doc = "PLL clock selected"]
    Pll = 0x3,
}
impl Mco1selVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Mco2selVal {
    #[doc = "System clock (SYSCLK) selected"]
    Sys = 0x0,
    #[doc = "PLLI2S clock selected"]
    Plli2s = 0x1,
    #[doc = "HSE oscillator clock selected"]
    Hse = 0x2,
    #[doc = "PLL clock selected"]
    Pll = 0x3,
}
impl Mco2selVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum McopreVal {
    #[doc = "No division"]
    Div1 = 0x0,
    #[doc = "Division by 2"]
    Div2 = 0x4,
    #[doc = "Division by 3"]
    Div3 = 0x5,
    #[doc = "Division by 4"]
    Div4 = 0x6,
    #[doc = "Division by 5"]
    Div5 = 0x7,
}
impl McopreVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Plli2sdivqVal {
    #[doc = "PLLI2SDIVQ = /1"]
    Div1 = 0x0,
    #[doc = "PLLI2SDIVQ = /2"]
    Div2 = 0x1,
    #[doc = "PLLI2SDIVQ = /3"]
    Div3 = 0x2,
    #[doc = "PLLI2SDIVQ = /4"]
    Div4 = 0x3,
    #[doc = "PLLI2SDIVQ = /5"]
    Div5 = 0x4,
    #[doc = "PLLI2SDIVQ = /6"]
    Div6 = 0x5,
    #[doc = "PLLI2SDIVQ = /7"]
    Div7 = 0x6,
    #[doc = "PLLI2SDIVQ = /8"]
    Div8 = 0x7,
    #[doc = "PLLI2SDIVQ = /9"]
    Div9 = 0x8,
    #[doc = "PLLI2SDIVQ = /10"]
    Div10 = 0x9,
    #[doc = "PLLI2SDIVQ = /11"]
    Div11 = 0xa,
    #[doc = "PLLI2SDIVQ = /12"]
    Div12 = 0xb,
    #[doc = "PLLI2SDIVQ = /13"]
    Div13 = 0xc,
    #[doc = "PLLI2SDIVQ = /14"]
    Div14 = 0xd,
    #[doc = "PLLI2SDIVQ = /15"]
    Div15 = 0xe,
    #[doc = "PLLI2SDIVQ = /16"]
    Div16 = 0xf,
    #[doc = "PLLI2SDIVQ = /17"]
    Div17 = 0x10,
    #[doc = "PLLI2SDIVQ = /18"]
    Div18 = 0x11,
    #[doc = "PLLI2SDIVQ = /19"]
    Div19 = 0x12,
    #[doc = "PLLI2SDIVQ = /20"]
    Div20 = 0x13,
    #[doc = "PLLI2SDIVQ = /21"]
    Div21 = 0x14,
    #[doc = "PLLI2SDIVQ = /22"]
    Div22 = 0x15,
    #[doc = "PLLI2SDIVQ = /23"]
    Div23 = 0x16,
    #[doc = "PLLI2SDIVQ = /24"]
    Div24 = 0x17,
    #[doc = "PLLI2SDIVQ = /25"]
    Div25 = 0x18,
    #[doc = "PLLI2SDIVQ = /26"]
    Div26 = 0x19,
    #[doc = "PLLI2SDIVQ = /27"]
    Div27 = 0x1a,
    #[doc = "PLLI2SDIVQ = /28"]
    Div28 = 0x1b,
    #[doc = "PLLI2SDIVQ = /29"]
    Div29 = 0x1c,
    #[doc = "PLLI2SDIVQ = /30"]
    Div30 = 0x1d,
    #[doc = "PLLI2SDIVQ = /31"]
    Div31 = 0x1e,
    #[doc = "PLLI2SDIVQ = /32"]
    Div32 = 0x1f,
}
impl Plli2sdivqVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PllmVal {
    Div2 = 0x2,

    Div3 = 0x3,

    Div4 = 0x4,

    Div5 = 0x5,

    Div6 = 0x6,

    Div7 = 0x7,

    Div8 = 0x8,

    Div9 = 0x9,

    Div10 = 0xa,

    Div11 = 0xb,

    Div12 = 0xc,

    Div13 = 0xd,

    Div14 = 0xe,

    Div15 = 0xf,

    Div16 = 0x10,

    Div17 = 0x11,

    Div18 = 0x12,

    Div19 = 0x13,

    Div20 = 0x14,

    Div21 = 0x15,

    Div22 = 0x16,

    Div23 = 0x17,

    Div24 = 0x18,

    Div25 = 0x19,

    Div26 = 0x1a,

    Div27 = 0x1b,

    Div28 = 0x1c,

    Div29 = 0x1d,

    Div30 = 0x1e,

    Div31 = 0x1f,

    Div32 = 0x20,

    Div33 = 0x21,

    Div34 = 0x22,

    Div35 = 0x23,

    Div36 = 0x24,

    Div37 = 0x25,

    Div38 = 0x26,

    Div39 = 0x27,

    Div40 = 0x28,

    Div41 = 0x29,

    Div42 = 0x2a,

    Div43 = 0x2b,

    Div44 = 0x2c,

    Div45 = 0x2d,

    Div46 = 0x2e,

    Div47 = 0x2f,

    Div48 = 0x30,

    Div49 = 0x31,

    Div50 = 0x32,

    Div51 = 0x33,

    Div52 = 0x34,

    Div53 = 0x35,

    Div54 = 0x36,

    Div55 = 0x37,

    Div56 = 0x38,

    Div57 = 0x39,

    Div58 = 0x3a,

    Div59 = 0x3b,

    Div60 = 0x3c,

    Div61 = 0x3d,

    Div62 = 0x3e,

    Div63 = 0x3f,
}
impl PllmVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum PllnVal {
    Mul50 = 0x32,

    Mul51 = 0x33,

    Mul52 = 0x34,

    Mul53 = 0x35,

    Mul54 = 0x36,

    Mul55 = 0x37,

    Mul56 = 0x38,

    Mul57 = 0x39,

    Mul58 = 0x3a,

    Mul59 = 0x3b,

    Mul60 = 0x3c,

    Mul61 = 0x3d,

    Mul62 = 0x3e,

    Mul63 = 0x3f,

    Mul64 = 0x40,

    Mul65 = 0x41,

    Mul66 = 0x42,

    Mul67 = 0x43,

    Mul68 = 0x44,

    Mul69 = 0x45,

    Mul70 = 0x46,

    Mul71 = 0x47,

    Mul72 = 0x48,

    Mul73 = 0x49,

    Mul74 = 0x4a,

    Mul75 = 0x4b,

    Mul76 = 0x4c,

    Mul77 = 0x4d,

    Mul78 = 0x4e,

    Mul79 = 0x4f,

    Mul80 = 0x50,

    Mul81 = 0x51,

    Mul82 = 0x52,

    Mul83 = 0x53,

    Mul84 = 0x54,

    Mul85 = 0x55,

    Mul86 = 0x56,

    Mul87 = 0x57,

    Mul88 = 0x58,

    Mul89 = 0x59,

    Mul90 = 0x5a,

    Mul91 = 0x5b,

    Mul92 = 0x5c,

    Mul93 = 0x5d,

    Mul94 = 0x5e,

    Mul95 = 0x5f,

    Mul96 = 0x60,

    Mul97 = 0x61,

    Mul98 = 0x62,

    Mul99 = 0x63,

    Mul100 = 0x64,

    Mul101 = 0x65,

    Mul102 = 0x66,

    Mul103 = 0x67,

    Mul104 = 0x68,

    Mul105 = 0x69,

    Mul106 = 0x6a,

    Mul107 = 0x6b,

    Mul108 = 0x6c,

    Mul109 = 0x6d,

    Mul110 = 0x6e,

    Mul111 = 0x6f,

    Mul112 = 0x70,

    Mul113 = 0x71,

    Mul114 = 0x72,

    Mul115 = 0x73,

    Mul116 = 0x74,

    Mul117 = 0x75,

    Mul118 = 0x76,

    Mul119 = 0x77,

    Mul120 = 0x78,

    Mul121 = 0x79,

    Mul122 = 0x7a,

    Mul123 = 0x7b,

    Mul124 = 0x7c,

    Mul125 = 0x7d,

    Mul126 = 0x7e,

    Mul127 = 0x7f,

    Mul128 = 0x80,

    Mul129 = 0x81,

    Mul130 = 0x82,

    Mul131 = 0x83,

    Mul132 = 0x84,

    Mul133 = 0x85,

    Mul134 = 0x86,

    Mul135 = 0x87,

    Mul136 = 0x88,

    Mul137 = 0x89,

    Mul138 = 0x8a,

    Mul139 = 0x8b,

    Mul140 = 0x8c,

    Mul141 = 0x8d,

    Mul142 = 0x8e,

    Mul143 = 0x8f,

    Mul144 = 0x90,

    Mul145 = 0x91,

    Mul146 = 0x92,

    Mul147 = 0x93,

    Mul148 = 0x94,

    Mul149 = 0x95,

    Mul150 = 0x96,

    Mul151 = 0x97,

    Mul152 = 0x98,

    Mul153 = 0x99,

    Mul154 = 0x9a,

    Mul155 = 0x9b,

    Mul156 = 0x9c,

    Mul157 = 0x9d,

    Mul158 = 0x9e,

    Mul159 = 0x9f,

    Mul160 = 0xa0,

    Mul161 = 0xa1,

    Mul162 = 0xa2,

    Mul163 = 0xa3,

    Mul164 = 0xa4,

    Mul165 = 0xa5,

    Mul166 = 0xa6,

    Mul167 = 0xa7,

    Mul168 = 0xa8,

    Mul169 = 0xa9,

    Mul170 = 0xaa,

    Mul171 = 0xab,

    Mul172 = 0xac,

    Mul173 = 0xad,

    Mul174 = 0xae,

    Mul175 = 0xaf,

    Mul176 = 0xb0,

    Mul177 = 0xb1,

    Mul178 = 0xb2,

    Mul179 = 0xb3,

    Mul180 = 0xb4,

    Mul181 = 0xb5,

    Mul182 = 0xb6,

    Mul183 = 0xb7,

    Mul184 = 0xb8,

    Mul185 = 0xb9,

    Mul186 = 0xba,

    Mul187 = 0xbb,

    Mul188 = 0xbc,

    Mul189 = 0xbd,

    Mul190 = 0xbe,

    Mul191 = 0xbf,

    Mul192 = 0xc0,

    Mul193 = 0xc1,

    Mul194 = 0xc2,

    Mul195 = 0xc3,

    Mul196 = 0xc4,

    Mul197 = 0xc5,

    Mul198 = 0xc6,

    Mul199 = 0xc7,

    Mul200 = 0xc8,

    Mul201 = 0xc9,

    Mul202 = 0xca,

    Mul203 = 0xcb,

    Mul204 = 0xcc,

    Mul205 = 0xcd,

    Mul206 = 0xce,

    Mul207 = 0xcf,

    Mul208 = 0xd0,

    Mul209 = 0xd1,

    Mul210 = 0xd2,

    Mul211 = 0xd3,

    Mul212 = 0xd4,

    Mul213 = 0xd5,

    Mul214 = 0xd6,

    Mul215 = 0xd7,

    Mul216 = 0xd8,

    Mul217 = 0xd9,

    Mul218 = 0xda,

    Mul219 = 0xdb,

    Mul220 = 0xdc,

    Mul221 = 0xdd,

    Mul222 = 0xde,

    Mul223 = 0xdf,

    Mul224 = 0xe0,

    Mul225 = 0xe1,

    Mul226 = 0xe2,

    Mul227 = 0xe3,

    Mul228 = 0xe4,

    Mul229 = 0xe5,

    Mul230 = 0xe6,

    Mul231 = 0xe7,

    Mul232 = 0xe8,

    Mul233 = 0xe9,

    Mul234 = 0xea,

    Mul235 = 0xeb,

    Mul236 = 0xec,

    Mul237 = 0xed,

    Mul238 = 0xee,

    Mul239 = 0xef,

    Mul240 = 0xf0,

    Mul241 = 0xf1,

    Mul242 = 0xf2,

    Mul243 = 0xf3,

    Mul244 = 0xf4,

    Mul245 = 0xf5,

    Mul246 = 0xf6,

    Mul247 = 0xf7,

    Mul248 = 0xf8,

    Mul249 = 0xf9,

    Mul250 = 0xfa,

    Mul251 = 0xfb,

    Mul252 = 0xfc,

    Mul253 = 0xfd,

    Mul254 = 0xfe,

    Mul255 = 0xff,

    Mul256 = 0x100,

    Mul257 = 0x101,

    Mul258 = 0x102,

    Mul259 = 0x103,

    Mul260 = 0x104,

    Mul261 = 0x105,

    Mul262 = 0x106,

    Mul263 = 0x107,

    Mul264 = 0x108,

    Mul265 = 0x109,

    Mul266 = 0x10a,

    Mul267 = 0x10b,

    Mul268 = 0x10c,

    Mul269 = 0x10d,

    Mul270 = 0x10e,

    Mul271 = 0x10f,

    Mul272 = 0x110,

    Mul273 = 0x111,

    Mul274 = 0x112,

    Mul275 = 0x113,

    Mul276 = 0x114,

    Mul277 = 0x115,

    Mul278 = 0x116,

    Mul279 = 0x117,

    Mul280 = 0x118,

    Mul281 = 0x119,

    Mul282 = 0x11a,

    Mul283 = 0x11b,

    Mul284 = 0x11c,

    Mul285 = 0x11d,

    Mul286 = 0x11e,

    Mul287 = 0x11f,

    Mul288 = 0x120,

    Mul289 = 0x121,

    Mul290 = 0x122,

    Mul291 = 0x123,

    Mul292 = 0x124,

    Mul293 = 0x125,

    Mul294 = 0x126,

    Mul295 = 0x127,

    Mul296 = 0x128,

    Mul297 = 0x129,

    Mul298 = 0x12a,

    Mul299 = 0x12b,

    Mul300 = 0x12c,

    Mul301 = 0x12d,

    Mul302 = 0x12e,

    Mul303 = 0x12f,

    Mul304 = 0x130,

    Mul305 = 0x131,

    Mul306 = 0x132,

    Mul307 = 0x133,

    Mul308 = 0x134,

    Mul309 = 0x135,

    Mul310 = 0x136,

    Mul311 = 0x137,

    Mul312 = 0x138,

    Mul313 = 0x139,

    Mul314 = 0x13a,

    Mul315 = 0x13b,

    Mul316 = 0x13c,

    Mul317 = 0x13d,

    Mul318 = 0x13e,

    Mul319 = 0x13f,

    Mul320 = 0x140,

    Mul321 = 0x141,

    Mul322 = 0x142,

    Mul323 = 0x143,

    Mul324 = 0x144,

    Mul325 = 0x145,

    Mul326 = 0x146,

    Mul327 = 0x147,

    Mul328 = 0x148,

    Mul329 = 0x149,

    Mul330 = 0x14a,

    Mul331 = 0x14b,

    Mul332 = 0x14c,

    Mul333 = 0x14d,

    Mul334 = 0x14e,

    Mul335 = 0x14f,

    Mul336 = 0x150,

    Mul337 = 0x151,

    Mul338 = 0x152,

    Mul339 = 0x153,

    Mul340 = 0x154,

    Mul341 = 0x155,

    Mul342 = 0x156,

    Mul343 = 0x157,

    Mul344 = 0x158,

    Mul345 = 0x159,

    Mul346 = 0x15a,

    Mul347 = 0x15b,

    Mul348 = 0x15c,

    Mul349 = 0x15d,

    Mul350 = 0x15e,

    Mul351 = 0x15f,

    Mul352 = 0x160,

    Mul353 = 0x161,

    Mul354 = 0x162,

    Mul355 = 0x163,

    Mul356 = 0x164,

    Mul357 = 0x165,

    Mul358 = 0x166,

    Mul359 = 0x167,

    Mul360 = 0x168,

    Mul361 = 0x169,

    Mul362 = 0x16a,

    Mul363 = 0x16b,

    Mul364 = 0x16c,

    Mul365 = 0x16d,

    Mul366 = 0x16e,

    Mul367 = 0x16f,

    Mul368 = 0x170,

    Mul369 = 0x171,

    Mul370 = 0x172,

    Mul371 = 0x173,

    Mul372 = 0x174,

    Mul373 = 0x175,

    Mul374 = 0x176,

    Mul375 = 0x177,

    Mul376 = 0x178,

    Mul377 = 0x179,

    Mul378 = 0x17a,

    Mul379 = 0x17b,

    Mul380 = 0x17c,

    Mul381 = 0x17d,

    Mul382 = 0x17e,

    Mul383 = 0x17f,

    Mul384 = 0x180,

    Mul385 = 0x181,

    Mul386 = 0x182,

    Mul387 = 0x183,

    Mul388 = 0x184,

    Mul389 = 0x185,

    Mul390 = 0x186,

    Mul391 = 0x187,

    Mul392 = 0x188,

    Mul393 = 0x189,

    Mul394 = 0x18a,

    Mul395 = 0x18b,

    Mul396 = 0x18c,

    Mul397 = 0x18d,

    Mul398 = 0x18e,

    Mul399 = 0x18f,

    Mul400 = 0x190,

    Mul401 = 0x191,

    Mul402 = 0x192,

    Mul403 = 0x193,

    Mul404 = 0x194,

    Mul405 = 0x195,

    Mul406 = 0x196,

    Mul407 = 0x197,

    Mul408 = 0x198,

    Mul409 = 0x199,

    Mul410 = 0x19a,

    Mul411 = 0x19b,

    Mul412 = 0x19c,

    Mul413 = 0x19d,

    Mul414 = 0x19e,

    Mul415 = 0x19f,

    Mul416 = 0x1a0,

    Mul417 = 0x1a1,

    Mul418 = 0x1a2,

    Mul419 = 0x1a3,

    Mul420 = 0x1a4,

    Mul421 = 0x1a5,

    Mul422 = 0x1a6,

    Mul423 = 0x1a7,

    Mul424 = 0x1a8,

    Mul425 = 0x1a9,

    Mul426 = 0x1aa,

    Mul427 = 0x1ab,

    Mul428 = 0x1ac,

    Mul429 = 0x1ad,

    Mul430 = 0x1ae,

    Mul431 = 0x1af,

    Mul432 = 0x1b0,
}
impl PllnVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u16) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u16 {
        self as u16
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PllpVal {
    #[doc = "PLLP=2"]
    Div2 = 0x0,
    #[doc = "PLLP=4"]
    Div4 = 0x1,
    #[doc = "PLLP=6"]
    Div6 = 0x2,
    #[doc = "PLLP=8"]
    Div8 = 0x3,
}
impl PllpVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PllqVal {
    Div2 = 0x2,

    Div3 = 0x3,

    Div4 = 0x4,

    Div5 = 0x5,

    Div6 = 0x6,

    Div7 = 0x7,

    Div8 = 0x8,

    Div9 = 0x9,

    Div10 = 0xa,

    Div11 = 0xb,

    Div12 = 0xc,

    Div13 = 0xd,

    Div14 = 0xe,

    Div15 = 0xf,
}
impl PllqVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PllrVal {
    Div2 = 0x2,

    Div3 = 0x3,

    Div4 = 0x4,

    Div5 = 0x5,

    Div6 = 0x6,

    Div7 = 0x7,
}
impl PllrVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PllsaidivqVal {
    #[doc = "PLLSAIDIVQ = /1"]
    Div1 = 0x0,
    #[doc = "PLLSAIDIVQ = /2"]
    Div2 = 0x1,
    #[doc = "PLLSAIDIVQ = /3"]
    Div3 = 0x2,
    #[doc = "PLLSAIDIVQ = /4"]
    Div4 = 0x3,
    #[doc = "PLLSAIDIVQ = /5"]
    Div5 = 0x4,
    #[doc = "PLLSAIDIVQ = /6"]
    Div6 = 0x5,
    #[doc = "PLLSAIDIVQ = /7"]
    Div7 = 0x6,
    #[doc = "PLLSAIDIVQ = /8"]
    Div8 = 0x7,
    #[doc = "PLLSAIDIVQ = /9"]
    Div9 = 0x8,
    #[doc = "PLLSAIDIVQ = /10"]
    Div10 = 0x9,
    #[doc = "PLLSAIDIVQ = /11"]
    Div11 = 0xa,
    #[doc = "PLLSAIDIVQ = /12"]
    Div12 = 0xb,
    #[doc = "PLLSAIDIVQ = /13"]
    Div13 = 0xc,
    #[doc = "PLLSAIDIVQ = /14"]
    Div14 = 0xd,
    #[doc = "PLLSAIDIVQ = /15"]
    Div15 = 0xe,
    #[doc = "PLLSAIDIVQ = /16"]
    Div16 = 0xf,
    #[doc = "PLLSAIDIVQ = /17"]
    Div17 = 0x10,
    #[doc = "PLLSAIDIVQ = /18"]
    Div18 = 0x11,
    #[doc = "PLLSAIDIVQ = /19"]
    Div19 = 0x12,
    #[doc = "PLLSAIDIVQ = /20"]
    Div20 = 0x13,
    #[doc = "PLLSAIDIVQ = /21"]
    Div21 = 0x14,
    #[doc = "PLLSAIDIVQ = /22"]
    Div22 = 0x15,
    #[doc = "PLLSAIDIVQ = /23"]
    Div23 = 0x16,
    #[doc = "PLLSAIDIVQ = /24"]
    Div24 = 0x17,
    #[doc = "PLLSAIDIVQ = /25"]
    Div25 = 0x18,
    #[doc = "PLLSAIDIVQ = /26"]
    Div26 = 0x19,
    #[doc = "PLLSAIDIVQ = /27"]
    Div27 = 0x1a,
    #[doc = "PLLSAIDIVQ = /28"]
    Div28 = 0x1b,
    #[doc = "PLLSAIDIVQ = /29"]
    Div29 = 0x1c,
    #[doc = "PLLSAIDIVQ = /30"]
    Div30 = 0x1d,
    #[doc = "PLLSAIDIVQ = /31"]
    Div31 = 0x1e,
    #[doc = "PLLSAIDIVQ = /32"]
    Div32 = 0x1f,
}
impl PllsaidivqVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PllsaidivrVal {
    #[doc = "PLLSAIDIVR = /2"]
    Div2 = 0x0,
    #[doc = "PLLSAIDIVR = /4"]
    Div4 = 0x1,
    #[doc = "PLLSAIDIVR = /8"]
    Div8 = 0x2,
    #[doc = "PLLSAIDIVR = /16"]
    Div16 = 0x3,
}
impl PllsaidivrVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PllsrcVal {
    #[doc = "HSI clock selected as PLL and PLLI2S clock entry"]
    Hsi = 0x0,
    #[doc = "HSE oscillator clock selected as PLL and PLLI2S clock entry"]
    Hse = 0x1,
}
impl PllsrcVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PpreVal {
    #[doc = "HCLK not divided"]
    Div1 = 0x0,
    #[doc = "HCLK divided by 2"]
    Div2 = 0x4,
    #[doc = "HCLK divided by 4"]
    Div4 = 0x5,
    #[doc = "HCLK divided by 8"]
    Div8 = 0x6,
    #[doc = "HCLK divided by 16"]
    Div16 = 0x7,
}
impl PpreVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RtcselVal {
    #[doc = "No clock"]
    Disable = 0x0,
    #[doc = "LSE oscillator clock used as RTC clock"]
    Lse = 0x1,
    #[doc = "LSI oscillator clock used as RTC clock"]
    Lsi = 0x2,
    #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
    Hse = 0x3,
}
impl RtcselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SaiselVal {
    #[doc = "SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    Pllsai1Q = 0x0,
    #[doc = "SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    Plli2s1Q = 0x1,
    #[doc = "SAI2 clock frequency = Alternate function input frequency"]
    Afif = 0x2,
    #[doc = "SAI2 clock frequency = HSI or HSE"]
    HsiHse = 0x3,
}
impl SaiselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SdmmcselVal {
    #[doc = "48 MHz clock is selected as SD clock"]
    Clk48 = 0x0,
    #[doc = "System clock is selected as SD clock"]
    Sys = 0x1,
}
impl SdmmcselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SpreadselVal {
    #[doc = "Center spread"]
    Center = 0x0,
    #[doc = "Down spread"]
    Down = 0x1,
}
impl SpreadselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SwVal {
    #[doc = "HSI oscillator used as system clock"]
    Hsi = 0x0,
    #[doc = "HSE oscillator used as system clock"]
    Hse = 0x1,
    #[doc = "PLL used as system clock"]
    Pll1P = 0x2,
}
impl SwVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimpreVal {
    #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    Mul2 = 0x0,
    #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    Mul4 = 0x1,
}
impl TimpreVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Usart1selVal {
    #[doc = "APB2 clock (PCLK2) is selected as USART clock"]
    Pclk2 = 0x0,
    #[doc = "System clock is selected as USART clock"]
    Sys = 0x1,
    #[doc = "HSI clock is selected as USART clock"]
    Hsi = 0x2,
    #[doc = "LSE clock is selected as USART clock"]
    Lse = 0x3,
}
impl Usart1selVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Usart2selVal {
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    Pclk1 = 0x0,
    #[doc = "System clock is selected as USART clock"]
    Sys = 0x1,
    #[doc = "HSI clock is selected as USART clock"]
    Hsi = 0x2,
    #[doc = "LSE clock is selected as USART clock"]
    Lse = 0x3,
}
impl Usart2selVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
