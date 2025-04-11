
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Digital-to-analog converter"]
pub struct Dac {
    ptr: *mut u8,
}
impl Dac {
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
    #[doc = "control register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "software trigger register"]
    pub const fn swtrigr(&self) -> utils::Reg<SwtrigrBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<SwtrigrBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "channel 12-bit right-aligned data holding register"]
    pub const fn dhr12r(&self, idx: usize) -> utils::Reg<Dhr12rBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x8 + idx * 0xc);
            <utils::Reg<Dhr12rBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "channel 12-bit left-aligned data holding register"]
    pub const fn dhr12l(&self, idx: usize) -> utils::Reg<Dhr12lBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0xc + idx * 0xc);
            <utils::Reg<Dhr12lBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "channel 8-bit right-aligned data holding register"]
    pub const fn dhr8r(&self, idx: usize) -> utils::Reg<Dhr8rBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x10 + idx * 0xc);
            <utils::Reg<Dhr8rBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "dual 12-bit right-aligned data holding register"]
    pub const fn dhr12rd(&self) -> utils::Reg<Dhr12rdBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<Dhr12rdBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "dual 12-bit left aligned data holding register"]
    pub const fn dhr12ld(&self) -> utils::Reg<Dhr12ldBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<Dhr12ldBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "dual 8-bit right aligned data holding register"]
    pub const fn dhr8rd(&self) -> utils::Reg<Dhr8rdBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<Dhr8rdBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "channel data output register"]
    pub const fn dor(&self, idx: usize) -> utils::Reg<DorBits, utils::RO> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x2c + idx * 0x4);
            <utils::Reg<DorBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<SrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register"]
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
    #[doc = "channel enable"]
    pub const fn set_en(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0x0 + idx * 0x10));
        self.bits |= if val { 1 << (0x0 + idx * 0x10) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "channel enable"]
    pub const fn en(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0x0 + idx * 0x10)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "channel output buffer disable"]
    pub const fn set_boff(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0x1 + idx * 0x10));
        self.bits |= if val { 1 << (0x1 + idx * 0x10) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "channel output buffer disable"]
    pub const fn boff(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0x1 + idx * 0x10)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "channel trigger enable"]
    pub const fn set_ten(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0x2 + idx * 0x10));
        self.bits |= if val { 1 << (0x2 + idx * 0x10) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "channel trigger enable"]
    pub const fn ten(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0x2 + idx * 0x10)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "channel trigger selection"]
    pub const fn set_tsel(mut self, idx: usize, val: u8) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x7 << (0x3 + idx * 0x10));
        self.bits |= (val as u32 & 0x7) << (0x3 + idx * 0x10);
        self
    }
    #[inline(always)]
    #[doc = "channel trigger selection"]
    pub const fn tsel(self, idx: usize) -> u8 {
        assert!(idx < 2);
        ((self.bits >> (0x3 + idx * 0x10)) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "channel noise/triangle wave generation enable"]
    pub const fn set_wave(mut self, idx: usize, val: WaveVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x3 << (0x6 + idx * 0x10));
        self.bits |= (val.to_bits() as u32 & 0x3) << (0x6 + idx * 0x10);
        self
    }
    #[inline(always)]
    #[doc = "channel noise/triangle wave generation enable"]
    pub const fn wave(self, idx: usize) -> WaveVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0x6 + idx * 0x10)) & 0x3) as _;
        unsafe { WaveVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "channel mask/amplitude selector"]
    pub const fn set_mamp(mut self, idx: usize, val: u8) -> Self {
        assert!(idx < 2);
        self.bits &= !(0xf << (0x8 + idx * 0x10));
        self.bits |= (val as u32 & 0xf) << (0x8 + idx * 0x10);
        self
    }
    #[inline(always)]
    #[doc = "channel mask/amplitude selector"]
    pub const fn mamp(self, idx: usize) -> u8 {
        assert!(idx < 2);
        ((self.bits >> (0x8 + idx * 0x10)) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "channel DMA enable"]
    pub const fn set_dmaen(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0xc + idx * 0x10));
        self.bits |= if val { 1 << (0xc + idx * 0x10) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "channel DMA enable"]
    pub const fn dmaen(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0xc + idx * 0x10)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "channel DMA Underrun Interrupt enable"]
    pub const fn set_dmaudrie(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0xd + idx * 0x10));
        self.bits |= if val { 1 << (0xd + idx * 0x10) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "channel DMA Underrun Interrupt enable"]
    pub const fn dmaudrie(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0xd + idx * 0x10)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "channel 12-bit left-aligned data holding register"]
pub struct Dhr12lBits {
    bits: u32,
}
impl Default for Dhr12lBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhr12lBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "channel 12-bit left-aligned data"]
    pub const fn set_dhr(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x4);
        self.bits |= (val as u32 & 0xfff) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "channel 12-bit left-aligned data"]
    pub const fn dhr(self) -> u16 {
        ((self.bits >> 0x4) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "dual 12-bit left aligned data holding register"]
pub struct Dhr12ldBits {
    bits: u32,
}
impl Default for Dhr12ldBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhr12ldBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "channel 12-bit left-aligned data"]
    pub const fn set_dhr(mut self, idx: usize, val: u16) -> Self {
        assert!(idx < 2);
        self.bits &= !(0xfff << (0x4 + idx * 0x10));
        self.bits |= (val as u32 & 0xfff) << (0x4 + idx * 0x10);
        self
    }
    #[inline(always)]
    #[doc = "channel 12-bit left-aligned data"]
    pub const fn dhr(self, idx: usize) -> u16 {
        assert!(idx < 2);
        ((self.bits >> (0x4 + idx * 0x10)) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "channel 12-bit right-aligned data holding register"]
pub struct Dhr12rBits {
    bits: u32,
}
impl Default for Dhr12rBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhr12rBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "channel 12-bit right-aligned data"]
    pub const fn set_dhr(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "channel 12-bit right-aligned data"]
    pub const fn dhr(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "dual 12-bit right-aligned data holding register"]
pub struct Dhr12rdBits {
    bits: u32,
}
impl Default for Dhr12rdBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhr12rdBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "channel 12-bit right-aligned data"]
    pub const fn set_dhr(mut self, idx: usize, val: u16) -> Self {
        assert!(idx < 2);
        self.bits &= !(0xfff << (0x0 + idx * 0x10));
        self.bits |= (val as u32 & 0xfff) << (0x0 + idx * 0x10);
        self
    }
    #[inline(always)]
    #[doc = "channel 12-bit right-aligned data"]
    pub const fn dhr(self, idx: usize) -> u16 {
        assert!(idx < 2);
        ((self.bits >> (0x0 + idx * 0x10)) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "channel 8-bit right-aligned data holding register"]
pub struct Dhr8rBits {
    bits: u32,
}
impl Default for Dhr8rBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhr8rBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "channel 8-bit right-aligned data"]
    pub const fn set_dhr(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "channel 8-bit right-aligned data"]
    pub const fn dhr(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "dual 8-bit right aligned data holding register"]
pub struct Dhr8rdBits {
    bits: u32,
}
impl Default for Dhr8rdBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhr8rdBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "channel 8-bit right-aligned data"]
    pub const fn set_dhr(mut self, idx: usize, val: u8) -> Self {
        assert!(idx < 2);
        self.bits &= !(0xff << (0x0 + idx * 0x8));
        self.bits |= (val as u32 & 0xff) << (0x0 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "channel 8-bit right-aligned data"]
    pub const fn dhr(self, idx: usize) -> u8 {
        assert!(idx < 2);
        ((self.bits >> (0x0 + idx * 0x8)) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "channel data output register"]
pub struct DorBits {
    bits: u32,
}
impl Default for DorBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DorBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "channel data output"]
    pub const fn set_dor(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "channel data output"]
    pub const fn dor(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "status register"]
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
    #[doc = "channel DMA underrun flag"]
    pub const fn set_dmaudr(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0xd + idx * 0x10));
        self.bits |= if val { 1 << (0xd + idx * 0x10) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "channel DMA underrun flag"]
    pub const fn dmaudr(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0xd + idx * 0x10)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "software trigger register"]
pub struct SwtrigrBits {
    bits: u32,
}
impl Default for SwtrigrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SwtrigrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "channel software trigger"]
    pub const fn set_swtrig(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "channel software trigger"]
    pub const fn swtrig(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WaveVal {
    #[doc = "Wave generation disabled"]
    Disabled = 0x0,
    #[doc = "Noise wave generation enabled"]
    Noise = 0x1,
    #[doc = "Triangle wave generation enabled"]
    Triangle = 0x2,
}
impl WaveVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
