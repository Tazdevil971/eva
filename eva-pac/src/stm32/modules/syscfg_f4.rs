
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "System configuration controller"]
pub struct Syscfg {
    ptr: *mut u8,
}
impl Syscfg {
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
    #[doc = "memory remap register"]
    pub const fn memrm(&self) -> utils::Reg<MemrmBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<MemrmBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "peripheral mode configuration register"]
    pub const fn pmc(&self) -> utils::Reg<PmcBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<PmcBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "external interrupt configuration register"]
    pub const fn exticr(&self, idx: usize) -> utils::Reg<ExticrBits, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x8 + idx * 0x4);
            <utils::Reg<ExticrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Compensation cell control register"]
    pub const fn cmpcr(&self) -> utils::Reg<CmpcrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<CmpcrBits, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Compensation cell control register"]
pub struct CmpcrBits {
    bits: u32,
}
impl Default for CmpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CmpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Compensation cell power-down"]
    pub const fn set_cmp_pd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Compensation cell power-down"]
    pub const fn cmp_pd(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "READY"]
    pub const fn set_ready(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "READY"]
    pub const fn ready(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "external interrupt configuration register"]
pub struct ExticrBits {
    bits: u32,
}
impl Default for ExticrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ExticrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "EXTI x configuration"]
    pub const fn set_exti(mut self, idx: usize, val: u8) -> Self {
        assert!(idx < 4);
        self.bits &= !(0xf << (0x0 + idx * 0x4));
        self.bits |= (val as u32 & 0xf) << (0x0 + idx * 0x4);
        self
    }
    #[inline(always)]
    #[doc = "EXTI x configuration"]
    pub const fn exti(self, idx: usize) -> u8 {
        assert!(idx < 4);
        ((self.bits >> (0x0 + idx * 0x4)) & 0xf) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "memory remap register"]
pub struct MemrmBits {
    bits: u32,
}
impl Default for MemrmBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MemrmBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Memory mapping selection"]
    pub const fn set_mem_mode(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x0);
        self.bits |= (val as u32 & 0x7) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Memory mapping selection"]
    pub const fn mem_mode(self) -> u8 {
        ((self.bits >> 0x0) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Flash bank mode selection"]
    pub const fn set_fb_mode(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Flash bank mode selection"]
    pub const fn fb_mode(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FMC memory mapping swap"]
    pub const fn set_swp_fmc(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0xa);
        self.bits |= (val as u32 & 0x3) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "FMC memory mapping swap"]
    pub const fn swp_fmc(self) -> u8 {
        ((self.bits >> 0xa) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "peripheral mode configuration register"]
pub struct PmcBits {
    bits: u32,
}
impl Default for PmcBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PmcBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "ADC1DC2"]
    pub const fn set_adc1dc2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADC1DC2"]
    pub const fn adc1dc2(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADC2DC2"]
    pub const fn set_adc2dc2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADC2DC2"]
    pub const fn adc2dc2(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADC3DC2"]
    pub const fn set_adc3dc2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADC3DC2"]
    pub const fn adc3dc2(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Ethernet PHY interface selection"]
    pub const fn set_mii_rmii_sel(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Ethernet PHY interface selection"]
    pub const fn mii_rmii_sel(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
}
