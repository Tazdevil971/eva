
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Cyclic Redundancy Check calculation unit"]
pub struct Crc {
    ptr: *mut u8,
}
impl Crc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "Data register - half-word sized"]
    pub const fn dr16(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Data register"]
    pub const fn dr32(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Data register - byte sized"]
    pub const fn dr8(&self) -> utils::Reg<u8, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<u8, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Independent Data register"]
    pub const fn idr(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register"]
    pub const fn cr(&self) -> utils::Reg<fields::Cr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Cr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Initial CRC value"]
    pub const fn init(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "CRC polynomial"]
    pub const fn pol(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Control register"]
    pub struct Cr {
        bits: u32,
    }
    impl Default for Cr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "RESET bit"]
        pub const fn set_reset(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RESET bit"]
        pub const fn reset(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Polynomial size"]
        pub const fn set_polysize(mut self, val: vals::Polysize) -> Self {
            self.bits &= !(0x3 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Polynomial size"]
        pub const fn polysize(self) -> vals::Polysize {
            let val = ((self.bits >> 0x3) & 0x3) as _;
            unsafe { vals::Polysize::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Reverse input data"]
        pub const fn set_rev_in(mut self, val: vals::RevIn) -> Self {
            self.bits &= !(0x3 << 0x5);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x5;
            self
        }
        #[inline(always)]
        #[doc = "Reverse input data"]
        pub const fn rev_in(self) -> vals::RevIn {
            let val = ((self.bits >> 0x5) & 0x3) as _;
            unsafe { vals::RevIn::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Reverse output data"]
        pub const fn set_rev_out(mut self, val: vals::RevOut) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "Reverse output data"]
        pub const fn rev_out(self) -> vals::RevOut {
            let val = ((self.bits >> 0x7) & 0x1) as _;
            unsafe { vals::RevOut::from_bits_unchecked(val) }
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum Polysize {
        #[doc = "32-bit polynomial"]
        Polysize32 = 0x0,
        #[doc = "16-bit polynomial"]
        Polysize16 = 0x1,
        #[doc = "8-bit polynomial"]
        Polysize8 = 0x2,
        #[doc = "7-bit polynomial"]
        Polysize7 = 0x3,
    }
    impl Polysize {
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
    pub enum RevIn {
        #[doc = "Bit order not affected"]
        Normal = 0x0,
        #[doc = "Bit reversal done by byte"]
        Byte = 0x1,
        #[doc = "Bit reversal done by half-word"]
        HalfWord = 0x2,
        #[doc = "Bit reversal done by word"]
        Word = 0x3,
    }
    impl RevIn {
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
    pub enum RevOut {
        #[doc = "Bit order not affected"]
        Normal = 0x0,
        #[doc = "Bit reversed output"]
        Reversed = 0x1,
    }
    impl RevOut {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
            unsafe { ::core::mem::transmute(bits) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            self as u8
        }
    }
}
