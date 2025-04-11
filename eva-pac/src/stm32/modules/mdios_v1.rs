
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Management data input/output slave"]
pub struct Mdios {
    ptr: *mut u8,
}
impl Mdios {
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
    #[doc = "MDIOS configuration register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "MDIOS write flag register"]
    pub const fn wrfr(&self) -> utils::Reg<WrfrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<WrfrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "MDIOS clear write flag register"]
    pub const fn cwrfr(&self) -> utils::Reg<CwrfrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<CwrfrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "MDIOS read flag register"]
    pub const fn rdfr(&self) -> utils::Reg<RdfrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<RdfrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "MDIOS clear read flag register"]
    pub const fn crdfr(&self) -> utils::Reg<CrdfrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<CrdfrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "MDIOS status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<SrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "MDIOS clear flag register"]
    pub const fn clrfr(&self) -> utils::Reg<ClrfrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<ClrfrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "MDIOS input data register %s"]
    pub const fn dinr(&self, idx: usize) -> utils::Reg<DinrBits, utils::RO> {
        assert!(idx < 32);
        unsafe {
            let ptr = self.ptr.add(0x100 + idx * 0x4);
            <utils::Reg<DinrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "MDIOS output data register %s"]
    pub const fn doutr(&self, idx: usize) -> utils::Reg<DoutrBits, utils::RW> {
        assert!(idx < 32);
        unsafe {
            let ptr = self.ptr.add(0x180 + idx * 0x4);
            <utils::Reg<DoutrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "MDIOS clear flag register"]
pub struct ClrfrBits {
    bits: u32,
}
impl Default for ClrfrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ClrfrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clear the preamble error flag"]
    pub const fn set_cperf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear the preamble error flag"]
    pub const fn cperf(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear the start error flag"]
    pub const fn set_cserf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear the start error flag"]
    pub const fn cserf(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear the turnaround error flag"]
    pub const fn set_cterf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear the turnaround error flag"]
    pub const fn cterf(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "MDIOS configuration register"]
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
    #[doc = "Peripheral enable"]
    pub const fn set_en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Peripheral enable"]
    pub const fn en(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Register write interrupt enable"]
    pub const fn set_wrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Register write interrupt enable"]
    pub const fn wrie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Register Read Interrupt Enable"]
    pub const fn set_rdie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Register Read Interrupt Enable"]
    pub const fn rdie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Error interrupt enable"]
    pub const fn set_eie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Error interrupt enable"]
    pub const fn eie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Disable Preamble Check"]
    pub const fn set_dpc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Disable Preamble Check"]
    pub const fn dpc(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Slaves's address"]
    pub const fn set_port_address(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0x8);
        self.bits |= (val as u32 & 0x1f) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Slaves's address"]
    pub const fn port_address(self) -> u8 {
        ((self.bits >> 0x8) & 0x1f) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "MDIOS clear read flag register"]
pub struct CrdfrBits {
    bits: u32,
}
impl Default for CrdfrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CrdfrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clear the read flag"]
    pub const fn set_crdf(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Clear the read flag"]
    pub const fn crdf(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "MDIOS clear write flag register"]
pub struct CwrfrBits {
    bits: u32,
}
impl Default for CwrfrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CwrfrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clear the write flag"]
    pub const fn set_cwrf(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Clear the write flag"]
    pub const fn cwrf(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "MDIOS input data register %s"]
pub struct DinrBits {
    bits: u32,
}
impl Default for DinrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DinrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Input data received from MDIO Master during write frames"]
    pub const fn set_din(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Input data received from MDIO Master during write frames"]
    pub const fn din(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "MDIOS output data register %s"]
pub struct DoutrBits {
    bits: u32,
}
impl Default for DoutrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DoutrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Output data sent to MDIO Master during read frames"]
    pub const fn set_dout(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Output data sent to MDIO Master during read frames"]
    pub const fn dout(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "MDIOS read flag register"]
pub struct RdfrBits {
    bits: u32,
}
impl Default for RdfrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl RdfrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Read flags for MDIO registers 0 to 31"]
    pub const fn set_rdf(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Read flags for MDIO registers 0 to 31"]
    pub const fn rdf(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "MDIOS status register"]
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
    #[doc = "Preamble error flag"]
    pub const fn set_perf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Preamble error flag"]
    pub const fn perf(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Start error flag"]
    pub const fn set_serf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Start error flag"]
    pub const fn serf(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Turnaround error flag"]
    pub const fn set_terf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Turnaround error flag"]
    pub const fn terf(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "MDIOS write flag register"]
pub struct WrfrBits {
    bits: u32,
}
impl Default for WrfrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WrfrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Write flags for MDIO registers 0 to 31"]
    pub const fn set_wrf(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Write flags for MDIO registers 0 to 31"]
    pub const fn wrf(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
