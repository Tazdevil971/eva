
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Serial peripheral interface"]
pub struct Spi {
    ptr: *mut u8,
}
impl Spi {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::Sr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Sr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data register"]
    pub const fn dr(&self) -> utils::Reg<fields::Dr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Dr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "CRC polynomial register"]
    pub const fn crcpr(&self) -> utils::Reg<fields::Crcpr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Crcpr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "RX CRC register"]
    pub const fn rxcrcr(&self) -> utils::Reg<fields::Rxcrcr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Rxcrcr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "TX CRC register"]
    pub const fn txcrcr(&self) -> utils::Reg<fields::Txcrcr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Txcrcr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "I2S configuration register"]
    pub const fn i2scfgr(&self) -> utils::Reg<fields::I2scfgr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<fields::I2scfgr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "I2S prescaler register"]
    pub const fn i2spr(&self) -> utils::Reg<fields::I2spr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::I2spr, utils::RW>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 1"]
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
        #[doc = "Clock phase"]
        pub const fn set_cpha(mut self, val: vals::Cpha) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Clock phase"]
        pub const fn cpha(self) -> vals::Cpha {
            let val = ((self.bits >> 0x0) & 0x1) as _;
            unsafe { vals::Cpha::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clock polarity"]
        pub const fn set_cpol(mut self, val: vals::Cpol) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "Clock polarity"]
        pub const fn cpol(self) -> vals::Cpol {
            let val = ((self.bits >> 0x1) & 0x1) as _;
            unsafe { vals::Cpol::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Master selection"]
        pub const fn set_mstr(mut self, val: vals::Mstr) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Master selection"]
        pub const fn mstr(self) -> vals::Mstr {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Mstr::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Baud rate control"]
        pub const fn set_br(mut self, val: vals::Br) -> Self {
            self.bits &= !(0x7 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Baud rate control"]
        pub const fn br(self) -> vals::Br {
            let val = ((self.bits >> 0x3) & 0x7) as _;
            unsafe { vals::Br::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "SPI enable"]
        pub const fn set_spe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SPI enable"]
        pub const fn spe(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Frame format"]
        pub const fn set_lsbfirst(mut self, val: vals::Lsbfirst) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "Frame format"]
        pub const fn lsbfirst(self) -> vals::Lsbfirst {
            let val = ((self.bits >> 0x7) & 0x1) as _;
            unsafe { vals::Lsbfirst::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Internal slave select"]
        pub const fn set_ssi(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Internal slave select"]
        pub const fn ssi(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Software slave management"]
        pub const fn set_ssm(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Software slave management"]
        pub const fn ssm(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Receive only"]
        pub const fn set_rxonly(mut self, val: vals::Rxonly) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
            self
        }
        #[inline(always)]
        #[doc = "Receive only"]
        pub const fn rxonly(self) -> vals::Rxonly {
            let val = ((self.bits >> 0xa) & 0x1) as _;
            unsafe { vals::Rxonly::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Data frame format"]
        pub const fn set_dff(mut self, val: vals::Dff) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
            self
        }
        #[inline(always)]
        #[doc = "Data frame format"]
        pub const fn dff(self) -> vals::Dff {
            let val = ((self.bits >> 0xb) & 0x1) as _;
            unsafe { vals::Dff::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "CRC transfer next"]
        pub const fn set_crcnext(mut self, val: vals::Crcnext) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xc;
            self
        }
        #[inline(always)]
        #[doc = "CRC transfer next"]
        pub const fn crcnext(self) -> vals::Crcnext {
            let val = ((self.bits >> 0xc) & 0x1) as _;
            unsafe { vals::Crcnext::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Hardware CRC calculation enable"]
        pub const fn set_crcen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= if val { 1 << 0xd } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Hardware CRC calculation enable"]
        pub const fn crcen(self) -> bool {
            ((self.bits >> 0xd) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Select the direction of transfer in bidirectional mode"]
        pub const fn set_bidioe(mut self, val: vals::Bidioe) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xe;
            self
        }
        #[inline(always)]
        #[doc = "Select the direction of transfer in bidirectional mode"]
        pub const fn bidioe(self) -> vals::Bidioe {
            let val = ((self.bits >> 0xe) & 0x1) as _;
            unsafe { vals::Bidioe::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Bidirectional data mode enable"]
        pub const fn set_bidimode(mut self, val: vals::Bidimode) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
            self
        }
        #[inline(always)]
        #[doc = "Bidirectional data mode enable"]
        pub const fn bidimode(self) -> vals::Bidimode {
            let val = ((self.bits >> 0xf) & 0x1) as _;
            unsafe { vals::Bidimode::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 2"]
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
        #[doc = "Rx buffer DMA enable"]
        pub const fn set_rxdmaen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Rx buffer DMA enable"]
        pub const fn rxdmaen(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Tx buffer DMA enable"]
        pub const fn set_txdmaen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Tx buffer DMA enable"]
        pub const fn txdmaen(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SS output enable"]
        pub const fn set_ssoe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SS output enable"]
        pub const fn ssoe(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Frame format"]
        pub const fn set_frf(mut self, val: vals::Frf) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Frame format"]
        pub const fn frf(self) -> vals::Frf {
            let val = ((self.bits >> 0x4) & 0x1) as _;
            unsafe { vals::Frf::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Error interrupt enable"]
        pub const fn set_errie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Error interrupt enable"]
        pub const fn errie(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "RX buffer not empty interrupt enable"]
        pub const fn set_rxneie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RX buffer not empty interrupt enable"]
        pub const fn rxneie(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Tx buffer empty interrupt enable"]
        pub const fn set_txeie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Tx buffer empty interrupt enable"]
        pub const fn txeie(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "CRC polynomial register"]
    pub struct Crcpr {
        bits: u32,
    }
    impl Default for Crcpr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Crcpr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "CRC polynomial register"]
        pub const fn set_crcpoly(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "CRC polynomial register"]
        pub const fn crcpoly(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "data register"]
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
        #[doc = "Data register"]
        pub const fn set_dr(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Data register"]
        pub const fn dr(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "I2S configuration register"]
    pub struct I2scfgr {
        bits: u32,
    }
    impl Default for I2scfgr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl I2scfgr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Channel length (number of bits per audio channel)"]
        pub const fn set_chlen(mut self, val: vals::Chlen) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Channel length (number of bits per audio channel)"]
        pub const fn chlen(self) -> vals::Chlen {
            let val = ((self.bits >> 0x0) & 0x1) as _;
            unsafe { vals::Chlen::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Data length to be transferred"]
        pub const fn set_datlen(mut self, val: vals::Datlen) -> Self {
            self.bits &= !(0x3 << 0x1);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "Data length to be transferred"]
        pub const fn datlen(self) -> vals::Datlen {
            let val = ((self.bits >> 0x1) & 0x3) as _;
            unsafe { vals::Datlen::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Steady state clock polarity"]
        pub const fn set_ckpol(mut self, val: vals::Ckpol) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Steady state clock polarity"]
        pub const fn ckpol(self) -> vals::Ckpol {
            let val = ((self.bits >> 0x3) & 0x1) as _;
            unsafe { vals::Ckpol::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "I2S standard selection"]
        pub const fn set_i2sstd(mut self, val: vals::I2sstd) -> Self {
            self.bits &= !(0x3 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "I2S standard selection"]
        pub const fn i2sstd(self) -> vals::I2sstd {
            let val = ((self.bits >> 0x4) & 0x3) as _;
            unsafe { vals::I2sstd::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "PCM frame synchronization"]
        pub const fn set_pcmsync(mut self, val: vals::Pcmsync) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "PCM frame synchronization"]
        pub const fn pcmsync(self) -> vals::Pcmsync {
            let val = ((self.bits >> 0x7) & 0x1) as _;
            unsafe { vals::Pcmsync::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "I2S configuration mode"]
        pub const fn set_i2scfg(mut self, val: vals::I2scfg) -> Self {
            self.bits &= !(0x3 << 0x8);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "I2S configuration mode"]
        pub const fn i2scfg(self) -> vals::I2scfg {
            let val = ((self.bits >> 0x8) & 0x3) as _;
            unsafe { vals::I2scfg::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "I2S Enable"]
        pub const fn set_i2se(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= if val { 1 << 0xa } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "I2S Enable"]
        pub const fn i2se(self) -> bool {
            ((self.bits >> 0xa) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "I2S mode selection"]
        pub const fn set_i2smod(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "I2S mode selection"]
        pub const fn i2smod(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "I2S prescaler register"]
    pub struct I2spr {
        bits: u32,
    }
    impl Default for I2spr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl I2spr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "I2S Linear prescaler"]
        pub const fn set_i2sdiv(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "I2S Linear prescaler"]
        pub const fn i2sdiv(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Odd factor for the prescaler"]
        pub const fn set_odd(mut self, val: vals::Odd) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Odd factor for the prescaler"]
        pub const fn odd(self) -> vals::Odd {
            let val = ((self.bits >> 0x8) & 0x1) as _;
            unsafe { vals::Odd::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Master clock output enable"]
        pub const fn set_mckoe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Master clock output enable"]
        pub const fn mckoe(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "RX CRC register"]
    pub struct Rxcrcr {
        bits: u32,
    }
    impl Default for Rxcrcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Rxcrcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Rx CRC register"]
        pub const fn set_rx_crc(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Rx CRC register"]
        pub const fn rx_crc(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "status register"]
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
        #[doc = "Receive buffer not empty"]
        pub const fn set_rxne(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Receive buffer not empty"]
        pub const fn rxne(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transmit buffer empty"]
        pub const fn set_txe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transmit buffer empty"]
        pub const fn txe(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Channel side"]
        pub const fn set_chside(mut self, val: vals::Chside) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Channel side"]
        pub const fn chside(self) -> vals::Chside {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Chside::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Underrun flag"]
        pub const fn set_udr(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Underrun flag"]
        pub const fn udr(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CRC error flag"]
        pub const fn set_crcerr(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CRC error flag"]
        pub const fn crcerr(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Mode fault"]
        pub const fn set_modf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Mode fault"]
        pub const fn modf(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Overrun flag"]
        pub const fn set_ovr(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Overrun flag"]
        pub const fn ovr(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Busy flag"]
        pub const fn set_bsy(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Busy flag"]
        pub const fn bsy(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TI frame format error"]
        pub const fn set_fre(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TI frame format error"]
        pub const fn fre(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "TX CRC register"]
    pub struct Txcrcr {
        bits: u32,
    }
    impl Default for Txcrcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Txcrcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Tx CRC register"]
        pub const fn set_tx_crc(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Tx CRC register"]
        pub const fn tx_crc(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum Bidimode {
        #[doc = "2-line unidirectional data mode selected"]
        Unidirectional = 0x0,
        #[doc = "1-line bidirectional data mode selected"]
        Bidirectional = 0x1,
    }
    impl Bidimode {
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
    pub enum Bidioe {
        #[doc = "Output disabled (receive-only mode)"]
        Receive = 0x0,
        #[doc = "Output enabled (transmit-only mode)"]
        Transmit = 0x1,
    }
    impl Bidioe {
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
    pub enum Br {
        #[doc = "f_PCLK / 2"]
        Div2 = 0x0,
        #[doc = "f_PCLK / 4"]
        Div4 = 0x1,
        #[doc = "f_PCLK / 8"]
        Div8 = 0x2,
        #[doc = "f_PCLK / 16"]
        Div16 = 0x3,
        #[doc = "f_PCLK / 32"]
        Div32 = 0x4,
        #[doc = "f_PCLK / 64"]
        Div64 = 0x5,
        #[doc = "f_PCLK / 128"]
        Div128 = 0x6,
        #[doc = "f_PCLK / 256"]
        Div256 = 0x7,
    }
    impl Br {
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
    pub enum Chlen {
        #[doc = "16-bit wide"]
        Bits16 = 0x0,
        #[doc = "32-bit wide"]
        Bits32 = 0x1,
    }
    impl Chlen {
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
    pub enum Chside {
        #[doc = "Channel left has to be transmitted or has been received"]
        Left = 0x0,
        #[doc = "Channel right has to be transmitted or has been received"]
        Right = 0x1,
    }
    impl Chside {
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
    pub enum Ckpol {
        #[doc = "I2S clock inactive state is low level"]
        IdleLow = 0x0,
        #[doc = "I2S clock inactive state is high level"]
        IdleHigh = 0x1,
    }
    impl Ckpol {
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
    pub enum Cpha {
        #[doc = "The first clock transition is the first data capture edge"]
        FirstEdge = 0x0,
        #[doc = "The second clock transition is the first data capture edge"]
        SecondEdge = 0x1,
    }
    impl Cpha {
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
    pub enum Cpol {
        #[doc = "CK to 0 when idle"]
        IdleLow = 0x0,
        #[doc = "CK to 1 when idle"]
        IdleHigh = 0x1,
    }
    impl Cpol {
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
    pub enum Crcnext {
        #[doc = "Next transmit value is from Tx buffer"]
        TxBuffer = 0x0,
        #[doc = "Next transmit value is from Tx CRC register"]
        Crc = 0x1,
    }
    impl Crcnext {
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
    pub enum Datlen {
        #[doc = "16-bit data length"]
        Bits16 = 0x0,
        #[doc = "24-bit data length"]
        Bits24 = 0x1,
        #[doc = "32-bit data length"]
        Bits32 = 0x2,
    }
    impl Datlen {
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
    pub enum Dff {
        #[doc = "8-bit data frame format is selected for transmission/reception"]
        Bits8 = 0x0,
        #[doc = "16-bit data frame format is selected for transmission/reception"]
        Bits16 = 0x1,
    }
    impl Dff {
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
    pub enum Frf {
        #[doc = "SPI Motorola mode"]
        Motorola = 0x0,
        #[doc = "SPI TI mode"]
        Ti = 0x1,
    }
    impl Frf {
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
    pub enum I2scfg {
        #[doc = "Slave - transmit"]
        SlaveTx = 0x0,
        #[doc = "Slave - receive"]
        SlaveRx = 0x1,
        #[doc = "Master - transmit"]
        MasterTx = 0x2,
        #[doc = "Master - receive"]
        MasterRx = 0x3,
    }
    impl I2scfg {
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
    pub enum I2sstd {
        #[doc = "I2S Philips standard"]
        Philips = 0x0,
        #[doc = "MSB justified standard"]
        Msb = 0x1,
        #[doc = "LSB justified standard"]
        Lsb = 0x2,
        #[doc = "PCM standard"]
        Pcm = 0x3,
    }
    impl I2sstd {
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
    pub enum Lsbfirst {
        #[doc = "Data is transmitted/received with the MSB first"]
        MsbFirst = 0x0,
        #[doc = "Data is transmitted/received with the LSB first"]
        LsbFirst = 0x1,
    }
    impl Lsbfirst {
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
    pub enum Mstr {
        #[doc = "Slave configuration"]
        Slave = 0x0,
        #[doc = "Master configuration"]
        Master = 0x1,
    }
    impl Mstr {
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
    pub enum Odd {
        #[doc = "Real divider value is I2SDIV * 2"]
        Even = 0x0,
        #[doc = "Real divider value is (I2SDIV * 2) + 1"]
        Odd = 0x1,
    }
    impl Odd {
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
    pub enum Pcmsync {
        #[doc = "Short frame synchronisation"]
        Short = 0x0,
        #[doc = "Long frame synchronisation"]
        Long = 0x1,
    }
    impl Pcmsync {
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
    pub enum Rxonly {
        #[doc = "Full duplex (Transmit and receive)"]
        FullDuplex = 0x0,
        #[doc = "Output disabled (Receive-only mode)"]
        OutputDisabled = 0x1,
    }
    impl Rxonly {
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
