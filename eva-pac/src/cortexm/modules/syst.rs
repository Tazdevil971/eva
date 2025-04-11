#![doc = "System timer"]
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "System timer"]
pub struct Syst {
    ptr: *mut u8,
}
impl Syst {
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
    #[doc = "SysTick control and status register"]
    pub const fn csr(&self) -> utils::Reg<CsrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CsrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SysTick reload value register"]
    pub const fn rvr(&self) -> utils::Reg<RvrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<RvrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SysTick current value register"]
    pub const fn cvr(&self) -> utils::Reg<CvrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<CvrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SysTick calibration value register"]
    pub const fn calib(&self) -> utils::Reg<CalibBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<CalibBits, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "SysTick control and status register"]
pub struct CalibBits {
    bits: u32,
}
impl Default for CalibBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CalibBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Reload value for 10ms (100Hz) timing"]
    pub const fn set_tenms(mut self, val: u32) -> Self {
        self.bits &= !(0xffffff << 0x0);
        self.bits |= (val as u32 & 0xffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Reload value for 10ms (100Hz) timing"]
    pub const fn tenms(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffff) as _
    }
    #[inline(always)]
    #[doc = "Indicates whether the TENMS value is exact"]
    pub const fn set_skew(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Indicates whether the TENMS value is exact"]
    pub const fn skew(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Indicates whether the device provides a reference clock to the processor"]
    pub const fn set_noref(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Indicates whether the device provides a reference clock to the processor"]
    pub const fn noref(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "SysTick control and status register"]
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
    #[doc = "Enable the counter"]
    pub const fn set_enable(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable the counter"]
    pub const fn enable(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enables SysTick exception request"]
    pub const fn set_tickint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enables SysTick exception request"]
    pub const fn tickint(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Indicates the clock source"]
    pub const fn set_clksource(mut self, val: ClksourceVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Indicates the clock source"]
    pub const fn clksource(self) -> ClksourceVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { ClksourceVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "1 if timer counted to 0 since last time"]
    pub const fn set_countflag(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "1 if timer counted to 0 since last time"]
    pub const fn countflag(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "SysTick control and status register"]
pub struct CvrBits {
    bits: u32,
}
impl Default for CvrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CvrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Current value of the SysTick counter"]
    pub const fn set_current(mut self, val: u32) -> Self {
        self.bits &= !(0xffffff << 0x0);
        self.bits |= (val as u32 & 0xffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Current value of the SysTick counter"]
    pub const fn current(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "SysTick control and status register"]
pub struct RvrBits {
    bits: u32,
}
impl Default for RvrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl RvrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Value to load into the SYST_CVR"]
    pub const fn set_reload(mut self, val: u32) -> Self {
        self.bits &= !(0xffffff << 0x0);
        self.bits |= (val as u32 & 0xffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Value to load into the SYST_CVR"]
    pub const fn reload(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Indicates the clock source"]
#[repr(u8)]
pub enum ClksourceVal {
    #[doc = "External clock"]
    External = 0x0,
    #[doc = "Processor clock"]
    Processor = 0x1,
}
impl ClksourceVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
