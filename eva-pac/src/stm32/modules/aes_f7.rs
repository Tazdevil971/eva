
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Advanced encryption standard hardware accelerator"]
pub struct Aes {
    ptr: *mut u8,
}
impl Aes {
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
    #[doc = "Control register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<SrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Data input register"]
    pub const fn dinr(&self) -> utils::Reg<DinrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<DinrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Data output register"]
    pub const fn doutr(&self) -> utils::Reg<DoutrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<DoutrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Initialization vector register"]
    pub const fn ivr(&self, idx: usize) -> utils::Reg<IvrBits, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x20 + idx * 0x4);
            <utils::Reg<IvrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Suspend register"]
    pub const fn suspr(&self, idx: usize) -> utils::Reg<SusprBits, utils::RW> {
        assert!(idx < 8);
        unsafe {
            let ptr = self.ptr.add(0x40 + idx * 0x4);
            <utils::Reg<SusprBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Control register"]
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
    #[doc = "AES enable"]
    pub const fn set_en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "AES enable"]
    pub const fn en(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data type selection"]
    pub const fn set_datatype(mut self, val: DatatypeVal) -> Self {
        self.bits &= !(0x3 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Data type selection"]
    pub const fn datatype(self) -> DatatypeVal {
        let val = ((self.bits >> 0x1) & 0x3) as _;
        unsafe { DatatypeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Operating mode"]
    pub const fn set_mode(mut self, val: ModeVal) -> Self {
        self.bits &= !(0x3 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Operating mode"]
    pub const fn mode(self) -> ModeVal {
        let val = ((self.bits >> 0x3) & 0x3) as _;
        unsafe { ModeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Chaining mode bit1 bit0"]
    pub const fn set_chmod10(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x5);
        self.bits |= (val as u32 & 0x3) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Chaining mode bit1 bit0"]
    pub const fn chmod10(self) -> u8 {
        ((self.bits >> 0x5) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Computation Complete Flag Clear"]
    pub const fn set_ccfc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Computation Complete Flag Clear"]
    pub const fn ccfc(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Error clear"]
    pub const fn set_errc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Error clear"]
    pub const fn errc(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CCF flag interrupt enable"]
    pub const fn set_ccfie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CCF flag interrupt enable"]
    pub const fn ccfie(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Error interrupt enable"]
    pub const fn set_errie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Error interrupt enable"]
    pub const fn errie(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enable DMA management of data input phase"]
    pub const fn set_dmainen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable DMA management of data input phase"]
    pub const fn dmainen(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enable DMA management of data output phase"]
    pub const fn set_dmaouten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable DMA management of data output phase"]
    pub const fn dmaouten(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "GCM or CCM phase selection"]
    pub const fn set_gcmph(mut self, val: GcmphVal) -> Self {
        self.bits &= !(0x3 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "GCM or CCM phase selection"]
    pub const fn gcmph(self) -> GcmphVal {
        let val = ((self.bits >> 0xd) & 0x3) as _;
        unsafe { GcmphVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Chaining mode bit2"]
    pub const fn set_chmod2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Chaining mode bit2"]
    pub const fn chmod2(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Key size selection"]
    pub const fn set_keysize(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Key size selection"]
    pub const fn keysize(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Data input register"]
pub struct DinrBits {
    bits: u32,
}
impl Default for DinrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DinrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Input data word"]
    pub const fn set_din(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Input data word"]
    pub const fn din(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Data output register"]
pub struct DoutrBits {
    bits: u32,
}
impl Default for DoutrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DoutrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Output data word"]
    pub const fn set_dout(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Output data word"]
    pub const fn dout(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Initialization vector register"]
pub struct IvrBits {
    bits: u32,
}
impl Default for IvrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IvrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Initialization vector input"]
    pub const fn set_ivi(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Initialization vector input"]
    pub const fn ivi(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Key register"]
pub struct KeyrBits {
    bits: u32,
}
impl Default for KeyrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl KeyrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Cryptographic key"]
    pub const fn set_key(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Cryptographic key"]
    pub const fn key(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Status register"]
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
    #[doc = "Computation complete flag"]
    pub const fn set_ccf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Computation complete flag"]
    pub const fn ccf(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Read error flag"]
    pub const fn set_rderr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Read error flag"]
    pub const fn rderr(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Write error flag"]
    pub const fn set_wrerr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Write error flag"]
    pub const fn wrerr(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Busy flag"]
    pub const fn set_busy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Busy flag"]
    pub const fn busy(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Suspend register"]
pub struct SusprBits {
    bits: u32,
}
impl Default for SusprBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SusprBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "AES suspend"]
    pub const fn set_susp(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "AES suspend"]
    pub const fn susp(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DatatypeVal {
    #[doc = "Word"]
    None = 0x0,
    #[doc = "Half-word (16-bit)"]
    HalfWord = 0x1,
    #[doc = "Byte (8-bit)"]
    Byte = 0x2,
    #[doc = "Bit"]
    Bit = 0x3,
}
impl DatatypeVal {
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
pub enum GcmphVal {
    #[doc = "Init phase"]
    InitPhase = 0x0,
    #[doc = "Header phase"]
    HeaderPhase = 0x1,
    #[doc = "Payload phase"]
    PayloadPhase = 0x2,
    #[doc = "Final phase"]
    FinalPhase = 0x3,
}
impl GcmphVal {
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
pub enum ModeVal {
    #[doc = "Encryption"]
    Mode1 = 0x0,
    #[doc = "Key derivation (or key preparation for ECB/CBC decryption)"]
    Mode2 = 0x1,
    #[doc = "Decryption"]
    Mode3 = 0x2,
    #[doc = "Key derivation then single decryption"]
    Mode4 = 0x3,
}
impl ModeVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
