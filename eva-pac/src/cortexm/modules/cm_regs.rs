#![doc = "Cortex-M registers"]
#[allow(unused_imports)]
use super::utils;
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Fault mask register"]
    pub struct Faultmask {
        bits: u32,
    }
    impl Default for Faultmask {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Faultmask {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Prioritizable interrupt mask"]
        pub const fn set_faultmask(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Prioritizable interrupt mask"]
        pub const fn faultmask(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Priority mask register"]
    pub struct Primask {
        bits: u32,
    }
    impl Default for Primask {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Primask {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Prioritizable interrupt mask"]
        pub const fn set_primask(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Prioritizable interrupt mask"]
        pub const fn primask(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
}
