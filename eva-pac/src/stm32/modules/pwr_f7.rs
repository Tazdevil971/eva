
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
    #[inline(always)]
    #[doc = "power control register"]
    pub const fn cr2(&self) -> utils::Reg<Cr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<Cr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "power control/status register"]
    pub const fn csr2(&self) -> utils::Reg<Csr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<Csr2Bits, utils::RW>>::from_ptr(ptr)
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
    #[doc = "Low-power regulator in deepsleep under-drive mode"]
    pub const fn set_lpuds(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-power regulator in deepsleep under-drive mode"]
    pub const fn lpuds(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Main regulator in deepsleep under-drive mode"]
    pub const fn set_mruds(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Main regulator in deepsleep under-drive mode"]
    pub const fn mruds(self) -> bool {
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
    #[doc = "Over-drive enable"]
    pub const fn set_oden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Over-drive enable"]
    pub const fn oden(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Over-drive switching enabled"]
    pub const fn set_odswen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Over-drive switching enabled"]
    pub const fn odswen(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Under-drive enable in stop mode"]
    pub const fn set_uden(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x12);
        self.bits |= (val as u32 & 0x3) << 0x12;
        self
    }
    #[inline(always)]
    #[doc = "Under-drive enable in stop mode"]
    pub const fn uden(self) -> u8 {
        ((self.bits >> 0x12) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "power control register"]
pub struct Cr2Bits {
    bits: u32,
}
impl Default for Cr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clear Wakeup Pin flag for PA0"]
    pub const fn set_cwupf(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 6);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear Wakeup Pin flag for PA0"]
    pub const fn cwupf(self, idx: usize) -> bool {
        assert!(idx < 6);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wakeup pin polarity bit for PA0"]
    pub const fn set_wupp(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 6);
        self.bits &= !(0x1 << (0x8 + idx * 0x1));
        self.bits |= if val { 1 << (0x8 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wakeup pin polarity bit for PA0"]
    pub const fn wupp(self, idx: usize) -> bool {
        assert!(idx < 6);
        ((self.bits >> (0x8 + idx * 0x1)) & 0x1) != 0
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
    #[doc = "Wakeup internal flag"]
    pub const fn set_wuif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wakeup internal flag"]
    pub const fn wuif(self) -> bool {
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
    #[doc = "Enable internal wakeup"]
    pub const fn set_eiwup(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable internal wakeup"]
    pub const fn eiwup(self) -> bool {
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
    #[doc = "Regulator voltage scaling output selection ready bit"]
    pub const fn set_vosrdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Regulator voltage scaling output selection ready bit"]
    pub const fn vosrdy(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Over-drive mode ready"]
    pub const fn set_odrdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Over-drive mode ready"]
    pub const fn odrdy(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Over-drive mode switching ready"]
    pub const fn set_odswrdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Over-drive mode switching ready"]
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
#[repr(transparent)]
#[doc = "power control/status register"]
pub struct Csr2Bits {
    bits: u32,
}
impl Default for Csr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Csr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Wakeup Pin flag for PA0"]
    pub const fn set_wupf(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 6);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wakeup Pin flag for PA0"]
    pub const fn wupf(self, idx: usize) -> bool {
        assert!(idx < 6);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enable Wakeup pin for PA0"]
    pub const fn set_ewup(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 6);
        self.bits &= !(0x1 << (0x8 + idx * 0x1));
        self.bits |= if val { 1 << (0x8 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable Wakeup pin for PA0"]
    pub const fn ewup(self, idx: usize) -> bool {
        assert!(idx < 6);
        ((self.bits >> (0x8 + idx * 0x1)) & 0x1) != 0
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
    #[doc = "Scale 3 mode"]
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
