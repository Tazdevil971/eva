
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "External interrupt/event controller"]
pub struct Exti {
    ptr: *mut u8,
}
impl Exti {
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
    #[doc = "Interrupt mask register"]
    pub const fn imr(&self, idx: usize) -> utils::Reg<LinesBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x0 + idx * 0x20);
            <utils::Reg<LinesBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt mask register"]
    pub const fn emr(&self, idx: usize) -> utils::Reg<LinesBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x4 + idx * 0x20);
            <utils::Reg<LinesBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Rising Trigger selection register"]
    pub const fn rtsr(&self, idx: usize) -> utils::Reg<LinesBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x8 + idx * 0x20);
            <utils::Reg<LinesBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Falling Trigger selection register"]
    pub const fn ftsr(&self, idx: usize) -> utils::Reg<LinesBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0xc + idx * 0x20);
            <utils::Reg<LinesBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Software interrupt event register"]
    pub const fn swier(&self, idx: usize) -> utils::Reg<LinesBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x10 + idx * 0x20);
            <utils::Reg<LinesBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Pending register"]
    pub const fn pr(&self, idx: usize) -> utils::Reg<LinesBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x14 + idx * 0x20);
            <utils::Reg<LinesBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "EXTI lines register, 1 bit per line"]
pub struct LinesBits {
    bits: u32,
}
impl Default for LinesBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LinesBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "EXTI line"]
    pub const fn set_line(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 32);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EXTI line"]
    pub const fn line(self, idx: usize) -> bool {
        assert!(idx < 32);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
}
