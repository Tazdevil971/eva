
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "DMA2D controller"]
pub struct Dma2d {
    ptr: *mut u8,
}
impl Dma2d {
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
    #[doc = "Interrupt Status Register"]
    pub const fn isr(&self) -> utils::Reg<IsrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<IsrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "interrupt flag clear register"]
    pub const fn ifcr(&self) -> utils::Reg<IfcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<IfcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "foreground memory address register"]
    pub const fn fgmar(&self) -> utils::Reg<FgmarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<FgmarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "foreground offset register"]
    pub const fn fgor(&self) -> utils::Reg<FgorBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<FgorBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "background memory address register"]
    pub const fn bgmar(&self) -> utils::Reg<BgmarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<BgmarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "background offset register"]
    pub const fn bgor(&self) -> utils::Reg<BgorBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<BgorBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "foreground PFC control register"]
    pub const fn fgpfccr(&self) -> utils::Reg<FgpfccrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<FgpfccrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "foreground color register"]
    pub const fn fgcolr(&self) -> utils::Reg<FgcolrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<FgcolrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "background PFC control register"]
    pub const fn bgpfccr(&self) -> utils::Reg<BgpfccrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<BgpfccrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "background color register"]
    pub const fn bgcolr(&self) -> utils::Reg<BgcolrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<BgcolrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "foreground CLUT memory address register"]
    pub const fn fgcmar(&self) -> utils::Reg<FgcmarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<FgcmarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "background CLUT memory address register"]
    pub const fn bgcmar(&self) -> utils::Reg<BgcmarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<BgcmarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "output PFC control register"]
    pub const fn opfccr(&self) -> utils::Reg<OpfccrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<OpfccrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "output color register"]
    pub const fn ocolr(&self) -> utils::Reg<OcolrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<OcolrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "output memory address register"]
    pub const fn omar(&self) -> utils::Reg<OmarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c);
            <utils::Reg<OmarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "output offset register"]
    pub const fn oor(&self) -> utils::Reg<OorBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<OorBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "number of line register"]
    pub const fn nlr(&self) -> utils::Reg<NlrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<NlrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "line watermark register"]
    pub const fn lwr(&self) -> utils::Reg<LwrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<LwrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB master timer configuration register"]
    pub const fn amtcr(&self) -> utils::Reg<AmtcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<AmtcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "FGCLUT"]
    pub const fn fgclut(&self) -> utils::Reg<FgclutBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x400);
            <utils::Reg<FgclutBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "BGCLUT"]
    pub const fn bgclut(&self) -> utils::Reg<BgclutBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x800);
            <utils::Reg<BgclutBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB master timer configuration register"]
pub struct AmtcrBits {
    bits: u32,
}
impl Default for AmtcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl AmtcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Enable"]
    pub const fn set_en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable"]
    pub const fn en(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Dead Time"]
    pub const fn set_dt(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Dead Time"]
    pub const fn dt(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "BGCLUT"]
pub struct BgclutBits {
    bits: u32,
}
impl Default for BgclutBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BgclutBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "BLUE"]
    pub const fn set_blue(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "BLUE"]
    pub const fn blue(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "GREEN"]
    pub const fn set_green(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "GREEN"]
    pub const fn green(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "RED"]
    pub const fn set_red(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "RED"]
    pub const fn red(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "APLHA"]
    pub const fn set_aplha(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "APLHA"]
    pub const fn aplha(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "background CLUT memory address register"]
pub struct BgcmarBits {
    bits: u32,
}
impl Default for BgcmarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BgcmarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Memory address"]
    pub const fn set_ma(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Memory address"]
    pub const fn ma(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "background color register"]
pub struct BgcolrBits {
    bits: u32,
}
impl Default for BgcolrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BgcolrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Blue Value"]
    pub const fn set_blue(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Blue Value"]
    pub const fn blue(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Green Value"]
    pub const fn set_green(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Green Value"]
    pub const fn green(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Red Value"]
    pub const fn set_red(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Red Value"]
    pub const fn red(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "background memory address register"]
pub struct BgmarBits {
    bits: u32,
}
impl Default for BgmarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BgmarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Memory address"]
    pub const fn set_ma(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Memory address"]
    pub const fn ma(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "background offset register"]
pub struct BgorBits {
    bits: u32,
}
impl Default for BgorBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BgorBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Line offset"]
    pub const fn set_lo(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x0);
        self.bits |= (val as u32 & 0x3fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Line offset"]
    pub const fn lo(self) -> u16 {
        ((self.bits >> 0x0) & 0x3fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "background PFC control register"]
pub struct BgpfccrBits {
    bits: u32,
}
impl Default for BgpfccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BgpfccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Color mode"]
    pub const fn set_cm(mut self, val: BgpfccrCmVal) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Color mode"]
    pub const fn cm(self) -> BgpfccrCmVal {
        let val = ((self.bits >> 0x0) & 0xf) as _;
        unsafe { BgpfccrCmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "CLUT Color mode"]
    pub const fn set_ccm(mut self, val: BgpfccrCcmVal) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "CLUT Color mode"]
    pub const fn ccm(self) -> BgpfccrCcmVal {
        let val = ((self.bits >> 0x4) & 0x1) as _;
        unsafe { BgpfccrCcmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Start"]
    pub const fn set_start(mut self, val: BgpfccrStartVal) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Start"]
    pub const fn start(self) -> BgpfccrStartVal {
        let val = ((self.bits >> 0x5) & 0x1) as _;
        unsafe { BgpfccrStartVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "CLUT size"]
    pub const fn set_cs(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "CLUT size"]
    pub const fn cs(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Alpha mode"]
    pub const fn set_am(mut self, val: BgpfccrAmVal) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Alpha mode"]
    pub const fn am(self) -> BgpfccrAmVal {
        let val = ((self.bits >> 0x10) & 0x3) as _;
        unsafe { BgpfccrAmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Alpha value"]
    pub const fn set_alpha(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Alpha value"]
    pub const fn alpha(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
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
    #[doc = "Start"]
    pub const fn set_start(mut self, val: CrStartVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Start"]
    pub const fn start(self) -> CrStartVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { CrStartVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Suspend"]
    pub const fn set_susp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Suspend"]
    pub const fn susp(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Abort"]
    pub const fn set_abort(mut self, val: AbortVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Abort"]
    pub const fn abort(self) -> AbortVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { AbortVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Transfer error interrupt enable"]
    pub const fn set_teie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer error interrupt enable"]
    pub const fn teie(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer complete interrupt enable"]
    pub const fn set_tcie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer complete interrupt enable"]
    pub const fn tcie(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer watermark interrupt enable"]
    pub const fn set_twie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer watermark interrupt enable"]
    pub const fn twie(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CLUT access error interrupt enable"]
    pub const fn set_caeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CLUT access error interrupt enable"]
    pub const fn caeie(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CLUT transfer complete interrupt enable"]
    pub const fn set_ctcie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CLUT transfer complete interrupt enable"]
    pub const fn ctcie(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Configuration Error Interrupt Enable"]
    pub const fn set_ceie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Configuration Error Interrupt Enable"]
    pub const fn ceie(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA2D mode"]
    pub const fn set_mode(mut self, val: ModeVal) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "DMA2D mode"]
    pub const fn mode(self) -> ModeVal {
        let val = ((self.bits >> 0x10) & 0x3) as _;
        unsafe { ModeVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "FGCLUT"]
pub struct FgclutBits {
    bits: u32,
}
impl Default for FgclutBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FgclutBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "BLUE"]
    pub const fn set_blue(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "BLUE"]
    pub const fn blue(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "GREEN"]
    pub const fn set_green(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "GREEN"]
    pub const fn green(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "RED"]
    pub const fn set_red(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "RED"]
    pub const fn red(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "APLHA"]
    pub const fn set_aplha(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "APLHA"]
    pub const fn aplha(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "foreground CLUT memory address register"]
pub struct FgcmarBits {
    bits: u32,
}
impl Default for FgcmarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FgcmarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Memory Address"]
    pub const fn set_ma(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Memory Address"]
    pub const fn ma(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "foreground color register"]
pub struct FgcolrBits {
    bits: u32,
}
impl Default for FgcolrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FgcolrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Blue Value"]
    pub const fn set_blue(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Blue Value"]
    pub const fn blue(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Green Value"]
    pub const fn set_green(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Green Value"]
    pub const fn green(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Red Value"]
    pub const fn set_red(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Red Value"]
    pub const fn red(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "foreground memory address register"]
pub struct FgmarBits {
    bits: u32,
}
impl Default for FgmarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FgmarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Memory address"]
    pub const fn set_ma(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Memory address"]
    pub const fn ma(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "foreground offset register"]
pub struct FgorBits {
    bits: u32,
}
impl Default for FgorBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FgorBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Line offset"]
    pub const fn set_lo(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x0);
        self.bits |= (val as u32 & 0x3fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Line offset"]
    pub const fn lo(self) -> u16 {
        ((self.bits >> 0x0) & 0x3fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "foreground PFC control register"]
pub struct FgpfccrBits {
    bits: u32,
}
impl Default for FgpfccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FgpfccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Color mode"]
    pub const fn set_cm(mut self, val: FgpfccrCmVal) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Color mode"]
    pub const fn cm(self) -> FgpfccrCmVal {
        let val = ((self.bits >> 0x0) & 0xf) as _;
        unsafe { FgpfccrCmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "CLUT color mode"]
    pub const fn set_ccm(mut self, val: FgpfccrCcmVal) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "CLUT color mode"]
    pub const fn ccm(self) -> FgpfccrCcmVal {
        let val = ((self.bits >> 0x4) & 0x1) as _;
        unsafe { FgpfccrCcmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Start"]
    pub const fn set_start(mut self, val: FgpfccrStartVal) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Start"]
    pub const fn start(self) -> FgpfccrStartVal {
        let val = ((self.bits >> 0x5) & 0x1) as _;
        unsafe { FgpfccrStartVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "CLUT size"]
    pub const fn set_cs(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "CLUT size"]
    pub const fn cs(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Alpha mode"]
    pub const fn set_am(mut self, val: FgpfccrAmVal) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Alpha mode"]
    pub const fn am(self) -> FgpfccrAmVal {
        let val = ((self.bits >> 0x10) & 0x3) as _;
        unsafe { FgpfccrAmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Alpha value"]
    pub const fn set_alpha(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Alpha value"]
    pub const fn alpha(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "interrupt flag clear register"]
pub struct IfcrBits {
    bits: u32,
}
impl Default for IfcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IfcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clear Transfer error interrupt flag"]
    pub const fn set_cteif(mut self, val: CteifVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Clear Transfer error interrupt flag"]
    pub const fn cteif(self) -> CteifVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { CteifVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clear transfer complete interrupt flag"]
    pub const fn set_ctcif(mut self, val: CtcifVal) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Clear transfer complete interrupt flag"]
    pub const fn ctcif(self) -> CtcifVal {
        let val = ((self.bits >> 0x1) & 0x1) as _;
        unsafe { CtcifVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clear transfer watermark interrupt flag"]
    pub const fn set_ctwif(mut self, val: CtwifVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Clear transfer watermark interrupt flag"]
    pub const fn ctwif(self) -> CtwifVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { CtwifVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clear CLUT access error interrupt flag"]
    pub const fn set_caecif(mut self, val: CaecifVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Clear CLUT access error interrupt flag"]
    pub const fn caecif(self) -> CaecifVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { CaecifVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clear CLUT transfer complete interrupt flag"]
    pub const fn set_cctcif(mut self, val: CctcifVal) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Clear CLUT transfer complete interrupt flag"]
    pub const fn cctcif(self) -> CctcifVal {
        let val = ((self.bits >> 0x4) & 0x1) as _;
        unsafe { CctcifVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clear configuration error interrupt flag"]
    pub const fn set_cceif(mut self, val: CceifVal) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Clear configuration error interrupt flag"]
    pub const fn cceif(self) -> CceifVal {
        let val = ((self.bits >> 0x5) & 0x1) as _;
        unsafe { CceifVal::from_bits_unchecked(val) }
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
    #[doc = "Transfer error interrupt flag"]
    pub const fn set_teif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer error interrupt flag"]
    pub const fn teif(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer complete interrupt flag"]
    pub const fn set_tcif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer complete interrupt flag"]
    pub const fn tcif(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer watermark interrupt flag"]
    pub const fn set_twif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer watermark interrupt flag"]
    pub const fn twif(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CLUT access error interrupt flag"]
    pub const fn set_caeif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CLUT access error interrupt flag"]
    pub const fn caeif(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CLUT transfer complete interrupt flag"]
    pub const fn set_ctcif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CLUT transfer complete interrupt flag"]
    pub const fn ctcif(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Configuration error interrupt flag"]
    pub const fn set_ceif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Configuration error interrupt flag"]
    pub const fn ceif(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "line watermark register"]
pub struct LwrBits {
    bits: u32,
}
impl Default for LwrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LwrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Line watermark"]
    pub const fn set_lw(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Line watermark"]
    pub const fn lw(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "number of line register"]
pub struct NlrBits {
    bits: u32,
}
impl Default for NlrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl NlrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Number of lines"]
    pub const fn set_nl(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Number of lines"]
    pub const fn nl(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Pixel per lines"]
    pub const fn set_pl(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x10);
        self.bits |= (val as u32 & 0x3fff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Pixel per lines"]
    pub const fn pl(self) -> u16 {
        ((self.bits >> 0x10) & 0x3fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "output color register"]
pub struct OcolrBits {
    bits: u32,
}
impl Default for OcolrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl OcolrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Blue Value"]
    pub const fn set_blue(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Blue Value"]
    pub const fn blue(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Green Value"]
    pub const fn set_green(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Green Value"]
    pub const fn green(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Red Value"]
    pub const fn set_red(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Red Value"]
    pub const fn red(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Alpha Channel Value"]
    pub const fn set_aplha(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Alpha Channel Value"]
    pub const fn aplha(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "output memory address register"]
pub struct OmarBits {
    bits: u32,
}
impl Default for OmarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl OmarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Memory Address"]
    pub const fn set_ma(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Memory Address"]
    pub const fn ma(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "output offset register"]
pub struct OorBits {
    bits: u32,
}
impl Default for OorBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl OorBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Line Offset"]
    pub const fn set_lo(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x0);
        self.bits |= (val as u32 & 0x3fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Line Offset"]
    pub const fn lo(self) -> u16 {
        ((self.bits >> 0x0) & 0x3fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "output PFC control register"]
pub struct OpfccrBits {
    bits: u32,
}
impl Default for OpfccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl OpfccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Color mode"]
    pub const fn set_cm(mut self, val: OpfccrCmVal) -> Self {
        self.bits &= !(0x7 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Color mode"]
    pub const fn cm(self) -> OpfccrCmVal {
        let val = ((self.bits >> 0x0) & 0x7) as _;
        unsafe { OpfccrCmVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AbortVal {
    #[doc = "Transfer abort requested"]
    AbortRequest = 0x1,
}
impl AbortVal {
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
pub enum BgpfccrAmVal {
    #[doc = "No modification of alpha channel"]
    NoModify = 0x0,
    #[doc = "Replace with value in ALPHA[7:0]"]
    Replace = 0x1,
    #[doc = "Multiply with value in ALPHA[7:0]"]
    Multiply = 0x2,
}
impl BgpfccrAmVal {
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
pub enum BgpfccrCcmVal {
    #[doc = "CLUT color format ARGB8888"]
    Argb8888 = 0x0,
    #[doc = "CLUT color format RGB888"]
    Rgb888 = 0x1,
}
impl BgpfccrCcmVal {
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
pub enum BgpfccrCmVal {
    #[doc = "Color mode ARGB8888"]
    Argb8888 = 0x0,
    #[doc = "Color mode RGB888"]
    Rgb888 = 0x1,
    #[doc = "Color mode RGB565"]
    Rgb565 = 0x2,
    #[doc = "Color mode ARGB1555"]
    Argb1555 = 0x3,
    #[doc = "Color mode ARGB4444"]
    Argb4444 = 0x4,
    #[doc = "Color mode L8"]
    L8 = 0x5,
    #[doc = "Color mode AL44"]
    Al44 = 0x6,
    #[doc = "Color mode AL88"]
    Al88 = 0x7,
    #[doc = "Color mode L4"]
    L4 = 0x8,
    #[doc = "Color mode A8"]
    A8 = 0x9,
    #[doc = "Color mode A4"]
    A4 = 0xa,
}
impl BgpfccrCmVal {
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
pub enum BgpfccrStartVal {
    #[doc = "Start the automatic loading of the CLUT"]
    Start = 0x1,
}
impl BgpfccrStartVal {
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
pub enum CaecifVal {
    #[doc = "Clear the CAEIF flag in the ISR register"]
    Clear = 0x1,
}
impl CaecifVal {
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
pub enum CceifVal {
    #[doc = "Clear the CEIF flag in the ISR register"]
    Clear = 0x1,
}
impl CceifVal {
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
pub enum CctcifVal {
    #[doc = "Clear the CTCIF flag in the ISR register"]
    Clear = 0x1,
}
impl CctcifVal {
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
pub enum CrStartVal {
    #[doc = "Launch the DMA2D"]
    Start = 0x1,
}
impl CrStartVal {
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
pub enum CtcifVal {
    #[doc = "Clear the TCIF flag in the ISR register"]
    Clear = 0x1,
}
impl CtcifVal {
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
pub enum CteifVal {
    #[doc = "Clear the TEIF flag in the ISR register"]
    Clear = 0x1,
}
impl CteifVal {
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
pub enum CtwifVal {
    #[doc = "Clear the TWIF flag in the ISR register"]
    Clear = 0x1,
}
impl CtwifVal {
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
pub enum FgpfccrAmVal {
    #[doc = "No modification of alpha channel"]
    NoModify = 0x0,
    #[doc = "Replace with value in ALPHA[7:0]"]
    Replace = 0x1,
    #[doc = "Multiply with value in ALPHA[7:0]"]
    Multiply = 0x2,
}
impl FgpfccrAmVal {
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
pub enum FgpfccrCcmVal {
    #[doc = "CLUT color format ARGB8888"]
    Argb8888 = 0x0,
    #[doc = "CLUT color format RGB888"]
    Rgb888 = 0x1,
}
impl FgpfccrCcmVal {
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
pub enum FgpfccrCmVal {
    #[doc = "Color mode ARGB8888"]
    Argb8888 = 0x0,
    #[doc = "Color mode RGB888"]
    Rgb888 = 0x1,
    #[doc = "Color mode RGB565"]
    Rgb565 = 0x2,
    #[doc = "Color mode ARGB1555"]
    Argb1555 = 0x3,
    #[doc = "Color mode ARGB4444"]
    Argb4444 = 0x4,
    #[doc = "Color mode L8"]
    L8 = 0x5,
    #[doc = "Color mode AL44"]
    Al44 = 0x6,
    #[doc = "Color mode AL88"]
    Al88 = 0x7,
    #[doc = "Color mode L4"]
    L4 = 0x8,
    #[doc = "Color mode A8"]
    A8 = 0x9,
    #[doc = "Color mode A4"]
    A4 = 0xa,
}
impl FgpfccrCmVal {
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
pub enum FgpfccrStartVal {
    #[doc = "Start the automatic loading of the CLUT"]
    Start = 0x1,
}
impl FgpfccrStartVal {
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
pub enum ModeVal {
    #[doc = "Memory-to-memory (FG fetch only)"]
    MemoryToMemory = 0x0,
    #[doc = "Memory-to-memory with PFC (FG fetch only with FG PFC active)"]
    MemoryToMemoryPfc = 0x1,
    #[doc = "Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
    MemoryToMemoryPfcBlending = 0x2,
    #[doc = "Register-to-memory"]
    RegisterToMemory = 0x3,
}
impl ModeVal {
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
pub enum OpfccrCmVal {
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
}
impl OpfccrCmVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
