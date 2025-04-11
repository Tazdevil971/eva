
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "QuadSPI interface"]
pub struct Quadspi {
    ptr: *mut u8,
}
impl Quadspi {
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
    #[doc = "device configuration register"]
    pub const fn dcr(&self) -> utils::Reg<DcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<DcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<SrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "flag clear register"]
    pub const fn fcr(&self) -> utils::Reg<FcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<FcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data length register"]
    pub const fn dlr(&self) -> utils::Reg<DlrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<DlrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "communication configuration register"]
    pub const fn ccr(&self) -> utils::Reg<CcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<CcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "address register"]
    pub const fn ar(&self) -> utils::Reg<ArBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<ArBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "ABR"]
    pub const fn abr(&self) -> utils::Reg<AbrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<AbrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data register"]
    pub const fn dr(&self) -> utils::Reg<DrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<DrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "polling status mask register"]
    pub const fn psmkr(&self) -> utils::Reg<PsmkrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<PsmkrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "polling status match register"]
    pub const fn psmar(&self) -> utils::Reg<PsmarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<PsmarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "polling interval register"]
    pub const fn pir(&self) -> utils::Reg<PirBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<PirBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "low-power timeout register"]
    pub const fn lptr(&self) -> utils::Reg<LptrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<LptrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "ABR"]
pub struct AbrBits {
    bits: u32,
}
impl Default for AbrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl AbrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "ALTERNATE"]
    pub const fn set_alternate(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "ALTERNATE"]
    pub const fn alternate(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "address register"]
pub struct ArBits {
    bits: u32,
}
impl Default for ArBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ArBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Address"]
    pub const fn set_address(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Address"]
    pub const fn address(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "communication configuration register"]
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
    #[doc = "Instruction"]
    pub const fn set_instruction(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Instruction"]
    pub const fn instruction(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Instruction mode"]
    pub const fn set_imode(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Instruction mode"]
    pub const fn imode(self) -> u8 {
        ((self.bits >> 0x8) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Address mode"]
    pub const fn set_admode(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0xa);
        self.bits |= (val as u32 & 0x3) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "Address mode"]
    pub const fn admode(self) -> u8 {
        ((self.bits >> 0xa) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Address size"]
    pub const fn set_adsize(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0xc);
        self.bits |= (val as u32 & 0x3) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "Address size"]
    pub const fn adsize(self) -> u8 {
        ((self.bits >> 0xc) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Alternate bytes mode"]
    pub const fn set_abmode(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0xe);
        self.bits |= (val as u32 & 0x3) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Alternate bytes mode"]
    pub const fn abmode(self) -> u8 {
        ((self.bits >> 0xe) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Alternate bytes size"]
    pub const fn set_absize(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Alternate bytes size"]
    pub const fn absize(self) -> u8 {
        ((self.bits >> 0x10) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Number of dummy cycles"]
    pub const fn set_dcyc(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0x12);
        self.bits |= (val as u32 & 0x1f) << 0x12;
        self
    }
    #[inline(always)]
    #[doc = "Number of dummy cycles"]
    pub const fn dcyc(self) -> u8 {
        ((self.bits >> 0x12) & 0x1f) as _
    }
    #[inline(always)]
    #[doc = "Data mode"]
    pub const fn set_dmode(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x18);
        self.bits |= (val as u32 & 0x3) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Data mode"]
    pub const fn dmode(self) -> u8 {
        ((self.bits >> 0x18) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Functional mode"]
    pub const fn set_fmode(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x1a);
        self.bits |= (val as u32 & 0x3) << 0x1a;
        self
    }
    #[inline(always)]
    #[doc = "Functional mode"]
    pub const fn fmode(self) -> u8 {
        ((self.bits >> 0x1a) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Send instruction only once mode"]
    pub const fn set_sioo(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Send instruction only once mode"]
    pub const fn sioo(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Free-running clock mode (not available on all chips!)"]
    pub const fn set_frcm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Free-running clock mode (not available on all chips!)"]
    pub const fn frcm(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DDR hold half cycle"]
    pub const fn set_dhhc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DDR hold half cycle"]
    pub const fn dhhc(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Double data rate mode"]
    pub const fn set_ddrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Double data rate mode"]
    pub const fn ddrm(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
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
    #[doc = "Abort request"]
    pub const fn set_abort(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Abort request"]
    pub const fn abort(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA enable (not available on all chips!)"]
    pub const fn set_dmaen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA enable (not available on all chips!)"]
    pub const fn dmaen(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timeout counter enable"]
    pub const fn set_tcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timeout counter enable"]
    pub const fn tcen(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Sample shift"]
    pub const fn set_sshift(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Sample shift"]
    pub const fn sshift(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Dual-flash mode"]
    pub const fn set_dfm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Dual-flash mode"]
    pub const fn dfm(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FLASH memory selection"]
    pub const fn set_fsel(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FLASH memory selection"]
    pub const fn fsel(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IFO threshold level"]
    pub const fn set_fthres(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "IFO threshold level"]
    pub const fn fthres(self) -> u8 {
        ((self.bits >> 0x8) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Transfer error interrupt enable"]
    pub const fn set_teie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer error interrupt enable"]
    pub const fn teie(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer complete interrupt enable"]
    pub const fn set_tcie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer complete interrupt enable"]
    pub const fn tcie(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FIFO threshold interrupt enable"]
    pub const fn set_ftie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FIFO threshold interrupt enable"]
    pub const fn ftie(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Status match interrupt enable"]
    pub const fn set_smie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Status match interrupt enable"]
    pub const fn smie(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TimeOut interrupt enable"]
    pub const fn set_toie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TimeOut interrupt enable"]
    pub const fn toie(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Automatic poll mode stop"]
    pub const fn set_apms(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Automatic poll mode stop"]
    pub const fn apms(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Polling match mode"]
    pub const fn set_pmm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Polling match mode"]
    pub const fn pmm(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock prescaler"]
    pub const fn set_prescaler(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Clock prescaler"]
    pub const fn prescaler(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "device configuration register"]
pub struct DcrBits {
    bits: u32,
}
impl Default for DcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Mode 0 / mode 3"]
    pub const fn set_ckmode(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mode 0 / mode 3"]
    pub const fn ckmode(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Chip select high time"]
    pub const fn set_csht(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x8);
        self.bits |= (val as u32 & 0x7) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Chip select high time"]
    pub const fn csht(self) -> u8 {
        ((self.bits >> 0x8) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "FLASH memory size"]
    pub const fn set_fsize(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0x10);
        self.bits |= (val as u32 & 0x1f) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "FLASH memory size"]
    pub const fn fsize(self) -> u8 {
        ((self.bits >> 0x10) & 0x1f) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "data length register"]
pub struct DlrBits {
    bits: u32,
}
impl Default for DlrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DlrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Data length"]
    pub const fn set_dl(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Data length"]
    pub const fn dl(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
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
    #[doc = "Data"]
    pub const fn set_data(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Data"]
    pub const fn data(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "flag clear register"]
pub struct FcrBits {
    bits: u32,
}
impl Default for FcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clear transfer error flag"]
    pub const fn set_ctef(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear transfer error flag"]
    pub const fn ctef(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear transfer complete flag"]
    pub const fn set_ctcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear transfer complete flag"]
    pub const fn ctcf(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear status match flag"]
    pub const fn set_csmf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear status match flag"]
    pub const fn csmf(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear timeout flag"]
    pub const fn set_ctof(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear timeout flag"]
    pub const fn ctof(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "low-power timeout register"]
pub struct LptrBits {
    bits: u32,
}
impl Default for LptrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LptrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Timeout period"]
    pub const fn set_timeout(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Timeout period"]
    pub const fn timeout(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "polling interval register"]
pub struct PirBits {
    bits: u32,
}
impl Default for PirBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PirBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Polling interval"]
    pub const fn set_interval(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Polling interval"]
    pub const fn interval(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "polling status match register"]
pub struct PsmarBits {
    bits: u32,
}
impl Default for PsmarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PsmarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Status match"]
    pub const fn set_match_(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Status match"]
    pub const fn match_(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "polling status mask register"]
pub struct PsmkrBits {
    bits: u32,
}
impl Default for PsmkrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PsmkrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Status mask"]
    pub const fn set_mask(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Status mask"]
    pub const fn mask(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
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
    #[doc = "Transfer error flag"]
    pub const fn set_tef(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer error flag"]
    pub const fn tef(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer complete flag"]
    pub const fn set_tcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer complete flag"]
    pub const fn tcf(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FIFO threshold flag"]
    pub const fn set_ftf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FIFO threshold flag"]
    pub const fn ftf(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Status match flag"]
    pub const fn set_smf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Status match flag"]
    pub const fn smf(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timeout flag"]
    pub const fn set_tof(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timeout flag"]
    pub const fn tof(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Busy"]
    pub const fn set_busy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Busy"]
    pub const fn busy(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FIFO level"]
    pub const fn set_flevel(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x8);
        self.bits |= (val as u32 & 0x7f) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "FIFO level"]
    pub const fn flevel(self) -> u8 {
        ((self.bits >> 0x8) & 0x7f) as _
    }
}
