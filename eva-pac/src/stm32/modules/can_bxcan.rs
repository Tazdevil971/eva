
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Controller area network"]
pub struct Can {
    ptr: *mut u8,
}
impl Can {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "master control register"]
    pub const fn mcr(&self) -> utils::Reg<fields::Mcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Mcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "master status register"]
    pub const fn msr(&self) -> utils::Reg<fields::Msr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Msr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "transmit status register"]
    pub const fn tsr(&self) -> utils::Reg<fields::Tsr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Tsr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "receive FIFO 0 register"]
    pub const fn rfr(&self, idx: usize) -> utils::Reg<fields::Rfr, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0xc + idx * 0x4);
            <utils::Reg<fields::Rfr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "interrupt enable register"]
    pub const fn ier(&self) -> utils::Reg<fields::Ier, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Ier, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "error status register"]
    pub const fn esr(&self) -> utils::Reg<fields::Esr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Esr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "bit timing register"]
    pub const fn btr(&self) -> utils::Reg<fields::Btr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<fields::Btr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "CAN Transmit cluster"]
    pub const fn tx(&self, idx: usize) -> Tx {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x180 + idx * 0x10);
            <Tx>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "CAN Receive cluster"]
    pub const fn rx(&self, idx: usize) -> Rx {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x1b0 + idx * 0x10);
            <Rx>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "filter master register"]
    pub const fn fmr(&self) -> utils::Reg<fields::Fmr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x200);
            <utils::Reg<fields::Fmr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "filter mode register"]
    pub const fn fm1r(&self) -> utils::Reg<fields::Fm1r, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x204);
            <utils::Reg<fields::Fm1r, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "filter scale register"]
    pub const fn fs1r(&self) -> utils::Reg<fields::Fs1r, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20c);
            <utils::Reg<fields::Fs1r, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "filter FIFO assignment register"]
    pub const fn ffa1r(&self) -> utils::Reg<fields::Ffa1r, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x214);
            <utils::Reg<fields::Ffa1r, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "filter activation register"]
    pub const fn fa1r(&self) -> utils::Reg<fields::Fa1r, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x21c);
            <utils::Reg<fields::Fa1r, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "CAN Filter Bank cluster"]
    pub const fn fb(&self, idx: usize) -> Fb {
        assert!(idx < 28);
        unsafe {
            let ptr = self.ptr.add(0x240 + idx * 0x8);
            <Fb>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "CAN Filter Bank cluster"]
pub struct Fb {
    ptr: *mut u8,
}
impl Fb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "Filter bank 0 register 1"]
    pub const fn fr1(&self) -> utils::Reg<fields::Fr1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Fr1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Filter bank 0 register 2"]
    pub const fn fr2(&self) -> utils::Reg<fields::Fr2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Fr2, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "CAN Receive cluster"]
pub struct Rx {
    ptr: *mut u8,
}
impl Rx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "receive FIFO mailbox identifier register"]
    pub const fn rir(&self) -> utils::Reg<fields::Rir, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Rir, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "mailbox data high register"]
    pub const fn rdtr(&self) -> utils::Reg<fields::Rdtr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Rdtr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "mailbox data high register"]
    pub const fn rdlr(&self) -> utils::Reg<fields::Rdlr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Rdlr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "receive FIFO mailbox data high register"]
    pub const fn rdhr(&self) -> utils::Reg<fields::Rdhr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Rdhr, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "CAN Transmit cluster"]
pub struct Tx {
    ptr: *mut u8,
}
impl Tx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "TX mailbox identifier register"]
    pub const fn tir(&self) -> utils::Reg<fields::Tir, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Tir, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "mailbox data length control and time stamp register"]
    pub const fn tdtr(&self) -> utils::Reg<fields::Tdtr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Tdtr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "mailbox data low register"]
    pub const fn tdlr(&self) -> utils::Reg<fields::Tdlr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Tdlr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "mailbox data high register"]
    pub const fn tdhr(&self) -> utils::Reg<fields::Tdhr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Tdhr, utils::RW>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "bit timing register"]
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
        #[doc = "BRP"]
        pub const fn set_brp(mut self, val: u16) -> Self {
            self.bits &= !(0x3ff << 0x0);
            self.bits |= (val as u32 & 0x3ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "BRP"]
        pub const fn brp(self) -> u16 {
            ((self.bits >> 0x0) & 0x3ff) as _
        }
        #[inline(always)]
        #[doc = "TS1"]
        pub const fn set_ts(mut self, idx: usize, val: u8) -> Self {
            assert!(idx < 2);
            self.bits &= !(0xf << (0x10 + idx * 0x4));
            self.bits |= (val as u32 & 0xf) << (0x10 + idx * 0x4);
            self
        }
        #[inline(always)]
        #[doc = "TS1"]
        pub const fn ts(self, idx: usize) -> u8 {
            assert!(idx < 2);
            ((self.bits >> (0x10 + idx * 0x4)) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "SJW"]
        pub const fn set_sjw(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x18);
            self.bits |= (val as u32 & 0x3) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "SJW"]
        pub const fn sjw(self) -> u8 {
            ((self.bits >> 0x18) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Loop Back Mode enabled"]
        pub const fn set_lbkm(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1e);
            self.bits |= if val { 1 << 0x1e } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Loop Back Mode enabled"]
        pub const fn lbkm(self) -> bool {
            ((self.bits >> 0x1e) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SILM"]
        pub const fn set_silm(mut self, val: vals::Silm) -> Self {
            self.bits &= !(0x1 << 0x1f);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1f;
            self
        }
        #[inline(always)]
        #[doc = "SILM"]
        pub const fn silm(self) -> vals::Silm {
            let val = ((self.bits >> 0x1f) & 0x1) as _;
            unsafe { vals::Silm::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "interrupt enable register"]
    pub struct Esr {
        bits: u32,
    }
    impl Default for Esr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Esr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "EWGF"]
        pub const fn set_ewgf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "EWGF"]
        pub const fn ewgf(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "EPVF"]
        pub const fn set_epvf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "EPVF"]
        pub const fn epvf(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "BOFF"]
        pub const fn set_boff(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "BOFF"]
        pub const fn boff(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "LEC"]
        pub const fn set_lec(mut self, val: vals::Lec) -> Self {
            self.bits &= !(0x7 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "LEC"]
        pub const fn lec(self) -> vals::Lec {
            let val = ((self.bits >> 0x4) & 0x7) as _;
            unsafe { vals::Lec::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "TEC"]
        pub const fn set_tec(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x10);
            self.bits |= (val as u32 & 0xff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "TEC"]
        pub const fn tec(self) -> u8 {
            ((self.bits >> 0x10) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "REC"]
        pub const fn set_rec(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x18);
            self.bits |= (val as u32 & 0xff) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "REC"]
        pub const fn rec(self) -> u8 {
            ((self.bits >> 0x18) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "filter activation register"]
    pub struct Fa1r {
        bits: u32,
    }
    impl Default for Fa1r {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fa1r {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Filter active"]
        pub const fn set_fact(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 28);
            self.bits &= !(0x1 << (0x0 + idx * 0x1));
            self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Filter active"]
        pub const fn fact(self, idx: usize) -> bool {
            assert!(idx < 28);
            ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "filter FIFO assignment register"]
    pub struct Ffa1r {
        bits: u32,
    }
    impl Default for Ffa1r {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ffa1r {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Filter FIFO assignment for filter 0"]
        pub const fn set_ffa(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 28);
            self.bits &= !(0x1 << (0x0 + idx * 0x1));
            self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Filter FIFO assignment for filter 0"]
        pub const fn ffa(self, idx: usize) -> bool {
            assert!(idx < 28);
            ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "filter mode register"]
    pub struct Fm1r {
        bits: u32,
    }
    impl Default for Fm1r {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fm1r {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Filter mode"]
        pub const fn set_fbm(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 28);
            self.bits &= !(0x1 << (0x0 + idx * 0x1));
            self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Filter mode"]
        pub const fn fbm(self, idx: usize) -> bool {
            assert!(idx < 28);
            ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "filter master register"]
    pub struct Fmr {
        bits: u32,
    }
    impl Default for Fmr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fmr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "FINIT"]
        pub const fn set_finit(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "FINIT"]
        pub const fn finit(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CAN2SB"]
        pub const fn set_can2sb(mut self, val: u8) -> Self {
            self.bits &= !(0x3f << 0x8);
            self.bits |= (val as u32 & 0x3f) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "CAN2SB"]
        pub const fn can2sb(self) -> u8 {
            ((self.bits >> 0x8) & 0x3f) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Filter bank 0 register 1"]
    pub struct Fr1 {
        bits: u32,
    }
    impl Default for Fr1 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fr1 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Filter bits"]
        pub const fn set_fb(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 32);
            self.bits &= !(0x1 << (0x0 + idx * 0x1));
            self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Filter bits"]
        pub const fn fb(self, idx: usize) -> bool {
            assert!(idx < 32);
            ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Filter bank 0 register 2"]
    pub struct Fr2 {
        bits: u32,
    }
    impl Default for Fr2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fr2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Filter bits"]
        pub const fn set_fb(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 32);
            self.bits &= !(0x1 << (0x0 + idx * 0x1));
            self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Filter bits"]
        pub const fn fb(self, idx: usize) -> bool {
            assert!(idx < 32);
            ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "filter scale register"]
    pub struct Fs1r {
        bits: u32,
    }
    impl Default for Fs1r {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fs1r {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Filter scale configuration"]
        pub const fn set_fsc(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 28);
            self.bits &= !(0x1 << (0x0 + idx * 0x1));
            self.bits |= if val { 1 << (0x0 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Filter scale configuration"]
        pub const fn fsc(self, idx: usize) -> bool {
            assert!(idx < 28);
            ((self.bits >> (0x0 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "interrupt enable register"]
    pub struct Ier {
        bits: u32,
    }
    impl Default for Ier {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ier {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "TMEIE"]
        pub const fn set_tmeie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TMEIE"]
        pub const fn tmeie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "FMPIE0"]
        pub const fn set_fmpie(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x1 + idx * 0x3));
            self.bits |= if val { 1 << (0x1 + idx * 0x3) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "FMPIE0"]
        pub const fn fmpie(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x1 + idx * 0x3)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "FFIE0"]
        pub const fn set_ffie(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x2 + idx * 0x3));
            self.bits |= if val { 1 << (0x2 + idx * 0x3) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "FFIE0"]
        pub const fn ffie(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x2 + idx * 0x3)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "FOVIE0"]
        pub const fn set_fovie(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x3 + idx * 0x3));
            self.bits |= if val { 1 << (0x3 + idx * 0x3) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "FOVIE0"]
        pub const fn fovie(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x3 + idx * 0x3)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "EWGIE"]
        pub const fn set_ewgie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "EWGIE"]
        pub const fn ewgie(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "EPVIE"]
        pub const fn set_epvie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "EPVIE"]
        pub const fn epvie(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "BOFIE"]
        pub const fn set_bofie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= if val { 1 << 0xa } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "BOFIE"]
        pub const fn bofie(self) -> bool {
            ((self.bits >> 0xa) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "LECIE"]
        pub const fn set_lecie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "LECIE"]
        pub const fn lecie(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "ERRIE"]
        pub const fn set_errie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= if val { 1 << 0xf } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "ERRIE"]
        pub const fn errie(self) -> bool {
            ((self.bits >> 0xf) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "WKUIE"]
        pub const fn set_wkuie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x10);
            self.bits |= if val { 1 << 0x10 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "WKUIE"]
        pub const fn wkuie(self) -> bool {
            ((self.bits >> 0x10) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SLKIE"]
        pub const fn set_slkie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x11);
            self.bits |= if val { 1 << 0x11 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SLKIE"]
        pub const fn slkie(self) -> bool {
            ((self.bits >> 0x11) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "master control register"]
    pub struct Mcr {
        bits: u32,
    }
    impl Default for Mcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Mcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "INRQ"]
        pub const fn set_inrq(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "INRQ"]
        pub const fn inrq(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SLEEP"]
        pub const fn set_sleep(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SLEEP"]
        pub const fn sleep(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TXFP"]
        pub const fn set_txfp(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TXFP"]
        pub const fn txfp(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "RFLM"]
        pub const fn set_rflm(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RFLM"]
        pub const fn rflm(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "NART"]
        pub const fn set_nart(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "NART"]
        pub const fn nart(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "AWUM"]
        pub const fn set_awum(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "AWUM"]
        pub const fn awum(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "ABOM"]
        pub const fn set_abom(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "ABOM"]
        pub const fn abom(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TTCM"]
        pub const fn set_ttcm(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TTCM"]
        pub const fn ttcm(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "RESET"]
        pub const fn set_reset(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= if val { 1 << 0xf } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RESET"]
        pub const fn reset(self) -> bool {
            ((self.bits >> 0xf) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "DBF"]
        pub const fn set_dbf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x10);
            self.bits |= if val { 1 << 0x10 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DBF"]
        pub const fn dbf(self) -> bool {
            ((self.bits >> 0x10) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "master status register"]
    pub struct Msr {
        bits: u32,
    }
    impl Default for Msr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Msr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "INAK"]
        pub const fn set_inak(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "INAK"]
        pub const fn inak(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SLAK"]
        pub const fn set_slak(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SLAK"]
        pub const fn slak(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "ERRI"]
        pub const fn set_erri(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "ERRI"]
        pub const fn erri(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "WKUI"]
        pub const fn set_wkui(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "WKUI"]
        pub const fn wkui(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SLAKI"]
        pub const fn set_slaki(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SLAKI"]
        pub const fn slaki(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TXM"]
        pub const fn set_txm(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TXM"]
        pub const fn txm(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "RXM"]
        pub const fn set_rxm(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RXM"]
        pub const fn rxm(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "SAMP"]
        pub const fn set_samp(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= if val { 1 << 0xa } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "SAMP"]
        pub const fn samp(self) -> bool {
            ((self.bits >> 0xa) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "RX"]
        pub const fn set_rx(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RX"]
        pub const fn rx(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "receive FIFO mailbox data high register"]
    pub struct Rdhr {
        bits: u32,
    }
    impl Default for Rdhr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Rdhr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DATA4"]
        pub const fn set_data(mut self, idx: usize, val: u8) -> Self {
            assert!(idx < 4);
            self.bits &= !(0xff << (0x0 + idx * 0x8));
            self.bits |= (val as u32 & 0xff) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "DATA4"]
        pub const fn data(self, idx: usize) -> u8 {
            assert!(idx < 4);
            ((self.bits >> (0x0 + idx * 0x8)) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "mailbox data high register"]
    pub struct Rdlr {
        bits: u32,
    }
    impl Default for Rdlr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Rdlr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DATA0"]
        pub const fn set_data(mut self, idx: usize, val: u8) -> Self {
            assert!(idx < 4);
            self.bits &= !(0xff << (0x0 + idx * 0x8));
            self.bits |= (val as u32 & 0xff) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "DATA0"]
        pub const fn data(self, idx: usize) -> u8 {
            assert!(idx < 4);
            ((self.bits >> (0x0 + idx * 0x8)) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "mailbox data high register"]
    pub struct Rdtr {
        bits: u32,
    }
    impl Default for Rdtr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Rdtr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DLC"]
        pub const fn set_dlc(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x0);
            self.bits |= (val as u32 & 0xf) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DLC"]
        pub const fn dlc(self) -> u8 {
            ((self.bits >> 0x0) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "FMI"]
        pub const fn set_fmi(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "FMI"]
        pub const fn fmi(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "TIME"]
        pub const fn set_time(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x10);
            self.bits |= (val as u32 & 0xffff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "TIME"]
        pub const fn time(self) -> u16 {
            ((self.bits >> 0x10) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "receive FIFO 0 register"]
    pub struct Rfr {
        bits: u32,
    }
    impl Default for Rfr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Rfr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "FMP0"]
        pub const fn set_fmp(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x0);
            self.bits |= (val as u32 & 0x3) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "FMP0"]
        pub const fn fmp(self) -> u8 {
            ((self.bits >> 0x0) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "FULL0"]
        pub const fn set_full(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "FULL0"]
        pub const fn full(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "FOVR0"]
        pub const fn set_fovr(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "FOVR0"]
        pub const fn fovr(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "RFOM0"]
        pub const fn set_rfom(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RFOM0"]
        pub const fn rfom(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "receive FIFO mailbox identifier register"]
    pub struct Rir {
        bits: u32,
    }
    impl Default for Rir {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Rir {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "RTR"]
        pub const fn set_rtr(mut self, val: vals::Rtr) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "RTR"]
        pub const fn rtr(self) -> vals::Rtr {
            let val = ((self.bits >> 0x1) & 0x1) as _;
            unsafe { vals::Rtr::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "IDE"]
        pub const fn set_ide(mut self, val: vals::Ide) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "IDE"]
        pub const fn ide(self) -> vals::Ide {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Ide::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "EXID"]
        pub const fn set_exid(mut self, val: u32) -> Self {
            self.bits &= !(0x3ffff << 0x3);
            self.bits |= (val as u32 & 0x3ffff) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "EXID"]
        pub const fn exid(self) -> u32 {
            ((self.bits >> 0x3) & 0x3ffff) as _
        }
        #[inline(always)]
        #[doc = "STID"]
        pub const fn set_stid(mut self, val: u16) -> Self {
            self.bits &= !(0x7ff << 0x15);
            self.bits |= (val as u32 & 0x7ff) << 0x15;
            self
        }
        #[inline(always)]
        #[doc = "STID"]
        pub const fn stid(self) -> u16 {
            ((self.bits >> 0x15) & 0x7ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "mailbox data high register"]
    pub struct Tdhr {
        bits: u32,
    }
    impl Default for Tdhr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tdhr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DATA4"]
        pub const fn set_data(mut self, idx: usize, val: u8) -> Self {
            assert!(idx < 4);
            self.bits &= !(0xff << (0x0 + idx * 0x8));
            self.bits |= (val as u32 & 0xff) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "DATA4"]
        pub const fn data(self, idx: usize) -> u8 {
            assert!(idx < 4);
            ((self.bits >> (0x0 + idx * 0x8)) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "mailbox data low register"]
    pub struct Tdlr {
        bits: u32,
    }
    impl Default for Tdlr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tdlr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DATA0"]
        pub const fn set_data(mut self, idx: usize, val: u8) -> Self {
            assert!(idx < 4);
            self.bits &= !(0xff << (0x0 + idx * 0x8));
            self.bits |= (val as u32 & 0xff) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "DATA0"]
        pub const fn data(self, idx: usize) -> u8 {
            assert!(idx < 4);
            ((self.bits >> (0x0 + idx * 0x8)) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "mailbox data length control and time stamp register"]
    pub struct Tdtr {
        bits: u32,
    }
    impl Default for Tdtr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tdtr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DLC"]
        pub const fn set_dlc(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x0);
            self.bits |= (val as u32 & 0xf) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DLC"]
        pub const fn dlc(self) -> u8 {
            ((self.bits >> 0x0) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "TGT"]
        pub const fn set_tgt(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TGT"]
        pub const fn tgt(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TIME"]
        pub const fn set_time(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x10);
            self.bits |= (val as u32 & 0xffff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "TIME"]
        pub const fn time(self) -> u16 {
            ((self.bits >> 0x10) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "TX mailbox identifier register"]
    pub struct Tir {
        bits: u32,
    }
    impl Default for Tir {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tir {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "TXRQ"]
        pub const fn set_txrq(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TXRQ"]
        pub const fn txrq(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "RTR"]
        pub const fn set_rtr(mut self, val: vals::Rtr) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "RTR"]
        pub const fn rtr(self) -> vals::Rtr {
            let val = ((self.bits >> 0x1) & 0x1) as _;
            unsafe { vals::Rtr::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "IDE"]
        pub const fn set_ide(mut self, val: vals::Ide) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "IDE"]
        pub const fn ide(self) -> vals::Ide {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Ide::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "EXID"]
        pub const fn set_exid(mut self, val: u32) -> Self {
            self.bits &= !(0x3ffff << 0x3);
            self.bits |= (val as u32 & 0x3ffff) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "EXID"]
        pub const fn exid(self) -> u32 {
            ((self.bits >> 0x3) & 0x3ffff) as _
        }
        #[inline(always)]
        #[doc = "STID"]
        pub const fn set_stid(mut self, val: u16) -> Self {
            self.bits &= !(0x7ff << 0x15);
            self.bits |= (val as u32 & 0x7ff) << 0x15;
            self
        }
        #[inline(always)]
        #[doc = "STID"]
        pub const fn stid(self) -> u16 {
            ((self.bits >> 0x15) & 0x7ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "transmit status register"]
    pub struct Tsr {
        bits: u32,
    }
    impl Default for Tsr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tsr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "RQCP0"]
        pub const fn set_rqcp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 3);
            self.bits &= !(0x1 << (0x0 + idx * 0x8));
            self.bits |= if val { 1 << (0x0 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RQCP0"]
        pub const fn rqcp(self, idx: usize) -> bool {
            assert!(idx < 3);
            ((self.bits >> (0x0 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TXOK0"]
        pub const fn set_txok(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 3);
            self.bits &= !(0x1 << (0x1 + idx * 0x8));
            self.bits |= if val { 1 << (0x1 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TXOK0"]
        pub const fn txok(self, idx: usize) -> bool {
            assert!(idx < 3);
            ((self.bits >> (0x1 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "ALST0"]
        pub const fn set_alst(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 3);
            self.bits &= !(0x1 << (0x2 + idx * 0x8));
            self.bits |= if val { 1 << (0x2 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "ALST0"]
        pub const fn alst(self, idx: usize) -> bool {
            assert!(idx < 3);
            ((self.bits >> (0x2 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TERR0"]
        pub const fn set_terr(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 3);
            self.bits &= !(0x1 << (0x3 + idx * 0x8));
            self.bits |= if val { 1 << (0x3 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TERR0"]
        pub const fn terr(self, idx: usize) -> bool {
            assert!(idx < 3);
            ((self.bits >> (0x3 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "ABRQ0"]
        pub const fn set_abrq(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 3);
            self.bits &= !(0x1 << (0x7 + idx * 0x8));
            self.bits |= if val { 1 << (0x7 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "ABRQ0"]
        pub const fn abrq(self, idx: usize) -> bool {
            assert!(idx < 3);
            ((self.bits >> (0x7 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CODE"]
        pub const fn set_code(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x18);
            self.bits |= (val as u32 & 0x3) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "CODE"]
        pub const fn code(self) -> u8 {
            ((self.bits >> 0x18) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Lowest priority flag for mailbox 0"]
        pub const fn set_tme(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 3);
            self.bits &= !(0x1 << (0x1a + idx * 0x1));
            self.bits |= if val { 1 << (0x1a + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Lowest priority flag for mailbox 0"]
        pub const fn tme(self, idx: usize) -> bool {
            assert!(idx < 3);
            ((self.bits >> (0x1a + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Lowest priority flag for mailbox 0"]
        pub const fn set_low(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 3);
            self.bits &= !(0x1 << (0x1d + idx * 0x1));
            self.bits |= if val { 1 << (0x1d + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Lowest priority flag for mailbox 0"]
        pub const fn low(self, idx: usize) -> bool {
            assert!(idx < 3);
            ((self.bits >> (0x1d + idx * 0x1)) & 0x1) != 0
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum Ide {
        #[doc = "Standard identifier"]
        Standard = 0x0,
        #[doc = "Extended identifier"]
        Extended = 0x1,
    }
    impl Ide {
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
    pub enum Lec {
        #[doc = "No Error"]
        NoError = 0x0,
        #[doc = "Stuff Error"]
        Stuff = 0x1,
        #[doc = "Form Error"]
        Form = 0x2,
        #[doc = "Acknowledgment Error"]
        Ack = 0x3,
        #[doc = "Bit recessive Error"]
        BitRecessive = 0x4,
        #[doc = "Bit dominant Error"]
        BitDominant = 0x5,
        #[doc = "CRC Error"]
        Crc = 0x6,
        #[doc = "Set by software"]
        Custom = 0x7,
    }
    impl Lec {
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
    pub enum Rtr {
        #[doc = "Data frame"]
        Data = 0x0,
        #[doc = "Remote frame"]
        Remote = 0x1,
    }
    impl Rtr {
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
    pub enum Silm {
        #[doc = "Normal operation"]
        Normal = 0x0,
        #[doc = "Silent Mode"]
        Silent = 0x1,
    }
    impl Silm {
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
