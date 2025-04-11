
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "DMA controller"]
pub struct Dma {
    ptr: *mut u8,
}
impl Dma {
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
    #[doc = "low interrupt status register"]
    pub const fn isr(&self, idx: usize) -> utils::Reg<IxrBits, utils::RO> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x0 + idx * 0x4);
            <utils::Reg<IxrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "low interrupt flag clear register"]
    pub const fn ifcr(&self, idx: usize) -> utils::Reg<IxrBits, utils::WO> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x8 + idx * 0x4);
            <utils::Reg<IxrBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
    pub const fn st(&self, idx: usize) -> St {
        assert!(idx < 8);
        unsafe {
            let ptr = self.ptr.add(0x10 + idx * 0x18);
            <St>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
pub struct St {
    ptr: *mut u8,
}
impl St {
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
    #[doc = "stream x configuration register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "stream x number of data register"]
    pub const fn ndtr(&self) -> utils::Reg<NdtrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<NdtrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "stream x peripheral address register"]
    pub const fn par(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "stream x memory 0 address register"]
    pub const fn m0ar(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "stream x memory 1 address register"]
    pub const fn m1ar(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "stream x FIFO control register"]
    pub const fn fcr(&self) -> utils::Reg<FcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<FcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "stream x configuration register"]
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
    #[doc = "Stream enable / flag stream ready when read low"]
    pub const fn set_en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Stream enable / flag stream ready when read low"]
    pub const fn en(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Direct mode error interrupt enable"]
    pub const fn set_dmeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Direct mode error interrupt enable"]
    pub const fn dmeie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer error interrupt enable"]
    pub const fn set_teie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer error interrupt enable"]
    pub const fn teie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Half transfer interrupt enable"]
    pub const fn set_htie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Half transfer interrupt enable"]
    pub const fn htie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer complete interrupt enable"]
    pub const fn set_tcie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer complete interrupt enable"]
    pub const fn tcie(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Peripheral flow controller"]
    pub const fn set_pfctrl(mut self, val: PfctrlVal) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Peripheral flow controller"]
    pub const fn pfctrl(self) -> PfctrlVal {
        let val = ((self.bits >> 0x5) & 0x1) as _;
        unsafe { PfctrlVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Data transfer direction"]
    pub const fn set_dir(mut self, val: DirVal) -> Self {
        self.bits &= !(0x3 << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Data transfer direction"]
    pub const fn dir(self) -> DirVal {
        let val = ((self.bits >> 0x6) & 0x3) as _;
        unsafe { DirVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Circular mode enabled"]
    pub const fn set_circ(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Circular mode enabled"]
    pub const fn circ(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Peripheral increment mode enabled"]
    pub const fn set_pinc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Peripheral increment mode enabled"]
    pub const fn pinc(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Memory increment mode enabled"]
    pub const fn set_minc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Memory increment mode enabled"]
    pub const fn minc(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Peripheral data size"]
    pub const fn set_psize(mut self, val: SizeVal) -> Self {
        self.bits &= !(0x3 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "Peripheral data size"]
    pub const fn psize(self) -> SizeVal {
        let val = ((self.bits >> 0xb) & 0x3) as _;
        unsafe { SizeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Memory data size"]
    pub const fn set_msize(mut self, val: SizeVal) -> Self {
        self.bits &= !(0x3 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Memory data size"]
    pub const fn msize(self) -> SizeVal {
        let val = ((self.bits >> 0xd) & 0x3) as _;
        unsafe { SizeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Peripheral increment offset size"]
    pub const fn set_pincos(mut self, val: PincosVal) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
        self
    }
    #[inline(always)]
    #[doc = "Peripheral increment offset size"]
    pub const fn pincos(self) -> PincosVal {
        let val = ((self.bits >> 0xf) & 0x1) as _;
        unsafe { PincosVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Priority level"]
    pub const fn set_pl(mut self, val: PlVal) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Priority level"]
    pub const fn pl(self) -> PlVal {
        let val = ((self.bits >> 0x10) & 0x3) as _;
        unsafe { PlVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Double buffer mode enabled"]
    pub const fn set_dbm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Double buffer mode enabled"]
    pub const fn dbm(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Current target (only in double buffer mode)"]
    pub const fn set_ct(mut self, val: CtVal) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x13;
        self
    }
    #[inline(always)]
    #[doc = "Current target (only in double buffer mode)"]
    pub const fn ct(self) -> CtVal {
        let val = ((self.bits >> 0x13) & 0x1) as _;
        unsafe { CtVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Peripheral burst transfer configuration"]
    pub const fn set_pburst(mut self, val: BurstVal) -> Self {
        self.bits &= !(0x3 << 0x15);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x15;
        self
    }
    #[inline(always)]
    #[doc = "Peripheral burst transfer configuration"]
    pub const fn pburst(self) -> BurstVal {
        let val = ((self.bits >> 0x15) & 0x3) as _;
        unsafe { BurstVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Memory burst transfer configuration"]
    pub const fn set_mburst(mut self, val: BurstVal) -> Self {
        self.bits &= !(0x3 << 0x17);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x17;
        self
    }
    #[inline(always)]
    #[doc = "Memory burst transfer configuration"]
    pub const fn mburst(self) -> BurstVal {
        let val = ((self.bits >> 0x17) & 0x3) as _;
        unsafe { BurstVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Channel selection"]
    pub const fn set_chsel(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x19);
        self.bits |= (val as u32 & 0xf) << 0x19;
        self
    }
    #[inline(always)]
    #[doc = "Channel selection"]
    pub const fn chsel(self) -> u8 {
        ((self.bits >> 0x19) & 0xf) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "stream x FIFO control register"]
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
    #[doc = "FIFO threshold selection"]
    pub const fn set_fth(mut self, val: FthVal) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "FIFO threshold selection"]
    pub const fn fth(self) -> FthVal {
        let val = ((self.bits >> 0x0) & 0x3) as _;
        unsafe { FthVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Direct mode disable"]
    pub const fn set_dmdis(mut self, val: DmdisVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Direct mode disable"]
    pub const fn dmdis(self) -> DmdisVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { DmdisVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "FIFO status"]
    pub const fn set_fs(mut self, val: FsVal) -> Self {
        self.bits &= !(0x7 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "FIFO status"]
    pub const fn fs(self) -> FsVal {
        let val = ((self.bits >> 0x3) & 0x7) as _;
        unsafe { FsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "FIFO error interrupt enable"]
    pub const fn set_feie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FIFO error interrupt enable"]
    pub const fn feie(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "interrupt register"]
pub struct IxrBits {
    bits: u32,
}
impl Default for IxrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IxrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "stream x number of data register"]
pub struct NdtrBits {
    bits: u32,
}
impl Default for NdtrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl NdtrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Number of data items to transfer"]
    pub const fn set_ndt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Number of data items to transfer"]
    pub const fn ndt(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BurstVal {
    #[doc = "Single transfer"]
    Single = 0x0,
    #[doc = "Incremental burst of 4 beats"]
    Incr4 = 0x1,
    #[doc = "Incremental burst of 8 beats"]
    Incr8 = 0x2,
    #[doc = "Incremental burst of 16 beats"]
    Incr16 = 0x3,
}
impl BurstVal {
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
pub enum CtVal {
    #[doc = "The current target memory is Memory 0"]
    Memory0 = 0x0,
    #[doc = "The current target memory is Memory 1"]
    Memory1 = 0x1,
}
impl CtVal {
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
pub enum DirVal {
    #[doc = "Peripheral-to-memory"]
    PeripheralToMemory = 0x0,
    #[doc = "Memory-to-peripheral"]
    MemoryToPeripheral = 0x1,
    #[doc = "Memory-to-memory"]
    MemoryToMemory = 0x2,
}
impl DirVal {
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
pub enum DmdisVal {
    #[doc = "Direct mode is enabled"]
    Enabled = 0x0,
    #[doc = "Direct mode is disabled"]
    Disabled = 0x1,
}
impl DmdisVal {
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
pub enum FsVal {
    #[doc = "0 < fifo_level < 1/4"]
    Quarter1 = 0x0,
    #[doc = "1/4 <= fifo_level < 1/2"]
    Quarter2 = 0x1,
    #[doc = "1/2 <= fifo_level < 3/4"]
    Quarter3 = 0x2,
    #[doc = "3/4 <= fifo_level < full"]
    Quarter4 = 0x3,
    #[doc = "FIFO is empty"]
    Empty = 0x4,
    #[doc = "FIFO is full"]
    Full = 0x5,
}
impl FsVal {
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
pub enum FthVal {
    #[doc = "1/4 full FIFO"]
    Quarter = 0x0,
    #[doc = "1/2 full FIFO"]
    Half = 0x1,
    #[doc = "3/4 full FIFO"]
    ThreeQuarters = 0x2,
    #[doc = "Full FIFO"]
    Full = 0x3,
}
impl FthVal {
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
pub enum PfctrlVal {
    #[doc = "The DMA is the flow controller"]
    Dma = 0x0,
    #[doc = "The peripheral is the flow controller"]
    Peripheral = 0x1,
}
impl PfctrlVal {
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
pub enum PincosVal {
    #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
    Psize = 0x0,
    #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    Fixed4 = 0x1,
}
impl PincosVal {
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
pub enum PlVal {
    #[doc = "Low"]
    Low = 0x0,
    #[doc = "Medium"]
    Medium = 0x1,
    #[doc = "High"]
    High = 0x2,
    #[doc = "Very high"]
    VeryHigh = 0x3,
}
impl PlVal {
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
pub enum SizeVal {
    #[doc = "Byte (8-bit)"]
    Bits8 = 0x0,
    #[doc = "Half-word (16-bit)"]
    Bits16 = 0x1,
    #[doc = "Word (32-bit)"]
    Bits32 = 0x2,
}
impl SizeVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
