
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Power control"]
pub struct Pwr {
    ptr: *mut u8,
}
impl Pwr {
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
    #[doc = "power control register"]
    pub const fn cr1(&self) -> utils::Reg<Cr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "power control/status register"]
    pub const fn csr1(&self) -> utils::Reg<Csr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Csr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "power control register"]
pub struct Cr1Bits {
    bits: u32,
}
impl Default for Cr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Low-power deep sleep"]
    pub const fn set_lpds(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-power deep sleep"]
    pub const fn lpds(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Power down deepsleep"]
    pub const fn set_pdds(mut self, val: PddsVal) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Power down deepsleep"]
    pub const fn pdds(self) -> PddsVal {
        let val = ((self.bits >> 0x1) & 0x1) as _;
        unsafe { PddsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clear wakeup flag"]
    pub const fn set_cwuf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear wakeup flag"]
    pub const fn cwuf(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear standby flag"]
    pub const fn set_csbf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear standby flag"]
    pub const fn csbf(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Power voltage detector enable"]
    pub const fn set_pvde(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Power voltage detector enable"]
    pub const fn pvde(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PVD level selection"]
    pub const fn set_pls(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x5);
        self.bits |= (val as u32 & 0x7) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "PVD level selection"]
    pub const fn pls(self) -> u8 {
        ((self.bits >> 0x5) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Disable backup domain write protection"]
    pub const fn set_dbp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Disable backup domain write protection"]
    pub const fn dbp(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Flash power down in Stop mode"]
    pub const fn set_fpds(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Flash power down in Stop mode"]
    pub const fn fpds(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-Power Regulator Low Voltage in deepsleep"]
    pub const fn set_lplvds(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Regulator Low Voltage in deepsleep"]
    pub const fn lplvds(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Main regulator low voltage in deepsleep mode"]
    pub const fn set_mrlvds(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Main regulator low voltage in deepsleep mode"]
    pub const fn mrlvds(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADCDC1"]
    pub const fn set_adcdc1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADCDC1"]
    pub const fn adcdc1(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Regulator voltage scaling output selection"]
    pub const fn set_vos(mut self, val: VosVal) -> Self {
        self.bits &= !(0x3 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Regulator voltage scaling output selection"]
    pub const fn vos(self) -> VosVal {
        let val = ((self.bits >> 0xe) & 0x3) as _;
        unsafe { VosVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Over-drive enable (STM32F4[23] ONLY)"]
    pub const fn set_oden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Over-drive enable (STM32F4[23] ONLY)"]
    pub const fn oden(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Over-drive switching enabled (STM32F4[23] ONLY)"]
    pub const fn set_odswen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Over-drive switching enabled (STM32F4[23] ONLY)"]
    pub const fn odswen(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Under-drive enable in stop mode (STM32F4[23] ONLY)"]
    pub const fn set_uden(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x12);
        self.bits |= (val as u32 & 0x3) << 0x12;
        self
    }
    #[inline(always)]
    #[doc = "Under-drive enable in stop mode (STM32F4[23] ONLY)"]
    pub const fn uden(self) -> u8 {
        ((self.bits >> 0x12) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Flash Memory Stop while System Run"]
    pub const fn set_fmssr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Flash Memory Stop while System Run"]
    pub const fn fmssr(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Flash Interface Stop while System Run"]
    pub const fn set_fissr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Flash Interface Stop while System Run"]
    pub const fn fissr(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "power control/status register"]
pub struct Csr1Bits {
    bits: u32,
}
impl Default for Csr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Csr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Wakeup flag"]
    pub const fn set_wuf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wakeup flag"]
    pub const fn wuf(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Standby flag"]
    pub const fn set_sbf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Standby flag"]
    pub const fn sbf(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PVD output"]
    pub const fn set_pvdo(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PVD output"]
    pub const fn pvdo(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Backup regulator ready"]
    pub const fn set_brr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Backup regulator ready"]
    pub const fn brr(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enable WKUP2 pin"]
    pub const fn set_ewup2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable WKUP2 pin"]
    pub const fn ewup2(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enable WKUP pin"]
    pub const fn set_ewup(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable WKUP pin"]
    pub const fn ewup(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Backup regulator enable"]
    pub const fn set_bre(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Backup regulator enable"]
    pub const fn bre(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Regulator voltage scaling output selection ready bit (STM32F4[23] ONLY)"]
    pub const fn set_vosrdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Regulator voltage scaling output selection ready bit (STM32F4[23] ONLY)"]
    pub const fn vosrdy(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Over-drive mode ready (STM32F4[23] ONLY)"]
    pub const fn set_odrdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Over-drive mode ready (STM32F4[23] ONLY)"]
    pub const fn odrdy(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Over-drive mode switching ready (STM32F4[23] ONLY)"]
    pub const fn set_odswrdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Over-drive mode switching ready (STM32F4[23] ONLY)"]
    pub const fn odswrdy(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Under-drive ready flag"]
    pub const fn set_udrdy(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x12);
        self.bits |= (val as u32 & 0x3) << 0x12;
        self
    }
    #[inline(always)]
    #[doc = "Under-drive ready flag"]
    pub const fn udrdy(self) -> u8 {
        ((self.bits >> 0x12) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PddsVal {
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    StopMode = 0x0,
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    StandbyMode = 0x1,
}
impl PddsVal {
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
pub enum VosVal {
    #[doc = "Scale 3 mode (STM32F4[23] ONLY)"]
    Scale3 = 0x1,
    #[doc = "Scale 2 mode"]
    Scale2 = 0x2,
    #[doc = "Scale 1 mode (reset value)"]
    Scale1 = 0x3,
}
impl VosVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
