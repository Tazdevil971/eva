
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Hash processor."]
pub struct Hash {
    ptr: *mut u8,
}
impl Hash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "control register."]
    pub const fn cr(&self) -> utils::Reg<fields::Cr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data input register."]
    pub const fn din(&self) -> utils::Reg<u32, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<u32, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "start register."]
    pub const fn str(&self) -> utils::Reg<fields::Str, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Str, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "digest registers."]
    pub const fn hr(&self, idx: usize) -> utils::Reg<u32, utils::RO> {
        assert!(idx < 5);
        unsafe {
            let ptr = self.ptr.add(0xc + idx * 0x4);
            <utils::Reg<u32, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "interrupt enable register."]
    pub const fn imr(&self) -> utils::Reg<fields::Imr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::Imr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register."]
    pub const fn sr(&self) -> utils::Reg<fields::Sr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::Sr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "context swap registers."]
    pub const fn csr(&self, idx: usize) -> utils::Reg<u32, utils::RW> {
        assert!(idx < 51);
        unsafe {
            let ptr = self.ptr.add(0xf8 + idx * 0x4);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register."]
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
        #[doc = "Initialize message digest calculation."]
        pub const fn set_init(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Initialize message digest calculation."]
        pub const fn init(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "DMA enable."]
        pub const fn set_dmae(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DMA enable."]
        pub const fn dmae(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Data type selection."]
        pub const fn set_datatype(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x4);
            self.bits |= (val as u32 & 0x3) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Data type selection."]
        pub const fn datatype(self) -> u8 {
            ((self.bits >> 0x4) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Mode selection."]
        pub const fn set_mode(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Mode selection."]
        pub const fn mode(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Algorithm selection."]
        pub const fn set_algo(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Algorithm selection."]
        pub const fn algo(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Number of words already pushed."]
        pub const fn set_nbw(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x8);
            self.bits |= (val as u32 & 0xf) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Number of words already pushed."]
        pub const fn nbw(self) -> u8 {
            ((self.bits >> 0x8) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "DIN not empty."]
        pub const fn set_dinne(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= if val { 1 << 0xc } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DIN not empty."]
        pub const fn dinne(self) -> bool {
            ((self.bits >> 0xc) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Long key selection."]
        pub const fn set_lkey(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x10);
            self.bits |= if val { 1 << 0x10 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Long key selection."]
        pub const fn lkey(self) -> bool {
            ((self.bits >> 0x10) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "interrupt enable register."]
    pub struct Imr {
        bits: u32,
    }
    impl Default for Imr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Imr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Data input interrupt enable."]
        pub const fn set_dinie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Data input interrupt enable."]
        pub const fn dinie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Digest calculation completion interrupt enable."]
        pub const fn set_dcie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Digest calculation completion interrupt enable."]
        pub const fn dcie(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "status register."]
    pub struct Sr {
        bits: u32,
    }
    impl Default for Sr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Data input interrupt status."]
        pub const fn set_dinis(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Data input interrupt status."]
        pub const fn dinis(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Digest calculation completion interrupt status."]
        pub const fn set_dcis(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Digest calculation completion interrupt status."]
        pub const fn dcis(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "DMA Status."]
        pub const fn set_dmas(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DMA Status."]
        pub const fn dmas(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Busy bit."]
        pub const fn set_busy(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Busy bit."]
        pub const fn busy(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "start register."]
    pub struct Str {
        bits: u32,
    }
    impl Default for Str {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Str {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Number of valid bits in the last word of the message."]
        pub const fn set_nblw(mut self, val: u8) -> Self {
            self.bits &= !(0x1f << 0x0);
            self.bits |= (val as u32 & 0x1f) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Number of valid bits in the last word of the message."]
        pub const fn nblw(self) -> u8 {
            ((self.bits >> 0x0) & 0x1f) as _
        }
        #[inline(always)]
        #[doc = "Digest calculation."]
        pub const fn set_dcal(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Digest calculation."]
        pub const fn dcal(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
}
