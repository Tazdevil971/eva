
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Flexible static memory controller"]
pub struct Fsmc {
    ptr: *mut u8,
}
impl Fsmc {
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
    #[doc = "SRAM/NOR-Flash chip-select control register 1-4"]
    pub const fn bcr(&self, idx: usize) -> utils::Reg<BcrBits, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x0 + idx * 0x8);
            <utils::Reg<BcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SRAM/NOR-Flash chip-select timing register 1-4"]
    pub const fn btr(&self, idx: usize) -> utils::Reg<BtrBits, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x4 + idx * 0x8);
            <utils::Reg<BtrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "PC Card/NAND Flash control register 2-4"]
    pub const fn pcr(&self, idx: usize) -> utils::Reg<PcrBits, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x60 + idx * 0x20);
            <utils::Reg<PcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "FIFO status and interrupt register 2-4"]
    pub const fn sr(&self, idx: usize) -> utils::Reg<SrBits, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x64 + idx * 0x20);
            <utils::Reg<SrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Common memory space timing register 2-4"]
    pub const fn pmem(&self, idx: usize) -> utils::Reg<PmemBits, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x68 + idx * 0x20);
            <utils::Reg<PmemBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Attribute memory space timing register 2-4"]
    pub const fn patt(&self, idx: usize) -> utils::Reg<PattBits, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x6c + idx * 0x20);
            <utils::Reg<PattBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "ECC result register 2-3"]
    pub const fn eccr(&self, idx: usize) -> utils::Reg<EccrBits, utils::RO> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x74 + idx * 0x20);
            <utils::Reg<EccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "I/O space timing register 4"]
    pub const fn pio4(&self) -> utils::Reg<Pio4Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xb0);
            <utils::Reg<Pio4Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SRAM/NOR-Flash write timing registers 1-4"]
    pub const fn bwtr(&self, idx: usize) -> utils::Reg<BwtrBits, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x104 + idx * 0x8);
            <utils::Reg<BwtrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "SRAM/NOR-Flash chip-select control register"]
pub struct BcrBits {
    bits: u32,
}
impl Default for BcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Memory bank enable bit"]
    pub const fn set_mbken(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Memory bank enable bit"]
    pub const fn mbken(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Address/data multiplexing enable bit"]
    pub const fn set_muxen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Address/data multiplexing enable bit"]
    pub const fn muxen(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Memory type"]
    pub const fn set_mtyp(mut self, val: MtypVal) -> Self {
        self.bits &= !(0x3 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Memory type"]
    pub const fn mtyp(self) -> MtypVal {
        let val = ((self.bits >> 0x2) & 0x3) as _;
        unsafe { MtypVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Memory data bus width"]
    pub const fn set_mwid(mut self, val: MwidVal) -> Self {
        self.bits &= !(0x3 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Memory data bus width"]
    pub const fn mwid(self) -> MwidVal {
        let val = ((self.bits >> 0x4) & 0x3) as _;
        unsafe { MwidVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Flash access enable"]
    pub const fn set_faccen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Flash access enable"]
    pub const fn faccen(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Burst enable bit"]
    pub const fn set_bursten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Burst enable bit"]
    pub const fn bursten(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wait signal polarity bit"]
    pub const fn set_waitpol(mut self, val: WaitpolVal) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "Wait signal polarity bit"]
    pub const fn waitpol(self) -> WaitpolVal {
        let val = ((self.bits >> 0x9) & 0x1) as _;
        unsafe { WaitpolVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "WRAPMOD"]
    pub const fn set_wrapmod(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "WRAPMOD"]
    pub const fn wrapmod(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wait timing configuration"]
    pub const fn set_waitcfg(mut self, val: WaitcfgVal) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "Wait timing configuration"]
    pub const fn waitcfg(self) -> WaitcfgVal {
        let val = ((self.bits >> 0xb) & 0x1) as _;
        unsafe { WaitcfgVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Write enable bit"]
    pub const fn set_wren(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Write enable bit"]
    pub const fn wren(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wait enable bit"]
    pub const fn set_waiten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wait enable bit"]
    pub const fn waiten(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Extended mode enable"]
    pub const fn set_extmod(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Extended mode enable"]
    pub const fn extmod(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wait signal during asynchronous transfers"]
    pub const fn set_asyncwait(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wait signal during asynchronous transfers"]
    pub const fn asyncwait(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CRAM page size"]
    pub const fn set_cpsize(mut self, val: CpsizeVal) -> Self {
        self.bits &= !(0x7 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "CRAM page size"]
    pub const fn cpsize(self) -> CpsizeVal {
        let val = ((self.bits >> 0x10) & 0x7) as _;
        unsafe { CpsizeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Write burst enable"]
    pub const fn set_cburstrw(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Write burst enable"]
    pub const fn cburstrw(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "SRAM/NOR-Flash chip-select timing register"]
pub struct BtrBits {
    bits: u32,
}
impl Default for BtrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BtrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Address setup phase duration"]
    pub const fn set_addset(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Address setup phase duration"]
    pub const fn addset(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Address-hold phase duration"]
    pub const fn set_addhld(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x4);
        self.bits |= (val as u32 & 0xf) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Address-hold phase duration"]
    pub const fn addhld(self) -> u8 {
        ((self.bits >> 0x4) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Data-phase duration"]
    pub const fn set_datast(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Data-phase duration"]
    pub const fn datast(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Bus turnaround phase duration"]
    pub const fn set_busturn(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x10);
        self.bits |= (val as u32 & 0xf) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Bus turnaround phase duration"]
    pub const fn busturn(self) -> u8 {
        ((self.bits >> 0x10) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Clock divide ratio (for FMC_CLK signal)"]
    pub const fn set_clkdiv(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x14);
        self.bits |= (val as u32 & 0xf) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Clock divide ratio (for FMC_CLK signal)"]
    pub const fn clkdiv(self) -> u8 {
        ((self.bits >> 0x14) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Data latency for synchronous memory"]
    pub const fn set_datlat(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x18);
        self.bits |= (val as u32 & 0xf) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Data latency for synchronous memory"]
    pub const fn datlat(self) -> u8 {
        ((self.bits >> 0x18) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Access mode"]
    pub const fn set_accmod(mut self, val: AccmodVal) -> Self {
        self.bits &= !(0x3 << 0x1c);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "Access mode"]
    pub const fn accmod(self) -> AccmodVal {
        let val = ((self.bits >> 0x1c) & 0x3) as _;
        unsafe { AccmodVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "SRAM/NOR-Flash write timing registers"]
pub struct BwtrBits {
    bits: u32,
}
impl Default for BwtrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BwtrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Address setup phase duration"]
    pub const fn set_addset(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Address setup phase duration"]
    pub const fn addset(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Address-hold phase duration"]
    pub const fn set_addhld(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x4);
        self.bits |= (val as u32 & 0xf) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Address-hold phase duration"]
    pub const fn addhld(self) -> u8 {
        ((self.bits >> 0x4) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Data-phase duration"]
    pub const fn set_datast(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Data-phase duration"]
    pub const fn datast(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Bus turnaround phase duration"]
    pub const fn set_busturn(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x10);
        self.bits |= (val as u32 & 0xf) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Bus turnaround phase duration"]
    pub const fn busturn(self) -> u8 {
        ((self.bits >> 0x10) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Access mode"]
    pub const fn set_accmod(mut self, val: AccmodVal) -> Self {
        self.bits &= !(0x3 << 0x1c);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "Access mode"]
    pub const fn accmod(self) -> AccmodVal {
        let val = ((self.bits >> 0x1c) & 0x3) as _;
        unsafe { AccmodVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "ECC result register"]
pub struct EccrBits {
    bits: u32,
}
impl Default for EccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl EccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "ECC computation result value"]
    pub const fn set_ecc(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "ECC computation result value"]
    pub const fn ecc(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Attribute memory space timing register"]
pub struct PattBits {
    bits: u32,
}
impl Default for PattBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PattBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Attribute memory setup time"]
    pub const fn set_attset(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Attribute memory setup time"]
    pub const fn attset(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Attribute memory wait time"]
    pub const fn set_attwait(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Attribute memory wait time"]
    pub const fn attwait(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Attribute memory hold time"]
    pub const fn set_atthold(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Attribute memory hold time"]
    pub const fn atthold(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Attribute memory data bus Hi-Z time"]
    pub const fn set_atthiz(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Attribute memory data bus Hi-Z time"]
    pub const fn atthiz(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "PC Card/NAND Flash control register"]
pub struct PcrBits {
    bits: u32,
}
impl Default for PcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Wait feature enable bit"]
    pub const fn set_pwaiten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wait feature enable bit"]
    pub const fn pwaiten(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "NAND Flash memory bank enable bit"]
    pub const fn set_pbken(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "NAND Flash memory bank enable bit"]
    pub const fn pbken(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Memory type"]
    pub const fn set_ptyp(mut self, val: PtypVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Memory type"]
    pub const fn ptyp(self) -> PtypVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { PtypVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Data bus width"]
    pub const fn set_pwid(mut self, val: PwidVal) -> Self {
        self.bits &= !(0x3 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Data bus width"]
    pub const fn pwid(self) -> PwidVal {
        let val = ((self.bits >> 0x4) & 0x3) as _;
        unsafe { PwidVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "ECC computation logic enable bit"]
    pub const fn set_eccen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ECC computation logic enable bit"]
    pub const fn eccen(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CLE to RE delay"]
    pub const fn set_tclr(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x9);
        self.bits |= (val as u32 & 0xf) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "CLE to RE delay"]
    pub const fn tclr(self) -> u8 {
        ((self.bits >> 0x9) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "ALE to RE delay"]
    pub const fn set_tar(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0xd);
        self.bits |= (val as u32 & 0xf) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "ALE to RE delay"]
    pub const fn tar(self) -> u8 {
        ((self.bits >> 0xd) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "ECC page size"]
    pub const fn set_eccps(mut self, val: EccpsVal) -> Self {
        self.bits &= !(0x7 << 0x11);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "ECC page size"]
    pub const fn eccps(self) -> EccpsVal {
        let val = ((self.bits >> 0x11) & 0x7) as _;
        unsafe { EccpsVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "I/O space timing register 4"]
pub struct Pio4Bits {
    bits: u32,
}
impl Default for Pio4Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Pio4Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "IOSETx"]
    pub const fn set_iose_tx(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "IOSETx"]
    pub const fn iose_tx(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "IOWAITx"]
    pub const fn set_iowai_tx(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "IOWAITx"]
    pub const fn iowai_tx(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "IOHOLDx"]
    pub const fn set_iohol_dx(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "IOHOLDx"]
    pub const fn iohol_dx(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "IOHIZx"]
    pub const fn set_iohi_zx(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "IOHIZx"]
    pub const fn iohi_zx(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Common memory space timing register"]
pub struct PmemBits {
    bits: u32,
}
impl Default for PmemBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PmemBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Common memory x setup time"]
    pub const fn set_memset(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Common memory x setup time"]
    pub const fn memset(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Common memory wait time"]
    pub const fn set_memwait(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Common memory wait time"]
    pub const fn memwait(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Common memory hold time"]
    pub const fn set_memhold(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Common memory hold time"]
    pub const fn memhold(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Common memory x data bus Hi-Z time"]
    pub const fn set_memhiz(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Common memory x data bus Hi-Z time"]
    pub const fn memhiz(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "FIFO status and interrupt register"]
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
    #[doc = "Interrupt rising edge status"]
    pub const fn set_irs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt rising edge status"]
    pub const fn irs(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Interrupt high-level status"]
    pub const fn set_ils(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt high-level status"]
    pub const fn ils(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Interrupt falling edge status"]
    pub const fn set_ifs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt falling edge status"]
    pub const fn ifs(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Interrupt rising edge detection enable bit"]
    pub const fn set_iren(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt rising edge detection enable bit"]
    pub const fn iren(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Interrupt high-level detection enable bit"]
    pub const fn set_ilen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt high-level detection enable bit"]
    pub const fn ilen(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Interrupt falling edge detection enable bit"]
    pub const fn set_ifen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt falling edge detection enable bit"]
    pub const fn ifen(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FIFO empty status"]
    pub const fn set_fempt(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FIFO empty status"]
    pub const fn fempt(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AccmodVal {
    #[doc = "Access mode A"]
    A = 0x0,
    #[doc = "Access mode B"]
    B = 0x1,
    #[doc = "Access mode C"]
    C = 0x2,
    #[doc = "Access mode D"]
    D = 0x3,
}
impl AccmodVal {
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
pub enum CpsizeVal {
    #[doc = "No burst split when crossing page boundary"]
    NoBurstSplit = 0x0,
    #[doc = "128 bytes CRAM page size"]
    Bytes128 = 0x1,
    #[doc = "256 bytes CRAM page size"]
    Bytes256 = 0x2,
    #[doc = "512 bytes CRAM page size"]
    Bytes512 = 0x3,
    #[doc = "1024 bytes CRAM page size"]
    Bytes1024 = 0x4,
}
impl CpsizeVal {
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
pub enum EccpsVal {
    #[doc = "ECC page size 256 bytes"]
    Bytes256 = 0x0,
    #[doc = "ECC page size 512 bytes"]
    Bytes512 = 0x1,
    #[doc = "ECC page size 1024 bytes"]
    Bytes1024 = 0x2,
    #[doc = "ECC page size 2048 bytes"]
    Bytes2048 = 0x3,
    #[doc = "ECC page size 4096 bytes"]
    Bytes4096 = 0x4,
    #[doc = "ECC page size 8192 bytes"]
    Bytes8192 = 0x5,
}
impl EccpsVal {
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
pub enum MtypVal {
    #[doc = "SRAM memory type"]
    Sram = 0x0,
    #[doc = "PSRAM (CRAM) memory type"]
    Psram = 0x1,
    #[doc = "NOR Flash/OneNAND Flash"]
    Flash = 0x2,
}
impl MtypVal {
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
pub enum MwidVal {
    #[doc = "Memory data bus width 8 bits"]
    Bits8 = 0x0,
    #[doc = "Memory data bus width 16 bits"]
    Bits16 = 0x1,
    #[doc = "Memory data bus width 32 bits"]
    Bits32 = 0x2,
}
impl MwidVal {
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
pub enum PtypVal {
    #[doc = "NAND Flash"]
    NandFlash = 0x1,
}
impl PtypVal {
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
pub enum PwidVal {
    #[doc = "External memory device width 8 bits"]
    Bits8 = 0x0,
    #[doc = "External memory device width 16 bits"]
    Bits16 = 0x1,
}
impl PwidVal {
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
pub enum WaitcfgVal {
    #[doc = "NWAIT signal is active one data cycle before wait state"]
    BeforeWaitState = 0x0,
    #[doc = "NWAIT signal is active during wait state"]
    DuringWaitState = 0x1,
}
impl WaitcfgVal {
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
pub enum WaitpolVal {
    #[doc = "NWAIT active low"]
    ActiveLow = 0x0,
    #[doc = "NWAIT active high"]
    ActiveHigh = 0x1,
}
impl WaitpolVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
