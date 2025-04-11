
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Receiver Interface"]
pub struct Spdifrx {
    ptr: *mut u8,
}
impl Spdifrx {
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
    #[doc = "Control register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt mask register"]
    pub const fn imr(&self) -> utils::Reg<ImrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<ImrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<SrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt Flag Clear register"]
    pub const fn ifcr(&self) -> utils::Reg<IfcrBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<IfcrBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Data input register"]
    pub const fn dr(&self) -> utils::Reg<DrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<DrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Channel Status register"]
    pub const fn csr(&self) -> utils::Reg<CsrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<CsrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Debug Information register"]
    pub const fn dir(&self) -> utils::Reg<DirBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<DirBits, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Control register"]
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
    #[doc = "Peripheral Block Enable"]
    pub const fn set_spdifen(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Peripheral Block Enable"]
    pub const fn spdifen(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Receiver DMA ENable for data flow"]
    pub const fn set_rxdmaen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receiver DMA ENable for data flow"]
    pub const fn rxdmaen(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "STerEO Mode"]
    pub const fn set_rxsteo(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "STerEO Mode"]
    pub const fn rxsteo(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "RX Data format"]
    pub const fn set_drfmt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x4);
        self.bits |= (val as u32 & 0x3) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "RX Data format"]
    pub const fn drfmt(self) -> u8 {
        ((self.bits >> 0x4) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Mask Parity error bit"]
    pub const fn set_pmsk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mask Parity error bit"]
    pub const fn pmsk(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mask of Validity bit"]
    pub const fn set_vmsk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mask of Validity bit"]
    pub const fn vmsk(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mask of channel status and user bits"]
    pub const fn set_cumsk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mask of channel status and user bits"]
    pub const fn cumsk(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mask of Preamble Type bits"]
    pub const fn set_ptmsk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mask of Preamble Type bits"]
    pub const fn ptmsk(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Control Buffer DMA ENable for control flow"]
    pub const fn set_cbdmaen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Control Buffer DMA ENable for control flow"]
    pub const fn cbdmaen(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Channel Selection"]
    pub const fn set_chsel(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Channel Selection"]
    pub const fn chsel(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Maximum allowed re-tries during synchronization phase"]
    pub const fn set_nbtr(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0xc);
        self.bits |= (val as u32 & 0x3) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "Maximum allowed re-tries during synchronization phase"]
    pub const fn nbtr(self) -> u8 {
        ((self.bits >> 0xc) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Wait For Activity"]
    pub const fn set_wfa(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wait For Activity"]
    pub const fn wfa(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "input selection"]
    pub const fn set_insel(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x10);
        self.bits |= (val as u32 & 0x7) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "input selection"]
    pub const fn insel(self) -> u8 {
        ((self.bits >> 0x10) & 0x7) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Channel Status register"]
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
    #[doc = "User data information"]
    pub const fn set_usr(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "User data information"]
    pub const fn usr(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Channel A status information"]
    pub const fn set_cs(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Channel A status information"]
    pub const fn cs(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Start Of Block"]
    pub const fn set_sob(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Start Of Block"]
    pub const fn sob(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Debug Information register"]
pub struct DirBits {
    bits: u32,
}
impl Default for DirBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DirBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Threshold HIGH"]
    pub const fn set_thi(mut self, val: u16) -> Self {
        self.bits &= !(0x1fff << 0x0);
        self.bits |= (val as u32 & 0x1fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Threshold HIGH"]
    pub const fn thi(self) -> u16 {
        ((self.bits >> 0x0) & 0x1fff) as _
    }
    #[inline(always)]
    #[doc = "Threshold LOW"]
    pub const fn set_tlo(mut self, val: u16) -> Self {
        self.bits &= !(0x1fff << 0x10);
        self.bits |= (val as u32 & 0x1fff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Threshold LOW"]
    pub const fn tlo(self) -> u16 {
        ((self.bits >> 0x10) & 0x1fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Data input register"]
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
    #[doc = "Parity Error bit"]
    pub const fn set_dr(mut self, val: u32) -> Self {
        self.bits &= !(0xffffff << 0x0);
        self.bits |= (val as u32 & 0xffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Parity Error bit"]
    pub const fn dr(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffff) as _
    }
    #[inline(always)]
    #[doc = "Parity Error bit"]
    pub const fn set_pe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Parity Error bit"]
    pub const fn pe(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Validity bit"]
    pub const fn set_v(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Validity bit"]
    pub const fn v(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "User bit"]
    pub const fn set_u(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "User bit"]
    pub const fn u(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Channel Status bit"]
    pub const fn set_c(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Channel Status bit"]
    pub const fn c(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Preamble Type"]
    pub const fn set_pt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x1c);
        self.bits |= (val as u32 & 0x3) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "Preamble Type"]
    pub const fn pt(self) -> u8 {
        ((self.bits >> 0x1c) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt Flag Clear register"]
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
    #[doc = "Clears the Parity error flag"]
    pub const fn set_perrcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clears the Parity error flag"]
    pub const fn perrcf(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clears the Overrun error flag"]
    pub const fn set_ovrcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clears the Overrun error flag"]
    pub const fn ovrcf(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clears the Synchronization Block Detected flag"]
    pub const fn set_sbdcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clears the Synchronization Block Detected flag"]
    pub const fn sbdcf(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clears the Synchronization Done flag"]
    pub const fn set_syncdcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clears the Synchronization Done flag"]
    pub const fn syncdcf(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt mask register"]
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
    #[doc = "RXNE interrupt enable"]
    pub const fn set_rxneie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RXNE interrupt enable"]
    pub const fn rxneie(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Control Buffer Ready Interrupt Enable"]
    pub const fn set_csrneie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Control Buffer Ready Interrupt Enable"]
    pub const fn csrneie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Parity error interrupt enable"]
    pub const fn set_perrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Parity error interrupt enable"]
    pub const fn perrie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Overrun error Interrupt Enable"]
    pub const fn set_ovrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun error Interrupt Enable"]
    pub const fn ovrie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Synchronization Block Detected Interrupt Enable"]
    pub const fn set_sblkie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Synchronization Block Detected Interrupt Enable"]
    pub const fn sblkie(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Synchronization Done"]
    pub const fn set_syncdie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Synchronization Done"]
    pub const fn syncdie(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Serial Interface Error Interrupt Enable"]
    pub const fn set_ifeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Serial Interface Error Interrupt Enable"]
    pub const fn ifeie(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Status register"]
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
    #[doc = "Read data register not empty"]
    pub const fn set_rxne(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Read data register not empty"]
    pub const fn rxne(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Control Buffer register is not empty"]
    pub const fn set_csrne(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Control Buffer register is not empty"]
    pub const fn csrne(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Parity error"]
    pub const fn set_perr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Parity error"]
    pub const fn perr(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Overrun error"]
    pub const fn set_ovr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun error"]
    pub const fn ovr(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Synchronization Block Detected"]
    pub const fn set_sbd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Synchronization Block Detected"]
    pub const fn sbd(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Synchronization Done"]
    pub const fn set_syncd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Synchronization Done"]
    pub const fn syncd(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Framing error"]
    pub const fn set_ferr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Framing error"]
    pub const fn ferr(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Synchronization error"]
    pub const fn set_serr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Synchronization error"]
    pub const fn serr(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Time-out error"]
    pub const fn set_terr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Time-out error"]
    pub const fn terr(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Duration of 5 symbols counted with SPDIF_CLK"]
    pub const fn set_width(mut self, val: u16) -> Self {
        self.bits &= !(0x7fff << 0x10);
        self.bits |= (val as u32 & 0x7fff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Duration of 5 symbols counted with SPDIF_CLK"]
    pub const fn width(self) -> u16 {
        ((self.bits >> 0x10) & 0x7fff) as _
    }
}
