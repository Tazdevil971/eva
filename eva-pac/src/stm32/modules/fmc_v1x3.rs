
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Flexible memory controller"]
pub struct Fmc {
    ptr: *mut u8,
}
impl Fmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "SRAM/NOR-Flash chip-select control register 1"]
    pub const fn bcr1(&self) -> utils::Reg<fields::Bcr1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Bcr1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SRAM/NOR-Flash chip-select timing register 1-4"]
    pub const fn btr(&self, idx: usize) -> utils::Reg<fields::Btr, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x4 + idx * 0x8);
            <utils::Reg<fields::Btr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SRAM/NOR-Flash chip-select control register 2-4"]
    pub const fn bcr(&self, idx: usize) -> utils::Reg<fields::Bcr, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x8 + idx * 0x8);
            <utils::Reg<fields::Bcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "PC Card/NAND Flash control register 2-4"]
    pub const fn pcr(&self, idx: usize) -> utils::Reg<fields::Pcr, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x60 + idx * 0x20);
            <utils::Reg<fields::Pcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "FIFO status and interrupt register 2-4"]
    pub const fn sr(&self, idx: usize) -> utils::Reg<fields::Sr, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x64 + idx * 0x20);
            <utils::Reg<fields::Sr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Common memory space timing register 2-4"]
    pub const fn pmem(&self, idx: usize) -> utils::Reg<fields::Pmem, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x68 + idx * 0x20);
            <utils::Reg<fields::Pmem, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Attribute memory space timing register 2-4"]
    pub const fn patt(&self, idx: usize) -> utils::Reg<fields::Patt, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x6c + idx * 0x20);
            <utils::Reg<fields::Patt, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "ECC result register 2-3"]
    pub const fn eccr(&self, idx: usize) -> utils::Reg<fields::Eccr, utils::RO> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x74 + idx * 0x20);
            <utils::Reg<fields::Eccr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "I/O space timing register 4"]
    pub const fn pio4(&self) -> utils::Reg<fields::Pio4, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xb0);
            <utils::Reg<fields::Pio4, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SRAM/NOR-Flash write timing registers 1-4"]
    pub const fn bwtr(&self, idx: usize) -> utils::Reg<fields::Bwtr, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x104 + idx * 0x8);
            <utils::Reg<fields::Bwtr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SDRAM Control Register 1-2"]
    pub const fn sdcr(&self, idx: usize) -> utils::Reg<fields::Sdcr, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x140 + idx * 0x4);
            <utils::Reg<fields::Sdcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SDRAM Timing register 1-2"]
    pub const fn sdtr(&self, idx: usize) -> utils::Reg<fields::Sdtr, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x148 + idx * 0x4);
            <utils::Reg<fields::Sdtr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SDRAM Command Mode register"]
    pub const fn sdcmr(&self) -> utils::Reg<fields::Sdcmr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x150);
            <utils::Reg<fields::Sdcmr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SDRAM Refresh Timer register"]
    pub const fn sdrtr(&self) -> utils::Reg<fields::Sdrtr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x154);
            <utils::Reg<fields::Sdrtr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "SDRAM Status register"]
    pub const fn sdsr(&self) -> utils::Reg<fields::Sdsr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x158);
            <utils::Reg<fields::Sdsr, utils::RO>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "SRAM/NOR-Flash chip-select control register 2-4"]
    pub struct Bcr {
        bits: u32,
    }
    impl Default for Bcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bcr {
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
        pub const fn set_mtyp(mut self, val: vals::Mtyp) -> Self {
            self.bits &= !(0x3 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Memory type"]
        pub const fn mtyp(self) -> vals::Mtyp {
            let val = ((self.bits >> 0x2) & 0x3) as _;
            unsafe { vals::Mtyp::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Memory data bus width"]
        pub const fn set_mwid(mut self, val: vals::Mwid) -> Self {
            self.bits &= !(0x3 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Memory data bus width"]
        pub const fn mwid(self) -> vals::Mwid {
            let val = ((self.bits >> 0x4) & 0x3) as _;
            unsafe { vals::Mwid::from_bits_unchecked(val) }
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
        pub const fn set_waitpol(mut self, val: vals::Waitpol) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
            self
        }
        #[inline(always)]
        #[doc = "Wait signal polarity bit"]
        pub const fn waitpol(self) -> vals::Waitpol {
            let val = ((self.bits >> 0x9) & 0x1) as _;
            unsafe { vals::Waitpol::from_bits_unchecked(val) }
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
        pub const fn set_waitcfg(mut self, val: vals::Waitcfg) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
            self
        }
        #[inline(always)]
        #[doc = "Wait timing configuration"]
        pub const fn waitcfg(self) -> vals::Waitcfg {
            let val = ((self.bits >> 0xb) & 0x1) as _;
            unsafe { vals::Waitcfg::from_bits_unchecked(val) }
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
        pub const fn set_cpsize(mut self, val: vals::Cpsize) -> Self {
            self.bits &= !(0x7 << 0x10);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "CRAM page size"]
        pub const fn cpsize(self) -> vals::Cpsize {
            let val = ((self.bits >> 0x10) & 0x7) as _;
            unsafe { vals::Cpsize::from_bits_unchecked(val) }
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
    #[doc = "SRAM/NOR-Flash chip-select control register 1"]
    pub struct Bcr1 {
        bits: u32,
    }
    impl Default for Bcr1 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bcr1 {
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
        pub const fn set_mtyp(mut self, val: vals::Mtyp) -> Self {
            self.bits &= !(0x3 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Memory type"]
        pub const fn mtyp(self) -> vals::Mtyp {
            let val = ((self.bits >> 0x2) & 0x3) as _;
            unsafe { vals::Mtyp::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Memory data bus width"]
        pub const fn set_mwid(mut self, val: vals::Mwid) -> Self {
            self.bits &= !(0x3 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Memory data bus width"]
        pub const fn mwid(self) -> vals::Mwid {
            let val = ((self.bits >> 0x4) & 0x3) as _;
            unsafe { vals::Mwid::from_bits_unchecked(val) }
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
        pub const fn set_waitpol(mut self, val: vals::Waitpol) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
            self
        }
        #[inline(always)]
        #[doc = "Wait signal polarity bit"]
        pub const fn waitpol(self) -> vals::Waitpol {
            let val = ((self.bits >> 0x9) & 0x1) as _;
            unsafe { vals::Waitpol::from_bits_unchecked(val) }
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
        pub const fn set_waitcfg(mut self, val: vals::Waitcfg) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
            self
        }
        #[inline(always)]
        #[doc = "Wait timing configuration"]
        pub const fn waitcfg(self) -> vals::Waitcfg {
            let val = ((self.bits >> 0xb) & 0x1) as _;
            unsafe { vals::Waitcfg::from_bits_unchecked(val) }
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
        pub const fn set_cpsize(mut self, val: vals::Cpsize) -> Self {
            self.bits &= !(0x7 << 0x10);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "CRAM page size"]
        pub const fn cpsize(self) -> vals::Cpsize {
            let val = ((self.bits >> 0x10) & 0x7) as _;
            unsafe { vals::Cpsize::from_bits_unchecked(val) }
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
        #[inline(always)]
        #[doc = "Continuous clock enable"]
        pub const fn set_cclken(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x14);
            self.bits |= if val { 1 << 0x14 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Continuous clock enable"]
        pub const fn cclken(self) -> bool {
            ((self.bits >> 0x14) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "SRAM/NOR-Flash chip-select timing register"]
    pub struct Btr {
        bits: u32,
    }
    impl Default for Btr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Btr {
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
        pub const fn set_accmod(mut self, val: vals::Accmod) -> Self {
            self.bits &= !(0x3 << 0x1c);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x1c;
            self
        }
        #[inline(always)]
        #[doc = "Access mode"]
        pub const fn accmod(self) -> vals::Accmod {
            let val = ((self.bits >> 0x1c) & 0x3) as _;
            unsafe { vals::Accmod::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "SRAM/NOR-Flash write timing registers"]
    pub struct Bwtr {
        bits: u32,
    }
    impl Default for Bwtr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bwtr {
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
        pub const fn set_accmod(mut self, val: vals::Accmod) -> Self {
            self.bits &= !(0x3 << 0x1c);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x1c;
            self
        }
        #[inline(always)]
        #[doc = "Access mode"]
        pub const fn accmod(self) -> vals::Accmod {
            let val = ((self.bits >> 0x1c) & 0x3) as _;
            unsafe { vals::Accmod::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "ECC result register"]
    pub struct Eccr {
        bits: u32,
    }
    impl Default for Eccr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Eccr {
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
    pub struct Patt {
        bits: u32,
    }
    impl Default for Patt {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Patt {
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
    pub struct Pcr {
        bits: u32,
    }
    impl Default for Pcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Pcr {
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
        pub const fn set_ptyp(mut self, val: vals::Ptyp) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Memory type"]
        pub const fn ptyp(self) -> vals::Ptyp {
            let val = ((self.bits >> 0x3) & 0x1) as _;
            unsafe { vals::Ptyp::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Data bus width"]
        pub const fn set_pwid(mut self, val: vals::Pwid) -> Self {
            self.bits &= !(0x3 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Data bus width"]
        pub const fn pwid(self) -> vals::Pwid {
            let val = ((self.bits >> 0x4) & 0x3) as _;
            unsafe { vals::Pwid::from_bits_unchecked(val) }
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
        pub const fn set_eccps(mut self, val: vals::Eccps) -> Self {
            self.bits &= !(0x7 << 0x11);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x11;
            self
        }
        #[inline(always)]
        #[doc = "ECC page size"]
        pub const fn eccps(self) -> vals::Eccps {
            let val = ((self.bits >> 0x11) & 0x7) as _;
            unsafe { vals::Eccps::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "I/O space timing register 4"]
    pub struct Pio4 {
        bits: u32,
    }
    impl Default for Pio4 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Pio4 {
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
    pub struct Pmem {
        bits: u32,
    }
    impl Default for Pmem {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Pmem {
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
    #[doc = "SDRAM Command Mode register"]
    pub struct Sdcmr {
        bits: u32,
    }
    impl Default for Sdcmr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sdcmr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Command mode"]
        pub const fn set_mode(mut self, val: vals::Mode) -> Self {
            self.bits &= !(0x7 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Command mode"]
        pub const fn mode(self) -> vals::Mode {
            let val = ((self.bits >> 0x0) & 0x7) as _;
            unsafe { vals::Mode::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Command target bank 2"]
        pub const fn set_ctb2(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Command target bank 2"]
        pub const fn ctb2(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Command target bank 1"]
        pub const fn set_ctb1(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Command target bank 1"]
        pub const fn ctb1(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Number of Auto-refresh"]
        pub const fn set_nrfs(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x5);
            self.bits |= (val as u32 & 0xf) << 0x5;
            self
        }
        #[inline(always)]
        #[doc = "Number of Auto-refresh"]
        pub const fn nrfs(self) -> u8 {
            ((self.bits >> 0x5) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Mode Register definition"]
        pub const fn set_mrd(mut self, val: u16) -> Self {
            self.bits &= !(0x1fff << 0x9);
            self.bits |= (val as u32 & 0x1fff) << 0x9;
            self
        }
        #[inline(always)]
        #[doc = "Mode Register definition"]
        pub const fn mrd(self) -> u16 {
            ((self.bits >> 0x9) & 0x1fff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "SDRAM Control Register"]
    pub struct Sdcr {
        bits: u32,
    }
    impl Default for Sdcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sdcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Number of column address bits"]
        pub const fn set_nc(mut self, val: vals::Nc) -> Self {
            self.bits &= !(0x3 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Number of column address bits"]
        pub const fn nc(self) -> vals::Nc {
            let val = ((self.bits >> 0x0) & 0x3) as _;
            unsafe { vals::Nc::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Number of row address bits"]
        pub const fn set_nr(mut self, val: vals::Nr) -> Self {
            self.bits &= !(0x3 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Number of row address bits"]
        pub const fn nr(self) -> vals::Nr {
            let val = ((self.bits >> 0x2) & 0x3) as _;
            unsafe { vals::Nr::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Memory data bus width"]
        pub const fn set_mwid(mut self, val: vals::Mwid) -> Self {
            self.bits &= !(0x3 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Memory data bus width"]
        pub const fn mwid(self) -> vals::Mwid {
            let val = ((self.bits >> 0x4) & 0x3) as _;
            unsafe { vals::Mwid::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Number of internal banks"]
        pub const fn set_nb(mut self, val: vals::Nb) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x6;
            self
        }
        #[inline(always)]
        #[doc = "Number of internal banks"]
        pub const fn nb(self) -> vals::Nb {
            let val = ((self.bits >> 0x6) & 0x1) as _;
            unsafe { vals::Nb::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "CAS latency"]
        pub const fn set_cas(mut self, val: vals::Cas) -> Self {
            self.bits &= !(0x3 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "CAS latency"]
        pub const fn cas(self) -> vals::Cas {
            let val = ((self.bits >> 0x7) & 0x3) as _;
            unsafe { vals::Cas::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Write protection"]
        pub const fn set_wp(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Write protection"]
        pub const fn wp(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SDRAM clock configuration"]
        pub const fn set_sdclk(mut self, val: vals::Sdclk) -> Self {
            self.bits &= !(0x3 << 0xa);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0xa;
            self
        }
        #[inline(always)]
        #[doc = "SDRAM clock configuration"]
        pub const fn sdclk(self) -> vals::Sdclk {
            let val = ((self.bits >> 0xa) & 0x3) as _;
            unsafe { vals::Sdclk::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Burst read"]
        pub const fn set_rburst(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= if val { 1 << 0xc } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Burst read"]
        pub const fn rburst(self) -> bool {
            ((self.bits >> 0xc) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Read pipe"]
        pub const fn set_rpipe(mut self, val: vals::Rpipe) -> Self {
            self.bits &= !(0x3 << 0xd);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0xd;
            self
        }
        #[inline(always)]
        #[doc = "Read pipe"]
        pub const fn rpipe(self) -> vals::Rpipe {
            let val = ((self.bits >> 0xd) & 0x3) as _;
            unsafe { vals::Rpipe::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "SDRAM Refresh Timer register"]
    pub struct Sdrtr {
        bits: u32,
    }
    impl Default for Sdrtr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sdrtr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Clear Refresh error flag"]
        pub const fn set_cre(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Clear Refresh error flag"]
        pub const fn cre(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Refresh Timer Count"]
        pub const fn set_count(mut self, val: u16) -> Self {
            self.bits &= !(0x1fff << 0x1);
            self.bits |= (val as u32 & 0x1fff) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "Refresh Timer Count"]
        pub const fn count(self) -> u16 {
            ((self.bits >> 0x1) & 0x1fff) as _
        }
        #[inline(always)]
        #[doc = "RES Interrupt Enable"]
        pub const fn set_reie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RES Interrupt Enable"]
        pub const fn reie(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "SDRAM Status register"]
    pub struct Sdsr {
        bits: u32,
    }
    impl Default for Sdsr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sdsr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Refresh error flag"]
        pub const fn set_re(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Refresh error flag"]
        pub const fn re(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Status Mode for Bank 1"]
        pub const fn set_modes1(mut self, val: vals::Modes) -> Self {
            self.bits &= !(0x3 << 0x1);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "Status Mode for Bank 1"]
        pub const fn modes1(self) -> vals::Modes {
            let val = ((self.bits >> 0x1) & 0x3) as _;
            unsafe { vals::Modes::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Status Mode for Bank 2"]
        pub const fn set_modes2(mut self, val: vals::Modes) -> Self {
            self.bits &= !(0x3 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Status Mode for Bank 2"]
        pub const fn modes2(self) -> vals::Modes {
            let val = ((self.bits >> 0x3) & 0x3) as _;
            unsafe { vals::Modes::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Busy status"]
        pub const fn set_busy(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Busy status"]
        pub const fn busy(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "SDRAM Timing register"]
    pub struct Sdtr {
        bits: u32,
    }
    impl Default for Sdtr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sdtr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Load Mode Register to Active"]
        pub const fn set_tmrd(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x0);
            self.bits |= (val as u32 & 0xf) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Load Mode Register to Active"]
        pub const fn tmrd(self) -> u8 {
            ((self.bits >> 0x0) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Exit self-refresh delay"]
        pub const fn set_txsr(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x4);
            self.bits |= (val as u32 & 0xf) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Exit self-refresh delay"]
        pub const fn txsr(self) -> u8 {
            ((self.bits >> 0x4) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Self refresh time"]
        pub const fn set_tras(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x8);
            self.bits |= (val as u32 & 0xf) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Self refresh time"]
        pub const fn tras(self) -> u8 {
            ((self.bits >> 0x8) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Row cycle delay"]
        pub const fn set_trc(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0xc);
            self.bits |= (val as u32 & 0xf) << 0xc;
            self
        }
        #[inline(always)]
        #[doc = "Row cycle delay"]
        pub const fn trc(self) -> u8 {
            ((self.bits >> 0xc) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Recovery delay"]
        pub const fn set_twr(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x10);
            self.bits |= (val as u32 & 0xf) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Recovery delay"]
        pub const fn twr(self) -> u8 {
            ((self.bits >> 0x10) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Row precharge delay"]
        pub const fn set_trp(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x14);
            self.bits |= (val as u32 & 0xf) << 0x14;
            self
        }
        #[inline(always)]
        #[doc = "Row precharge delay"]
        pub const fn trp(self) -> u8 {
            ((self.bits >> 0x14) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Row to column delay"]
        pub const fn set_trcd(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x18);
            self.bits |= (val as u32 & 0xf) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "Row to column delay"]
        pub const fn trcd(self) -> u8 {
            ((self.bits >> 0x18) & 0xf) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "FIFO status and interrupt register"]
    pub struct Sr {
        bits: u32,
    }
    impl Default for Sr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sr {
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
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum Accmod {
        #[doc = "Access mode A"]
        A = 0x0,
        #[doc = "Access mode B"]
        B = 0x1,
        #[doc = "Access mode C"]
        C = 0x2,
        #[doc = "Access mode D"]
        D = 0x3,
    }
    impl Accmod {
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
    pub enum Cas {
        #[doc = "1 cycle"]
        Clocks1 = 0x1,
        #[doc = "2 cycles"]
        Clocks2 = 0x2,
        #[doc = "3 cycles"]
        Clocks3 = 0x3,
    }
    impl Cas {
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
    pub enum Cpsize {
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
    impl Cpsize {
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
    pub enum Eccps {
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
    impl Eccps {
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
    pub enum Mode {
        #[doc = "Normal Mode"]
        Normal = 0x0,
        #[doc = "Clock Configuration Enable"]
        ClockConfigurationEnable = 0x1,
        #[doc = "PALL (All Bank Precharge) command"]
        Pall = 0x2,
        #[doc = "Auto-refresh command"]
        AutoRefreshCommand = 0x3,
        #[doc = "Load Mode Resgier"]
        LoadModeRegister = 0x4,
        #[doc = "Self-refresh command"]
        SelfRefreshCommand = 0x5,
        #[doc = "Power-down command"]
        PowerDownCommand = 0x6,
    }
    impl Mode {
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
    pub enum Modes {
        #[doc = "Normal Mode"]
        Normal = 0x0,
        #[doc = "Self-refresh mode"]
        SelfRefresh = 0x1,
        #[doc = "Power-down mode"]
        PowerDown = 0x2,
    }
    impl Modes {
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
    pub enum Mtyp {
        #[doc = "SRAM memory type"]
        Sram = 0x0,
        #[doc = "PSRAM (CRAM) memory type"]
        Psram = 0x1,
        #[doc = "NOR Flash/OneNAND Flash"]
        Flash = 0x2,
    }
    impl Mtyp {
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
    pub enum Mwid {
        #[doc = "Memory data bus width 8 bits"]
        Bits8 = 0x0,
        #[doc = "Memory data bus width 16 bits"]
        Bits16 = 0x1,
        #[doc = "Memory data bus width 32 bits"]
        Bits32 = 0x2,
    }
    impl Mwid {
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
    pub enum Nb {
        #[doc = "Two internal Banks"]
        Nb2 = 0x0,
        #[doc = "Four internal Banks"]
        Nb4 = 0x1,
    }
    impl Nb {
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
    pub enum Nc {
        #[doc = "8 bits"]
        Bits8 = 0x0,
        #[doc = "9 bits"]
        Bits9 = 0x1,
        #[doc = "10 bits"]
        Bits10 = 0x2,
        #[doc = "11 bits"]
        Bits11 = 0x3,
    }
    impl Nc {
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
    pub enum Nr {
        #[doc = "11 bits"]
        Bits11 = 0x0,
        #[doc = "12 bits"]
        Bits12 = 0x1,
        #[doc = "13 bits"]
        Bits13 = 0x2,
    }
    impl Nr {
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
    pub enum Ptyp {
        #[doc = "NAND Flash"]
        NandFlash = 0x1,
    }
    impl Ptyp {
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
    pub enum Pwid {
        #[doc = "External memory device width 8 bits"]
        Bits8 = 0x0,
        #[doc = "External memory device width 16 bits"]
        Bits16 = 0x1,
    }
    impl Pwid {
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
    pub enum Rpipe {
        #[doc = "No clock cycle delay"]
        NoDelay = 0x0,
        #[doc = "One clock cycle delay"]
        Clocks1 = 0x1,
        #[doc = "Two clock cycles delay"]
        Clocks2 = 0x2,
    }
    impl Rpipe {
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
    pub enum Sdclk {
        #[doc = "SDCLK clock disabled"]
        Disabled = 0x0,
        #[doc = "SDCLK period = 2 x HCLK period"]
        Div2 = 0x2,
        #[doc = "SDCLK period = 3 x HCLK period"]
        Div3 = 0x3,
    }
    impl Sdclk {
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
    pub enum Waitcfg {
        #[doc = "NWAIT signal is active one data cycle before wait state"]
        BeforeWaitState = 0x0,
        #[doc = "NWAIT signal is active during wait state"]
        DuringWaitState = 0x1,
    }
    impl Waitcfg {
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
    pub enum Waitpol {
        #[doc = "NWAIT active low"]
        ActiveLow = 0x0,
        #[doc = "NWAIT active high"]
        ActiveHigh = 0x1,
    }
    impl Waitpol {
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
