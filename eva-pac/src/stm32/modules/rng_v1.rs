
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Random number generator"]
pub struct Rng {
    ptr: *mut u8,
}
impl Rng {
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
    #[doc = "control register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<SrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data register"]
    pub const fn dr(&self) -> utils::Reg<u32, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<u32, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register"]
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
    #[doc = "Random number generator enable"]
    pub const fn set_rngen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Random number generator enable"]
    pub const fn rngen(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Interrupt enable"]
    pub const fn set_ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt enable"]
    pub const fn ie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock error detection"]
    pub const fn set_ced(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock error detection"]
    pub const fn ced(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "status register"]
pub struct SrBits {
    bits: u32,
}
impl Default for SrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Data ready"]
    pub const fn set_drdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data ready"]
    pub const fn drdy(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock error current status"]
    pub const fn set_cecs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock error current status"]
    pub const fn cecs(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Seed error current status"]
    pub const fn set_secs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Seed error current status"]
    pub const fn secs(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock error interrupt status"]
    pub const fn set_ceis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock error interrupt status"]
    pub const fn ceis(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Seed error interrupt status"]
    pub const fn set_seis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Seed error interrupt status"]
    pub const fn seis(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
