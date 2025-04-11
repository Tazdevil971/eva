
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Digital camera interface"]
pub struct Dcmi {
    ptr: *mut u8,
}
impl Dcmi {
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
    #[doc = "control register 1"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<SrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "raw interrupt status register"]
    pub const fn ris(&self) -> utils::Reg<RisBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<RisBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "interrupt enable register"]
    pub const fn ier(&self) -> utils::Reg<IerBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<IerBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "masked interrupt status register"]
    pub const fn mis(&self) -> utils::Reg<MisBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<MisBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "interrupt clear register"]
    pub const fn icr(&self) -> utils::Reg<IcrBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<IcrBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "embedded synchronization code register"]
    pub const fn escr(&self) -> utils::Reg<EscrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<EscrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "embedded synchronization unmask register"]
    pub const fn esur(&self) -> utils::Reg<EsurBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<EsurBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "crop window start"]
    pub const fn cwstrt(&self) -> utils::Reg<CwstrtBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<CwstrtBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "crop window size"]
    pub const fn cwsize(&self) -> utils::Reg<CwsizeBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<CwsizeBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data register"]
    pub const fn dr(&self) -> utils::Reg<DrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<DrBits, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register 1"]
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
    #[doc = "Capture enable"]
    pub const fn set_capture(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Capture enable"]
    pub const fn capture(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Capture mode"]
    pub const fn set_cm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Capture mode"]
    pub const fn cm(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Crop feature"]
    pub const fn set_crop(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Crop feature"]
    pub const fn crop(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "JPEG format"]
    pub const fn set_jpeg(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "JPEG format"]
    pub const fn jpeg(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Embedded synchronization select"]
    pub const fn set_ess(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Embedded synchronization select"]
    pub const fn ess(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Pixel clock polarity"]
    pub const fn set_pckpol(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Pixel clock polarity"]
    pub const fn pckpol(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Horizontal synchronization polarity"]
    pub const fn set_hspol(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Horizontal synchronization polarity"]
    pub const fn hspol(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Vertical synchronization polarity"]
    pub const fn set_vspol(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Vertical synchronization polarity"]
    pub const fn vspol(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Frame capture rate control"]
    pub const fn set_fcrc(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Frame capture rate control"]
    pub const fn fcrc(self) -> u8 {
        ((self.bits >> 0x8) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Extended data mode"]
    pub const fn set_edm(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0xa);
        self.bits |= (val as u32 & 0x3) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "Extended data mode"]
    pub const fn edm(self) -> u8 {
        ((self.bits >> 0xa) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "DCMI enable"]
    pub const fn set_enable(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DCMI enable"]
    pub const fn enable(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "crop window size"]
pub struct CwsizeBits {
    bits: u32,
}
impl Default for CwsizeBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CwsizeBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Capture count"]
    pub const fn set_capcnt(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x0);
        self.bits |= (val as u32 & 0x3fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Capture count"]
    pub const fn capcnt(self) -> u16 {
        ((self.bits >> 0x0) & 0x3fff) as _
    }
    #[inline(always)]
    #[doc = "Vertical line count"]
    pub const fn set_vline(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x10);
        self.bits |= (val as u32 & 0x3fff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Vertical line count"]
    pub const fn vline(self) -> u16 {
        ((self.bits >> 0x10) & 0x3fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "crop window start"]
pub struct CwstrtBits {
    bits: u32,
}
impl Default for CwstrtBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CwstrtBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Horizontal offset count"]
    pub const fn set_hoffcnt(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x0);
        self.bits |= (val as u32 & 0x3fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Horizontal offset count"]
    pub const fn hoffcnt(self) -> u16 {
        ((self.bits >> 0x0) & 0x3fff) as _
    }
    #[inline(always)]
    #[doc = "Vertical start line count"]
    pub const fn set_vst(mut self, val: u16) -> Self {
        self.bits &= !(0x1fff << 0x10);
        self.bits |= (val as u32 & 0x1fff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Vertical start line count"]
    pub const fn vst(self) -> u16 {
        ((self.bits >> 0x10) & 0x1fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "data register"]
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
    #[doc = "Data byte 0"]
    pub const fn set_byte0(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Data byte 0"]
    pub const fn byte0(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Data byte 1"]
    pub const fn set_byte1(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Data byte 1"]
    pub const fn byte1(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Data byte 2"]
    pub const fn set_byte2(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Data byte 2"]
    pub const fn byte2(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Data byte 3"]
    pub const fn set_byte3(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Data byte 3"]
    pub const fn byte3(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "embedded synchronization code register"]
pub struct EscrBits {
    bits: u32,
}
impl Default for EscrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl EscrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Frame start delimiter code"]
    pub const fn set_fsc(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Frame start delimiter code"]
    pub const fn fsc(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Line start delimiter code"]
    pub const fn set_lsc(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Line start delimiter code"]
    pub const fn lsc(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Line end delimiter code"]
    pub const fn set_lec(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Line end delimiter code"]
    pub const fn lec(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Frame end delimiter code"]
    pub const fn set_fec(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Frame end delimiter code"]
    pub const fn fec(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "embedded synchronization unmask register"]
pub struct EsurBits {
    bits: u32,
}
impl Default for EsurBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl EsurBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Frame start delimiter unmask"]
    pub const fn set_fsu(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Frame start delimiter unmask"]
    pub const fn fsu(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Line start delimiter unmask"]
    pub const fn set_lsu(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Line start delimiter unmask"]
    pub const fn lsu(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Line end delimiter unmask"]
    pub const fn set_leu(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Line end delimiter unmask"]
    pub const fn leu(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Frame end delimiter unmask"]
    pub const fn set_feu(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Frame end delimiter unmask"]
    pub const fn feu(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "interrupt clear register"]
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
    #[doc = "Capture complete interrupt status clear"]
    pub const fn set_frame_isc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Capture complete interrupt status clear"]
    pub const fn frame_isc(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Overrun interrupt status clear"]
    pub const fn set_ovr_isc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun interrupt status clear"]
    pub const fn ovr_isc(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Synchronization error interrupt status clear"]
    pub const fn set_err_isc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Synchronization error interrupt status clear"]
    pub const fn err_isc(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Vertical synch interrupt status clear"]
    pub const fn set_vsync_isc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Vertical synch interrupt status clear"]
    pub const fn vsync_isc(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "line interrupt status clear"]
    pub const fn set_line_isc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "line interrupt status clear"]
    pub const fn line_isc(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "interrupt enable register"]
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
    #[doc = "Capture complete interrupt enable"]
    pub const fn set_frame_ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Capture complete interrupt enable"]
    pub const fn frame_ie(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Overrun interrupt enable"]
    pub const fn set_ovr_ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun interrupt enable"]
    pub const fn ovr_ie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Synchronization error interrupt enable"]
    pub const fn set_err_ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Synchronization error interrupt enable"]
    pub const fn err_ie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VSYNC interrupt enable"]
    pub const fn set_vsync_ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VSYNC interrupt enable"]
    pub const fn vsync_ie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Line interrupt enable"]
    pub const fn set_line_ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Line interrupt enable"]
    pub const fn line_ie(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "masked interrupt status register"]
pub struct MisBits {
    bits: u32,
}
impl Default for MisBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MisBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Capture complete masked interrupt status"]
    pub const fn set_frame_mis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Capture complete masked interrupt status"]
    pub const fn frame_mis(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Overrun masked interrupt status"]
    pub const fn set_ovr_mis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun masked interrupt status"]
    pub const fn ovr_mis(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Synchronization error masked interrupt status"]
    pub const fn set_err_mis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Synchronization error masked interrupt status"]
    pub const fn err_mis(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VSYNC masked interrupt status"]
    pub const fn set_vsync_mis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VSYNC masked interrupt status"]
    pub const fn vsync_mis(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Line masked interrupt status"]
    pub const fn set_line_mis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Line masked interrupt status"]
    pub const fn line_mis(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "raw interrupt status register"]
pub struct RisBits {
    bits: u32,
}
impl Default for RisBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl RisBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Capture complete raw interrupt status"]
    pub const fn set_frame_ris(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Capture complete raw interrupt status"]
    pub const fn frame_ris(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Overrun raw interrupt status"]
    pub const fn set_ovr_ris(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun raw interrupt status"]
    pub const fn ovr_ris(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Synchronization error raw interrupt status"]
    pub const fn set_err_ris(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Synchronization error raw interrupt status"]
    pub const fn err_ris(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VSYNC raw interrupt status"]
    pub const fn set_vsync_ris(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VSYNC raw interrupt status"]
    pub const fn vsync_ris(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Line raw interrupt status"]
    pub const fn set_line_ris(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Line raw interrupt status"]
    pub const fn line_ris(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
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
    #[doc = "HSYNC"]
    pub const fn set_hsync(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSYNC"]
    pub const fn hsync(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VSYNC"]
    pub const fn set_vsync(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VSYNC"]
    pub const fn vsync(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FIFO not empty"]
    pub const fn set_fne(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FIFO not empty"]
    pub const fn fne(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
}
