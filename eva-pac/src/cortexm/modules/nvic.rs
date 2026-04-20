#![doc = "Nested Vector Interrupt Controller"]
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Nested Vector Interrupt Controller"]
pub struct Nvic {
    ptr: *mut u8,
}
impl Nvic {
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
    #[doc = "Interrupt set-enable registers"]
    pub const fn iser(&self, idx: usize) -> utils::Reg<IserBits, utils::RW> {
        assert!(idx < 8);
        unsafe {
            let ptr = self.ptr.add(0x0 + idx * 0x4);
            <utils::Reg<IserBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt clear-enable registers"]
    pub const fn icer(&self, idx: usize) -> utils::Reg<IcerBits, utils::RW> {
        assert!(idx < 8);
        unsafe {
            let ptr = self.ptr.add(0x80 + idx * 0x4);
            <utils::Reg<IcerBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt set-pending registers"]
    pub const fn ispr(&self, idx: usize) -> utils::Reg<IsprBits, utils::RW> {
        assert!(idx < 8);
        unsafe {
            let ptr = self.ptr.add(0x100 + idx * 0x4);
            <utils::Reg<IsprBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt clear-pending registers"]
    pub const fn icpr(&self, idx: usize) -> utils::Reg<IcprBits, utils::RW> {
        assert!(idx < 8);
        unsafe {
            let ptr = self.ptr.add(0x180 + idx * 0x4);
            <utils::Reg<IcprBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt active bit registers"]
    pub const fn iabr(&self, idx: usize) -> utils::Reg<IabrBits, utils::RW> {
        assert!(idx < 8);
        unsafe {
            let ptr = self.ptr.add(0x200 + idx * 0x4);
            <utils::Reg<IabrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt priority registers"]
    pub const fn ip(&self, idx: usize) -> utils::Reg<u8, utils::RW> {
        assert!(idx < 240);
        unsafe {
            let ptr = self.ptr.add(0x300 + idx * 0x1);
            <utils::Reg<u8, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Software trigger interrupt register"]
    pub const fn stir(&self) -> utils::Reg<StirBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0xe00);
            <utils::Reg<StirBits, utils::WO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt active bit registers"]
pub struct IabrBits {
    bits: u32,
}
impl Default for IabrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IabrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Interrupt active flags"]
    pub const fn set_active(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 32);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt active flags"]
    pub const fn active(self, idx: usize) -> bool {
        assert!(idx < 32);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt clear-enable registers"]
pub struct IcerBits {
    bits: u32,
}
impl Default for IcerBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IcerBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Interrupt clear-enable bits"]
    pub const fn set_clrena(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 32);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt clear-enable bits"]
    pub const fn clrena(self, idx: usize) -> bool {
        assert!(idx < 32);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt clear-pending registers"]
pub struct IcprBits {
    bits: u32,
}
impl Default for IcprBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IcprBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Interrupt clear-pending bits"]
    pub const fn set_clrpend(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 32);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt clear-pending bits"]
    pub const fn clrpend(self, idx: usize) -> bool {
        assert!(idx < 32);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt set-enable registers"]
pub struct IserBits {
    bits: u32,
}
impl Default for IserBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IserBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Interrupt set-enable bits"]
    pub const fn set_setena(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 32);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt set-enable bits"]
    pub const fn setena(self, idx: usize) -> bool {
        assert!(idx < 32);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt set-pending registers"]
pub struct IsprBits {
    bits: u32,
}
impl Default for IsprBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IsprBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Interrupt set-pending bits"]
    pub const fn set_setpend(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 32);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt set-pending bits"]
    pub const fn setpend(self, idx: usize) -> bool {
        assert!(idx < 32);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Software trigger interrupt register"]
pub struct StirBits {
    bits: u32,
}
impl Default for StirBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl StirBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Interrupt ID"]
    pub const fn set_intid(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Interrupt ID"]
    pub const fn intid(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
}
