
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Independent watchdog"]
pub struct Iwdg {
    ptr: *mut u8,
}
impl Iwdg {
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
    #[doc = "Key register"]
    pub const fn kr(&self) -> utils::Reg<KrBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<KrBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Prescaler register"]
    pub const fn pr(&self) -> utils::Reg<PrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<PrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Reload register"]
    pub const fn rlr(&self) -> utils::Reg<RlrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<RlrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<SrBits, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Key register"]
pub struct KrBits {
    bits: u32,
}
impl Default for KrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl KrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Key value (write only, read 0000h)"]
    pub const fn set_key(mut self, val: KeyVal) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val.to_bits() as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Key value (write only, read 0000h)"]
    pub const fn key(self) -> KeyVal {
        let val = ((self.bits >> 0x0) & 0xffff) as _;
        unsafe { KeyVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Prescaler register"]
pub struct PrBits {
    bits: u32,
}
impl Default for PrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Prescaler divider"]
    pub const fn set_pr(mut self, val: PrVal) -> Self {
        self.bits &= !(0x7 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Prescaler divider"]
    pub const fn pr(self) -> PrVal {
        let val = ((self.bits >> 0x0) & 0x7) as _;
        unsafe { PrVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Reload register"]
pub struct RlrBits {
    bits: u32,
}
impl Default for RlrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl RlrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Watchdog counter reload value"]
    pub const fn set_rl(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Watchdog counter reload value"]
    pub const fn rl(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
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
    #[doc = "Watchdog prescaler value update"]
    pub const fn set_pvu(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Watchdog prescaler value update"]
    pub const fn pvu(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Watchdog counter reload value update"]
    pub const fn set_rvu(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Watchdog counter reload value update"]
    pub const fn rvu(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum KeyVal {
    #[doc = "Enable access to PR, RLR and WINR registers (0x5555)"]
    Enable = 0x5555,
    #[doc = "Reset the watchdog value (0xAAAA)"]
    Reset = 0xaaaa,
    #[doc = "Start the watchdog (0xCCCC)"]
    Start = 0xcccc,
}
impl KeyVal {
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
pub enum PrVal {
    #[doc = "Divider /4"]
    DivideBy4 = 0x0,
    #[doc = "Divider /8"]
    DivideBy8 = 0x1,
    #[doc = "Divider /16"]
    DivideBy16 = 0x2,
    #[doc = "Divider /32"]
    DivideBy32 = 0x3,
    #[doc = "Divider /64"]
    DivideBy64 = 0x4,
    #[doc = "Divider /128"]
    DivideBy128 = 0x5,
    #[doc = "Divider /256"]
    DivideBy256 = 0x6,
    #[doc = "Divider /256"]
    DivideBy256bis = 0x7,
}
impl PrVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
