
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Hash processor."]
pub struct Hash {
    ptr: *mut u8,
}
impl Hash {
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
    #[doc = "control register."]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn str(&self) -> utils::Reg<StrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<StrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "digest registers."]
    pub const fn hra(&self, idx: usize) -> utils::Reg<u32, utils::RO> {
        assert!(idx < 5);
        unsafe {
            let ptr = self.ptr.add(0xc + idx * 0x4);
            <utils::Reg<u32, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "interrupt enable register."]
    pub const fn imr(&self) -> utils::Reg<ImrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<ImrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register."]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<SrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "context swap registers."]
    pub const fn csr(&self, idx: usize) -> utils::Reg<u32, utils::RW> {
        assert!(idx < 54);
        unsafe {
            let ptr = self.ptr.add(0xf8 + idx * 0x4);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "HASH digest register."]
    pub const fn hr(&self, idx: usize) -> utils::Reg<u32, utils::RO> {
        assert!(idx < 8);
        unsafe {
            let ptr = self.ptr.add(0x310 + idx * 0x4);
            <utils::Reg<u32, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register."]
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
    pub const fn set_algo0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Algorithm selection."]
    pub const fn algo0(self) -> bool {
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
    #[doc = "Multiple DMA Transfers."]
    pub const fn set_mdmat(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Multiple DMA Transfers."]
    pub const fn mdmat(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
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
    #[inline(always)]
    #[doc = "ALGO."]
    pub const fn set_algo1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ALGO."]
    pub const fn algo1(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "interrupt enable register."]
pub struct ImrBits {
    bits: u32,
}
impl Default for ImrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ImrBits {
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
pub struct StrBits {
    bits: u32,
}
impl Default for StrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl StrBits {
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
