
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "FLASH"]
pub struct Flash {
    ptr: *mut u8,
}
impl Flash {
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
    #[doc = "Flash access control register"]
    pub const fn acr(&self) -> utils::Reg<AcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<AcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Flash key register"]
    pub const fn keyr(&self) -> utils::Reg<u32, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<u32, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Flash option key register"]
    pub const fn optkeyr(&self) -> utils::Reg<u32, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<u32, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<SrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Flash option control register"]
    pub const fn optcr(&self) -> utils::Reg<OptcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<OptcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Flash option control register 1"]
    pub const fn optcr1(&self) -> utils::Reg<Optcr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<Optcr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Flash option control register"]
    pub const fn optcr2(&self) -> utils::Reg<Optcr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<Optcr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Flash access control register"]
pub struct AcrBits {
    bits: u32,
}
impl Default for AcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl AcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Latency"]
    pub const fn set_latency(mut self, val: LatencyVal) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Latency"]
    pub const fn latency(self) -> LatencyVal {
        let val = ((self.bits >> 0x0) & 0xf) as _;
        unsafe { LatencyVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Prefetch enable"]
    pub const fn set_prften(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Prefetch enable"]
    pub const fn prften(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ART Accelerator Enable"]
    pub const fn set_arten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ART Accelerator Enable"]
    pub const fn arten(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ART Accelerator reset"]
    pub const fn set_artrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ART Accelerator reset"]
    pub const fn artrst(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
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
    #[doc = "Programming"]
    pub const fn set_pg(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Programming"]
    pub const fn pg(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Sector Erase"]
    pub const fn set_ser(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Sector Erase"]
    pub const fn ser(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mass Erase of sectors 0 to 11"]
    pub const fn set_mer(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mass Erase of sectors 0 to 11"]
    pub const fn mer(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Sector number"]
    pub const fn set_snb(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x3);
        self.bits |= (val as u32 & 0xf) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Sector number"]
    pub const fn snb(self) -> u8 {
        ((self.bits >> 0x3) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Program size"]
    pub const fn set_psize(mut self, val: PsizeVal) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Program size"]
    pub const fn psize(self) -> PsizeVal {
        let val = ((self.bits >> 0x8) & 0x3) as _;
        unsafe { PsizeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Start"]
    pub const fn set_strt(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Start"]
    pub const fn strt(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "End of operation interrupt enable"]
    pub const fn set_eopie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "End of operation interrupt enable"]
    pub const fn eopie(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Error interrupt enable"]
    pub const fn set_errie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Error interrupt enable"]
    pub const fn errie(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PCROP error interrupt enable"]
    pub const fn set_rderrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PCROP error interrupt enable"]
    pub const fn rderrie(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Lock"]
    pub const fn set_lock(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Lock"]
    pub const fn lock(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Flash option control register"]
pub struct OptcrBits {
    bits: u32,
}
impl Default for OptcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl OptcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Option lock"]
    pub const fn set_optlock(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Option lock"]
    pub const fn optlock(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Option start"]
    pub const fn set_optstrt(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Option start"]
    pub const fn optstrt(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "BOR reset Level"]
    pub const fn set_bor_lev(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x2);
        self.bits |= (val as u32 & 0x3) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "BOR reset Level"]
    pub const fn bor_lev(self) -> u8 {
        ((self.bits >> 0x2) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "User option bytes"]
    pub const fn set_wwdg_sw(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "User option bytes"]
    pub const fn wwdg_sw(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "WDG_SW User option bytes"]
    pub const fn set_iwdg_sw(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "WDG_SW User option bytes"]
    pub const fn iwdg_sw(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "nRST_STOP User option bytes"]
    pub const fn set_n_rst_stop(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "nRST_STOP User option bytes"]
    pub const fn n_rst_stop(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "nRST_STDBY User option bytes"]
    pub const fn set_n_rst_stdby(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "nRST_STDBY User option bytes"]
    pub const fn n_rst_stdby(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Read protect"]
    pub const fn set_rdp(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Read protect"]
    pub const fn rdp(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Not write protect"]
    pub const fn set_n_wrp(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Not write protect"]
    pub const fn n_wrp(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Dual Boot mode (valid only when nDBANK=0)"]
    pub const fn set_n_dboot(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Dual Boot mode (valid only when nDBANK=0)"]
    pub const fn n_dboot(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Not dual bank mode"]
    pub const fn set_n_dbank(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Not dual bank mode"]
    pub const fn n_dbank(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Independent watchdog counter freeze in standby mode"]
    pub const fn set_iwdg_stdby(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Independent watchdog counter freeze in standby mode"]
    pub const fn iwdg_stdby(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Independent watchdog counter freeze in Stop mode"]
    pub const fn set_iwdg_stop(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Independent watchdog counter freeze in Stop mode"]
    pub const fn iwdg_stop(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Flash option control register 1"]
pub struct Optcr1Bits {
    bits: u32,
}
impl Default for Optcr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Optcr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Boot base address when Boot pin =0"]
    pub const fn set_boot_add0(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Boot base address when Boot pin =0"]
    pub const fn boot_add0(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Boot base address when Boot pin =1"]
    pub const fn set_boot_add1(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Boot base address when Boot pin =1"]
    pub const fn boot_add1(self) -> u16 {
        ((self.bits >> 0x10) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Flash option control register"]
pub struct Optcr2Bits {
    bits: u32,
}
impl Default for Optcr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Optcr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "PCROP option byte"]
    pub const fn set_pcro_pi(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "PCROP option byte"]
    pub const fn pcro_pi(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "PCROP zone preserved when RDP level decreased"]
    pub const fn set_pcrop_rdp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PCROP zone preserved when RDP level decreased"]
    pub const fn pcrop_rdp(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
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
    #[doc = "End of operation"]
    pub const fn set_eop(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "End of operation"]
    pub const fn eop(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Operation error"]
    pub const fn set_operr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Operation error"]
    pub const fn operr(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Write protection error"]
    pub const fn set_wrperr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Write protection error"]
    pub const fn wrperr(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Programming alignment error"]
    pub const fn set_pgaerr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Programming alignment error"]
    pub const fn pgaerr(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Programming parallelism error"]
    pub const fn set_pgperr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Programming parallelism error"]
    pub const fn pgperr(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Erase Sequence Error"]
    pub const fn set_erserr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Erase Sequence Error"]
    pub const fn erserr(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "RDERR"]
    pub const fn set_rderr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RDERR"]
    pub const fn rderr(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Busy"]
    pub const fn set_bsy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Busy"]
    pub const fn bsy(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LatencyVal {
    #[doc = "0 wait states"]
    Ws0 = 0x0,
    #[doc = "1 wait states"]
    Ws1 = 0x1,
    #[doc = "2 wait states"]
    Ws2 = 0x2,
    #[doc = "3 wait states"]
    Ws3 = 0x3,
    #[doc = "4 wait states"]
    Ws4 = 0x4,
    #[doc = "5 wait states"]
    Ws5 = 0x5,
    #[doc = "6 wait states"]
    Ws6 = 0x6,
    #[doc = "7 wait states"]
    Ws7 = 0x7,
    #[doc = "8 wait states"]
    Ws8 = 0x8,
    #[doc = "9 wait states"]
    Ws9 = 0x9,
    #[doc = "10 wait states"]
    Ws10 = 0xa,
    #[doc = "11 wait states"]
    Ws11 = 0xb,
    #[doc = "12 wait states"]
    Ws12 = 0xc,
    #[doc = "13 wait states"]
    Ws13 = 0xd,
    #[doc = "14 wait states"]
    Ws14 = 0xe,
    #[doc = "15 wait states"]
    Ws15 = 0xf,
}
impl LatencyVal {
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
pub enum PsizeVal {
    #[doc = "Program x8"]
    Psize8 = 0x0,
    #[doc = "Program x16"]
    Psize16 = 0x1,
    #[doc = "Program x32"]
    Psize32 = 0x2,
    #[doc = "Program x64"]
    Psize64 = 0x3,
}
impl PsizeVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
