
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Real-time clock"]
pub struct Rtc {
    ptr: *mut u8,
}
impl Rtc {
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
    #[doc = "Time register"]
    pub const fn tr(&self) -> utils::Reg<TrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<TrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Date register"]
    pub const fn dr(&self) -> utils::Reg<DrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<DrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Initialization and status register"]
    pub const fn isr(&self) -> utils::Reg<IsrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<IsrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Prescaler register"]
    pub const fn prer(&self) -> utils::Reg<PrerBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<PrerBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Wakeup timer register"]
    pub const fn wutr(&self) -> utils::Reg<WutrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<WutrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Alarm register"]
    pub const fn alrmr(&self, idx: usize) -> utils::Reg<AlrmrBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x1c + idx * 0x4);
            <utils::Reg<AlrmrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Write protection register"]
    pub const fn wpr(&self) -> utils::Reg<WprBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<WprBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Sub second register"]
    pub const fn ssr(&self) -> utils::Reg<SsrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<SsrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Shift control register"]
    pub const fn shiftr(&self) -> utils::Reg<ShiftrBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<ShiftrBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Timestamp time register"]
    pub const fn tstr(&self) -> utils::Reg<TstrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<TstrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Timestamp date register"]
    pub const fn tsdr(&self) -> utils::Reg<TsdrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<TsdrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Timestamp sub second register"]
    pub const fn tsssr(&self) -> utils::Reg<TsssrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<TsssrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Calibration register"]
    pub const fn calr(&self) -> utils::Reg<CalrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c);
            <utils::Reg<CalrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Tamper configuration register"]
    pub const fn tampcr(&self) -> utils::Reg<TampcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<TampcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Alarm sub second register"]
    pub const fn alrmssr(&self, idx: usize) -> utils::Reg<AlrmssrBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x44 + idx * 0x4);
            <utils::Reg<AlrmssrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Option register"]
    pub const fn or(&self) -> utils::Reg<OrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<OrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Backup register"]
    pub const fn bkpr(&self, idx: usize) -> utils::Reg<BkprBits, utils::RW> {
        assert!(idx < 32);
        unsafe {
            let ptr = self.ptr.add(0x50 + idx * 0x4);
            <utils::Reg<BkprBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Alarm register"]
pub struct AlrmrBits {
    bits: u32,
}
impl Default for AlrmrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl AlrmrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Second units in BCD format"]
    pub const fn set_su(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Second units in BCD format"]
    pub const fn su(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Second tens in BCD format"]
    pub const fn set_st(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x4);
        self.bits |= (val as u32 & 0x7) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Second tens in BCD format"]
    pub const fn st(self) -> u8 {
        ((self.bits >> 0x4) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Alarm seconds mask"]
    pub const fn set_msk1(mut self, val: AlrmrMskVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "Alarm seconds mask"]
    pub const fn msk1(self) -> AlrmrMskVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { AlrmrMskVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Minute units in BCD format"]
    pub const fn set_mnu(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Minute units in BCD format"]
    pub const fn mnu(self) -> u8 {
        ((self.bits >> 0x8) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Minute tens in BCD format"]
    pub const fn set_mnt(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0xc);
        self.bits |= (val as u32 & 0x7) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "Minute tens in BCD format"]
    pub const fn mnt(self) -> u8 {
        ((self.bits >> 0xc) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Alarm minutes mask"]
    pub const fn set_msk2(mut self, val: AlrmrMskVal) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
        self
    }
    #[inline(always)]
    #[doc = "Alarm minutes mask"]
    pub const fn msk2(self) -> AlrmrMskVal {
        let val = ((self.bits >> 0xf) & 0x1) as _;
        unsafe { AlrmrMskVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Hour units in BCD format"]
    pub const fn set_hu(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x10);
        self.bits |= (val as u32 & 0xf) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Hour units in BCD format"]
    pub const fn hu(self) -> u8 {
        ((self.bits >> 0x10) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Hour tens in BCD format"]
    pub const fn set_ht(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x14);
        self.bits |= (val as u32 & 0x3) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Hour tens in BCD format"]
    pub const fn ht(self) -> u8 {
        ((self.bits >> 0x14) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "AM/PM notation"]
    pub const fn set_pm(mut self, val: AlrmrPmVal) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x16;
        self
    }
    #[inline(always)]
    #[doc = "AM/PM notation"]
    pub const fn pm(self) -> AlrmrPmVal {
        let val = ((self.bits >> 0x16) & 0x1) as _;
        unsafe { AlrmrPmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Alarm hours mask"]
    pub const fn set_msk3(mut self, val: AlrmrMskVal) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x17;
        self
    }
    #[inline(always)]
    #[doc = "Alarm hours mask"]
    pub const fn msk3(self) -> AlrmrMskVal {
        let val = ((self.bits >> 0x17) & 0x1) as _;
        unsafe { AlrmrMskVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Date units or day in BCD format"]
    pub const fn set_du(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x18);
        self.bits |= (val as u32 & 0xf) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Date units or day in BCD format"]
    pub const fn du(self) -> u8 {
        ((self.bits >> 0x18) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Date tens in BCD format"]
    pub const fn set_dt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x1c);
        self.bits |= (val as u32 & 0x3) << 0x1c;
        self
    }
    #[inline(always)]
    #[doc = "Date tens in BCD format"]
    pub const fn dt(self) -> u8 {
        ((self.bits >> 0x1c) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Week day selection"]
    pub const fn set_wdsel(mut self, val: AlrmrWdselVal) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1e;
        self
    }
    #[inline(always)]
    #[doc = "Week day selection"]
    pub const fn wdsel(self) -> AlrmrWdselVal {
        let val = ((self.bits >> 0x1e) & 0x1) as _;
        unsafe { AlrmrWdselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Alarm date mask"]
    pub const fn set_msk4(mut self, val: AlrmrMskVal) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1f;
        self
    }
    #[inline(always)]
    #[doc = "Alarm date mask"]
    pub const fn msk4(self) -> AlrmrMskVal {
        let val = ((self.bits >> 0x1f) & 0x1) as _;
        unsafe { AlrmrMskVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Alarm sub second register"]
pub struct AlrmssrBits {
    bits: u32,
}
impl Default for AlrmssrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl AlrmssrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Sub seconds value"]
    pub const fn set_ss(mut self, val: u16) -> Self {
        self.bits &= !(0x7fff << 0x0);
        self.bits |= (val as u32 & 0x7fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Sub seconds value"]
    pub const fn ss(self) -> u16 {
        ((self.bits >> 0x0) & 0x7fff) as _
    }
    #[inline(always)]
    #[doc = "Mask the most-significant bits starting at this bit"]
    pub const fn set_maskss(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x18);
        self.bits |= (val as u32 & 0xf) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Mask the most-significant bits starting at this bit"]
    pub const fn maskss(self) -> u8 {
        ((self.bits >> 0x18) & 0xf) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Backup register"]
pub struct BkprBits {
    bits: u32,
}
impl Default for BkprBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BkprBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "BKP"]
    pub const fn set_bkp(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "BKP"]
    pub const fn bkp(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Calibration register"]
pub struct CalrBits {
    bits: u32,
}
impl Default for CalrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CalrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Calibration minus"]
    pub const fn set_calm(mut self, val: u16) -> Self {
        self.bits &= !(0x1ff << 0x0);
        self.bits |= (val as u32 & 0x1ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Calibration minus"]
    pub const fn calm(self) -> u16 {
        ((self.bits >> 0x0) & 0x1ff) as _
    }
    #[inline(always)]
    #[doc = "Use a 16-second calibration cycle period"]
    pub const fn set_calw16(mut self, val: Calw16Val) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Use a 16-second calibration cycle period"]
    pub const fn calw16(self) -> Calw16Val {
        let val = ((self.bits >> 0xd) & 0x1) as _;
        unsafe { Calw16Val::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Use an 8-second calibration cycle period"]
    pub const fn set_calw8(mut self, val: Calw8Val) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Use an 8-second calibration cycle period"]
    pub const fn calw8(self) -> Calw8Val {
        let val = ((self.bits >> 0xe) & 0x1) as _;
        unsafe { Calw8Val::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Increase frequency of RTC by 488.5 ppm"]
    pub const fn set_calp(mut self, val: CalpVal) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
        self
    }
    #[inline(always)]
    #[doc = "Increase frequency of RTC by 488.5 ppm"]
    pub const fn calp(self) -> CalpVal {
        let val = ((self.bits >> 0xf) & 0x1) as _;
        unsafe { CalpVal::from_bits_unchecked(val) }
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
    #[doc = "Wakeup clock selection"]
    pub const fn set_wucksel(mut self, val: WuckselVal) -> Self {
        self.bits &= !(0x7 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Wakeup clock selection"]
    pub const fn wucksel(self) -> WuckselVal {
        let val = ((self.bits >> 0x0) & 0x7) as _;
        unsafe { WuckselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Timestamp event active edge"]
    pub const fn set_tsedge(mut self, val: TsedgeVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Timestamp event active edge"]
    pub const fn tsedge(self) -> TsedgeVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { TsedgeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Reference clock detection enable (50 or 60 Hz)"]
    pub const fn set_refckon(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Reference clock detection enable (50 or 60 Hz)"]
    pub const fn refckon(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Bypass the shadow registers"]
    pub const fn set_bypshad(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Bypass the shadow registers"]
    pub const fn bypshad(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Hour format"]
    pub const fn set_fmt(mut self, val: FmtVal) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Hour format"]
    pub const fn fmt(self) -> FmtVal {
        let val = ((self.bits >> 0x6) & 0x1) as _;
        unsafe { FmtVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Alarm enable"]
    pub const fn set_alre(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0x8 + idx * 0x1));
        self.bits |= if val { 1 << (0x8 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Alarm enable"]
    pub const fn alre(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0x8 + idx * 0x1)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wakeup timer enable"]
    pub const fn set_wute(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wakeup timer enable"]
    pub const fn wute(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timestamp enable"]
    pub const fn set_tse(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timestamp enable"]
    pub const fn tse(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Alarm interrupt enable"]
    pub const fn set_alrie(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0xc + idx * 0x1));
        self.bits |= if val { 1 << (0xc + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Alarm interrupt enable"]
    pub const fn alrie(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0xc + idx * 0x1)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wakeup timer interrupt enable"]
    pub const fn set_wutie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wakeup timer interrupt enable"]
    pub const fn wutie(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timestamp interrupt enable"]
    pub const fn set_tsie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timestamp interrupt enable"]
    pub const fn tsie(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Add 1 hour (summer time change)"]
    pub const fn set_add1h(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Add 1 hour (summer time change)"]
    pub const fn add1h(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Subtract 1 hour (winter time change)"]
    pub const fn set_sub1h(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Subtract 1 hour (winter time change)"]
    pub const fn sub1h(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Backup"]
    pub const fn set_bkp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Backup"]
    pub const fn bkp(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Calibration output selection"]
    pub const fn set_cosel(mut self, val: CoselVal) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x13;
        self
    }
    #[inline(always)]
    #[doc = "Calibration output selection"]
    pub const fn cosel(self) -> CoselVal {
        let val = ((self.bits >> 0x13) & 0x1) as _;
        unsafe { CoselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Output polarity"]
    pub const fn set_pol(mut self, val: PolVal) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Output polarity"]
    pub const fn pol(self) -> PolVal {
        let val = ((self.bits >> 0x14) & 0x1) as _;
        unsafe { PolVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Output selection"]
    pub const fn set_osel(mut self, val: OselVal) -> Self {
        self.bits &= !(0x3 << 0x15);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x15;
        self
    }
    #[inline(always)]
    #[doc = "Output selection"]
    pub const fn osel(self) -> OselVal {
        let val = ((self.bits >> 0x15) & 0x3) as _;
        unsafe { OselVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Calibration output enable"]
    pub const fn set_coe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Calibration output enable"]
    pub const fn coe(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timestamp on internal event enable"]
    pub const fn set_itse(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timestamp on internal event enable"]
    pub const fn itse(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Date register"]
pub struct DrBits {
    bits: u32,
}
impl Default for DrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Date units in BCD format"]
    pub const fn set_du(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Date units in BCD format"]
    pub const fn du(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Date tens in BCD format"]
    pub const fn set_dt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x4);
        self.bits |= (val as u32 & 0x3) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Date tens in BCD format"]
    pub const fn dt(self) -> u8 {
        ((self.bits >> 0x4) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Month units in BCD format"]
    pub const fn set_mu(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Month units in BCD format"]
    pub const fn mu(self) -> u8 {
        ((self.bits >> 0x8) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Month tens in BCD format"]
    pub const fn set_mt(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Month tens in BCD format"]
    pub const fn mt(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Week day units"]
    pub const fn set_wdu(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0xd);
        self.bits |= (val as u32 & 0x7) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Week day units"]
    pub const fn wdu(self) -> u8 {
        ((self.bits >> 0xd) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Year units in BCD format"]
    pub const fn set_yu(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x10);
        self.bits |= (val as u32 & 0xf) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Year units in BCD format"]
    pub const fn yu(self) -> u8 {
        ((self.bits >> 0x10) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Year tens in BCD format"]
    pub const fn set_yt(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x14);
        self.bits |= (val as u32 & 0xf) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Year tens in BCD format"]
    pub const fn yt(self) -> u8 {
        ((self.bits >> 0x14) & 0xf) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Initialization and status register"]
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
    #[doc = "Alarm write enabled"]
    pub const fn set_alrwf(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0x0 + idx * 0x1));
        self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Alarm write enabled"]
    pub const fn alrwf(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wakeup timer write enabled"]
    pub const fn set_wutwf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wakeup timer write enabled"]
    pub const fn wutwf(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Shift operation pending"]
    pub const fn set_shpf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Shift operation pending"]
    pub const fn shpf(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Initialization status flag"]
    pub const fn set_inits(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Initialization status flag"]
    pub const fn inits(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Registers synchronization flag"]
    pub const fn set_rsf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Registers synchronization flag"]
    pub const fn rsf(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Initialization flag"]
    pub const fn set_initf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Initialization flag"]
    pub const fn initf(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enter Initialization mode"]
    pub const fn set_init(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enter Initialization mode"]
    pub const fn init(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Alarm flag"]
    pub const fn set_alrf(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0x8 + idx * 0x1));
        self.bits |= if val { 1 << (0x8 + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Alarm flag"]
    pub const fn alrf(self, idx: usize) -> bool {
        assert!(idx < 2);
        ((self.bits >> (0x8 + idx * 0x1)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wakeup timer flag"]
    pub const fn set_wutf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wakeup timer flag"]
    pub const fn wutf(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timestamp flag"]
    pub const fn set_tsf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timestamp flag"]
    pub const fn tsf(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timestamp overflow flag"]
    pub const fn set_tsovf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timestamp overflow flag"]
    pub const fn tsovf(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tamper detection flag"]
    pub const fn set_tampf(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0xd + idx * 0x1));
        self.bits |= if val { 1 << (0xd + idx * 0x1) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tamper detection flag"]
    pub const fn tampf(self, idx: usize) -> bool {
        assert!(idx < 3);
        ((self.bits >> (0xd + idx * 0x1)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Recalibration pending flag"]
    pub const fn set_recalpf(mut self, val: RecalpfVal) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Recalibration pending flag"]
    pub const fn recalpf(self) -> RecalpfVal {
        let val = ((self.bits >> 0x10) & 0x1) as _;
        unsafe { RecalpfVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Internal time-stamp flag"]
    pub const fn set_itsf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Internal time-stamp flag"]
    pub const fn itsf(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Option register"]
pub struct OrBits {
    bits: u32,
}
impl Default for OrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl OrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Timestamp mapping"]
    pub const fn set_tsinsel(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x1);
        self.bits |= (val as u32 & 0x3) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Timestamp mapping"]
    pub const fn tsinsel(self) -> u8 {
        ((self.bits >> 0x1) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "RTC_ALARM on PC13 output type"]
    pub const fn set_rtc_alarm_type(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RTC_ALARM on PC13 output type"]
    pub const fn rtc_alarm_type(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Prescaler register"]
pub struct PrerBits {
    bits: u32,
}
impl Default for PrerBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PrerBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Synchronous prescaler factor"]
    pub const fn set_prediv_s(mut self, val: u16) -> Self {
        self.bits &= !(0x7fff << 0x0);
        self.bits |= (val as u32 & 0x7fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Synchronous prescaler factor"]
    pub const fn prediv_s(self) -> u16 {
        ((self.bits >> 0x0) & 0x7fff) as _
    }
    #[inline(always)]
    #[doc = "Asynchronous prescaler factor"]
    pub const fn set_prediv_a(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x10);
        self.bits |= (val as u32 & 0x7f) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Asynchronous prescaler factor"]
    pub const fn prediv_a(self) -> u8 {
        ((self.bits >> 0x10) & 0x7f) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Shift control register"]
pub struct ShiftrBits {
    bits: u32,
}
impl Default for ShiftrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ShiftrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Subtract a fraction of a second"]
    pub const fn set_subfs(mut self, val: u16) -> Self {
        self.bits &= !(0x7fff << 0x0);
        self.bits |= (val as u32 & 0x7fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Subtract a fraction of a second"]
    pub const fn subfs(self) -> u16 {
        ((self.bits >> 0x0) & 0x7fff) as _
    }
    #[inline(always)]
    #[doc = "Add one second"]
    pub const fn set_add1s(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Add one second"]
    pub const fn add1s(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Sub second register"]
pub struct SsrBits {
    bits: u32,
}
impl Default for SsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Sub second value"]
    pub const fn set_ss(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Sub second value"]
    pub const fn ss(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Tamper configuration register"]
pub struct TampcrBits {
    bits: u32,
}
impl Default for TampcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TampcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Tamper interrupt enable"]
    pub const fn set_tampie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tamper interrupt enable"]
    pub const fn tampie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Activate timestamp on tamper detection event"]
    pub const fn set_tampts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Activate timestamp on tamper detection event"]
    pub const fn tampts(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tamper sampling frequency"]
    pub const fn set_tampfreq(mut self, val: TampfreqVal) -> Self {
        self.bits &= !(0x7 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Tamper sampling frequency"]
    pub const fn tampfreq(self) -> TampfreqVal {
        let val = ((self.bits >> 0x8) & 0x7) as _;
        unsafe { TampfreqVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Tamper filter count"]
    pub const fn set_tampflt(mut self, val: TampfltVal) -> Self {
        self.bits &= !(0x3 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "Tamper filter count"]
    pub const fn tampflt(self) -> TampfltVal {
        let val = ((self.bits >> 0xb) & 0x3) as _;
        unsafe { TampfltVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Tamper precharge duration"]
    pub const fn set_tampprch(mut self, val: TampprchVal) -> Self {
        self.bits &= !(0x3 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Tamper precharge duration"]
    pub const fn tampprch(self) -> TampprchVal {
        let val = ((self.bits >> 0xd) & 0x3) as _;
        unsafe { TampprchVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Tamper pull-up disable"]
    pub const fn set_tamppudis(mut self, val: TamppudisVal) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
        self
    }
    #[inline(always)]
    #[doc = "Tamper pull-up disable"]
    pub const fn tamppudis(self) -> TamppudisVal {
        let val = ((self.bits >> 0xf) & 0x1) as _;
        unsafe { TamppudisVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Tamper interrupt enable"]
    pub const fn set_tampxie(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0x10 + idx * 0x3));
        self.bits |= if val { 1 << (0x10 + idx * 0x3) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tamper interrupt enable"]
    pub const fn tampxie(self, idx: usize) -> bool {
        assert!(idx < 3);
        ((self.bits >> (0x10 + idx * 0x3)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tamper no erase"]
    pub const fn set_tampxnoerase(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0x11 + idx * 0x3));
        self.bits |= if val { 1 << (0x11 + idx * 0x3) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tamper no erase"]
    pub const fn tampxnoerase(self, idx: usize) -> bool {
        assert!(idx < 3);
        ((self.bits >> (0x11 + idx * 0x3)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tamper mask flag"]
    pub const fn set_tampxmf(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0x12 + idx * 0x3));
        self.bits |= if val { 1 << (0x12 + idx * 0x3) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tamper mask flag"]
    pub const fn tampxmf(self, idx: usize) -> bool {
        assert!(idx < 3);
        ((self.bits >> (0x12 + idx * 0x3)) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Time register"]
pub struct TrBits {
    bits: u32,
}
impl Default for TrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Second units in BCD format"]
    pub const fn set_su(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Second units in BCD format"]
    pub const fn su(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Second tens in BCD format"]
    pub const fn set_st(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x4);
        self.bits |= (val as u32 & 0x7) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Second tens in BCD format"]
    pub const fn st(self) -> u8 {
        ((self.bits >> 0x4) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Minute units in BCD format"]
    pub const fn set_mnu(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Minute units in BCD format"]
    pub const fn mnu(self) -> u8 {
        ((self.bits >> 0x8) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Minute tens in BCD format"]
    pub const fn set_mnt(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0xc);
        self.bits |= (val as u32 & 0x7) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "Minute tens in BCD format"]
    pub const fn mnt(self) -> u8 {
        ((self.bits >> 0xc) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Hour units in BCD format"]
    pub const fn set_hu(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x10);
        self.bits |= (val as u32 & 0xf) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Hour units in BCD format"]
    pub const fn hu(self) -> u8 {
        ((self.bits >> 0x10) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Hour tens in BCD format"]
    pub const fn set_ht(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x14);
        self.bits |= (val as u32 & 0x3) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Hour tens in BCD format"]
    pub const fn ht(self) -> u8 {
        ((self.bits >> 0x14) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "AM/PM notation"]
    pub const fn set_pm(mut self, val: AmpmVal) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x16;
        self
    }
    #[inline(always)]
    #[doc = "AM/PM notation"]
    pub const fn pm(self) -> AmpmVal {
        let val = ((self.bits >> 0x16) & 0x1) as _;
        unsafe { AmpmVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Timestamp date register"]
pub struct TsdrBits {
    bits: u32,
}
impl Default for TsdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TsdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Date units in BCD format"]
    pub const fn set_du(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Date units in BCD format"]
    pub const fn du(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Date tens in BCD format"]
    pub const fn set_dt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x4);
        self.bits |= (val as u32 & 0x3) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Date tens in BCD format"]
    pub const fn dt(self) -> u8 {
        ((self.bits >> 0x4) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Month units in BCD format"]
    pub const fn set_mu(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Month units in BCD format"]
    pub const fn mu(self) -> u8 {
        ((self.bits >> 0x8) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Month tens in BCD format"]
    pub const fn set_mt(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Month tens in BCD format"]
    pub const fn mt(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Week day units"]
    pub const fn set_wdu(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0xd);
        self.bits |= (val as u32 & 0x7) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Week day units"]
    pub const fn wdu(self) -> u8 {
        ((self.bits >> 0xd) & 0x7) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Timestamp sub second register"]
pub struct TsssrBits {
    bits: u32,
}
impl Default for TsssrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TsssrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Sub second value"]
    pub const fn set_ss(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Sub second value"]
    pub const fn ss(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Timestamp time register"]
pub struct TstrBits {
    bits: u32,
}
impl Default for TstrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TstrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Second units in BCD format"]
    pub const fn set_su(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Second units in BCD format"]
    pub const fn su(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Second tens in BCD format"]
    pub const fn set_st(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x4);
        self.bits |= (val as u32 & 0x7) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Second tens in BCD format"]
    pub const fn st(self) -> u8 {
        ((self.bits >> 0x4) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Minute units in BCD format"]
    pub const fn set_mnu(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Minute units in BCD format"]
    pub const fn mnu(self) -> u8 {
        ((self.bits >> 0x8) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Minute tens in BCD format"]
    pub const fn set_mnt(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0xc);
        self.bits |= (val as u32 & 0x7) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "Minute tens in BCD format"]
    pub const fn mnt(self) -> u8 {
        ((self.bits >> 0xc) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Hour units in BCD format"]
    pub const fn set_hu(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x10);
        self.bits |= (val as u32 & 0xf) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Hour units in BCD format"]
    pub const fn hu(self) -> u8 {
        ((self.bits >> 0x10) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Hour tens in BCD format"]
    pub const fn set_ht(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x14);
        self.bits |= (val as u32 & 0x3) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Hour tens in BCD format"]
    pub const fn ht(self) -> u8 {
        ((self.bits >> 0x14) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "AM/PM notation"]
    pub const fn set_pm(mut self, val: AmpmVal) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x16;
        self
    }
    #[inline(always)]
    #[doc = "AM/PM notation"]
    pub const fn pm(self) -> AmpmVal {
        let val = ((self.bits >> 0x16) & 0x1) as _;
        unsafe { AmpmVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Write protection register"]
pub struct WprBits {
    bits: u32,
}
impl Default for WprBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WprBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Write protection key"]
    pub const fn set_key(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Write protection key"]
    pub const fn key(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Wakeup timer register"]
pub struct WutrBits {
    bits: u32,
}
impl Default for WutrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WutrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Wakeup auto-reload value bits"]
    pub const fn set_wut(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Wakeup auto-reload value bits"]
    pub const fn wut(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AlrmrMskVal {
    #[doc = "Alarm set if the date/day match"]
    ToMatch = 0x0,
    #[doc = "Date/day don’t care in Alarm comparison"]
    NotMatch = 0x1,
}
impl AlrmrMskVal {
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
pub enum AlrmrPmVal {
    #[doc = "AM or 24-hour format"]
    Am = 0x0,
    #[doc = "PM"]
    Pm = 0x1,
}
impl AlrmrPmVal {
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
pub enum AlrmrWdselVal {
    #[doc = "DU[3:0] represents the date units"]
    DateUnits = 0x0,
    #[doc = "DU[3:0] represents the week day. DT[1:0] is don’t care"]
    WeekDay = 0x1,
}
impl AlrmrWdselVal {
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
pub enum AmpmVal {
    #[doc = "AM or 24-hour format"]
    Am = 0x0,
    #[doc = "PM"]
    Pm = 0x1,
}
impl AmpmVal {
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
pub enum CalpVal {
    #[doc = "No RTCCLK pulses are added"]
    NoChange = 0x0,
    #[doc = "One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)"]
    IncreaseFreq = 0x1,
}
impl CalpVal {
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
pub enum Calw16Val {
    #[doc = "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1"]
    SixteenSecond = 0x1,
}
impl Calw16Val {
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
pub enum Calw8Val {
    #[doc = "When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected"]
    EightSecond = 0x1,
}
impl Calw8Val {
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
pub enum CoselVal {
    #[doc = "Calibration output is 512 Hz (with default prescaler setting)"]
    CalFreq512hz = 0x0,
    #[doc = "Calibration output is 1 Hz (with default prescaler setting)"]
    CalFreq1hz = 0x1,
}
impl CoselVal {
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
pub enum FmtVal {
    #[doc = "24 hour/day format"]
    TwentyFourHour = 0x0,
    #[doc = "AM/PM hour format"]
    AmPm = 0x1,
}
impl FmtVal {
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
pub enum OselVal {
    #[doc = "Output disabled"]
    Disabled = 0x0,
    #[doc = "Alarm A output enabled"]
    AlarmA = 0x1,
    #[doc = "Alarm B output enabled"]
    AlarmB = 0x2,
    #[doc = "Wakeup output enabled"]
    Wakeup = 0x3,
}
impl OselVal {
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
pub enum PolVal {
    #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0])"]
    High = 0x0,
    #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0])"]
    Low = 0x1,
}
impl PolVal {
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
pub enum RecalpfVal {
    #[doc = "The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
    Pending = 0x1,
}
impl RecalpfVal {
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
pub enum TampfltVal {
    #[doc = "Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)"]
    Immediate = 0x0,
    #[doc = "Tamper event is activated after 2 consecutive samples at the active level"]
    Samples2 = 0x1,
    #[doc = "Tamper event is activated after 4 consecutive samples at the active level"]
    Samples4 = 0x2,
    #[doc = "Tamper event is activated after 8 consecutive samples at the active level"]
    Samples8 = 0x3,
}
impl TampfltVal {
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
pub enum TampfreqVal {
    #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    Div32768 = 0x0,
    #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    Div16384 = 0x1,
    #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    Div8192 = 0x2,
    #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    Div4096 = 0x3,
    #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    Div2048 = 0x4,
    #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    Div1024 = 0x5,
    #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    Div512 = 0x6,
    #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    Div256 = 0x7,
}
impl TampfreqVal {
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
pub enum TampprchVal {
    #[doc = "1 RTCCLK cycle"]
    Cycles1 = 0x0,
    #[doc = "2 RTCCLK cycles"]
    Cycles2 = 0x1,
    #[doc = "4 RTCCLK cycles"]
    Cycles4 = 0x2,
    #[doc = "8 RTCCLK cycles"]
    Cycles8 = 0x3,
}
impl TampprchVal {
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
pub enum TamppudisVal {
    #[doc = "Precharge RTC_TAMPx pins before sampling (enable internal pull-up)"]
    Enabled = 0x0,
    #[doc = "Disable precharge of RTC_TAMPx pins"]
    Disabled = 0x1,
}
impl TamppudisVal {
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
pub enum TamptrgVal {
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
    RisingEdge = 0x0,
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
    FallingEdge = 0x1,
}
impl TamptrgVal {
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
pub enum TsedgeVal {
    #[doc = "RTC_TS input rising edge generates a time-stamp event"]
    RisingEdge = 0x0,
    #[doc = "RTC_TS input falling edge generates a time-stamp event"]
    FallingEdge = 0x1,
}
impl TsedgeVal {
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
pub enum WuckselVal {
    #[doc = "RTC/16 clock is selected"]
    Div16 = 0x0,
    #[doc = "RTC/8 clock is selected"]
    Div8 = 0x1,
    #[doc = "RTC/4 clock is selected"]
    Div4 = 0x2,
    #[doc = "RTC/2 clock is selected"]
    Div2 = 0x3,
    #[doc = "ck_spre (usually 1 Hz) clock is selected"]
    ClockSpare = 0x4,
    #[doc = "ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
    ClockSpareWithOffset = 0x6,
}
impl WuckselVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
