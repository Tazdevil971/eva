
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Analog-to-digital converter"]
pub struct Adc {
    ptr: *mut u8,
}
impl Adc {
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
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<SrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 1"]
    pub const fn cr1(&self) -> utils::Reg<Cr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<Cr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "sample time register 1"]
    pub const fn smpr1(&self) -> utils::Reg<Smpr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<Smpr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "sample time register 2"]
    pub const fn smpr2(&self) -> utils::Reg<Smpr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<Smpr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "injected channel data offset register x"]
    pub const fn jofr(&self, idx: usize) -> utils::Reg<JofrBits, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x14 + idx * 0x4);
            <utils::Reg<JofrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "watchdog higher threshold register"]
    pub const fn htr(&self) -> utils::Reg<HtrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<HtrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "watchdog lower threshold register"]
    pub const fn ltr(&self) -> utils::Reg<LtrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<LtrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "regular sequence register 1"]
    pub const fn sqr1(&self) -> utils::Reg<Sqr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<Sqr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "regular sequence register 2"]
    pub const fn sqr2(&self) -> utils::Reg<Sqr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<Sqr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "regular sequence register 3"]
    pub const fn sqr3(&self) -> utils::Reg<Sqr3Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<Sqr3Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "injected sequence register"]
    pub const fn jsqr(&self) -> utils::Reg<JsqrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<JsqrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "injected data register x"]
    pub const fn jdr(&self, idx: usize) -> utils::Reg<JdrBits, utils::RO> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x3c + idx * 0x4);
            <utils::Reg<JdrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "regular data register"]
    pub const fn dr(&self) -> utils::Reg<DrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<DrBits, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register 1"]
pub struct Cr1Bits {
    bits: u32,
}
impl Default for Cr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Analog watchdog channel select bits"]
    pub const fn set_awdch(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0x0);
        self.bits |= (val as u32 & 0x1f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Analog watchdog channel select bits"]
    pub const fn awdch(self) -> u8 {
        ((self.bits >> 0x0) & 0x1f) as _
    }
    #[inline(always)]
    #[doc = "Interrupt enable for EOC"]
    pub const fn set_eocie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt enable for EOC"]
    pub const fn eocie(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Analog watchdog interrupt enable"]
    pub const fn set_awdie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Analog watchdog interrupt enable"]
    pub const fn awdie(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Interrupt enable for injected channels"]
    pub const fn set_jeocie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt enable for injected channels"]
    pub const fn jeocie(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Scan mode"]
    pub const fn set_scan(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Scan mode"]
    pub const fn scan(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enable the watchdog on a single channel in scan mode"]
    pub const fn set_awdsgl(mut self, val: AwdsglVal) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "Enable the watchdog on a single channel in scan mode"]
    pub const fn awdsgl(self) -> AwdsglVal {
        let val = ((self.bits >> 0x9) & 0x1) as _;
        unsafe { AwdsglVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Automatic injected group conversion"]
    pub const fn set_jauto(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Automatic injected group conversion"]
    pub const fn jauto(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Discontinuous mode on regular channels"]
    pub const fn set_discen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Discontinuous mode on regular channels"]
    pub const fn discen(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Discontinuous mode on injected channels"]
    pub const fn set_jdiscen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Discontinuous mode on injected channels"]
    pub const fn jdiscen(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Discontinuous mode channel count"]
    pub const fn set_discnum(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0xd);
        self.bits |= (val as u32 & 0x7) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Discontinuous mode channel count"]
    pub const fn discnum(self) -> u8 {
        ((self.bits >> 0xd) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Analog watchdog enable on injected channels"]
    pub const fn set_jawden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Analog watchdog enable on injected channels"]
    pub const fn jawden(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Analog watchdog enable on regular channels"]
    pub const fn set_awden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Analog watchdog enable on regular channels"]
    pub const fn awden(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Resolution"]
    pub const fn set_res(mut self, val: ResVal) -> Self {
        self.bits &= !(0x3 << 0x18);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Resolution"]
    pub const fn res(self) -> ResVal {
        let val = ((self.bits >> 0x18) & 0x3) as _;
        unsafe { ResVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Overrun interrupt enable"]
    pub const fn set_ovrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun interrupt enable"]
    pub const fn ovrie(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register 2"]
pub struct Cr2Bits {
    bits: u32,
}
impl Default for Cr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "A/D Converter ON / OFF"]
    pub const fn set_adon(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "A/D Converter ON / OFF"]
    pub const fn adon(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Continuous conversion"]
    pub const fn set_cont(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Continuous conversion"]
    pub const fn cont(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Direct memory access mode (for single ADC mode)"]
    pub const fn set_dma(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Direct memory access mode (for single ADC mode)"]
    pub const fn dma(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA disable selection (for single ADC mode)"]
    pub const fn set_dds(mut self, val: DdsVal) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "DMA disable selection (for single ADC mode)"]
    pub const fn dds(self) -> DdsVal {
        let val = ((self.bits >> 0x9) & 0x1) as _;
        unsafe { DdsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "End of conversion selection"]
    pub const fn set_eocs(mut self, val: EocsVal) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "End of conversion selection"]
    pub const fn eocs(self) -> EocsVal {
        let val = ((self.bits >> 0xa) & 0x1) as _;
        unsafe { EocsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Data alignment"]
    pub const fn set_align(mut self, val: AlignVal) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "Data alignment"]
    pub const fn align(self) -> AlignVal {
        let val = ((self.bits >> 0xb) & 0x1) as _;
        unsafe { AlignVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "External event select for injected group"]
    pub const fn set_jextsel(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x10);
        self.bits |= (val as u32 & 0xf) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "External event select for injected group"]
    pub const fn jextsel(self) -> u8 {
        ((self.bits >> 0x10) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "External trigger enable for injected channels"]
    pub const fn set_jexten(mut self, val: ExtenVal) -> Self {
        self.bits &= !(0x3 << 0x14);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "External trigger enable for injected channels"]
    pub const fn jexten(self) -> ExtenVal {
        let val = ((self.bits >> 0x14) & 0x3) as _;
        unsafe { ExtenVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Start conversion of injected channels"]
    pub const fn set_jswstart(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Start conversion of injected channels"]
    pub const fn jswstart(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "External event select for regular group"]
    pub const fn set_extsel(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x18);
        self.bits |= (val as u32 & 0xf) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "External event select for regular group"]
    pub const fn extsel(self) -> u8 {
        ((self.bits >> 0x18) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "External trigger enable for regular channels"]
    pub const fn set_exten(mut self, val: ExtenVal) -> Self {
        self.bits &= !(0x3 << 0x1c);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "External trigger enable for regular channels"]
    pub const fn exten(self) -> ExtenVal {
        let val = ((self.bits >> 0x1c) & 0x3) as _;
        unsafe { ExtenVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Start conversion of regular channels"]
    pub const fn set_swstart(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Start conversion of regular channels"]
    pub const fn swstart(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "regular data register"]
pub struct DrBits {
    bits: u32,
}
impl Default for DrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Regular data"]
    pub const fn set_data(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Regular data"]
    pub const fn data(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "watchdog higher threshold register"]
pub struct HtrBits {
    bits: u32,
}
impl Default for HtrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HtrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Analog watchdog higher threshold"]
    pub const fn set_ht(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Analog watchdog higher threshold"]
    pub const fn ht(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "injected data register x"]
pub struct JdrBits {
    bits: u32,
}
impl Default for JdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Injected data"]
    pub const fn set_jdata(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Injected data"]
    pub const fn jdata(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "injected channel data offset register x"]
pub struct JofrBits {
    bits: u32,
}
impl Default for JofrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JofrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Data offset for injected channel x"]
    pub const fn set_joffset(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Data offset for injected channel x"]
    pub const fn joffset(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "injected sequence register"]
pub struct JsqrBits {
    bits: u32,
}
impl Default for JsqrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JsqrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "1st conversion in injected sequence"]
    pub const fn set_jsq(mut self, idx: usize, val: u8) -> Self {
        assert!(idx < 4);
        self.bits &= !(0x1f << (0x0 + idx * 0x5));
        self.bits |= (val as u32 & 0x1f) << (0x0 + idx * 0x5);
        self
    }
    #[inline(always)]
    #[doc = "1st conversion in injected sequence"]
    pub const fn jsq(self, idx: usize) -> u8 {
        assert!(idx < 4);
        ((self.bits >> (0x0 + idx * 0x5)) & 0x1f) as _
    }
    #[inline(always)]
    #[doc = "Injected sequence length"]
    pub const fn set_jl(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x14);
        self.bits |= (val as u32 & 0x3) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Injected sequence length"]
    pub const fn jl(self) -> u8 {
        ((self.bits >> 0x14) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "watchdog lower threshold register"]
pub struct LtrBits {
    bits: u32,
}
impl Default for LtrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LtrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Analog watchdog lower threshold"]
    pub const fn set_lt(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Analog watchdog lower threshold"]
    pub const fn lt(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "sample time register 1"]
pub struct Smpr1Bits {
    bits: u32,
}
impl Default for Smpr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Smpr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Channel 10 sampling time selection"]
    pub const fn set_smp(mut self, idx: usize, val: SampleTimeVal) -> Self {
        assert!(idx < 9);
        self.bits &= !(0x7 << (0x0 + idx * 0x3));
        self.bits |= (val.to_bits() as u32 & 0x7) << (0x0 + idx * 0x3);
        self
    }
    #[inline(always)]
    #[doc = "Channel 10 sampling time selection"]
    pub const fn smp(self, idx: usize) -> SampleTimeVal {
        assert!(idx < 9);
        let val = ((self.bits >> (0x0 + idx * 0x3)) & 0x7) as _;
        unsafe { SampleTimeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Sample time bits"]
    pub const fn set_sm_px_x(mut self, val: SmprSmPxXVal) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val.to_bits() as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Sample time bits"]
    pub const fn sm_px_x(self) -> SmprSmPxXVal {
        let val = ((self.bits >> 0x0) & 0xffffffff) as _;
        unsafe { SmprSmPxXVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "sample time register 2"]
pub struct Smpr2Bits {
    bits: u32,
}
impl Default for Smpr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Smpr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Channel 0 sampling time selection"]
    pub const fn set_smp(mut self, idx: usize, val: SampleTimeVal) -> Self {
        assert!(idx < 10);
        self.bits &= !(0x7 << (0x0 + idx * 0x3));
        self.bits |= (val.to_bits() as u32 & 0x7) << (0x0 + idx * 0x3);
        self
    }
    #[inline(always)]
    #[doc = "Channel 0 sampling time selection"]
    pub const fn smp(self, idx: usize) -> SampleTimeVal {
        assert!(idx < 10);
        let val = ((self.bits >> (0x0 + idx * 0x3)) & 0x7) as _;
        unsafe { SampleTimeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Sample time bits"]
    pub const fn set_sm_px_x(mut self, val: SmprSmPxXVal) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val.to_bits() as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Sample time bits"]
    pub const fn sm_px_x(self) -> SmprSmPxXVal {
        let val = ((self.bits >> 0x0) & 0xffffffff) as _;
        unsafe { SmprSmPxXVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "regular sequence register 1"]
pub struct Sqr1Bits {
    bits: u32,
}
impl Default for Sqr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Sqr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "13th conversion in regular sequence"]
    pub const fn set_sq(mut self, idx: usize, val: u8) -> Self {
        assert!(idx < 4);
        self.bits &= !(0x1f << (0x0 + idx * 0x5));
        self.bits |= (val as u32 & 0x1f) << (0x0 + idx * 0x5);
        self
    }
    #[inline(always)]
    #[doc = "13th conversion in regular sequence"]
    pub const fn sq(self, idx: usize) -> u8 {
        assert!(idx < 4);
        ((self.bits >> (0x0 + idx * 0x5)) & 0x1f) as _
    }
    #[inline(always)]
    #[doc = "Regular channel sequence length"]
    pub const fn set_l(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x14);
        self.bits |= (val as u32 & 0xf) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Regular channel sequence length"]
    pub const fn l(self) -> u8 {
        ((self.bits >> 0x14) & 0xf) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "regular sequence register 2"]
pub struct Sqr2Bits {
    bits: u32,
}
impl Default for Sqr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Sqr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "7th conversion in regular sequence"]
    pub const fn set_sq(mut self, idx: usize, val: u8) -> Self {
        assert!(idx < 6);
        self.bits &= !(0x1f << (0x0 + idx * 0x5));
        self.bits |= (val as u32 & 0x1f) << (0x0 + idx * 0x5);
        self
    }
    #[inline(always)]
    #[doc = "7th conversion in regular sequence"]
    pub const fn sq(self, idx: usize) -> u8 {
        assert!(idx < 6);
        ((self.bits >> (0x0 + idx * 0x5)) & 0x1f) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "regular sequence register 3"]
pub struct Sqr3Bits {
    bits: u32,
}
impl Default for Sqr3Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Sqr3Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "1st conversion in regular sequence"]
    pub const fn set_sq(mut self, idx: usize, val: u8) -> Self {
        assert!(idx < 6);
        self.bits &= !(0x1f << (0x0 + idx * 0x5));
        self.bits |= (val as u32 & 0x1f) << (0x0 + idx * 0x5);
        self
    }
    #[inline(always)]
    #[doc = "1st conversion in regular sequence"]
    pub const fn sq(self, idx: usize) -> u8 {
        assert!(idx < 6);
        ((self.bits >> (0x0 + idx * 0x5)) & 0x1f) as _
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
    #[doc = "Analog watchdog event occurred"]
    pub const fn set_awd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Analog watchdog event occurred"]
    pub const fn awd(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Regular channel end of conversion"]
    pub const fn set_eoc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Regular channel end of conversion"]
    pub const fn eoc(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Injected channel end of conversion"]
    pub const fn set_jeoc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Injected channel end of conversion"]
    pub const fn jeoc(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Injected channel conversion has started"]
    pub const fn set_jstrt(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Injected channel conversion has started"]
    pub const fn jstrt(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Regular channel conversion has started"]
    pub const fn set_strt(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Regular channel conversion has started"]
    pub const fn strt(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Overrun occurred"]
    pub const fn set_ovr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun occurred"]
    pub const fn ovr(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AlignVal {
    #[doc = "Right alignment"]
    Right = 0x0,
    #[doc = "Left alignment"]
    Left = 0x1,
}
impl AlignVal {
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
pub enum AwdsglVal {
    #[doc = "Analog watchdog enabled on all channels"]
    AllChannels = 0x0,
    #[doc = "Analog watchdog enabled on a single channel"]
    SingleChannel = 0x1,
}
impl AwdsglVal {
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
    #[doc = "DMA requests are issued as long as data are converted and DMA=1"]
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
pub enum EocsVal {
    #[doc = "The EOC bit is set at the end of each sequence of regular conversions"]
    EachSequence = 0x0,
    #[doc = "The EOC bit is set at the end of each regular conversion"]
    EachConversion = 0x1,
}
impl EocsVal {
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
pub enum ExtenVal {
    #[doc = "Trigger detection disabled"]
    Disabled = 0x0,
    #[doc = "Trigger detection on the rising edge"]
    RisingEdge = 0x1,
    #[doc = "Trigger detection on the falling edge"]
    FallingEdge = 0x2,
    #[doc = "Trigger detection on both the rising and falling edges"]
    BothEdges = 0x3,
}
impl ExtenVal {
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
pub enum ResVal {
    #[doc = "12-bit (15 ADCCLK cycles)"]
    Bits12 = 0x0,
    #[doc = "10-bit (13 ADCCLK cycles)"]
    Bits10 = 0x1,
    #[doc = "8-bit (11 ADCCLK cycles)"]
    Bits8 = 0x2,
    #[doc = "6-bit (9 ADCCLK cycles)"]
    Bits6 = 0x3,
}
impl ResVal {
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
pub enum SampleTimeVal {
    #[doc = "3 cycles"]
    Cycles3 = 0x0,
    #[doc = "15 cycles"]
    Cycles15 = 0x1,
    #[doc = "28 cycles"]
    Cycles28 = 0x2,
    #[doc = "56 cycles"]
    Cycles56 = 0x3,
    #[doc = "84 cycles"]
    Cycles84 = 0x4,
    #[doc = "112 cycles"]
    Cycles112 = 0x5,
    #[doc = "144 cycles"]
    Cycles144 = 0x6,
    #[doc = "480 cycles"]
    Cycles480 = 0x7,
}
impl SampleTimeVal {
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
#[repr(u32)]
pub enum SmprSmPxXVal {
    #[doc = "3 cycles"]
    Cycles3 = 0x0,
    #[doc = "15 cycles"]
    Cycles15 = 0x1,
    #[doc = "28 cycles"]
    Cycles28 = 0x2,
    #[doc = "56 cycles"]
    Cycles56 = 0x3,
    #[doc = "84 cycles"]
    Cycles84 = 0x4,
    #[doc = "112 cycles"]
    Cycles112 = 0x5,
    #[doc = "144 cycles"]
    Cycles144 = 0x6,
    #[doc = "480 cycles"]
    Cycles480 = 0x7,
}
impl SmprSmPxXVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self as u32
    }
}
