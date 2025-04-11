
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Real-time clock"]
pub struct Rtc {
    ptr: *mut u8,
}
impl Rtc {
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
    pub const fn tr(&self) -> utils::Reg<fields::Tr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Tr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Date register"]
    pub const fn dr(&self) -> utils::Reg<fields::Dr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Dr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register"]
    pub const fn cr(&self) -> utils::Reg<fields::Cr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Cr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Initialization and status register"]
    pub const fn isr(&self) -> utils::Reg<fields::Isr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Isr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Prescaler register"]
    pub const fn prer(&self) -> utils::Reg<fields::Prer, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Prer, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Wakeup timer register"]
    pub const fn wutr(&self) -> utils::Reg<fields::Wutr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Wutr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Calibration register"]
    pub const fn calibr(&self) -> utils::Reg<fields::Calibr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Calibr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Alarm register"]
    pub const fn alrmr(&self, idx: usize) -> utils::Reg<fields::Alrmr, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x1c + idx * 0x4);
            <utils::Reg<fields::Alrmr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Write protection register"]
    pub const fn wpr(&self) -> utils::Reg<fields::Wpr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::Wpr, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Sub second register"]
    pub const fn ssr(&self) -> utils::Reg<fields::Ssr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<fields::Ssr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Shift control register"]
    pub const fn shiftr(&self) -> utils::Reg<fields::Shiftr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::Shiftr, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Timestamp time register"]
    pub const fn tstr(&self) -> utils::Reg<fields::Tstr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<fields::Tstr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Timestamp date register"]
    pub const fn tsdr(&self) -> utils::Reg<fields::Tsdr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<fields::Tsdr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Timestamp sub second register"]
    pub const fn tsssr(&self) -> utils::Reg<fields::Tsssr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<fields::Tsssr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Calibration register"]
    pub const fn calr(&self) -> utils::Reg<fields::Calr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c);
            <utils::Reg<fields::Calr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Tamper and alternate function configuration register"]
    pub const fn tafcr(&self) -> utils::Reg<fields::Tafcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<fields::Tafcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Alarm sub second register"]
    pub const fn alrmssr(&self, idx: usize) -> utils::Reg<fields::Alrmssr, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x44 + idx * 0x4);
            <utils::Reg<fields::Alrmssr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Backup register"]
    pub const fn bkpr(&self, idx: usize) -> utils::Reg<fields::Bkpr, utils::RW> {
        assert!(idx < 20);
        unsafe {
            let ptr = self.ptr.add(0x50 + idx * 0x4);
            <utils::Reg<fields::Bkpr, utils::RW>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Alarm register"]
    pub struct Alrmr {
        bits: u32,
    }
    impl Default for Alrmr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Alrmr {
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
        pub const fn set_msk1(mut self, val: vals::AlrmrMsk) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "Alarm seconds mask"]
        pub const fn msk1(self) -> vals::AlrmrMsk {
            let val = ((self.bits >> 0x7) & 0x1) as _;
            unsafe { vals::AlrmrMsk::from_bits_unchecked(val) }
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
        pub const fn set_msk2(mut self, val: vals::AlrmrMsk) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
            self
        }
        #[inline(always)]
        #[doc = "Alarm minutes mask"]
        pub const fn msk2(self) -> vals::AlrmrMsk {
            let val = ((self.bits >> 0xf) & 0x1) as _;
            unsafe { vals::AlrmrMsk::from_bits_unchecked(val) }
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
        pub const fn set_pm(mut self, val: vals::AlrmrPm) -> Self {
            self.bits &= !(0x1 << 0x16);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x16;
            self
        }
        #[inline(always)]
        #[doc = "AM/PM notation"]
        pub const fn pm(self) -> vals::AlrmrPm {
            let val = ((self.bits >> 0x16) & 0x1) as _;
            unsafe { vals::AlrmrPm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Alarm hours mask"]
        pub const fn set_msk3(mut self, val: vals::AlrmrMsk) -> Self {
            self.bits &= !(0x1 << 0x17);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x17;
            self
        }
        #[inline(always)]
        #[doc = "Alarm hours mask"]
        pub const fn msk3(self) -> vals::AlrmrMsk {
            let val = ((self.bits >> 0x17) & 0x1) as _;
            unsafe { vals::AlrmrMsk::from_bits_unchecked(val) }
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
        pub const fn set_wdsel(mut self, val: vals::AlrmrWdsel) -> Self {
            self.bits &= !(0x1 << 0x1e);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1e;
            self
        }
        #[inline(always)]
        #[doc = "Week day selection"]
        pub const fn wdsel(self) -> vals::AlrmrWdsel {
            let val = ((self.bits >> 0x1e) & 0x1) as _;
            unsafe { vals::AlrmrWdsel::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Alarm date mask"]
        pub const fn set_msk4(mut self, val: vals::AlrmrMsk) -> Self {
            self.bits &= !(0x1 << 0x1f);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1f;
            self
        }
        #[inline(always)]
        #[doc = "Alarm date mask"]
        pub const fn msk4(self) -> vals::AlrmrMsk {
            let val = ((self.bits >> 0x1f) & 0x1) as _;
            unsafe { vals::AlrmrMsk::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Alarm sub second register"]
    pub struct Alrmssr {
        bits: u32,
    }
    impl Default for Alrmssr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Alrmssr {
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
    pub struct Bkpr {
        bits: u32,
    }
    impl Default for Bkpr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bkpr {
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
    pub struct Calibr {
        bits: u32,
    }
    impl Default for Calibr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Calibr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Digital calibration"]
        pub const fn set_dc(mut self, val: u8) -> Self {
            self.bits &= !(0x1f << 0x0);
            self.bits |= (val as u32 & 0x1f) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Digital calibration"]
        pub const fn dc(self) -> u8 {
            ((self.bits >> 0x0) & 0x1f) as _
        }
        #[inline(always)]
        #[doc = "Digital calibration sign"]
        pub const fn set_dcs(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Digital calibration sign"]
        pub const fn dcs(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Calibration register"]
    pub struct Calr {
        bits: u32,
    }
    impl Default for Calr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Calr {
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
        pub const fn set_calw16(mut self, val: vals::Calw16) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xd;
            self
        }
        #[inline(always)]
        #[doc = "Use a 16-second calibration cycle period"]
        pub const fn calw16(self) -> vals::Calw16 {
            let val = ((self.bits >> 0xd) & 0x1) as _;
            unsafe { vals::Calw16::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Use an 8-second calibration cycle period"]
        pub const fn set_calw8(mut self, val: vals::Calw8) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xe;
            self
        }
        #[inline(always)]
        #[doc = "Use an 8-second calibration cycle period"]
        pub const fn calw8(self) -> vals::Calw8 {
            let val = ((self.bits >> 0xe) & 0x1) as _;
            unsafe { vals::Calw8::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Increase frequency of RTC by 488.5 ppm"]
        pub const fn set_calp(mut self, val: vals::Calp) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
            self
        }
        #[inline(always)]
        #[doc = "Increase frequency of RTC by 488.5 ppm"]
        pub const fn calp(self) -> vals::Calp {
            let val = ((self.bits >> 0xf) & 0x1) as _;
            unsafe { vals::Calp::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Control register"]
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
        #[doc = "Wakeup clock selection"]
        pub const fn set_wucksel(mut self, val: vals::Wucksel) -> Self {
            self.bits &= !(0x7 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Wakeup clock selection"]
        pub const fn wucksel(self) -> vals::Wucksel {
            let val = ((self.bits >> 0x0) & 0x7) as _;
            unsafe { vals::Wucksel::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Timestamp event active edge"]
        pub const fn set_tsedge(mut self, val: vals::Tsedge) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Timestamp event active edge"]
        pub const fn tsedge(self) -> vals::Tsedge {
            let val = ((self.bits >> 0x3) & 0x1) as _;
            unsafe { vals::Tsedge::from_bits_unchecked(val) }
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
        pub const fn set_fmt(mut self, val: vals::Fmt) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x6;
            self
        }
        #[inline(always)]
        #[doc = "Hour format"]
        pub const fn fmt(self) -> vals::Fmt {
            let val = ((self.bits >> 0x6) & 0x1) as _;
            unsafe { vals::Fmt::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Coarse digital calibration enable"]
        pub const fn set_dce(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Coarse digital calibration enable"]
        pub const fn dce(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
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
        pub const fn set_cosel(mut self, val: vals::Cosel) -> Self {
            self.bits &= !(0x1 << 0x13);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x13;
            self
        }
        #[inline(always)]
        #[doc = "Calibration output selection"]
        pub const fn cosel(self) -> vals::Cosel {
            let val = ((self.bits >> 0x13) & 0x1) as _;
            unsafe { vals::Cosel::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Output polarity"]
        pub const fn set_pol(mut self, val: vals::Pol) -> Self {
            self.bits &= !(0x1 << 0x14);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x14;
            self
        }
        #[inline(always)]
        #[doc = "Output polarity"]
        pub const fn pol(self) -> vals::Pol {
            let val = ((self.bits >> 0x14) & 0x1) as _;
            unsafe { vals::Pol::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Output selection"]
        pub const fn set_osel(mut self, val: vals::Osel) -> Self {
            self.bits &= !(0x3 << 0x15);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x15;
            self
        }
        #[inline(always)]
        #[doc = "Output selection"]
        pub const fn osel(self) -> vals::Osel {
            let val = ((self.bits >> 0x15) & 0x3) as _;
            unsafe { vals::Osel::from_bits_unchecked(val) }
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
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Date register"]
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
    pub struct Isr {
        bits: u32,
    }
    impl Default for Isr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Isr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Alarm write allowed"]
        pub const fn set_alrwf(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x0 + idx * 0x1));
            self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Alarm write allowed"]
        pub const fn alrwf(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Wakeup timer write allowed"]
        pub const fn set_wutwf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Wakeup timer write allowed"]
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
        pub const fn set_recalpf(mut self, val: vals::Recalpf) -> Self {
            self.bits &= !(0x1 << 0x10);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Recalibration pending flag"]
        pub const fn recalpf(self) -> vals::Recalpf {
            let val = ((self.bits >> 0x10) & 0x1) as _;
            unsafe { vals::Recalpf::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Prescaler register"]
    pub struct Prer {
        bits: u32,
    }
    impl Default for Prer {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Prer {
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
    pub struct Shiftr {
        bits: u32,
    }
    impl Default for Shiftr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Shiftr {
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
    pub struct Ssr {
        bits: u32,
    }
    impl Default for Ssr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ssr {
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
    #[doc = "Tamper and alternate function configuration register"]
    pub struct Tafcr {
        bits: u32,
    }
    impl Default for Tafcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tafcr {
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
        pub const fn set_tampfreq(mut self, val: vals::Tampfreq) -> Self {
            self.bits &= !(0x7 << 0x8);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Tamper sampling frequency"]
        pub const fn tampfreq(self) -> vals::Tampfreq {
            let val = ((self.bits >> 0x8) & 0x7) as _;
            unsafe { vals::Tampfreq::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Tamper filter count"]
        pub const fn set_tampflt(mut self, val: vals::Tampflt) -> Self {
            self.bits &= !(0x3 << 0xb);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0xb;
            self
        }
        #[inline(always)]
        #[doc = "Tamper filter count"]
        pub const fn tampflt(self) -> vals::Tampflt {
            let val = ((self.bits >> 0xb) & 0x3) as _;
            unsafe { vals::Tampflt::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Tamper precharge duration"]
        pub const fn set_tampprch(mut self, val: vals::Tampprch) -> Self {
            self.bits &= !(0x3 << 0xd);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0xd;
            self
        }
        #[inline(always)]
        #[doc = "Tamper precharge duration"]
        pub const fn tampprch(self) -> vals::Tampprch {
            let val = ((self.bits >> 0xd) & 0x3) as _;
            unsafe { vals::Tampprch::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Tamper pull-up disable"]
        pub const fn set_tamppudis(mut self, val: vals::Tamppudis) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
            self
        }
        #[inline(always)]
        #[doc = "Tamper pull-up disable"]
        pub const fn tamppudis(self) -> vals::Tamppudis {
            let val = ((self.bits >> 0xf) & 0x1) as _;
            unsafe { vals::Tamppudis::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Tamper 1 mapping"]
        pub const fn set_tamp1insel(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x10);
            self.bits |= if val { 1 << 0x10 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Tamper 1 mapping"]
        pub const fn tamp1insel(self) -> bool {
            ((self.bits >> 0x10) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Timestamp mapping"]
        pub const fn set_tsinsel(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x11);
            self.bits |= if val { 1 << 0x11 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Timestamp mapping"]
        pub const fn tsinsel(self) -> bool {
            ((self.bits >> 0x11) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "AFO_ALARM output type"]
        pub const fn set_alarmouttype(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x12);
            self.bits |= if val { 1 << 0x12 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "AFO_ALARM output type"]
        pub const fn alarmouttype(self) -> bool {
            ((self.bits >> 0x12) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Time register"]
    pub struct Tr {
        bits: u32,
    }
    impl Default for Tr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tr {
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
        pub const fn set_pm(mut self, val: vals::Ampm) -> Self {
            self.bits &= !(0x1 << 0x16);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x16;
            self
        }
        #[inline(always)]
        #[doc = "AM/PM notation"]
        pub const fn pm(self) -> vals::Ampm {
            let val = ((self.bits >> 0x16) & 0x1) as _;
            unsafe { vals::Ampm::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Timestamp date register"]
    pub struct Tsdr {
        bits: u32,
    }
    impl Default for Tsdr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tsdr {
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
    pub struct Tsssr {
        bits: u32,
    }
    impl Default for Tsssr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tsssr {
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
    pub struct Tstr {
        bits: u32,
    }
    impl Default for Tstr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tstr {
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
        pub const fn set_pm(mut self, val: vals::Ampm) -> Self {
            self.bits &= !(0x1 << 0x16);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x16;
            self
        }
        #[inline(always)]
        #[doc = "AM/PM notation"]
        pub const fn pm(self) -> vals::Ampm {
            let val = ((self.bits >> 0x16) & 0x1) as _;
            unsafe { vals::Ampm::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Write protection register"]
    pub struct Wpr {
        bits: u32,
    }
    impl Default for Wpr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Wpr {
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
    pub struct Wutr {
        bits: u32,
    }
    impl Default for Wutr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Wutr {
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
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum AlrmrMsk {
        #[doc = "Alarm set if the date/day match"]
        ToMatch = 0x0,
        #[doc = "Date/day don’t care in Alarm comparison"]
        NotMatch = 0x1,
    }
    impl AlrmrMsk {
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
    pub enum AlrmrPm {
        #[doc = "AM or 24-hour format"]
        Am = 0x0,
        #[doc = "PM"]
        Pm = 0x1,
    }
    impl AlrmrPm {
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
    pub enum AlrmrWdsel {
        #[doc = "DU[3:0] represents the date units"]
        DateUnits = 0x0,
        #[doc = "DU[3:0] represents the week day. DT[1:0] is don’t care"]
        WeekDay = 0x1,
    }
    impl AlrmrWdsel {
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
    pub enum Ampm {
        #[doc = "AM or 24-hour format"]
        Am = 0x0,
        #[doc = "PM"]
        Pm = 0x1,
    }
    impl Ampm {
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
    pub enum Calp {
        #[doc = "No RTCCLK pulses are added"]
        NoChange = 0x0,
        #[doc = "One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)"]
        IncreaseFreq = 0x1,
    }
    impl Calp {
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
    pub enum Calw16 {
        #[doc = "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1"]
        SixteenSecond = 0x1,
    }
    impl Calw16 {
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
    pub enum Calw8 {
        #[doc = "When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected"]
        EightSecond = 0x1,
    }
    impl Calw8 {
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
    pub enum Cosel {
        #[doc = "Calibration output is 512 Hz (with default prescaler setting)"]
        CalFreq512hz = 0x0,
        #[doc = "Calibration output is 1 Hz (with default prescaler setting)"]
        CalFreq1hz = 0x1,
    }
    impl Cosel {
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
    pub enum Fmt {
        #[doc = "24 hour/day format"]
        TwentyFourHour = 0x0,
        #[doc = "AM/PM hour format"]
        AmPm = 0x1,
    }
    impl Fmt {
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
    pub enum Osel {
        #[doc = "Output disabled"]
        Disabled = 0x0,
        #[doc = "Alarm A output enabled"]
        AlarmA = 0x1,
        #[doc = "Alarm B output enabled"]
        AlarmB = 0x2,
        #[doc = "Wakeup output enabled"]
        Wakeup = 0x3,
    }
    impl Osel {
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
    pub enum Pol {
        #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0])"]
        High = 0x0,
        #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0])"]
        Low = 0x1,
    }
    impl Pol {
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
    pub enum Recalpf {
        #[doc = "The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
        Pending = 0x1,
    }
    impl Recalpf {
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
    pub enum Tampflt {
        #[doc = "Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)"]
        Immediate = 0x0,
        #[doc = "Tamper event is activated after 2 consecutive samples at the active level"]
        Samples2 = 0x1,
        #[doc = "Tamper event is activated after 4 consecutive samples at the active level"]
        Samples4 = 0x2,
        #[doc = "Tamper event is activated after 8 consecutive samples at the active level"]
        Samples8 = 0x3,
    }
    impl Tampflt {
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
    pub enum Tampfreq {
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
    impl Tampfreq {
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
    pub enum Tampprch {
        #[doc = "1 RTCCLK cycle"]
        Cycles1 = 0x0,
        #[doc = "2 RTCCLK cycles"]
        Cycles2 = 0x1,
        #[doc = "4 RTCCLK cycles"]
        Cycles4 = 0x2,
        #[doc = "8 RTCCLK cycles"]
        Cycles8 = 0x3,
    }
    impl Tampprch {
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
    pub enum Tamppudis {
        #[doc = "Precharge RTC_TAMPx pins before sampling (enable internal pull-up)"]
        Enabled = 0x0,
        #[doc = "Disable precharge of RTC_TAMPx pins"]
        Disabled = 0x1,
    }
    impl Tamppudis {
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
    pub enum Tamptrg {
        #[doc = "If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
        RisingEdge = 0x0,
        #[doc = "If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
        FallingEdge = 0x1,
    }
    impl Tamptrg {
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
    pub enum Tsedge {
        #[doc = "RTC_TS input rising edge generates a time-stamp event"]
        RisingEdge = 0x0,
        #[doc = "RTC_TS input falling edge generates a time-stamp event"]
        FallingEdge = 0x1,
    }
    impl Tsedge {
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
    pub enum Wucksel {
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
    impl Wucksel {
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
