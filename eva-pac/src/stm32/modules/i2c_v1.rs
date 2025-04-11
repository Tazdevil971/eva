
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Inter-integrated circuit"]
pub struct I2c {
    ptr: *mut u8,
}
impl I2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "Control register 1"]
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Own address register 1"]
    pub const fn oar1(&self) -> utils::Reg<fields::Oar1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Oar1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Own address register 2"]
    pub const fn oar2(&self) -> utils::Reg<fields::Oar2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Oar2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Data register"]
    pub const fn dr(&self) -> utils::Reg<fields::Dr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Dr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Status register 1"]
    pub const fn sr1(&self) -> utils::Reg<fields::Sr1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Sr1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Status register 2"]
    pub const fn sr2(&self) -> utils::Reg<fields::Sr2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Sr2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Clock control register"]
    pub const fn ccr(&self) -> utils::Reg<fields::Ccr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<fields::Ccr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "TRISE register"]
    pub const fn trise(&self) -> utils::Reg<fields::Trise, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::Trise, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "FLTR register"]
    pub const fn fltr(&self) -> utils::Reg<fields::Fltr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::Fltr, utils::RW>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Clock control register"]
    pub struct Ccr {
        bits: u32,
    }
    impl Default for Ccr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ccr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Clock control register in Fast/Standard mode (Master mode)"]
        pub const fn set_ccr(mut self, val: u16) -> Self {
            self.bits &= !(0xfff << 0x0);
            self.bits |= (val as u32 & 0xfff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Clock control register in Fast/Standard mode (Master mode)"]
        pub const fn ccr(self) -> u16 {
            ((self.bits >> 0x0) & 0xfff) as _
        }
        #[inline(always)]
        #[doc = "Fast mode duty cycle"]
        pub const fn set_duty(mut self, val: vals::Duty) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xe;
            self
        }
        #[inline(always)]
        #[doc = "Fast mode duty cycle"]
        pub const fn duty(self) -> vals::Duty {
            let val = ((self.bits >> 0xe) & 0x1) as _;
            unsafe { vals::Duty::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "I2C master mode selection"]
        pub const fn set_f_s(mut self, val: vals::FS) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
            self
        }
        #[inline(always)]
        #[doc = "I2C master mode selection"]
        pub const fn f_s(self) -> vals::FS {
            let val = ((self.bits >> 0xf) & 0x1) as _;
            unsafe { vals::FS::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Control register 1"]
    pub struct Cr1 {
        bits: u32,
    }
    impl Default for Cr1 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr1 {
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
        pub const fn set_pe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Peripheral enable"]
        pub const fn pe(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SMBus mode"]
        pub const fn set_smbus(mut self, val: vals::Smbus) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "SMBus mode"]
        pub const fn smbus(self) -> vals::Smbus {
            let val = ((self.bits >> 0x1) & 0x1) as _;
            unsafe { vals::Smbus::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "SMBus type"]
        pub const fn set_smbtype(mut self, val: vals::Smbtype) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "SMBus type"]
        pub const fn smbtype(self) -> vals::Smbtype {
            let val = ((self.bits >> 0x3) & 0x1) as _;
            unsafe { vals::Smbtype::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "ARP enable"]
        pub const fn set_enarp(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "ARP enable"]
        pub const fn enarp(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "PEC enable"]
        pub const fn set_enpec(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "PEC enable"]
        pub const fn enpec(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "General call enable"]
        pub const fn set_engc(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "General call enable"]
        pub const fn engc(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Clock stretching disable (Slave mode)"]
        pub const fn set_nostretch(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Clock stretching disable (Slave mode)"]
        pub const fn nostretch(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Start generation"]
        pub const fn set_start(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Start generation"]
        pub const fn start(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Stop generation"]
        pub const fn set_stop(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Stop generation"]
        pub const fn stop(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Acknowledge enable"]
        pub const fn set_ack(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= if val { 1 << 0xa } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Acknowledge enable"]
        pub const fn ack(self) -> bool {
            ((self.bits >> 0xa) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Acknowledge/PEC Position (for data reception)"]
        pub const fn set_pos(mut self, val: vals::Pos) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
            self
        }
        #[inline(always)]
        #[doc = "Acknowledge/PEC Position (for data reception)"]
        pub const fn pos(self) -> vals::Pos {
            let val = ((self.bits >> 0xb) & 0x1) as _;
            unsafe { vals::Pos::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Packet error checking"]
        pub const fn set_pec(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= if val { 1 << 0xc } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Packet error checking"]
        pub const fn pec(self) -> bool {
            ((self.bits >> 0xc) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SMBus alert"]
        pub const fn set_alert(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= if val { 1 << 0xd } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SMBus alert"]
        pub const fn alert(self) -> bool {
            ((self.bits >> 0xd) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Software reset"]
        pub const fn set_swrst(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= if val { 1 << 0xf } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Software reset"]
        pub const fn swrst(self) -> bool {
            ((self.bits >> 0xf) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Control register 2"]
    pub struct Cr2 {
        bits: u32,
    }
    impl Default for Cr2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Peripheral clock frequency"]
        pub const fn set_freq(mut self, val: u8) -> Self {
            self.bits &= !(0x3f << 0x0);
            self.bits |= (val as u32 & 0x3f) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Peripheral clock frequency"]
        pub const fn freq(self) -> u8 {
            ((self.bits >> 0x0) & 0x3f) as _
        }
        #[inline(always)]
        #[doc = "Error interrupt enable"]
        pub const fn set_iterren(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Error interrupt enable"]
        pub const fn iterren(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Event interrupt enable"]
        pub const fn set_itevten(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Event interrupt enable"]
        pub const fn itevten(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Buffer interrupt enable"]
        pub const fn set_itbufen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= if val { 1 << 0xa } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Buffer interrupt enable"]
        pub const fn itbufen(self) -> bool {
            ((self.bits >> 0xa) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "DMA requests enable"]
        pub const fn set_dmaen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DMA requests enable"]
        pub const fn dmaen(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "DMA last transfer"]
        pub const fn set_last(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= if val { 1 << 0xc } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DMA last transfer"]
        pub const fn last(self) -> bool {
            ((self.bits >> 0xc) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Data register"]
    pub struct Dr {
        bits: u32,
    }
    impl Default for Dr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "8-bit data register"]
        pub const fn set_dr(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "8-bit data register"]
        pub const fn dr(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "FLTR register"]
    pub struct Fltr {
        bits: u32,
    }
    impl Default for Fltr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fltr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Digital noise filter"]
        pub const fn set_dnf(mut self, val: vals::Dnf) -> Self {
            self.bits &= !(0xf << 0x0);
            self.bits |= (val.to_bits() as u32 & 0xf) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Digital noise filter"]
        pub const fn dnf(self) -> vals::Dnf {
            let val = ((self.bits >> 0x0) & 0xf) as _;
            unsafe { vals::Dnf::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Analog noise filter"]
        pub const fn set_anoff(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Analog noise filter"]
        pub const fn anoff(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Own address register 1"]
    pub struct Oar1 {
        bits: u32,
    }
    impl Default for Oar1 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Oar1 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Interface address"]
        pub const fn set_add(mut self, val: u16) -> Self {
            self.bits &= !(0x3ff << 0x0);
            self.bits |= (val as u32 & 0x3ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Interface address"]
        pub const fn add(self) -> u16 {
            ((self.bits >> 0x0) & 0x3ff) as _
        }
        #[inline(always)]
        #[doc = "Addressing mode (slave mode)"]
        pub const fn set_addmode(mut self, val: vals::Addmode) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
            self
        }
        #[inline(always)]
        #[doc = "Addressing mode (slave mode)"]
        pub const fn addmode(self) -> vals::Addmode {
            let val = ((self.bits >> 0xf) & 0x1) as _;
            unsafe { vals::Addmode::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Own address register 2"]
    pub struct Oar2 {
        bits: u32,
    }
    impl Default for Oar2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Oar2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Dual addressing mode enable"]
        pub const fn set_endual(mut self, val: vals::Endual) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Dual addressing mode enable"]
        pub const fn endual(self) -> vals::Endual {
            let val = ((self.bits >> 0x0) & 0x1) as _;
            unsafe { vals::Endual::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Interface address"]
        pub const fn set_add2(mut self, val: u8) -> Self {
            self.bits &= !(0x7f << 0x1);
            self.bits |= (val as u32 & 0x7f) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "Interface address"]
        pub const fn add2(self) -> u8 {
            ((self.bits >> 0x1) & 0x7f) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Status register 1"]
    pub struct Sr1 {
        bits: u32,
    }
    impl Default for Sr1 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sr1 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Start bit (Master mode)"]
        pub const fn set_start(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Start bit (Master mode)"]
        pub const fn start(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Address sent (master mode)/matched (slave mode)"]
        pub const fn set_addr(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Address sent (master mode)/matched (slave mode)"]
        pub const fn addr(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Byte transfer finished"]
        pub const fn set_btf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Byte transfer finished"]
        pub const fn btf(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "10-bit header sent (Master mode)"]
        pub const fn set_add10(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "10-bit header sent (Master mode)"]
        pub const fn add10(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Stop detection (slave mode)"]
        pub const fn set_stopf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Stop detection (slave mode)"]
        pub const fn stopf(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Data register not empty (receivers)"]
        pub const fn set_rxne(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Data register not empty (receivers)"]
        pub const fn rxne(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Data register empty (transmitters)"]
        pub const fn set_txe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Data register empty (transmitters)"]
        pub const fn txe(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Bus error"]
        pub const fn set_berr(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Bus error"]
        pub const fn berr(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Arbitration lost (master mode)"]
        pub const fn set_arlo(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Arbitration lost (master mode)"]
        pub const fn arlo(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Acknowledge failure"]
        pub const fn set_af(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= if val { 1 << 0xa } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Acknowledge failure"]
        pub const fn af(self) -> bool {
            ((self.bits >> 0xa) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Overrun/Underrun"]
        pub const fn set_ovr(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Overrun/Underrun"]
        pub const fn ovr(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "PEC Error in reception"]
        pub const fn set_pecerr(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= if val { 1 << 0xc } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "PEC Error in reception"]
        pub const fn pecerr(self) -> bool {
            ((self.bits >> 0xc) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Timeout or t_low detection flag"]
        pub const fn set_timeout(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Timeout or t_low detection flag"]
        pub const fn timeout(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SMBus alert"]
        pub const fn set_alert(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= if val { 1 << 0xf } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SMBus alert"]
        pub const fn alert(self) -> bool {
            ((self.bits >> 0xf) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Status register 2"]
    pub struct Sr2 {
        bits: u32,
    }
    impl Default for Sr2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sr2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Master/slave"]
        pub const fn set_msl(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Master/slave"]
        pub const fn msl(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Bus busy"]
        pub const fn set_busy(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Bus busy"]
        pub const fn busy(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transmitter/receiver"]
        pub const fn set_tra(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transmitter/receiver"]
        pub const fn tra(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "General call address (Slave mode)"]
        pub const fn set_gencall(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "General call address (Slave mode)"]
        pub const fn gencall(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SMBus device default address (Slave mode)"]
        pub const fn set_smbdefault(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SMBus device default address (Slave mode)"]
        pub const fn smbdefault(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SMBus host header (Slave mode)"]
        pub const fn set_smbhost(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SMBus host header (Slave mode)"]
        pub const fn smbhost(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Dual flag (Slave mode)"]
        pub const fn set_dualf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Dual flag (Slave mode)"]
        pub const fn dualf(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Packet error checking register"]
        pub const fn set_pec(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Packet error checking register"]
        pub const fn pec(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "TRISE register"]
    pub struct Trise {
        bits: u32,
    }
    impl Default for Trise {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Trise {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Maximum rise time in Fast/Standard mode (Master mode)"]
        pub const fn set_trise(mut self, val: u8) -> Self {
            self.bits &= !(0x3f << 0x0);
            self.bits |= (val as u32 & 0x3f) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Maximum rise time in Fast/Standard mode (Master mode)"]
        pub const fn trise(self) -> u8 {
            ((self.bits >> 0x0) & 0x3f) as _
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum Addmode {
        #[doc = "7-bit addressing mode"]
        Bit7 = 0x0,
        #[doc = "10-bit addressing mode"]
        Bit10 = 0x1,
    }
    impl Addmode {
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
    pub enum Dnf {
        #[doc = "Digital filter disabled"]
        NoFilter = 0x0,
        #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
        Filter1 = 0x1,
        #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
        Filter2 = 0x2,
        #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
        Filter3 = 0x3,
        #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
        Filter4 = 0x4,
        #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
        Filter5 = 0x5,
        #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
        Filter6 = 0x6,
        #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
        Filter7 = 0x7,
        #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
        Filter8 = 0x8,
        #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
        Filter9 = 0x9,
        #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
        Filter10 = 0xa,
        #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
        Filter11 = 0xb,
        #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
        Filter12 = 0xc,
        #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
        Filter13 = 0xd,
        #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
        Filter14 = 0xe,
        #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
        Filter15 = 0xf,
    }
    impl Dnf {
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
    pub enum Duty {
        #[doc = "Duty cycle t_low/t_high = 2/1"]
        Duty21 = 0x0,
        #[doc = "Duty cycle t_low/t_high = 16/9"]
        Duty169 = 0x1,
    }
    impl Duty {
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
    pub enum Endual {
        #[doc = "Single addressing mode"]
        Single = 0x0,
        #[doc = "Dual addressing mode"]
        Dual = 0x1,
    }
    impl Endual {
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
    pub enum FS {
        #[doc = "Standard mode I2C"]
        Standard = 0x0,
        #[doc = "Fast mode I2C"]
        Fast = 0x1,
    }
    impl FS {
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
    pub enum Pos {
        #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
        Current = 0x0,
        #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
        Next = 0x1,
    }
    impl Pos {
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
    pub enum Smbtype {
        #[doc = "SMBus Device"]
        Device = 0x0,
        #[doc = "SMBus Host"]
        Host = 0x1,
    }
    impl Smbtype {
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
    pub enum Smbus {
        #[doc = "I2C Mode"]
        I2c = 0x0,
        #[doc = "SMBus"]
        SmBus = 0x1,
    }
    impl Smbus {
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
