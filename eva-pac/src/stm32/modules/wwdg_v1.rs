
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Window watchdog"]
pub struct Wwdg {
    ptr: *mut u8,
}
impl Wwdg {
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
    #[doc = "Control register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Configuration register"]
    pub const fn cfr(&self) -> utils::Reg<CfrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<CfrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<SrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Configuration register"]
pub struct CfrBits {
    bits: u32,
}
impl Default for CfrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CfrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "7-bit window value"]
    pub const fn set_w(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x0);
        self.bits |= (val as u32 & 0x7f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "7-bit window value"]
    pub const fn w(self) -> u8 {
        ((self.bits >> 0x0) & 0x7f) as _
    }
    #[inline(always)]
    #[doc = "Timer base"]
    pub const fn set_wdgtb(mut self, val: WdgtbVal) -> Self {
        self.bits &= !(0x3 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "Timer base"]
    pub const fn wdgtb(self) -> WdgtbVal {
        let val = ((self.bits >> 0x7) & 0x3) as _;
        unsafe { WdgtbVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Early wakeup interrupt"]
    pub const fn set_ewi(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Early wakeup interrupt"]
    pub const fn ewi(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Control register"]
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
    #[doc = "7-bit counter (MSB to LSB)"]
    pub const fn set_t(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x0);
        self.bits |= (val as u32 & 0x7f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "7-bit counter (MSB to LSB)"]
    pub const fn t(self) -> u8 {
        ((self.bits >> 0x0) & 0x7f) as _
    }
    #[inline(always)]
    #[doc = "Watchdog activated"]
    pub const fn set_wdga(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Watchdog activated"]
    pub const fn wdga(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Status register"]
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
    #[doc = "Early wakeup interrupt flag"]
    pub const fn set_ewif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Early wakeup interrupt flag"]
    pub const fn ewif(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WdgtbVal {
    #[doc = "Counter clock (PCLK1 div 4096) div 1"]
    Div1 = 0x0,
    #[doc = "Counter clock (PCLK1 div 4096) div 2"]
    Div2 = 0x1,
    #[doc = "Counter clock (PCLK1 div 4096) div 4"]
    Div4 = 0x2,
    #[doc = "Counter clock (PCLK1 div 4096) div 8"]
    Div8 = 0x3,
}
impl WdgtbVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
