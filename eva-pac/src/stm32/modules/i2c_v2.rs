
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Inter-integrated circuit"]
pub struct I2c {
    ptr: *mut u8,
}
impl I2c {
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
    #[doc = "Control register 1"]
    pub const fn cr1(&self) -> utils::Reg<Cr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Own address register 1"]
    pub const fn oar1(&self) -> utils::Reg<Oar1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<Oar1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Own address register 2"]
    pub const fn oar2(&self) -> utils::Reg<Oar2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<Oar2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Timing register"]
    pub const fn timingr(&self) -> utils::Reg<TimingrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<TimingrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Timeout register"]
    pub const fn timeoutr(&self) -> utils::Reg<TimeoutrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<TimeoutrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt and Status register"]
    pub const fn isr(&self) -> utils::Reg<IsrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<IsrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt clear register"]
    pub const fn icr(&self) -> utils::Reg<IcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<IcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "PEC register"]
    pub const fn pecr(&self) -> utils::Reg<PecrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<PecrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Receive data register"]
    pub const fn rxdr(&self) -> utils::Reg<RxdrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<RxdrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Transmit data register"]
    pub const fn txdr(&self) -> utils::Reg<TxdrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<TxdrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Control register 1"]
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
    #[doc = "TX Interrupt enable"]
    pub const fn set_txie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TX Interrupt enable"]
    pub const fn txie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "RX Interrupt enable"]
    pub const fn set_rxie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RX Interrupt enable"]
    pub const fn rxie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Address match interrupt enable (slave only)"]
    pub const fn set_addrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Address match interrupt enable (slave only)"]
    pub const fn addrie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Not acknowledge received interrupt enable"]
    pub const fn set_nackie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Not acknowledge received interrupt enable"]
    pub const fn nackie(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "STOP detection Interrupt enable"]
    pub const fn set_stopie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "STOP detection Interrupt enable"]
    pub const fn stopie(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer Complete interrupt enable"]
    pub const fn set_tcie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer Complete interrupt enable"]
    pub const fn tcie(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Error interrupts enable"]
    pub const fn set_errie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Error interrupts enable"]
    pub const fn errie(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Digital noise filter"]
    pub const fn set_dnf(mut self, val: DnfVal) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Digital noise filter"]
    pub const fn dnf(self) -> DnfVal {
        let val = ((self.bits >> 0x8) & 0xf) as _;
        unsafe { DnfVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Analog noise filter OFF"]
    pub const fn set_anfoff(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Analog noise filter OFF"]
    pub const fn anfoff(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA transmission requests enable"]
    pub const fn set_txdmaen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA transmission requests enable"]
    pub const fn txdmaen(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA reception requests enable"]
    pub const fn set_rxdmaen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA reception requests enable"]
    pub const fn rxdmaen(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Slave byte control"]
    pub const fn set_sbc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Slave byte control"]
    pub const fn sbc(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock stretching disable"]
    pub const fn set_nostretch(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock stretching disable"]
    pub const fn nostretch(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "General call enable"]
    pub const fn set_gcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "General call enable"]
    pub const fn gcen(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SMBus Host address enable"]
    pub const fn set_smbhen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SMBus Host address enable"]
    pub const fn smbhen(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SMBus Device Default address enable"]
    pub const fn set_smbden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SMBus Device Default address enable"]
    pub const fn smbden(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SMBUS alert enable"]
    pub const fn set_alerten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SMBUS alert enable"]
    pub const fn alerten(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PEC enable"]
    pub const fn set_pecen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PEC enable"]
    pub const fn pecen(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Control register 2"]
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
    #[doc = "Slave address bit (master mode)"]
    pub const fn set_sadd(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x0);
        self.bits |= (val as u32 & 0x3ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Slave address bit (master mode)"]
    pub const fn sadd(self) -> u16 {
        ((self.bits >> 0x0) & 0x3ff) as _
    }
    #[inline(always)]
    #[doc = "Transfer direction (master mode)"]
    pub const fn set_dir(mut self, val: DirVal) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "Transfer direction (master mode)"]
    pub const fn dir(self) -> DirVal {
        let val = ((self.bits >> 0xa) & 0x1) as _;
        unsafe { DirVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "10-bit addressing mode (master mode)"]
    pub const fn set_add10(mut self, val: AddmodeVal) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "10-bit addressing mode (master mode)"]
    pub const fn add10(self) -> AddmodeVal {
        let val = ((self.bits >> 0xb) & 0x1) as _;
        unsafe { AddmodeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "10-bit address header only read direction (master receiver mode)"]
    pub const fn set_head10r(mut self, val: HeadrVal) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "10-bit address header only read direction (master receiver mode)"]
    pub const fn head10r(self) -> HeadrVal {
        let val = ((self.bits >> 0xc) & 0x1) as _;
        unsafe { HeadrVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Start generation"]
    pub const fn set_start(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Start generation"]
    pub const fn start(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Stop generation (master mode)"]
    pub const fn set_stop(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Stop generation (master mode)"]
    pub const fn stop(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "NACK generation (slave mode)"]
    pub const fn set_nack(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "NACK generation (slave mode)"]
    pub const fn nack(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Number of bytes"]
    pub const fn set_nbytes(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Number of bytes"]
    pub const fn nbytes(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "NBYTES reload mode"]
    pub const fn set_reload(mut self, val: ReloadVal) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "NBYTES reload mode"]
    pub const fn reload(self) -> ReloadVal {
        let val = ((self.bits >> 0x18) & 0x1) as _;
        unsafe { ReloadVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Automatic end mode (master mode)"]
    pub const fn set_autoend(mut self, val: AutoendVal) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x19;
        self
    }
    #[inline(always)]
    #[doc = "Automatic end mode (master mode)"]
    pub const fn autoend(self) -> AutoendVal {
        let val = ((self.bits >> 0x19) & 0x1) as _;
        unsafe { AutoendVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Packet error checking byte"]
    pub const fn set_pecbyte(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Packet error checking byte"]
    pub const fn pecbyte(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt clear register"]
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
    #[doc = "Address Matched flag clear"]
    pub const fn set_addrcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Address Matched flag clear"]
    pub const fn addrcf(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Not Acknowledge flag clear"]
    pub const fn set_nackcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Not Acknowledge flag clear"]
    pub const fn nackcf(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Stop detection flag clear"]
    pub const fn set_stopcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Stop detection flag clear"]
    pub const fn stopcf(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Bus error flag clear"]
    pub const fn set_berrcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Bus error flag clear"]
    pub const fn berrcf(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Arbitration lost flag clear"]
    pub const fn set_arlocf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Arbitration lost flag clear"]
    pub const fn arlocf(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Overrun/Underrun flag clear"]
    pub const fn set_ovrcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun/Underrun flag clear"]
    pub const fn ovrcf(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PEC Error flag clear"]
    pub const fn set_peccf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PEC Error flag clear"]
    pub const fn peccf(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timeout detection flag clear"]
    pub const fn set_timoutcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timeout detection flag clear"]
    pub const fn timoutcf(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Alert flag clear"]
    pub const fn set_alertcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Alert flag clear"]
    pub const fn alertcf(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt and Status register"]
pub struct IsrBits {
    bits: u32,
}
impl Default for IsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transmit data register empty (transmitters)"]
    pub const fn set_txe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit data register empty (transmitters)"]
    pub const fn txe(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit interrupt status (transmitters)"]
    pub const fn set_txis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit interrupt status (transmitters)"]
    pub const fn txis(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive data register not empty (receivers)"]
    pub const fn set_rxne(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive data register not empty (receivers)"]
    pub const fn rxne(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Address matched (slave mode)"]
    pub const fn set_addr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Address matched (slave mode)"]
    pub const fn addr(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Not acknowledge received flag"]
    pub const fn set_nackf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Not acknowledge received flag"]
    pub const fn nackf(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Stop detection flag"]
    pub const fn set_stopf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Stop detection flag"]
    pub const fn stopf(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer Complete (master mode)"]
    pub const fn set_tc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer Complete (master mode)"]
    pub const fn tc(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer Complete Reload"]
    pub const fn set_tcr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer Complete Reload"]
    pub const fn tcr(self) -> bool {
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
    #[doc = "Arbitration lost"]
    pub const fn set_arlo(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Arbitration lost"]
    pub const fn arlo(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Overrun/Underrun (slave mode)"]
    pub const fn set_ovr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun/Underrun (slave mode)"]
    pub const fn ovr(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PEC Error in reception"]
    pub const fn set_pecerr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PEC Error in reception"]
    pub const fn pecerr(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timeout or t_low detection flag"]
    pub const fn set_timeout(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timeout or t_low detection flag"]
    pub const fn timeout(self) -> bool {
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
    #[doc = "Bus busy"]
    pub const fn set_busy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Bus busy"]
    pub const fn busy(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transfer direction (Slave mode)"]
    pub const fn set_dir(mut self, val: DirVal) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Transfer direction (Slave mode)"]
    pub const fn dir(self) -> DirVal {
        let val = ((self.bits >> 0x10) & 0x1) as _;
        unsafe { DirVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Address match code (Slave mode)"]
    pub const fn set_addcode(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x11);
        self.bits |= (val as u32 & 0x7f) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Address match code (Slave mode)"]
    pub const fn addcode(self) -> u8 {
        ((self.bits >> 0x11) & 0x7f) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Own address register 1"]
pub struct Oar1Bits {
    bits: u32,
}
impl Default for Oar1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Oar1Bits {
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
    pub const fn set_oa1(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x0);
        self.bits |= (val as u32 & 0x3ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Interface address"]
    pub const fn oa1(self) -> u16 {
        ((self.bits >> 0x0) & 0x3ff) as _
    }
    #[inline(always)]
    #[doc = "Own Address 1 10-bit mode"]
    pub const fn set_oa1mode(mut self, val: AddmodeVal) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "Own Address 1 10-bit mode"]
    pub const fn oa1mode(self) -> AddmodeVal {
        let val = ((self.bits >> 0xa) & 0x1) as _;
        unsafe { AddmodeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Own Address 1 enable"]
    pub const fn set_oa1en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Own Address 1 enable"]
    pub const fn oa1en(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Own address register 2"]
pub struct Oar2Bits {
    bits: u32,
}
impl Default for Oar2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Oar2Bits {
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
    pub const fn set_oa2(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x1);
        self.bits |= (val as u32 & 0x7f) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Interface address"]
    pub const fn oa2(self) -> u8 {
        ((self.bits >> 0x1) & 0x7f) as _
    }
    #[inline(always)]
    #[doc = "Own Address 2 masks"]
    pub const fn set_oa2msk(mut self, val: OamskVal) -> Self {
        self.bits &= !(0x7 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Own Address 2 masks"]
    pub const fn oa2msk(self) -> OamskVal {
        let val = ((self.bits >> 0x8) & 0x7) as _;
        unsafe { OamskVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Own Address 2 enable"]
    pub const fn set_oa2en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Own Address 2 enable"]
    pub const fn oa2en(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "PEC register"]
pub struct PecrBits {
    bits: u32,
}
impl Default for PecrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PecrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Packet error checking register"]
    pub const fn set_pec(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Packet error checking register"]
    pub const fn pec(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Receive data register"]
pub struct RxdrBits {
    bits: u32,
}
impl Default for RxdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl RxdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "8-bit receive data"]
    pub const fn set_rxdata(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "8-bit receive data"]
    pub const fn rxdata(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Timeout register"]
pub struct TimeoutrBits {
    bits: u32,
}
impl Default for TimeoutrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TimeoutrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Bus timeout A"]
    pub const fn set_timeouta(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Bus timeout A"]
    pub const fn timeouta(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
    #[inline(always)]
    #[doc = "Idle clock timeout detection"]
    pub const fn set_tidle(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Idle clock timeout detection"]
    pub const fn tidle(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock timeout enable"]
    pub const fn set_timouten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock timeout enable"]
    pub const fn timouten(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Bus timeout B"]
    pub const fn set_timeoutb(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x10);
        self.bits |= (val as u32 & 0xfff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Bus timeout B"]
    pub const fn timeoutb(self) -> u16 {
        ((self.bits >> 0x10) & 0xfff) as _
    }
    #[inline(always)]
    #[doc = "Extended clock timeout enable"]
    pub const fn set_texten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Extended clock timeout enable"]
    pub const fn texten(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Timing register"]
pub struct TimingrBits {
    bits: u32,
}
impl Default for TimingrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TimingrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "SCL low period (master mode)"]
    pub const fn set_scll(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "SCL low period (master mode)"]
    pub const fn scll(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "SCL high period (master mode)"]
    pub const fn set_sclh(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "SCL high period (master mode)"]
    pub const fn sclh(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Data hold time"]
    pub const fn set_sdadel(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x10);
        self.bits |= (val as u32 & 0xf) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Data hold time"]
    pub const fn sdadel(self) -> u8 {
        ((self.bits >> 0x10) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Data setup time"]
    pub const fn set_scldel(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x14);
        self.bits |= (val as u32 & 0xf) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Data setup time"]
    pub const fn scldel(self) -> u8 {
        ((self.bits >> 0x14) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Timing prescaler"]
    pub const fn set_presc(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x1c);
        self.bits |= (val as u32 & 0xf) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "Timing prescaler"]
    pub const fn presc(self) -> u8 {
        ((self.bits >> 0x1c) & 0xf) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Transmit data register"]
pub struct TxdrBits {
    bits: u32,
}
impl Default for TxdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TxdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "8-bit transmit data"]
    pub const fn set_txdata(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "8-bit transmit data"]
    pub const fn txdata(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AddmodeVal {
    #[doc = "7-bit addressing mode"]
    Bit7 = 0x0,
    #[doc = "10-bit addressing mode"]
    Bit10 = 0x1,
}
impl AddmodeVal {
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
pub enum AutoendVal {
    #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    Software = 0x0,
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    Automatic = 0x1,
}
impl AutoendVal {
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
    #[doc = "Write transfer, slave enters receiver mode"]
    Write = 0x0,
    #[doc = "Read transfer, slave enters transmitter mode"]
    Read = 0x1,
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
pub enum DnfVal {
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
impl DnfVal {
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
pub enum HeadrVal {
    #[doc = "The master sends the complete 10 bit slave address read sequence"]
    Complete = 0x0,
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    Partial = 0x1,
}
impl HeadrVal {
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
pub enum OamskVal {
    #[doc = "No mask"]
    NoMask = 0x0,
    #[doc = "OA2[1] is masked and don’t care. Only OA2[7:2] are compared"]
    Mask1 = 0x1,
    #[doc = "OA2[2:1] are masked and don’t care. Only OA2[7:3] are compared"]
    Mask2 = 0x2,
    #[doc = "OA2[3:1] are masked and don’t care. Only OA2[7:4] are compared"]
    Mask3 = 0x3,
    #[doc = "OA2[4:1] are masked and don’t care. Only OA2[7:5] are compared"]
    Mask4 = 0x4,
    #[doc = "OA2[5:1] are masked and don’t care. Only OA2[7:6] are compared"]
    Mask5 = 0x5,
    #[doc = "OA2[6:1] are masked and don’t care. Only OA2[7] is compared."]
    Mask6 = 0x6,
    #[doc = "OA2[7:1] are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    Mask7 = 0x7,
}
impl OamskVal {
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
pub enum ReloadVal {
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    Completed = 0x0,
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    NotCompleted = 0x1,
}
impl ReloadVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
