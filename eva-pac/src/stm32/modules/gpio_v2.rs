
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "General-purpose I/Os"]
pub struct Gpio {
    ptr: *mut u8,
}
impl Gpio {
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
    #[doc = "GPIO port mode register"]
    pub const fn moder(&self) -> utils::Reg<ModerBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<ModerBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "GPIO port output type register"]
    pub const fn otyper(&self) -> utils::Reg<OtyperBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<OtyperBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "GPIO port output speed register"]
    pub const fn ospeedr(&self) -> utils::Reg<OspeedrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<OspeedrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "GPIO port pull-up/pull-down register"]
    pub const fn pupdr(&self) -> utils::Reg<PupdrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<PupdrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "GPIO port input data register"]
    pub const fn idr(&self) -> utils::Reg<IdrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<IdrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "GPIO port output data register"]
    pub const fn odr(&self) -> utils::Reg<OdrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<OdrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "GPIO port bit set/reset register"]
    pub const fn bsrr(&self) -> utils::Reg<BsrrBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<BsrrBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "GPIO port configuration lock register"]
    pub const fn lckr(&self) -> utils::Reg<LckrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<LckrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "GPIO alternate function registers. The register described in the datasheet as AFRL is index 0 in this array, and AFRH is index 1. Note that when operating on AFRH, you need to subtract 8 from any operations on the field array it contains -- the alternate function for pin 9 is at index 1, for instance."]
    pub const fn afr(&self, idx: usize) -> utils::Reg<AfrBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x20 + idx * 0x4);
            <utils::Reg<AfrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "GPIO alternate function register. This contains an array of 8 fields, which correspond to pins 0-7 of the port (for AFRL) or pins 8-15 of the port (for AFRH)."]
pub struct AfrBits {
    bits: u32,
}
impl Default for AfrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl AfrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Alternate function selection for one of the pins controlled by this register (0-7)."]
    pub const fn set_afr(mut self, idx: usize, val: u8) -> Self {
        assert!(idx < 8);
        self.bits &= !(0xf << (0x0 + idx * 0x4));
        self.bits |= (val as u32 & 0xf) << (0x0 + idx * 0x4);
        self
    }
    #[inline(always)]
    #[doc = "Alternate function selection for one of the pins controlled by this register (0-7)."]
    pub const fn afr(self, idx: usize) -> u8 {
        assert!(idx < 8);
        ((self.bits >> (0x0 + idx * 0x4)) & 0xf) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "GPIO port bit set/reset register"]
pub struct BsrrBits {
    bits: u32,
}
impl Default for BsrrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BsrrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Port x set bit y (y= 0..15)"]
    pub const fn set_bs(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 16);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port x set bit y (y= 0..15)"]
    pub const fn bs(self, idx: usize) -> bool {
        assert!(idx < 16);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port x set bit y (y= 0..15)"]
    pub const fn set_br(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 16);
        self.bits &= !(0x1 << (0x10 + idx * 0x1));
        self.bits |= if val { 1 << (0x10 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port x set bit y (y= 0..15)"]
    pub const fn br(self, idx: usize) -> bool {
        assert!(idx < 16);
        ((self.bits >> (0x10 + idx * 0x1)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "GPIO port input data register"]
pub struct IdrBits {
    bits: u32,
}
impl Default for IdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Port input data (y = 0..15)"]
    pub const fn set_idr(mut self, idx: usize, val: IdrVal) -> Self {
        assert!(idx < 16);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= (val.to_bits() as u32 & 0x1) << (0x0 + idx * 0x1);
        self
    }
    #[inline(always)]
    #[doc = "Port input data (y = 0..15)"]
    pub const fn idr(self, idx: usize) -> IdrVal {
        assert!(idx < 16);
        let val = ((self.bits >> (0x0 + idx * 0x1)) & 0x1) as _;
        unsafe { IdrVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "GPIO port configuration lock register"]
pub struct LckrBits {
    bits: u32,
}
impl Default for LckrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LckrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Port configuration locked"]
    pub const fn set_lck(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 16);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port configuration locked"]
    pub const fn lck(self, idx: usize) -> bool {
        assert!(idx < 16);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port configuration lock key active"]
    pub const fn set_lckk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port configuration lock key active"]
    pub const fn lckk(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "GPIO port mode register"]
pub struct ModerBits {
    bits: u32,
}
impl Default for ModerBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ModerBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Port x configuration bits (y = 0..15)"]
    pub const fn set_moder(mut self, idx: usize, val: ModerVal) -> Self {
        assert!(idx < 16);
        self.bits &= !(0x3 << (0x0 + idx * 0x2));
        self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x2);
        self
    }
    #[inline(always)]
    #[doc = "Port x configuration bits (y = 0..15)"]
    pub const fn moder(self, idx: usize) -> ModerVal {
        assert!(idx < 16);
        let val = ((self.bits >> (0x0 + idx * 0x2)) & 0x3) as _;
        unsafe { ModerVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "GPIO port output data register"]
pub struct OdrBits {
    bits: u32,
}
impl Default for OdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl OdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Port output data (y = 0..15)"]
    pub const fn set_odr(mut self, idx: usize, val: OdrVal) -> Self {
        assert!(idx < 16);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= (val.to_bits() as u32 & 0x1) << (0x0 + idx * 0x1);
        self
    }
    #[inline(always)]
    #[doc = "Port output data (y = 0..15)"]
    pub const fn odr(self, idx: usize) -> OdrVal {
        assert!(idx < 16);
        let val = ((self.bits >> (0x0 + idx * 0x1)) & 0x1) as _;
        unsafe { OdrVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "GPIO port output speed register"]
pub struct OspeedrBits {
    bits: u32,
}
impl Default for OspeedrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl OspeedrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Port x configuration bits (y = 0..15)"]
    pub const fn set_ospeedr(mut self, idx: usize, val: OspeedrVal) -> Self {
        assert!(idx < 16);
        self.bits &= !(0x3 << (0x0 + idx * 0x2));
        self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x2);
        self
    }
    #[inline(always)]
    #[doc = "Port x configuration bits (y = 0..15)"]
    pub const fn ospeedr(self, idx: usize) -> OspeedrVal {
        assert!(idx < 16);
        let val = ((self.bits >> (0x0 + idx * 0x2)) & 0x3) as _;
        unsafe { OspeedrVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "GPIO port output type register"]
pub struct OtyperBits {
    bits: u32,
}
impl Default for OtyperBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl OtyperBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Port x configuration bits (y = 0..15)"]
    pub const fn set_ot(mut self, idx: usize, val: OtVal) -> Self {
        assert!(idx < 16);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= (val.to_bits() as u32 & 0x1) << (0x0 + idx * 0x1);
        self
    }
    #[inline(always)]
    #[doc = "Port x configuration bits (y = 0..15)"]
    pub const fn ot(self, idx: usize) -> OtVal {
        assert!(idx < 16);
        let val = ((self.bits >> (0x0 + idx * 0x1)) & 0x1) as _;
        unsafe { OtVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "GPIO port pull-up/pull-down register"]
pub struct PupdrBits {
    bits: u32,
}
impl Default for PupdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PupdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Port x configuration bits (y = 0..15)"]
    pub const fn set_pupdr(mut self, idx: usize, val: PupdrVal) -> Self {
        assert!(idx < 16);
        self.bits &= !(0x3 << (0x0 + idx * 0x2));
        self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x2);
        self
    }
    #[inline(always)]
    #[doc = "Port x configuration bits (y = 0..15)"]
    pub const fn pupdr(self, idx: usize) -> PupdrVal {
        assert!(idx < 16);
        let val = ((self.bits >> (0x0 + idx * 0x2)) & 0x3) as _;
        unsafe { PupdrVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IdrVal {
    #[doc = "Input is logic low"]
    Low = 0x0,
    #[doc = "Input is logic high"]
    High = 0x1,
}
impl IdrVal {
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
pub enum ModerVal {
    #[doc = "Input mode (reset state)"]
    Input = 0x0,
    #[doc = "General purpose output mode"]
    Output = 0x1,
    #[doc = "Alternate function mode"]
    Alternate = 0x2,
    #[doc = "Analog mode"]
    Analog = 0x3,
}
impl ModerVal {
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
pub enum OdrVal {
    #[doc = "Set output to logic low"]
    Low = 0x0,
    #[doc = "Set output to logic high"]
    High = 0x1,
}
impl OdrVal {
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
pub enum OspeedrVal {
    #[doc = "Low speed"]
    LowSpeed = 0x0,
    #[doc = "Medium speed"]
    MediumSpeed = 0x1,
    #[doc = "High speed"]
    HighSpeed = 0x2,
    #[doc = "Very high speed"]
    VeryHighSpeed = 0x3,
}
impl OspeedrVal {
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
pub enum OtVal {
    #[doc = "Output push-pull (reset state)"]
    PushPull = 0x0,
    #[doc = "Output open-drain"]
    OpenDrain = 0x1,
}
impl OtVal {
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
pub enum PupdrVal {
    #[doc = "No pull-up, pull-down"]
    Floating = 0x0,
    #[doc = "Pull-up"]
    PullUp = 0x1,
    #[doc = "Pull-down"]
    PullDown = 0x2,
}
impl PupdrVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
