
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Serial peripheral interface"]
pub struct Spi {
    ptr: *mut u8,
}
impl Spi {
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
    #[doc = "control register 1"]
    pub const fn cr1(&self) -> utils::Reg<Cr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<SrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data register - half-word sized"]
    pub const fn dr16(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "data register - byte sized"]
    pub const fn dr8(&self) -> utils::Reg<u8, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<u8, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "CRC polynomial register"]
    pub const fn crcpr(&self) -> utils::Reg<CrcprBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<CrcprBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "RX CRC register"]
    pub const fn rxcrcr(&self) -> utils::Reg<RxcrcrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<RxcrcrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "TX CRC register"]
    pub const fn txcrcr(&self) -> utils::Reg<TxcrcrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<TxcrcrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "I2S configuration register"]
    pub const fn i2scfgr(&self) -> utils::Reg<I2scfgrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<I2scfgrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "I2S prescaler register"]
    pub const fn i2spr(&self) -> utils::Reg<I2sprBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<I2sprBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register 1"]
pub struct Cr1Bits {
    bits: u32,
}
impl Default for Cr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr1Bits {
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
    pub const fn set_cpha(mut self, val: CphaVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Clock phase"]
    pub const fn cpha(self) -> CphaVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { CphaVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clock polarity"]
    pub const fn set_cpol(mut self, val: CpolVal) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Clock polarity"]
    pub const fn cpol(self) -> CpolVal {
        let val = ((self.bits >> 0x1) & 0x1) as _;
        unsafe { CpolVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Master selection"]
    pub const fn set_mstr(mut self, val: MstrVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Master selection"]
    pub const fn mstr(self) -> MstrVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { MstrVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Baud rate control"]
    pub const fn set_br(mut self, val: BrVal) -> Self {
        self.bits &= !(0x7 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Baud rate control"]
    pub const fn br(self) -> BrVal {
        let val = ((self.bits >> 0x3) & 0x7) as _;
        unsafe { BrVal::from_bits_unchecked(val) }
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
    pub const fn set_lsbfirst(mut self, val: LsbfirstVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "Frame format"]
    pub const fn lsbfirst(self) -> LsbfirstVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { LsbfirstVal::from_bits_unchecked(val) }
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
    pub const fn set_rxonly(mut self, val: RxonlyVal) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "Receive only"]
    pub const fn rxonly(self) -> RxonlyVal {
        let val = ((self.bits >> 0xa) & 0x1) as _;
        unsafe { RxonlyVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "CRC length"]
    pub const fn set_crcl(mut self, val: CrclVal) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "CRC length"]
    pub const fn crcl(self) -> CrclVal {
        let val = ((self.bits >> 0xb) & 0x1) as _;
        unsafe { CrclVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "CRC transfer next"]
    pub const fn set_crcnext(mut self, val: CrcnextVal) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "CRC transfer next"]
    pub const fn crcnext(self) -> CrcnextVal {
        let val = ((self.bits >> 0xc) & 0x1) as _;
        unsafe { CrcnextVal::from_bits_unchecked(val) }
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
    pub const fn set_bidioe(mut self, val: BidioeVal) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Select the direction of transfer in bidirectional mode"]
    pub const fn bidioe(self) -> BidioeVal {
        let val = ((self.bits >> 0xe) & 0x1) as _;
        unsafe { BidioeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Bidirectional data mode enable"]
    pub const fn set_bidimode(mut self, val: BidimodeVal) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
        self
    }
    #[inline(always)]
    #[doc = "Bidirectional data mode enable"]
    pub const fn bidimode(self) -> BidimodeVal {
        let val = ((self.bits >> 0xf) & 0x1) as _;
        unsafe { BidimodeVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register 2"]
pub struct Cr2Bits {
    bits: u32,
}
impl Default for Cr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr2Bits {
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
    #[doc = "NSS pulse management"]
    pub const fn set_nssp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "NSS pulse management"]
    pub const fn nssp(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Frame format"]
    pub const fn set_frf(mut self, val: FrfVal) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Frame format"]
    pub const fn frf(self) -> FrfVal {
        let val = ((self.bits >> 0x4) & 0x1) as _;
        unsafe { FrfVal::from_bits_unchecked(val) }
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
    #[inline(always)]
    #[doc = "Data size"]
    pub const fn set_ds(mut self, val: DsVal) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Data size"]
    pub const fn ds(self) -> DsVal {
        let val = ((self.bits >> 0x8) & 0xf) as _;
        unsafe { DsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "FIFO reception threshold"]
    pub const fn set_frxth(mut self, val: FrxthVal) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "FIFO reception threshold"]
    pub const fn frxth(self) -> FrxthVal {
        let val = ((self.bits >> 0xc) & 0x1) as _;
        unsafe { FrxthVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Last DMA transfer for reception"]
    pub const fn set_ldma_rx(mut self, val: LdmaRxVal) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Last DMA transfer for reception"]
    pub const fn ldma_rx(self) -> LdmaRxVal {
        let val = ((self.bits >> 0xd) & 0x1) as _;
        unsafe { LdmaRxVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Last DMA transfer for transmission"]
    pub const fn set_ldma_tx(mut self, val: LdmaTxVal) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Last DMA transfer for transmission"]
    pub const fn ldma_tx(self) -> LdmaTxVal {
        let val = ((self.bits >> 0xe) & 0x1) as _;
        unsafe { LdmaTxVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "CRC polynomial register"]
pub struct CrcprBits {
    bits: u32,
}
impl Default for CrcprBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CrcprBits {
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
#[doc = "I2S configuration register"]
pub struct I2scfgrBits {
    bits: u32,
}
impl Default for I2scfgrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl I2scfgrBits {
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
    pub const fn set_chlen(mut self, val: ChlenVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Channel length (number of bits per audio channel)"]
    pub const fn chlen(self) -> ChlenVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { ChlenVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Data length to be transferred"]
    pub const fn set_datlen(mut self, val: DatlenVal) -> Self {
        self.bits &= !(0x3 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Data length to be transferred"]
    pub const fn datlen(self) -> DatlenVal {
        let val = ((self.bits >> 0x1) & 0x3) as _;
        unsafe { DatlenVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Steady state clock polarity"]
    pub const fn set_ckpol(mut self, val: CkpolVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Steady state clock polarity"]
    pub const fn ckpol(self) -> CkpolVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { CkpolVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "I2S standard selection"]
    pub const fn set_i2sstd(mut self, val: IsstdVal) -> Self {
        self.bits &= !(0x3 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "I2S standard selection"]
    pub const fn i2sstd(self) -> IsstdVal {
        let val = ((self.bits >> 0x4) & 0x3) as _;
        unsafe { IsstdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "PCM frame synchronization"]
    pub const fn set_pcmsync(mut self, val: PcmsyncVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "PCM frame synchronization"]
    pub const fn pcmsync(self) -> PcmsyncVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { PcmsyncVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "I2S configuration mode"]
    pub const fn set_i2scfg(mut self, val: IscfgVal) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "I2S configuration mode"]
    pub const fn i2scfg(self) -> IscfgVal {
        let val = ((self.bits >> 0x8) & 0x3) as _;
        unsafe { IscfgVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "I2S Enabled"]
    pub const fn set_i2se(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2S Enabled"]
    pub const fn i2se(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2S mode selection"]
    pub const fn set_i2smod(mut self, val: IsmodVal) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "I2S mode selection"]
    pub const fn i2smod(self) -> IsmodVal {
        let val = ((self.bits >> 0xb) & 0x1) as _;
        unsafe { IsmodVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Asynchronous start enable"]
    pub const fn set_astrten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Asynchronous start enable"]
    pub const fn astrten(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "I2S prescaler register"]
pub struct I2sprBits {
    bits: u32,
}
impl Default for I2sprBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl I2sprBits {
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
    pub const fn set_odd(mut self, val: OddVal) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Odd factor for the prescaler"]
    pub const fn odd(self) -> OddVal {
        let val = ((self.bits >> 0x8) & 0x1) as _;
        unsafe { OddVal::from_bits_unchecked(val) }
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
pub struct RxcrcrBits {
    bits: u32,
}
impl Default for RxcrcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl RxcrcrBits {
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
    pub const fn set_chside(mut self, val: ChsideVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Channel side"]
    pub const fn chside(self) -> ChsideVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { ChsideVal::from_bits_unchecked(val) }
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
    #[doc = "frame format error"]
    pub const fn set_fre(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "frame format error"]
    pub const fn fre(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FIFO reception level"]
    pub const fn set_frlvl(mut self, val: FrlvlVal) -> Self {
        self.bits &= !(0x3 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "FIFO reception level"]
    pub const fn frlvl(self) -> FrlvlVal {
        let val = ((self.bits >> 0x9) & 0x3) as _;
        unsafe { FrlvlVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "FIFO Transmission Level"]
    pub const fn set_ftlvl(mut self, val: FtlvlVal) -> Self {
        self.bits &= !(0x3 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "FIFO Transmission Level"]
    pub const fn ftlvl(self) -> FtlvlVal {
        let val = ((self.bits >> 0xb) & 0x3) as _;
        unsafe { FtlvlVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "TX CRC register"]
pub struct TxcrcrBits {
    bits: u32,
}
impl Default for TxcrcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TxcrcrBits {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BidimodeVal {
    #[doc = "2-line unidirectional data mode selected"]
    Unidirectional = 0x0,
    #[doc = "1-line bidirectional data mode selected"]
    Bidirectional = 0x1,
}
impl BidimodeVal {
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
pub enum BidioeVal {
    #[doc = "Output disabled (receive-only mode)"]
    Receive = 0x0,
    #[doc = "Output enabled (transmit-only mode)"]
    Transmit = 0x1,
}
impl BidioeVal {
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
pub enum BrVal {
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
impl BrVal {
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
pub enum ChlenVal {
    #[doc = "16-bit wide"]
    Bits16 = 0x0,
    #[doc = "32-bit wide"]
    Bits32 = 0x1,
}
impl ChlenVal {
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
pub enum ChsideVal {
    #[doc = "Channel left has to be transmitted or has been received"]
    Left = 0x0,
    #[doc = "Channel right has to be transmitted or has been received"]
    Right = 0x1,
}
impl ChsideVal {
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
pub enum CkpolVal {
    #[doc = "I2S clock inactive state is low level"]
    IdleLow = 0x0,
    #[doc = "I2S clock inactive state is high level"]
    IdleHigh = 0x1,
}
impl CkpolVal {
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
pub enum CphaVal {
    #[doc = "The first clock transition is the first data capture edge"]
    FirstEdge = 0x0,
    #[doc = "The second clock transition is the first data capture edge"]
    SecondEdge = 0x1,
}
impl CphaVal {
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
pub enum CpolVal {
    #[doc = "CK to 0 when idle"]
    IdleLow = 0x0,
    #[doc = "CK to 1 when idle"]
    IdleHigh = 0x1,
}
impl CpolVal {
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
pub enum CrclVal {
    #[doc = "8-bit CRC length"]
    Bits8 = 0x0,
    #[doc = "16-bit CRC length"]
    Bits16 = 0x1,
}
impl CrclVal {
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
pub enum CrcnextVal {
    #[doc = "Next transmit value is from Tx buffer"]
    TxBuffer = 0x0,
    #[doc = "Next transmit value is from Tx CRC register"]
    Crc = 0x1,
}
impl CrcnextVal {
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
pub enum DatlenVal {
    #[doc = "16-bit data length"]
    Bits16 = 0x0,
    #[doc = "24-bit data length"]
    Bits24 = 0x1,
    #[doc = "32-bit data length"]
    Bits32 = 0x2,
}
impl DatlenVal {
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
pub enum DsVal {
    #[doc = "4-bit"]
    Bits4 = 0x3,
    #[doc = "5-bit"]
    Bits5 = 0x4,
    #[doc = "6-bit"]
    Bits6 = 0x5,
    #[doc = "7-bit"]
    Bits7 = 0x6,
    #[doc = "8-bit"]
    Bits8 = 0x7,
    #[doc = "9-bit"]
    Bits9 = 0x8,
    #[doc = "10-bit"]
    Bits10 = 0x9,
    #[doc = "11-bit"]
    Bits11 = 0xa,
    #[doc = "12-bit"]
    Bits12 = 0xb,
    #[doc = "13-bit"]
    Bits13 = 0xc,
    #[doc = "14-bit"]
    Bits14 = 0xd,
    #[doc = "15-bit"]
    Bits15 = 0xe,
    #[doc = "16-bit"]
    Bits16 = 0xf,
}
impl DsVal {
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
pub enum FrfVal {
    #[doc = "SPI Motorola mode"]
    Motorola = 0x0,
    #[doc = "SPI TI mode"]
    Ti = 0x1,
}
impl FrfVal {
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
pub enum FrlvlVal {
    #[doc = "Rx FIFO Empty"]
    Empty = 0x0,
    #[doc = "Rx 1/4 FIFO"]
    Quarter = 0x1,
    #[doc = "Rx 1/2 FIFO"]
    Half = 0x2,
    #[doc = "Rx FIFO full"]
    Full = 0x3,
}
impl FrlvlVal {
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
pub enum FrxthVal {
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    Half = 0x0,
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    Quarter = 0x1,
}
impl FrxthVal {
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
pub enum FtlvlVal {
    #[doc = "Tx FIFO Empty"]
    Empty = 0x0,
    #[doc = "Tx 1/4 FIFO"]
    Quarter = 0x1,
    #[doc = "Tx 1/2 FIFO"]
    Half = 0x2,
    #[doc = "Tx FIFO full"]
    Full = 0x3,
}
impl FtlvlVal {
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
pub enum IscfgVal {
    #[doc = "Slave - transmit"]
    SlaveTx = 0x0,
    #[doc = "Slave - receive"]
    SlaveRx = 0x1,
    #[doc = "Master - transmit"]
    MasterTx = 0x2,
    #[doc = "Master - receive"]
    MasterRx = 0x3,
}
impl IscfgVal {
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
pub enum IsmodVal {
    #[doc = "SPI mode is selected"]
    SpiMode = 0x0,
    #[doc = "I2S mode is selected"]
    I2sMode = 0x1,
}
impl IsmodVal {
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
pub enum IsstdVal {
    #[doc = "I2S Philips standard"]
    Philips = 0x0,
    #[doc = "MSB justified standard"]
    Msb = 0x1,
    #[doc = "LSB justified standard"]
    Lsb = 0x2,
    #[doc = "PCM standard"]
    Pcm = 0x3,
}
impl IsstdVal {
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
pub enum LdmaRxVal {
    #[doc = "Number of data to transfer for receive is even"]
    Even = 0x0,
    #[doc = "Number of data to transfer for receive is odd"]
    Odd = 0x1,
}
impl LdmaRxVal {
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
pub enum LdmaTxVal {
    #[doc = "Number of data to transfer for transmit is even"]
    Even = 0x0,
    #[doc = "Number of data to transfer for transmit is odd"]
    Odd = 0x1,
}
impl LdmaTxVal {
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
pub enum LsbfirstVal {
    #[doc = "Data is transmitted/received with the MSB first"]
    MsbFirst = 0x0,
    #[doc = "Data is transmitted/received with the LSB first"]
    LsbFirst = 0x1,
}
impl LsbfirstVal {
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
pub enum MstrVal {
    #[doc = "Slave configuration"]
    Slave = 0x0,
    #[doc = "Master configuration"]
    Master = 0x1,
}
impl MstrVal {
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
pub enum OddVal {
    #[doc = "Real divider value is I2SDIV * 2"]
    Even = 0x0,
    #[doc = "Real divider value is (I2SDIV * 2) + 1"]
    Odd = 0x1,
}
impl OddVal {
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
pub enum PcmsyncVal {
    #[doc = "Short frame synchronisation"]
    Short = 0x0,
    #[doc = "Long frame synchronisation"]
    Long = 0x1,
}
impl PcmsyncVal {
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
pub enum RxonlyVal {
    #[doc = "Full duplex (Transmit and receive)"]
    FullDuplex = 0x0,
    #[doc = "Output disabled (Receive-only mode)"]
    OutputDisabled = 0x1,
}
impl RxonlyVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
