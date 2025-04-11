
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "ADC common registers"]
pub struct AdcCommon {
    ptr: *mut u8,
}
impl AdcCommon {
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
    #[doc = "ADC Common status register"]
    pub const fn csr(&self) -> utils::Reg<CsrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CsrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "ADC common control register"]
    pub const fn ccr(&self) -> utils::Reg<CcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<CcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "ADC common regular data register for dual and triple modes"]
    pub const fn cdr(&self) -> utils::Reg<CdrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<CdrBits, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "ADC common control register"]
pub struct CcrBits {
    bits: u32,
}
impl Default for CcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Multi ADC mode selection"]
    pub const fn set_multi(mut self, val: MultiVal) -> Self {
        self.bits &= !(0x1f << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Multi ADC mode selection"]
    pub const fn multi(self) -> MultiVal {
        let val = ((self.bits >> 0x0) & 0x1f) as _;
        unsafe { MultiVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Delay between 2 sampling phases"]
    pub const fn set_delay(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Delay between 2 sampling phases"]
    pub const fn delay(self) -> u8 {
        ((self.bits >> 0x8) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "DMA disable selection for multi-ADC mode"]
    pub const fn set_dds(mut self, val: DdsVal) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "DMA disable selection for multi-ADC mode"]
    pub const fn dds(self) -> DdsVal {
        let val = ((self.bits >> 0xd) & 0x1) as _;
        unsafe { DdsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Direct memory access mode for multi ADC mode"]
    pub const fn set_dma(mut self, val: DmaVal) -> Self {
        self.bits &= !(0x3 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Direct memory access mode for multi ADC mode"]
    pub const fn dma(self) -> DmaVal {
        let val = ((self.bits >> 0xe) & 0x3) as _;
        unsafe { DmaVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "ADC prescaler"]
    pub const fn set_adcpre(mut self, val: AdcpreVal) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "ADC prescaler"]
    pub const fn adcpre(self) -> AdcpreVal {
        let val = ((self.bits >> 0x10) & 0x3) as _;
        unsafe { AdcpreVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "VBAT enable"]
    pub const fn set_vbate(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VBAT enable"]
    pub const fn vbate(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Temperature sensor and VREFINT enable"]
    pub const fn set_tsvrefe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Temperature sensor and VREFINT enable"]
    pub const fn tsvrefe(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "ADC common regular data register for dual and triple modes"]
pub struct CdrBits {
    bits: u32,
}
impl Default for CdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "1st data item of a pair of regular conversions"]
    pub const fn set_data(mut self, idx: usize, val: u16) -> Self {
        assert!(idx < 2);
        self.bits &= !(0xffff << (0x0 + idx * 0x10));
        self.bits |= (val as u32 & 0xffff) << (0x0 + idx * 0x10);
        self
    }
    #[inline(always)]
    #[doc = "1st data item of a pair of regular conversions"]
    pub const fn data(self, idx: usize) -> u16 {
        assert!(idx < 2);
        ((self.bits >> (0x0 + idx * 0x10)) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "ADC common status register"]
pub struct CsrBits {
    bits: u32,
}
impl Default for CsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Analog watchdog event occurred"]
    pub const fn set_awd(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0x0 + idx * 0x8));
        self.bits |= if val { 1 << (0x0 + idx * 0x8) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Analog watchdog event occurred"]
    pub const fn awd(self, idx: usize) -> bool {
        assert!(idx < 3);
        ((self.bits >> (0x0 + idx * 0x8)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "End of conversion of ADC"]
    pub const fn set_eoc(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0x1 + idx * 0x8));
        self.bits |= if val { 1 << (0x1 + idx * 0x8) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "End of conversion of ADC"]
    pub const fn eoc(self, idx: usize) -> bool {
        assert!(idx < 3);
        ((self.bits >> (0x1 + idx * 0x8)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Injected channel end of conversion of ADC"]
    pub const fn set_jeoc(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0x2 + idx * 0x8));
        self.bits |= if val { 1 << (0x2 + idx * 0x8) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Injected channel end of conversion of ADC"]
    pub const fn jeoc(self, idx: usize) -> bool {
        assert!(idx < 3);
        ((self.bits >> (0x2 + idx * 0x8)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Injected channel conversion started"]
    pub const fn set_jstrt(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0x3 + idx * 0x8));
        self.bits |= if val { 1 << (0x3 + idx * 0x8) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Injected channel conversion started"]
    pub const fn jstrt(self, idx: usize) -> bool {
        assert!(idx < 3);
        ((self.bits >> (0x3 + idx * 0x8)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "regular channel conversion started"]
    pub const fn set_strt(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0x4 + idx * 0x8));
        self.bits |= if val { 1 << (0x4 + idx * 0x8) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "regular channel conversion started"]
    pub const fn strt(self, idx: usize) -> bool {
        assert!(idx < 3);
        ((self.bits >> (0x4 + idx * 0x8)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Overrun occurred"]
    pub const fn set_ovr(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0x5 + idx * 0x8));
        self.bits |= if val { 1 << (0x5 + idx * 0x8) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun occurred"]
    pub const fn ovr(self, idx: usize) -> bool {
        assert!(idx < 3);
        ((self.bits >> (0x5 + idx * 0x8)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AdcpreVal {
    #[doc = "PCLK2 divided by 2"]
    Div2 = 0x0,
    #[doc = "PCLK2 divided by 4"]
    Div4 = 0x1,
    #[doc = "PCLK2 divided by 6"]
    Div6 = 0x2,
    #[doc = "PCLK2 divided by 8"]
    Div8 = 0x3,
}
impl AdcpreVal {
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
pub enum DdsVal {
    #[doc = "No new DMA request is issued after the last transfer"]
    Single = 0x0,
    #[doc = "DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    Continuous = 0x1,
}
impl DdsVal {
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
pub enum DmaVal {
    #[doc = "DMA mode disabled"]
    Disabled = 0x0,
    #[doc = "DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    Mode1 = 0x1,
    #[doc = "DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    Mode2 = 0x2,
    #[doc = "DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
    Mode3 = 0x3,
}
impl DmaVal {
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
pub enum MultiVal {
    #[doc = "All the ADCs independent: independent mode"]
    Independent = 0x0,
    #[doc = "Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    DualRj = 0x1,
    #[doc = "Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    DualRa = 0x2,
    #[doc = "Dual ADC1 and ADC2, injected simultaneous mode only"]
    DualJ = 0x5,
    #[doc = "Dual ADC1 and ADC2, regular simultaneous mode only"]
    DualR = 0x6,
    #[doc = "Dual ADC1 and ADC2, interleaved mode only"]
    DualI = 0x7,
    #[doc = "Dual ADC1 and ADC2, alternate trigger mode only"]
    DualA = 0x9,
    #[doc = "Triple ADC, regular and injected simultaneous mode"]
    TripleRj = 0x11,
    #[doc = "Triple ADC, regular and alternate trigger mode"]
    TripleRa = 0x12,
    #[doc = "Triple ADC, injected simultaneous mode only"]
    TripleJ = 0x15,
    #[doc = "Triple ADC, regular simultaneous mode only"]
    TripleR = 0x16,
    #[doc = "Triple ADC, interleaved mode only"]
    TripleI = 0x17,
    #[doc = "Triple ADC, alternate trigger mode only"]
    TripleA = 0x18,
}
impl MultiVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
