
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "DMA controller"]
pub struct Dma {
    ptr: *mut u8,
}
impl Dma {
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
    pub const fn isr(&self, idx: usize) -> utils::Reg<fields::Ixr, utils::RO> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x0 + idx * 0x4);
            <utils::Reg<fields::Ixr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "low interrupt flag clear register"]
    pub const fn ifcr(&self, idx: usize) -> utils::Reg<fields::Ixr, utils::WO> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x8 + idx * 0x4);
            <utils::Reg<fields::Ixr, utils::WO>>::from_ptr(ptr)
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
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "stream x configuration register"]
    pub const fn cr(&self) -> utils::Reg<fields::Cr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "stream x number of data register"]
    pub const fn ndtr(&self) -> utils::Reg<fields::Ndtr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Ndtr, utils::RW>>::from_ptr(ptr)
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
    pub const fn fcr(&self) -> utils::Reg<fields::Fcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Fcr, utils::RW>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "stream x configuration register"]
    pub struct Cr {
        bits: u32,
    }
    impl Default for Cr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr {
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
        pub const fn set_pfctrl(mut self, val: vals::Pfctrl) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
            self
        }
        #[inline(always)]
        #[doc = "Peripheral flow controller"]
        pub const fn pfctrl(self) -> vals::Pfctrl {
            let val = ((self.bits >> 0x5) & 0x1) as _;
            unsafe { vals::Pfctrl::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Data transfer direction"]
        pub const fn set_dir(mut self, val: vals::Dir) -> Self {
            self.bits &= !(0x3 << 0x6);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x6;
            self
        }
        #[inline(always)]
        #[doc = "Data transfer direction"]
        pub const fn dir(self) -> vals::Dir {
            let val = ((self.bits >> 0x6) & 0x3) as _;
            unsafe { vals::Dir::from_bits_unchecked(val) }
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
        pub const fn set_psize(mut self, val: vals::Size) -> Self {
            self.bits &= !(0x3 << 0xb);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0xb;
            self
        }
        #[inline(always)]
        #[doc = "Peripheral data size"]
        pub const fn psize(self) -> vals::Size {
            let val = ((self.bits >> 0xb) & 0x3) as _;
            unsafe { vals::Size::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Memory data size"]
        pub const fn set_msize(mut self, val: vals::Size) -> Self {
            self.bits &= !(0x3 << 0xd);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0xd;
            self
        }
        #[inline(always)]
        #[doc = "Memory data size"]
        pub const fn msize(self) -> vals::Size {
            let val = ((self.bits >> 0xd) & 0x3) as _;
            unsafe { vals::Size::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Peripheral increment offset size"]
        pub const fn set_pincos(mut self, val: vals::Pincos) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
            self
        }
        #[inline(always)]
        #[doc = "Peripheral increment offset size"]
        pub const fn pincos(self) -> vals::Pincos {
            let val = ((self.bits >> 0xf) & 0x1) as _;
            unsafe { vals::Pincos::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Priority level"]
        pub const fn set_pl(mut self, val: vals::Pl) -> Self {
            self.bits &= !(0x3 << 0x10);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Priority level"]
        pub const fn pl(self) -> vals::Pl {
            let val = ((self.bits >> 0x10) & 0x3) as _;
            unsafe { vals::Pl::from_bits_unchecked(val) }
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
        pub const fn set_ct(mut self, val: vals::Ct) -> Self {
            self.bits &= !(0x1 << 0x13);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x13;
            self
        }
        #[inline(always)]
        #[doc = "Current target (only in double buffer mode)"]
        pub const fn ct(self) -> vals::Ct {
            let val = ((self.bits >> 0x13) & 0x1) as _;
            unsafe { vals::Ct::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Peripheral burst transfer configuration"]
        pub const fn set_pburst(mut self, val: vals::Burst) -> Self {
            self.bits &= !(0x3 << 0x15);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x15;
            self
        }
        #[inline(always)]
        #[doc = "Peripheral burst transfer configuration"]
        pub const fn pburst(self) -> vals::Burst {
            let val = ((self.bits >> 0x15) & 0x3) as _;
            unsafe { vals::Burst::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Memory burst transfer configuration"]
        pub const fn set_mburst(mut self, val: vals::Burst) -> Self {
            self.bits &= !(0x3 << 0x17);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x17;
            self
        }
        #[inline(always)]
        #[doc = "Memory burst transfer configuration"]
        pub const fn mburst(self) -> vals::Burst {
            let val = ((self.bits >> 0x17) & 0x3) as _;
            unsafe { vals::Burst::from_bits_unchecked(val) }
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
    pub struct Fcr {
        bits: u32,
    }
    impl Default for Fcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fcr {
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
        pub const fn set_fth(mut self, val: vals::Fth) -> Self {
            self.bits &= !(0x3 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "FIFO threshold selection"]
        pub const fn fth(self) -> vals::Fth {
            let val = ((self.bits >> 0x0) & 0x3) as _;
            unsafe { vals::Fth::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Direct mode disable"]
        pub const fn set_dmdis(mut self, val: vals::Dmdis) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Direct mode disable"]
        pub const fn dmdis(self) -> vals::Dmdis {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Dmdis::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "FIFO status"]
        pub const fn set_fs(mut self, val: vals::Fs) -> Self {
            self.bits &= !(0x7 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "FIFO status"]
        pub const fn fs(self) -> vals::Fs {
            let val = ((self.bits >> 0x3) & 0x7) as _;
            unsafe { vals::Fs::from_bits_unchecked(val) }
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
    pub struct Ixr {
        bits: u32,
    }
    impl Default for Ixr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ixr {
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
    pub struct Ndtr {
        bits: u32,
    }
    impl Default for Ndtr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ndtr {
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
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum Burst {
        #[doc = "Single transfer"]
        Single = 0x0,
        #[doc = "Incremental burst of 4 beats"]
        Incr4 = 0x1,
        #[doc = "Incremental burst of 8 beats"]
        Incr8 = 0x2,
        #[doc = "Incremental burst of 16 beats"]
        Incr16 = 0x3,
    }
    impl Burst {
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
    pub enum Ct {
        #[doc = "The current target memory is Memory 0"]
        Memory0 = 0x0,
        #[doc = "The current target memory is Memory 1"]
        Memory1 = 0x1,
    }
    impl Ct {
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
    pub enum Dir {
        #[doc = "Peripheral-to-memory"]
        PeripheralToMemory = 0x0,
        #[doc = "Memory-to-peripheral"]
        MemoryToPeripheral = 0x1,
        #[doc = "Memory-to-memory"]
        MemoryToMemory = 0x2,
    }
    impl Dir {
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
    pub enum Dmdis {
        #[doc = "Direct mode is enabled"]
        Enabled = 0x0,
        #[doc = "Direct mode is disabled"]
        Disabled = 0x1,
    }
    impl Dmdis {
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
    pub enum Fs {
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
    impl Fs {
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
    pub enum Fth {
        #[doc = "1/4 full FIFO"]
        Quarter = 0x0,
        #[doc = "1/2 full FIFO"]
        Half = 0x1,
        #[doc = "3/4 full FIFO"]
        ThreeQuarters = 0x2,
        #[doc = "Full FIFO"]
        Full = 0x3,
    }
    impl Fth {
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
    pub enum Pfctrl {
        #[doc = "The DMA is the flow controller"]
        Dma = 0x0,
        #[doc = "The peripheral is the flow controller"]
        Peripheral = 0x1,
    }
    impl Pfctrl {
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
    pub enum Pincos {
        #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
        Psize = 0x0,
        #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
        Fixed4 = 0x1,
    }
    impl Pincos {
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
    pub enum Pl {
        #[doc = "Low"]
        Low = 0x0,
        #[doc = "Medium"]
        Medium = 0x1,
        #[doc = "High"]
        High = 0x2,
        #[doc = "Very high"]
        VeryHigh = 0x3,
    }
    impl Pl {
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
    pub enum Size {
        #[doc = "Byte (8-bit)"]
        Bits8 = 0x0,
        #[doc = "Half-word (16-bit)"]
        Bits16 = 0x1,
        #[doc = "Word (32-bit)"]
        Bits32 = 0x2,
    }
    impl Size {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
            unsafe { ::core::mem::transmute(bits) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            self as u8
        }
    }
}
