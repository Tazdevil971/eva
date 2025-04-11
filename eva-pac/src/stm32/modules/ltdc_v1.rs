
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
pub struct Layer {
    ptr: *mut u8,
}
impl Layer {
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
    #[doc = "Layerx Control Register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Window Horizontal Position Configuration Register"]
    pub const fn whpcr(&self) -> utils::Reg<WhpcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<WhpcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Window Vertical Position Configuration Register"]
    pub const fn wvpcr(&self) -> utils::Reg<WvpcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<WvpcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Color Keying Configuration Register"]
    pub const fn ckcr(&self) -> utils::Reg<CkcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<CkcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Pixel Format Configuration Register"]
    pub const fn pfcr(&self) -> utils::Reg<PfcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<PfcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Constant Alpha Configuration Register"]
    pub const fn cacr(&self) -> utils::Reg<CacrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<CacrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Default Color Configuration Register"]
    pub const fn dccr(&self) -> utils::Reg<DccrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<DccrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Blending Factors Configuration Register"]
    pub const fn bfcr(&self) -> utils::Reg<BfcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<BfcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Color Frame Buffer Address Register"]
    pub const fn cfbar(&self) -> utils::Reg<CfbarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<CfbarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Color Frame Buffer Length Register"]
    pub const fn cfblr(&self) -> utils::Reg<CfblrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<CfblrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx ColorFrame Buffer Line Number Register"]
    pub const fn cfblnr(&self) -> utils::Reg<CfblnrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<CfblnrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx CLUT Write Register"]
    pub const fn clutwr(&self) -> utils::Reg<ClutwrBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<ClutwrBits, utils::WO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "LCD-TFT Controller"]
pub struct Ltdc {
    ptr: *mut u8,
}
impl Ltdc {
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
    #[doc = "Synchronization Size Configuration Register"]
    pub const fn sscr(&self) -> utils::Reg<SscrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<SscrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Back Porch Configuration Register"]
    pub const fn bpcr(&self) -> utils::Reg<BpcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<BpcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Active Width Configuration Register"]
    pub const fn awcr(&self) -> utils::Reg<AwcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<AwcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Total Width Configuration Register"]
    pub const fn twcr(&self) -> utils::Reg<TwcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<TwcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Global Control Register"]
    pub const fn gcr(&self) -> utils::Reg<GcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<GcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Shadow Reload Configuration Register"]
    pub const fn srcr(&self) -> utils::Reg<SrcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<SrcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Background Color Configuration Register"]
    pub const fn bccr(&self) -> utils::Reg<BccrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<BccrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt Enable Register"]
    pub const fn ier(&self) -> utils::Reg<IerBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<IerBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt Status Register"]
    pub const fn isr(&self) -> utils::Reg<IsrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<IsrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt Clear Register"]
    pub const fn icr(&self) -> utils::Reg<IcrBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x3c);
            <utils::Reg<IcrBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Line Interrupt Position Configuration Register"]
    pub const fn lipcr(&self) -> utils::Reg<LipcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<LipcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Current Position Status Register"]
    pub const fn cpsr(&self) -> utils::Reg<CpsrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<CpsrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Current Display Status Register"]
    pub const fn cdsr(&self) -> utils::Reg<CdsrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<CdsrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    pub const fn layer(&self, idx: usize) -> Layer {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x84 + idx * 0x80);
            <Layer>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Active Width Configuration Register"]
pub struct AwcrBits {
    bits: u32,
}
impl Default for AwcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl AwcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Accumulated Active Height (in units of horizontal scan line)"]
    pub const fn set_aah(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x0);
        self.bits |= (val as u32 & 0x7ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Accumulated Active Height (in units of horizontal scan line)"]
    pub const fn aah(self) -> u16 {
        ((self.bits >> 0x0) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "Accumulated Active Width (in units of pixel clock period)"]
    pub const fn set_aaw(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x10);
        self.bits |= (val as u32 & 0xfff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Accumulated Active Width (in units of pixel clock period)"]
    pub const fn aaw(self) -> u16 {
        ((self.bits >> 0x10) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Background Color Configuration Register"]
pub struct BccrBits {
    bits: u32,
}
impl Default for BccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Background color blue value"]
    pub const fn set_bcblue(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Background color blue value"]
    pub const fn bcblue(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Background color green value"]
    pub const fn set_bcgreen(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Background color green value"]
    pub const fn bcgreen(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Background color red value"]
    pub const fn set_bcred(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Background color red value"]
    pub const fn bcred(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx Blending Factors Configuration Register"]
pub struct BfcrBits {
    bits: u32,
}
impl Default for BfcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BfcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Blending Factor 2"]
    pub const fn set_bf2(mut self, val: Bf2Val) -> Self {
        self.bits &= !(0x7 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Blending Factor 2"]
    pub const fn bf2(self) -> Bf2Val {
        let val = ((self.bits >> 0x0) & 0x7) as _;
        unsafe { Bf2Val::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Blending Factor 1"]
    pub const fn set_bf1(mut self, val: Bf1Val) -> Self {
        self.bits &= !(0x7 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Blending Factor 1"]
    pub const fn bf1(self) -> Bf1Val {
        let val = ((self.bits >> 0x8) & 0x7) as _;
        unsafe { Bf1Val::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Back Porch Configuration Register"]
pub struct BpcrBits {
    bits: u32,
}
impl Default for BpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Accumulated Vertical back porch (in units of horizontal scan line)"]
    pub const fn set_avbp(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x0);
        self.bits |= (val as u32 & 0x7ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Accumulated Vertical back porch (in units of horizontal scan line)"]
    pub const fn avbp(self) -> u16 {
        ((self.bits >> 0x0) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "Accumulated Horizontal back porch (in units of pixel clock period)"]
    pub const fn set_ahbp(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x10);
        self.bits |= (val as u32 & 0xfff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Accumulated Horizontal back porch (in units of pixel clock period)"]
    pub const fn ahbp(self) -> u16 {
        ((self.bits >> 0x10) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx Constant Alpha Configuration Register"]
pub struct CacrBits {
    bits: u32,
}
impl Default for CacrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CacrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Constant Alpha"]
    pub const fn set_consta(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Constant Alpha"]
    pub const fn consta(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Current Display Status Register"]
pub struct CdsrBits {
    bits: u32,
}
impl Default for CdsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CdsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Vertical Data Enable display Status"]
    pub const fn set_vdes(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Vertical Data Enable display Status"]
    pub const fn vdes(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Horizontal Data Enable display Status"]
    pub const fn set_hdes(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Horizontal Data Enable display Status"]
    pub const fn hdes(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Vertical Synchronization display Status"]
    pub const fn set_vsyncs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Vertical Synchronization display Status"]
    pub const fn vsyncs(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Horizontal Synchronization display Status"]
    pub const fn set_hsyncs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Horizontal Synchronization display Status"]
    pub const fn hsyncs(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx Color Frame Buffer Address Register"]
pub struct CfbarBits {
    bits: u32,
}
impl Default for CfbarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CfbarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Color Frame Buffer Start Address"]
    pub const fn set_cfbadd(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Color Frame Buffer Start Address"]
    pub const fn cfbadd(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub struct CfblnrBits {
    bits: u32,
}
impl Default for CfblnrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CfblnrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Frame Buffer Line Number"]
    pub const fn set_cfblnbr(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x0);
        self.bits |= (val as u32 & 0x7ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Frame Buffer Line Number"]
    pub const fn cfblnbr(self) -> u16 {
        ((self.bits >> 0x0) & 0x7ff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx Color Frame Buffer Length Register"]
pub struct CfblrBits {
    bits: u32,
}
impl Default for CfblrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CfblrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Color Frame Buffer Line Length"]
    pub const fn set_cfbll(mut self, val: u16) -> Self {
        self.bits &= !(0x1fff << 0x0);
        self.bits |= (val as u32 & 0x1fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Color Frame Buffer Line Length"]
    pub const fn cfbll(self) -> u16 {
        ((self.bits >> 0x0) & 0x1fff) as _
    }
    #[inline(always)]
    #[doc = "Color Frame Buffer Pitch in bytes"]
    pub const fn set_cfbp(mut self, val: u16) -> Self {
        self.bits &= !(0x1fff << 0x10);
        self.bits |= (val as u32 & 0x1fff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Color Frame Buffer Pitch in bytes"]
    pub const fn cfbp(self) -> u16 {
        ((self.bits >> 0x10) & 0x1fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx Color Keying Configuration Register"]
pub struct CkcrBits {
    bits: u32,
}
impl Default for CkcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CkcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Color Key Blue value"]
    pub const fn set_ckblue(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Color Key Blue value"]
    pub const fn ckblue(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Color Key Green value"]
    pub const fn set_ckgreen(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Color Key Green value"]
    pub const fn ckgreen(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Color Key Red value"]
    pub const fn set_ckred(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Color Key Red value"]
    pub const fn ckred(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx CLUT Write Register"]
pub struct ClutwrBits {
    bits: u32,
}
impl Default for ClutwrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ClutwrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Blue value"]
    pub const fn set_blue(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Blue value"]
    pub const fn blue(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Green value"]
    pub const fn set_green(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Green value"]
    pub const fn green(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Red value"]
    pub const fn set_red(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Red value"]
    pub const fn red(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "CLUT Address"]
    pub const fn set_clutadd(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "CLUT Address"]
    pub const fn clutadd(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Current Position Status Register"]
pub struct CpsrBits {
    bits: u32,
}
impl Default for CpsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CpsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Current Y Position"]
    pub const fn set_cypos(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Current Y Position"]
    pub const fn cypos(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Current X Position"]
    pub const fn set_cxpos(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Current X Position"]
    pub const fn cxpos(self) -> u16 {
        ((self.bits >> 0x10) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx Control Register"]
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
    #[doc = "Layer Enable"]
    pub const fn set_len(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Layer Enable"]
    pub const fn len(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Color Keying Enable"]
    pub const fn set_colken(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Color Keying Enable"]
    pub const fn colken(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Color Look-Up Table Enable"]
    pub const fn set_cluten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Color Look-Up Table Enable"]
    pub const fn cluten(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx Default Color Configuration Register"]
pub struct DccrBits {
    bits: u32,
}
impl Default for DccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Default Color Blue"]
    pub const fn set_dcblue(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Default Color Blue"]
    pub const fn dcblue(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Default Color Green"]
    pub const fn set_dcgreen(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Default Color Green"]
    pub const fn dcgreen(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Default Color Red"]
    pub const fn set_dcred(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Default Color Red"]
    pub const fn dcred(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Default Color Alpha"]
    pub const fn set_dcalpha(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Default Color Alpha"]
    pub const fn dcalpha(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Global Control Register"]
pub struct GcrBits {
    bits: u32,
}
impl Default for GcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "LCD-TFT controller enable bit"]
    pub const fn set_ltdcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LCD-TFT controller enable bit"]
    pub const fn ltdcen(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Dither Blue Width"]
    pub const fn set_dbw(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x4);
        self.bits |= (val as u32 & 0x7) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Dither Blue Width"]
    pub const fn dbw(self) -> u8 {
        ((self.bits >> 0x4) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Dither Green Width"]
    pub const fn set_dgw(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x8);
        self.bits |= (val as u32 & 0x7) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Dither Green Width"]
    pub const fn dgw(self) -> u8 {
        ((self.bits >> 0x8) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Dither Red Width"]
    pub const fn set_drw(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0xc);
        self.bits |= (val as u32 & 0x7) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "Dither Red Width"]
    pub const fn drw(self) -> u8 {
        ((self.bits >> 0xc) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Dither Enable"]
    pub const fn set_den(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Dither Enable"]
    pub const fn den(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Pixel Clock Polarity"]
    pub const fn set_pcpol(mut self, val: PcpolVal) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "Pixel Clock Polarity"]
    pub const fn pcpol(self) -> PcpolVal {
        let val = ((self.bits >> 0x1c) & 0x1) as _;
        unsafe { PcpolVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Data Enable Polarity"]
    pub const fn set_depol(mut self, val: DepolVal) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1d;
        self
    }
    #[inline(always)]
    #[doc = "Data Enable Polarity"]
    pub const fn depol(self) -> DepolVal {
        let val = ((self.bits >> 0x1d) & 0x1) as _;
        unsafe { DepolVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Vertical Synchronization Polarity"]
    pub const fn set_vspol(mut self, val: VspolVal) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1e;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Synchronization Polarity"]
    pub const fn vspol(self) -> VspolVal {
        let val = ((self.bits >> 0x1e) & 0x1) as _;
        unsafe { VspolVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Horizontal Synchronization Polarity"]
    pub const fn set_hspol(mut self, val: HspolVal) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1f;
        self
    }
    #[inline(always)]
    #[doc = "Horizontal Synchronization Polarity"]
    pub const fn hspol(self) -> HspolVal {
        let val = ((self.bits >> 0x1f) & 0x1) as _;
        unsafe { HspolVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt Clear Register"]
pub struct IcrBits {
    bits: u32,
}
impl Default for IcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clears the Line Interrupt Flag"]
    pub const fn set_clif(mut self, val: ClifVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Clears the Line Interrupt Flag"]
    pub const fn clif(self) -> ClifVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { ClifVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clears the FIFO Underrun Interrupt flag"]
    pub const fn set_cfuif(mut self, val: CfuifVal) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Clears the FIFO Underrun Interrupt flag"]
    pub const fn cfuif(self) -> CfuifVal {
        let val = ((self.bits >> 0x1) & 0x1) as _;
        unsafe { CfuifVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clears the Transfer Error Interrupt Flag"]
    pub const fn set_cterrif(mut self, val: CterrifVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Clears the Transfer Error Interrupt Flag"]
    pub const fn cterrif(self) -> CterrifVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { CterrifVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clears Register Reload Interrupt Flag"]
    pub const fn set_crrif(mut self, val: CrrifVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Clears Register Reload Interrupt Flag"]
    pub const fn crrif(self) -> CrrifVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { CrrifVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt Enable Register"]
pub struct IerBits {
    bits: u32,
}
impl Default for IerBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IerBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Line Interrupt Enable"]
    pub const fn set_lie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Line Interrupt Enable"]
    pub const fn lie(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FIFO Underrun Interrupt Enable"]
    pub const fn set_fuie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FIFO Underrun Interrupt Enable"]
    pub const fn fuie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer Error Interrupt Enable"]
    pub const fn set_terrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer Error Interrupt Enable"]
    pub const fn terrie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Register Reload interrupt enable"]
    pub const fn set_rrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Register Reload interrupt enable"]
    pub const fn rrie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt Status Register"]
pub struct IsrBits {
    bits: u32,
}
impl Default for IsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Line Interrupt flag"]
    pub const fn set_lif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Line Interrupt flag"]
    pub const fn lif(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FIFO Underrun Interrupt flag"]
    pub const fn set_fuif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FIFO Underrun Interrupt flag"]
    pub const fn fuif(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer Error interrupt flag"]
    pub const fn set_terrif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer Error interrupt flag"]
    pub const fn terrif(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Register Reload Interrupt Flag"]
    pub const fn set_rrif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Register Reload Interrupt Flag"]
    pub const fn rrif(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Line Interrupt Position Configuration Register"]
pub struct LipcrBits {
    bits: u32,
}
impl Default for LipcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LipcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Line Interrupt Position"]
    pub const fn set_lipos(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x0);
        self.bits |= (val as u32 & 0x7ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Line Interrupt Position"]
    pub const fn lipos(self) -> u16 {
        ((self.bits >> 0x0) & 0x7ff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx Pixel Format Configuration Register"]
pub struct PfcrBits {
    bits: u32,
}
impl Default for PfcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PfcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Pixel Format"]
    pub const fn set_pf(mut self, val: PfVal) -> Self {
        self.bits &= !(0x7 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Pixel Format"]
    pub const fn pf(self) -> PfVal {
        let val = ((self.bits >> 0x0) & 0x7) as _;
        unsafe { PfVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Shadow Reload Configuration Register"]
pub struct SrcrBits {
    bits: u32,
}
impl Default for SrcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SrcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Immediate Reload"]
    pub const fn set_imr(mut self, val: ImrVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Immediate Reload"]
    pub const fn imr(self) -> ImrVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { ImrVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Vertical Blanking Reload"]
    pub const fn set_vbr(mut self, val: VbrVal) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Blanking Reload"]
    pub const fn vbr(self) -> VbrVal {
        let val = ((self.bits >> 0x1) & 0x1) as _;
        unsafe { VbrVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Synchronization Size Configuration Register"]
pub struct SscrBits {
    bits: u32,
}
impl Default for SscrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SscrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Vertical Synchronization Height (in units of horizontal scan line)"]
    pub const fn set_vsh(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x0);
        self.bits |= (val as u32 & 0x7ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Synchronization Height (in units of horizontal scan line)"]
    pub const fn vsh(self) -> u16 {
        ((self.bits >> 0x0) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "Horizontal Synchronization Width (in units of pixel clock period)"]
    pub const fn set_hsw(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x10);
        self.bits |= (val as u32 & 0xfff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Horizontal Synchronization Width (in units of pixel clock period)"]
    pub const fn hsw(self) -> u16 {
        ((self.bits >> 0x10) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Total Width Configuration Register"]
pub struct TwcrBits {
    bits: u32,
}
impl Default for TwcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TwcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Total Height (in units of horizontal scan line)"]
    pub const fn set_totalh(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x0);
        self.bits |= (val as u32 & 0x7ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Total Height (in units of horizontal scan line)"]
    pub const fn totalh(self) -> u16 {
        ((self.bits >> 0x0) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "Total Width (in units of pixel clock period)"]
    pub const fn set_totalw(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x10);
        self.bits |= (val as u32 & 0xfff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Total Width (in units of pixel clock period)"]
    pub const fn totalw(self) -> u16 {
        ((self.bits >> 0x10) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub struct WhpcrBits {
    bits: u32,
}
impl Default for WhpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WhpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Window Horizontal Start Position"]
    pub const fn set_whstpos(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Window Horizontal Start Position"]
    pub const fn whstpos(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
    #[inline(always)]
    #[doc = "Window Horizontal Stop Position"]
    pub const fn set_whsppos(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x10);
        self.bits |= (val as u32 & 0xfff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Window Horizontal Stop Position"]
    pub const fn whsppos(self) -> u16 {
        ((self.bits >> 0x10) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub struct WvpcrBits {
    bits: u32,
}
impl Default for WvpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WvpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Window Vertical Start Position"]
    pub const fn set_wvstpos(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x0);
        self.bits |= (val as u32 & 0x7ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Window Vertical Start Position"]
    pub const fn wvstpos(self) -> u16 {
        ((self.bits >> 0x0) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "Window Vertical Stop Position"]
    pub const fn set_wvsppos(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x10);
        self.bits |= (val as u32 & 0x7ff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Window Vertical Stop Position"]
    pub const fn wvsppos(self) -> u16 {
        ((self.bits >> 0x10) & 0x7ff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Bf1Val {
    #[doc = "BF1 = constant alpha"]
    Constant = 0x4,
    #[doc = "BF1 = pixel alpha * constant alpha"]
    Pixel = 0x6,
}
impl Bf1Val {
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
pub enum Bf2Val {
    #[doc = "BF2 = 1 - constant alpha"]
    Constant = 0x5,
    #[doc = "BF2 = 1 - pixel alpha * constant alpha"]
    Pixel = 0x7,
}
impl Bf2Val {
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
pub enum CfuifVal {
    #[doc = "Clears the FUIF flag in the ISR register"]
    Clear = 0x1,
}
impl CfuifVal {
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
pub enum ClifVal {
    #[doc = "Clears the LIF flag in the ISR register"]
    Clear = 0x1,
}
impl ClifVal {
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
pub enum CrrifVal {
    #[doc = "Clears the RRIF flag in the ISR register"]
    Clear = 0x1,
}
impl CrrifVal {
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
pub enum CterrifVal {
    #[doc = "Clears the TERRIF flag in the ISR register"]
    Clear = 0x1,
}
impl CterrifVal {
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
pub enum DepolVal {
    #[doc = "Data enable polarity is active low"]
    ActiveLow = 0x0,
    #[doc = "Data enable polarity is active high"]
    ActiveHigh = 0x1,
}
impl DepolVal {
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
pub enum HspolVal {
    #[doc = "Horizontal synchronization polarity is active low"]
    ActiveLow = 0x0,
    #[doc = "Horizontal synchronization polarity is active high"]
    ActiveHigh = 0x1,
}
impl HspolVal {
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
pub enum ImrVal {
    #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    NoEffect = 0x0,
    #[doc = "The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
    Reload = 0x1,
}
impl ImrVal {
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
pub enum PcpolVal {
    #[doc = "Pixel clock on rising edge"]
    RisingEdge = 0x0,
    #[doc = "Pixel clock on falling edge"]
    FallingEdge = 0x1,
}
impl PcpolVal {
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
pub enum PfVal {
    #[doc = "ARGB8888"]
    Argb8888 = 0x0,
    #[doc = "RGB888"]
    Rgb888 = 0x1,
    #[doc = "RGB565"]
    Rgb565 = 0x2,
    #[doc = "ARGB1555"]
    Argb1555 = 0x3,
    #[doc = "ARGB4444"]
    Argb4444 = 0x4,
    #[doc = "L8 (8-bit luminance)"]
    L8 = 0x5,
    #[doc = "AL44 (4-bit alpha, 4-bit luminance)"]
    Al44 = 0x6,
    #[doc = "AL88 (8-bit alpha, 8-bit luminance)"]
    Al88 = 0x7,
}
impl PfVal {
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
pub enum VbrVal {
    #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
    NoEffect = 0x0,
    #[doc = "The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
    Reload = 0x1,
}
impl VbrVal {
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
pub enum VspolVal {
    #[doc = "Vertical synchronization polarity is active low"]
    ActiveLow = 0x0,
    #[doc = "Vertical synchronization polarity is active high"]
    ActiveHigh = 0x1,
}
impl VspolVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
