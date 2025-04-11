
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Secure digital input/output interface"]
pub struct Sdmmc {
    ptr: *mut u8,
}
impl Sdmmc {
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
    #[doc = "power control register"]
    pub const fn power(&self) -> utils::Reg<PowerBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<PowerBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SDI clock control register"]
    pub const fn clkcr(&self) -> utils::Reg<ClkcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<ClkcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "argument register"]
    pub const fn argr(&self) -> utils::Reg<ArgrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<ArgrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "command register"]
    pub const fn cmdr(&self) -> utils::Reg<CmdrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<CmdrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "command response register"]
    pub const fn respcmdr(&self) -> utils::Reg<RespcmdrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<RespcmdrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "response 1..4 register"]
    pub const fn respr(&self, idx: usize) -> utils::Reg<ResPxRBits, utils::RO> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x14 + idx * 0x4);
            <utils::Reg<ResPxRBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data timer register"]
    pub const fn dtimer(&self) -> utils::Reg<DtimerBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<DtimerBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data length register"]
    pub const fn dlenr(&self) -> utils::Reg<DlenrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<DlenrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data control register"]
    pub const fn dctrl(&self) -> utils::Reg<DctrlBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<DctrlBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data counter register"]
    pub const fn dcntr(&self) -> utils::Reg<DcntrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<DcntrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn star(&self) -> utils::Reg<StarBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<StarBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "interrupt clear register"]
    pub const fn icr(&self) -> utils::Reg<IcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<IcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "mask register"]
    pub const fn maskr(&self) -> utils::Reg<MaskrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c);
            <utils::Reg<MaskrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "FIFO counter register"]
    pub const fn fifocnt(&self) -> utils::Reg<FifocntBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<FifocntBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data FIFO register"]
    pub const fn fifor(&self) -> utils::Reg<FiforBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x80);
            <utils::Reg<FiforBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "argument register"]
pub struct ArgrBits {
    bits: u32,
}
impl Default for ArgrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ArgrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Command argument"]
    pub const fn set_cmdarg(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Command argument"]
    pub const fn cmdarg(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "SDI clock control register"]
pub struct ClkcrBits {
    bits: u32,
}
impl Default for ClkcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ClkcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clock divide factor"]
    pub const fn set_clkdiv(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Clock divide factor"]
    pub const fn clkdiv(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Clock enable bit"]
    pub const fn set_clken(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock enable bit"]
    pub const fn clken(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Power saving configuration bit"]
    pub const fn set_pwrsav(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Power saving configuration bit"]
    pub const fn pwrsav(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock divider bypass enable bit"]
    pub const fn set_bypass(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock divider bypass enable bit"]
    pub const fn bypass(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wide bus mode enable bit"]
    pub const fn set_widbus(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0xb);
        self.bits |= (val as u32 & 0x3) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "Wide bus mode enable bit"]
    pub const fn widbus(self) -> u8 {
        ((self.bits >> 0xb) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "SDIO_CK dephasing selection bit"]
    pub const fn set_negedge(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDIO_CK dephasing selection bit"]
    pub const fn negedge(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HW Flow Control enable"]
    pub const fn set_hwfc_en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HW Flow Control enable"]
    pub const fn hwfc_en(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "command register"]
pub struct CmdrBits {
    bits: u32,
}
impl Default for CmdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CmdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Command index"]
    pub const fn set_cmdindex(mut self, val: u8) -> Self {
        self.bits &= !(0x3f << 0x0);
        self.bits |= (val as u32 & 0x3f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Command index"]
    pub const fn cmdindex(self) -> u8 {
        ((self.bits >> 0x0) & 0x3f) as _
    }
    #[inline(always)]
    #[doc = "Wait for response bits"]
    pub const fn set_waitresp(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x6);
        self.bits |= (val as u32 & 0x3) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Wait for response bits"]
    pub const fn waitresp(self) -> u8 {
        ((self.bits >> 0x6) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "CPSM waits for interrupt request"]
    pub const fn set_waitint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CPSM waits for interrupt request"]
    pub const fn waitint(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CPSM Waits for ends of data transfer (CmdPend internal signal)"]
    pub const fn set_waitpend(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CPSM Waits for ends of data transfer (CmdPend internal signal)"]
    pub const fn waitpend(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Command path state machine (CPSM) Enable bit"]
    pub const fn set_cpsmen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command path state machine (CPSM) Enable bit"]
    pub const fn cpsmen(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SD I/O suspend command"]
    pub const fn set_sdio_suspend(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SD I/O suspend command"]
    pub const fn sdio_suspend(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "data counter register"]
pub struct DcntrBits {
    bits: u32,
}
impl Default for DcntrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DcntrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Data count value"]
    pub const fn set_datacount(mut self, val: u32) -> Self {
        self.bits &= !(0x1ffffff << 0x0);
        self.bits |= (val as u32 & 0x1ffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Data count value"]
    pub const fn datacount(self) -> u32 {
        ((self.bits >> 0x0) & 0x1ffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "data control register"]
pub struct DctrlBits {
    bits: u32,
}
impl Default for DctrlBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DctrlBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DTEN"]
    pub const fn set_dten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DTEN"]
    pub const fn dten(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data transfer direction selection"]
    pub const fn set_dtdir(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data transfer direction selection"]
    pub const fn dtdir(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data transfer mode selection 1: Stream or SDIO multibyte data transfer"]
    pub const fn set_dtmode(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data transfer mode selection 1: Stream or SDIO multibyte data transfer"]
    pub const fn dtmode(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA enable bit"]
    pub const fn set_dmaen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA enable bit"]
    pub const fn dmaen(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data block size"]
    pub const fn set_dblocksize(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x4);
        self.bits |= (val as u32 & 0xf) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Data block size"]
    pub const fn dblocksize(self) -> u8 {
        ((self.bits >> 0x4) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Read wait start"]
    pub const fn set_rwstart(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Read wait start"]
    pub const fn rwstart(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Read wait stop"]
    pub const fn set_rwstop(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Read wait stop"]
    pub const fn rwstop(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Read wait mode"]
    pub const fn set_rwmod(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Read wait mode"]
    pub const fn rwmod(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SD I/O enable functions"]
    pub const fn set_sdioen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SD I/O enable functions"]
    pub const fn sdioen(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "data length register"]
pub struct DlenrBits {
    bits: u32,
}
impl Default for DlenrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DlenrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Data length value"]
    pub const fn set_datalength(mut self, val: u32) -> Self {
        self.bits &= !(0x1ffffff << 0x0);
        self.bits |= (val as u32 & 0x1ffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Data length value"]
    pub const fn datalength(self) -> u32 {
        ((self.bits >> 0x0) & 0x1ffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "data timer register"]
pub struct DtimerBits {
    bits: u32,
}
impl Default for DtimerBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DtimerBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Data timeout period"]
    pub const fn set_datatime(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Data timeout period"]
    pub const fn datatime(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "FIFO counter register"]
pub struct FifocntBits {
    bits: u32,
}
impl Default for FifocntBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FifocntBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Remaining number of words to be written to or read from the FIFO"]
    pub const fn set_fifocount(mut self, val: u32) -> Self {
        self.bits &= !(0xffffff << 0x0);
        self.bits |= (val as u32 & 0xffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Remaining number of words to be written to or read from the FIFO"]
    pub const fn fifocount(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "data FIFO register"]
pub struct FiforBits {
    bits: u32,
}
impl Default for FiforBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FiforBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Receive and transmit FIFO data"]
    pub const fn set_fifo_data(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Receive and transmit FIFO data"]
    pub const fn fifo_data(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
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
    #[doc = "CCRCFAIL flag clear bit"]
    pub const fn set_ccrcfailc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CCRCFAIL flag clear bit"]
    pub const fn ccrcfailc(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DCRCFAIL flag clear bit"]
    pub const fn set_dcrcfailc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DCRCFAIL flag clear bit"]
    pub const fn dcrcfailc(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CTIMEOUT flag clear bit"]
    pub const fn set_ctimeoutc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CTIMEOUT flag clear bit"]
    pub const fn ctimeoutc(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DTIMEOUT flag clear bit"]
    pub const fn set_dtimeoutc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DTIMEOUT flag clear bit"]
    pub const fn dtimeoutc(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TXUNDERR flag clear bit"]
    pub const fn set_txunderrc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TXUNDERR flag clear bit"]
    pub const fn txunderrc(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "RXOVERR flag clear bit"]
    pub const fn set_rxoverrc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RXOVERR flag clear bit"]
    pub const fn rxoverrc(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CMDREND flag clear bit"]
    pub const fn set_cmdrendc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CMDREND flag clear bit"]
    pub const fn cmdrendc(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CMDSENT flag clear bit"]
    pub const fn set_cmdsentc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CMDSENT flag clear bit"]
    pub const fn cmdsentc(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DATAEND flag clear bit"]
    pub const fn set_dataendc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DATAEND flag clear bit"]
    pub const fn dataendc(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "STBITERR flag clear bit"]
    pub const fn set_stbiterrc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "STBITERR flag clear bit"]
    pub const fn stbiterrc(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DBCKEND flag clear bit"]
    pub const fn set_dbckendc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DBCKEND flag clear bit"]
    pub const fn dbckendc(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SDIOIT flag clear bit"]
    pub const fn set_sdioitc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDIOIT flag clear bit"]
    pub const fn sdioitc(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "mask register"]
pub struct MaskrBits {
    bits: u32,
}
impl Default for MaskrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MaskrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Command CRC fail interrupt enable"]
    pub const fn set_ccrcfailie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command CRC fail interrupt enable"]
    pub const fn ccrcfailie(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data CRC fail interrupt enable"]
    pub const fn set_dcrcfailie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data CRC fail interrupt enable"]
    pub const fn dcrcfailie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Command timeout interrupt enable"]
    pub const fn set_ctimeoutie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command timeout interrupt enable"]
    pub const fn ctimeoutie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data timeout interrupt enable"]
    pub const fn set_dtimeoutie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data timeout interrupt enable"]
    pub const fn dtimeoutie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tx FIFO underrun error interrupt enable"]
    pub const fn set_txunderrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tx FIFO underrun error interrupt enable"]
    pub const fn txunderrie(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Rx FIFO overrun error interrupt enable"]
    pub const fn set_rxoverrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Rx FIFO overrun error interrupt enable"]
    pub const fn rxoverrie(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Command response received interrupt enable"]
    pub const fn set_cmdrendie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command response received interrupt enable"]
    pub const fn cmdrendie(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Command sent interrupt enable"]
    pub const fn set_cmdsentie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command sent interrupt enable"]
    pub const fn cmdsentie(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data end interrupt enable"]
    pub const fn set_dataendie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data end interrupt enable"]
    pub const fn dataendie(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "STBITERR interrupt enable"]
    pub const fn set_stbiterre(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "STBITERR interrupt enable"]
    pub const fn stbiterre(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data block end interrupt enable"]
    pub const fn set_dbckendie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data block end interrupt enable"]
    pub const fn dbckendie(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Command acting interrupt enable"]
    pub const fn set_cmdactie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command acting interrupt enable"]
    pub const fn cmdactie(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data transmit acting interrupt enable"]
    pub const fn set_txactie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data transmit acting interrupt enable"]
    pub const fn txactie(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data receive acting interrupt enable"]
    pub const fn set_rxactie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data receive acting interrupt enable"]
    pub const fn rxactie(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tx FIFO half empty interrupt enable"]
    pub const fn set_txfifoheie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tx FIFO half empty interrupt enable"]
    pub const fn txfifoheie(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Rx FIFO half full interrupt enable"]
    pub const fn set_rxfifohfie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Rx FIFO half full interrupt enable"]
    pub const fn rxfifohfie(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tx FIFO full interrupt enable"]
    pub const fn set_txfifofie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tx FIFO full interrupt enable"]
    pub const fn txfifofie(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Rx FIFO full interrupt enable"]
    pub const fn set_rxfifofie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Rx FIFO full interrupt enable"]
    pub const fn rxfifofie(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tx FIFO empty interrupt enable"]
    pub const fn set_txfifoeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tx FIFO empty interrupt enable"]
    pub const fn txfifoeie(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Rx FIFO empty interrupt enable"]
    pub const fn set_rxfifoeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Rx FIFO empty interrupt enable"]
    pub const fn rxfifoeie(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data available in Tx FIFO interrupt enable"]
    pub const fn set_txdavlie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data available in Tx FIFO interrupt enable"]
    pub const fn txdavlie(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data available in Rx FIFO interrupt enable"]
    pub const fn set_rxdavlie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data available in Rx FIFO interrupt enable"]
    pub const fn rxdavlie(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SDIO mode interrupt received interrupt enable"]
    pub const fn set_sdioitie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDIO mode interrupt received interrupt enable"]
    pub const fn sdioitie(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "power control register"]
pub struct PowerBits {
    bits: u32,
}
impl Default for PowerBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PowerBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "PWRCTRL"]
    pub const fn set_pwrctrl(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "PWRCTRL"]
    pub const fn pwrctrl(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "command response register"]
pub struct RespcmdrBits {
    bits: u32,
}
impl Default for RespcmdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl RespcmdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Response command index"]
    pub const fn set_respcmd(mut self, val: u8) -> Self {
        self.bits &= !(0x3f << 0x0);
        self.bits |= (val as u32 & 0x3f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Response command index"]
    pub const fn respcmd(self) -> u8 {
        ((self.bits >> 0x0) & 0x3f) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "response 1..4 register"]
pub struct ResPxRBits {
    bits: u32,
}
impl Default for ResPxRBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ResPxRBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "see Table 132"]
    pub const fn set_cardstatus(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "see Table 132"]
    pub const fn cardstatus(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "status register"]
pub struct StarBits {
    bits: u32,
}
impl Default for StarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl StarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Command response received (CRC check failed)"]
    pub const fn set_ccrcfail(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command response received (CRC check failed)"]
    pub const fn ccrcfail(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data block sent/received (CRC check failed)"]
    pub const fn set_dcrcfail(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data block sent/received (CRC check failed)"]
    pub const fn dcrcfail(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Command response timeout"]
    pub const fn set_ctimeout(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command response timeout"]
    pub const fn ctimeout(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data timeout"]
    pub const fn set_dtimeout(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data timeout"]
    pub const fn dtimeout(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit FIFO underrun error"]
    pub const fn set_txunderr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit FIFO underrun error"]
    pub const fn txunderr(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Received FIFO overrun error"]
    pub const fn set_rxoverr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Received FIFO overrun error"]
    pub const fn rxoverr(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Command response received (CRC check passed)"]
    pub const fn set_cmdrend(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command response received (CRC check passed)"]
    pub const fn cmdrend(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Command sent (no response required)"]
    pub const fn set_cmdsent(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command sent (no response required)"]
    pub const fn cmdsent(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data end (data counter, SDIDCOUNT, is zero)"]
    pub const fn set_dataend(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data end (data counter, SDIDCOUNT, is zero)"]
    pub const fn dataend(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Start bit not detected on all data signals in wide bus mode"]
    pub const fn set_stbiterr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Start bit not detected on all data signals in wide bus mode"]
    pub const fn stbiterr(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data block sent/received (CRC check passed)"]
    pub const fn set_dbckend(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data block sent/received (CRC check passed)"]
    pub const fn dbckend(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Command transfer in progress"]
    pub const fn set_cmdact(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command transfer in progress"]
    pub const fn cmdact(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data transmit in progress"]
    pub const fn set_txact(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data transmit in progress"]
    pub const fn txact(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data receive in progress"]
    pub const fn set_rxact(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data receive in progress"]
    pub const fn rxact(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    pub const fn set_txfifohe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    pub const fn txfifohe(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive FIFO half full: there are at least 8 words in the FIFO"]
    pub const fn set_rxfifohf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive FIFO half full: there are at least 8 words in the FIFO"]
    pub const fn rxfifohf(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit FIFO full"]
    pub const fn set_txfifof(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit FIFO full"]
    pub const fn txfifof(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive FIFO full"]
    pub const fn set_rxfifof(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive FIFO full"]
    pub const fn rxfifof(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit FIFO empty"]
    pub const fn set_txfifoe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit FIFO empty"]
    pub const fn txfifoe(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive FIFO empty"]
    pub const fn set_rxfifoe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive FIFO empty"]
    pub const fn rxfifoe(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data available in transmit FIFO"]
    pub const fn set_txdavl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data available in transmit FIFO"]
    pub const fn txdavl(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data available in receive FIFO"]
    pub const fn set_rxdavl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data available in receive FIFO"]
    pub const fn rxdavl(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SDIO interrupt received"]
    pub const fn set_sdioit(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDIO interrupt received"]
    pub const fn sdioit(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
}
