
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Cryptographic processor."]
pub struct Cryp {
    ptr: *mut u8,
}
impl Cryp {
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
    #[doc = "status register."]
    pub const fn sr(&self) -> utils::Reg<fields::Sr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Sr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data input register."]
    pub const fn din(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data output register."]
    pub const fn dout(&self) -> utils::Reg<u32, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<u32, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA control register."]
    pub const fn dmacr(&self) -> utils::Reg<fields::Dmacr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Dmacr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "interrupt mask set/clear register."]
    pub const fn imscr(&self) -> utils::Reg<fields::Imscr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Imscr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "raw interrupt status register."]
    pub const fn risr(&self) -> utils::Reg<fields::Risr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Risr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "masked interrupt status register."]
    pub const fn misr(&self) -> utils::Reg<fields::Misr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<fields::Misr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Cluster KEY%s, containing K?LR, K?RR."]
    pub const fn key(&self, idx: usize) -> Key {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x20 + idx * 0x8);
            <Key>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Cluster INIT%s, containing IV?LR, IV?RR."]
    pub const fn init(&self, idx: usize) -> Init {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x40 + idx * 0x8);
            <Init>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Cluster INIT%s, containing IV?LR, IV?RR."]
pub struct Init {
    ptr: *mut u8,
}
impl Init {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "initialization vector registers."]
    pub const fn ivlr(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "initialization vector registers."]
    pub const fn ivrr(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Cluster KEY%s, containing K?LR, K?RR."]
pub struct Key {
    ptr: *mut u8,
}
impl Key {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "key registers."]
    pub const fn klr(&self) -> utils::Reg<u32, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<u32, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "key registers."]
    pub const fn krr(&self) -> utils::Reg<u32, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<u32, utils::WO>>::from_ptr(ptr)
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
        #[doc = "Algorithm direction."]
        pub const fn set_algodir(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Algorithm direction."]
        pub const fn algodir(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Algorithm mode."]
        pub const fn set_algomode(mut self, val: u8) -> Self {
            self.bits &= !(0x7 << 0x3);
            self.bits |= (val as u32 & 0x7) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Algorithm mode."]
        pub const fn algomode(self) -> u8 {
            ((self.bits >> 0x3) & 0x7) as _
        }
        #[inline(always)]
        #[doc = "Data type selection."]
        pub const fn set_datatype(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x6);
            self.bits |= (val as u32 & 0x3) << 0x6;
            self
        }
        #[inline(always)]
        #[doc = "Data type selection."]
        pub const fn datatype(self) -> u8 {
            ((self.bits >> 0x6) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Key size selection (AES mode only)."]
        pub const fn set_keysize(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x8);
            self.bits |= (val as u32 & 0x3) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Key size selection (AES mode only)."]
        pub const fn keysize(self) -> u8 {
            ((self.bits >> 0x8) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "FIFO flush."]
        pub const fn set_fflush(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "FIFO flush."]
        pub const fn fflush(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Cryptographic processor enable."]
        pub const fn set_crypen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= if val { 1 << 0xf } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Cryptographic processor enable."]
        pub const fn crypen(self) -> bool {
            ((self.bits >> 0xf) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA control register."]
    pub struct Dmacr {
        bits: u32,
    }
    impl Default for Dmacr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dmacr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DMA input enable."]
        pub const fn set_dien(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DMA input enable."]
        pub const fn dien(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "DMA output enable."]
        pub const fn set_doen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DMA output enable."]
        pub const fn doen(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "interrupt mask set/clear register."]
    pub struct Imscr {
        bits: u32,
    }
    impl Default for Imscr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Imscr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Input FIFO service interrupt mask."]
        pub const fn set_inim(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input FIFO service interrupt mask."]
        pub const fn inim(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output FIFO service interrupt mask."]
        pub const fn set_outim(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output FIFO service interrupt mask."]
        pub const fn outim(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "masked interrupt status register."]
    pub struct Misr {
        bits: u32,
    }
    impl Default for Misr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Misr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Input FIFO service masked interrupt status."]
        pub const fn set_inmis(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input FIFO service masked interrupt status."]
        pub const fn inmis(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output FIFO service masked interrupt status."]
        pub const fn set_outmis(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output FIFO service masked interrupt status."]
        pub const fn outmis(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "raw interrupt status register."]
    pub struct Risr {
        bits: u32,
    }
    impl Default for Risr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Risr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Input FIFO service raw interrupt status."]
        pub const fn set_inris(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input FIFO service raw interrupt status."]
        pub const fn inris(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output FIFO service raw interrupt status."]
        pub const fn set_outris(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output FIFO service raw interrupt status."]
        pub const fn outris(self) -> bool {
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
        #[doc = "Input FIFO empty."]
        pub const fn set_ifem(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input FIFO empty."]
        pub const fn ifem(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Input FIFO not full."]
        pub const fn set_ifnf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input FIFO not full."]
        pub const fn ifnf(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output FIFO not empty."]
        pub const fn set_ofne(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output FIFO not empty."]
        pub const fn ofne(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output FIFO full."]
        pub const fn set_offu(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output FIFO full."]
        pub const fn offu(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Busy bit."]
        pub const fn set_busy(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Busy bit."]
        pub const fn busy(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
}
