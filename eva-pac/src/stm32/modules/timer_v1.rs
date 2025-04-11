
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "1-channel timers"]
pub struct Tim1ch {
    ptr: *mut u8,
}
impl Tim1ch {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr11ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr11ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<fields::Dier1ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Dier1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::Sr1ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Sr1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<fields::Egr1ch, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Egr1ch, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<fields::CcmrInput1ch, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrInput1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<fields::CcmrOutput1ch, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrOutput1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<fields::Ccer1ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::Ccer1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<fields::CntCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::CntCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "prescaler"]
    pub const fn psc(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "auto-reload register"]
    pub const fn arr(&self) -> utils::Reg<fields::ArrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::ArrCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<fields::Ccr1ch, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<fields::Ccr1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Option register 1\nNote: Check Reference Manual to parse this register content"]
    pub const fn or(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<fields::Tisel1ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<fields::Tisel1ch, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "1-channel with one complementary output timers"]
pub struct Tim1chCmp {
    ptr: *mut u8,
}
impl Tim1chCmp {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr11ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr11ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr21chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr21chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<fields::Dier1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Dier1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::Sr1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Sr1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<fields::Egr1chCmp, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Egr1chCmp, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<fields::CcmrInput1ch, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrInput1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<fields::CcmrOutput1ch, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrOutput1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<fields::Ccer1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::Ccer1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<fields::CntCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::CntCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "prescaler"]
    pub const fn psc(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "auto-reload register"]
    pub const fn arr(&self) -> utils::Reg<fields::ArrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::ArrCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "repetition counter register"]
    pub const fn rcr(&self) -> utils::Reg<fields::Rcr1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<fields::Rcr1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<fields::Ccr1ch, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<fields::Ccr1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "break and dead-time register"]
    pub const fn bdtr(&self) -> utils::Reg<fields::Bdtr1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<fields::Bdtr1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA control register"]
    pub const fn dcr(&self) -> utils::Reg<fields::Dcr1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<fields::Dcr1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA address for full transfer"]
    pub const fn dmar(&self) -> utils::Reg<fields::DmarGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<fields::DmarGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Option register 1\nNote: Check Reference Manual to parse this register content"]
    pub const fn or(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "alternate function register 1"]
    pub const fn af1(&self) -> utils::Reg<fields::Af11chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<fields::Af11chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<fields::Tisel1ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<fields::Tisel1ch, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "2-channel timers"]
pub struct Tim2ch {
    ptr: *mut u8,
}
impl Tim2ch {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr11ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr11ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr22ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr22ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "slave mode control register"]
    pub const fn smcr(&self) -> utils::Reg<fields::Smcr2ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Smcr2ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<fields::Dier2ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Dier2ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::Sr2ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Sr2ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<fields::Egr2ch, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Egr2ch, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<fields::CcmrInput2ch, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrInput2ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<fields::CcmrOutput2ch, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrOutput2ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<fields::Ccer2ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::Ccer2ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<fields::CntCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::CntCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "prescaler"]
    pub const fn psc(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "auto-reload register"]
    pub const fn arr(&self) -> utils::Reg<fields::ArrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::ArrCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1-2)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<fields::Ccr1ch, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<fields::Ccr1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Option register 1\nNote: Check Reference Manual to parse this register content"]
    pub const fn or(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<fields::Tisel2ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<fields::Tisel2ch, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "2-channel with one complementary output timers"]
pub struct Tim2chCmp {
    ptr: *mut u8,
}
impl Tim2chCmp {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr11ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr11ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr22chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr22chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "slave mode control register"]
    pub const fn smcr(&self) -> utils::Reg<fields::Smcr2ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Smcr2ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<fields::Dier2chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Dier2chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::Sr2chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Sr2chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<fields::Egr2chCmp, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Egr2chCmp, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<fields::CcmrInput1ch, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrInput1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<fields::CcmrOutput1ch, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrOutput1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<fields::Ccer2chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::Ccer2chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<fields::CntCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::CntCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "prescaler"]
    pub const fn psc(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "auto-reload register"]
    pub const fn arr(&self) -> utils::Reg<fields::ArrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::ArrCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "repetition counter register"]
    pub const fn rcr(&self) -> utils::Reg<fields::Rcr1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<fields::Rcr1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1-2)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<fields::Ccr1ch, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<fields::Ccr1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "break and dead-time register"]
    pub const fn bdtr(&self) -> utils::Reg<fields::Bdtr1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<fields::Bdtr1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA control register"]
    pub const fn dcr(&self) -> utils::Reg<fields::Dcr1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<fields::Dcr1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA address for full transfer"]
    pub const fn dmar(&self) -> utils::Reg<fields::DmarGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<fields::DmarGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Option register 1\nNote: Check Reference Manual to parse this register content"]
    pub const fn or(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "alternate function register 1"]
    pub const fn af1(&self) -> utils::Reg<fields::Af11chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<fields::Af11chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<fields::Tisel2ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<fields::Tisel2ch, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Advanced Control timers"]
pub struct TimAdv {
    ptr: *mut u8,
}
impl TimAdv {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1Gp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr1Gp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr2Adv, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr2Adv, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "slave mode control register"]
    pub const fn smcr(&self) -> utils::Reg<fields::SmcrGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::SmcrGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<fields::DierAdv, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::DierAdv, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::SrAdv, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::SrAdv, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<fields::EgrAdv, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::EgrAdv, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<fields::CcmrInput2ch, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrInput2ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<fields::CcmrOutputGp16, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrOutputGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<fields::CcerAdv, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::CcerAdv, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<fields::CntCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::CntCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "prescaler"]
    pub const fn psc(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "auto-reload register"]
    pub const fn arr(&self) -> utils::Reg<fields::ArrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::ArrCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "repetition counter register"]
    pub const fn rcr(&self) -> utils::Reg<fields::RcrAdv, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<fields::RcrAdv, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1-4)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<fields::Ccr1ch, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<fields::Ccr1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "break and dead-time register"]
    pub const fn bdtr(&self) -> utils::Reg<fields::BdtrAdv, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<fields::BdtrAdv, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA control register"]
    pub const fn dcr(&self) -> utils::Reg<fields::Dcr1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<fields::Dcr1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA address for full transfer"]
    pub const fn dmar(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Option register 1\nNote: Check Reference Manual to parse this register content"]
    pub const fn or(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 3"]
    pub const fn ccmr3(&self) -> utils::Reg<fields::Ccmr3Adv, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x54);
            <utils::Reg<fields::Ccmr3Adv, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register 5"]
    pub const fn ccr5(&self) -> utils::Reg<fields::Ccr5Adv, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x58);
            <utils::Reg<fields::Ccr5Adv, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register 6"]
    pub const fn ccr6(&self) -> utils::Reg<fields::Ccr1ch, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c);
            <utils::Reg<fields::Ccr1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "alternate function register 1"]
    pub const fn af1(&self) -> utils::Reg<fields::Af1Adv, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<fields::Af1Adv, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "alternate function register 2"]
    pub const fn af2(&self) -> utils::Reg<fields::Af2Adv, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x64);
            <utils::Reg<fields::Af2Adv, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<fields::TiselGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<fields::TiselGp16, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Basic timers"]
pub struct TimBasic {
    ptr: *mut u8,
}
impl TimBasic {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1Core, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr1Core, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr2Basic, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr2Basic, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<fields::DierBasicNoCr2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::DierBasicNoCr2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::SrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::SrCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<fields::EgrCore, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::EgrCore, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<fields::CntCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::CntCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "prescaler"]
    pub const fn psc(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "auto-reload register"]
    pub const fn arr(&self) -> utils::Reg<fields::ArrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::ArrCore, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Virtual Basic timers without CR2 register for common part of TIM_BASIC and TIM_1CH_CMP"]
pub struct TimBasicNoCr2 {
    ptr: *mut u8,
}
impl TimBasicNoCr2 {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1Core, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr1Core, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<fields::DierBasicNoCr2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::DierBasicNoCr2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::SrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::SrCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<fields::EgrCore, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::EgrCore, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<fields::CntCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::CntCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "prescaler"]
    pub const fn psc(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "auto-reload register"]
    pub const fn arr(&self) -> utils::Reg<fields::ArrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::ArrCore, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Virtual timer for common part of TIM_BASIC and TIM_1CH"]
pub struct TimCore {
    ptr: *mut u8,
}
impl TimCore {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1Core, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr1Core, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<fields::DierCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::DierCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::SrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::SrCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<fields::EgrCore, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::EgrCore, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<fields::CntCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::CntCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "prescaler"]
    pub const fn psc(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "auto-reload register"]
    pub const fn arr(&self) -> utils::Reg<fields::ArrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::ArrCore, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "General purpose 16-bit timers"]
pub struct TimGp16 {
    ptr: *mut u8,
}
impl TimGp16 {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1Gp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr1Gp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr2Gp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr2Gp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "slave mode control register"]
    pub const fn smcr(&self) -> utils::Reg<fields::SmcrGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::SmcrGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<fields::DierGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::DierGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::SrGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::SrGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<fields::EgrGp16, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::EgrGp16, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<fields::CcmrInput2ch, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrInput2ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<fields::CcmrOutputGp16, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrOutputGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<fields::CcerGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::CcerGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<fields::CntCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::CntCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "prescaler"]
    pub const fn psc(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "auto-reload register"]
    pub const fn arr(&self) -> utils::Reg<fields::ArrCore, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::ArrCore, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1-4)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<fields::Ccr1ch, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<fields::Ccr1ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA control register"]
    pub const fn dcr(&self) -> utils::Reg<fields::Dcr1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<fields::Dcr1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA address for full transfer"]
    pub const fn dmar(&self) -> utils::Reg<fields::DmarGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<fields::DmarGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Option register 1\nNote: Check Reference Manual to parse this register content"]
    pub const fn or(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "alternate function register 1"]
    pub const fn af1(&self) -> utils::Reg<fields::Af1Gp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<fields::Af1Gp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<fields::TiselGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<fields::TiselGp16, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "General purpose 32-bit timers"]
pub struct TimGp32 {
    ptr: *mut u8,
}
impl TimGp32 {
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
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1Gp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr1Gp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr2Gp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr2Gp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "slave mode control register"]
    pub const fn smcr(&self) -> utils::Reg<fields::SmcrGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::SmcrGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<fields::DierGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::DierGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<fields::SrGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::SrGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<fields::EgrGp16, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::EgrGp16, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<fields::CcmrInput2ch, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrInput2ch, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<fields::CcmrOutputGp16, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<fields::CcmrOutputGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<fields::CcerGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::CcerGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "prescaler"]
    pub const fn psc(&self) -> utils::Reg<u16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "auto-reload register"]
    pub const fn arr(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1-4)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<u32, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA control register"]
    pub const fn dcr(&self) -> utils::Reg<fields::Dcr1chCmp, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<fields::Dcr1chCmp, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA address for full transfer"]
    pub const fn dmar(&self) -> utils::Reg<fields::DmarGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<fields::DmarGp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Option register 1\nNote: Check Reference Manual to parse this register content"]
    pub const fn or(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "alternate function register 1"]
    pub const fn af1(&self) -> utils::Reg<fields::Af1Gp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<fields::Af1Gp16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<fields::TiselGp16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<fields::TiselGp16, utils::RW>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "alternate function register 1"]
    pub struct Af11chCmp {
        bits: u32,
    }
    impl Default for Af11chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Af11chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "TIMx_BKIN input enable"]
        pub const fn set_bkine(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TIMx_BKIN input enable"]
        pub const fn bkine(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TIM_BRK_CMPx (x=1-2) enable"]
        pub const fn set_bkcmpe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TIM_BRK_CMPx (x=1-2) enable"]
        pub const fn bkcmpe(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "BRK DFSDM1_BREAKx enable (x=0 if TIM15, x=1 if TIM16, x=2 if TIM17)"]
        pub const fn set_bkdf1bke(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "BRK DFSDM1_BREAKx enable (x=0 if TIM15, x=1 if TIM16, x=2 if TIM17)"]
        pub const fn bkdf1bke(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TIMx_BKIN input polarity"]
        pub const fn set_bkinp(mut self, val: vals::Bkinp) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
            self
        }
        #[inline(always)]
        #[doc = "TIMx_BKIN input polarity"]
        pub const fn bkinp(self) -> vals::Bkinp {
            let val = ((self.bits >> 0x9) & 0x1) as _;
            unsafe { vals::Bkinp::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "TIM_BRK_CMPx (x=1-2) input polarity"]
        pub const fn set_bkcmpp(mut self, idx: usize, val: vals::Bkinp) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0xa + idx * 0x1));
            self.bits |= (val.to_bits() as u32 & 0x1) << (0xa + idx * 0x1);
            self
        }
        #[inline(always)]
        #[doc = "TIM_BRK_CMPx (x=1-2) input polarity"]
        pub const fn bkcmpp(self, idx: usize) -> vals::Bkinp {
            assert!(idx < 2);
            let val = ((self.bits >> (0xa + idx * 0x1)) & 0x1) as _;
            unsafe { vals::Bkinp::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "alternate function register 1"]
    pub struct Af1Adv {
        bits: u32,
    }
    impl Default for Af1Adv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Af1Adv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "TIMx_BKIN input enable"]
        pub const fn set_bkine(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TIMx_BKIN input enable"]
        pub const fn bkine(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TIM_BRK_CMPx (x=1-2) enable"]
        pub const fn set_bkcmpe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TIM_BRK_CMPx (x=1-2) enable"]
        pub const fn bkcmpe(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "BRK DFSDM1_BREAKx enable (x=0 if TIM15, x=1 if TIM16, x=2 if TIM17)"]
        pub const fn set_bkdf1bke(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "BRK DFSDM1_BREAKx enable (x=0 if TIM15, x=1 if TIM16, x=2 if TIM17)"]
        pub const fn bkdf1bke(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TIMx_BKIN input polarity"]
        pub const fn set_bkinp(mut self, val: vals::Bkinp) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
            self
        }
        #[inline(always)]
        #[doc = "TIMx_BKIN input polarity"]
        pub const fn bkinp(self) -> vals::Bkinp {
            let val = ((self.bits >> 0x9) & 0x1) as _;
            unsafe { vals::Bkinp::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "TIM_BRK_CMPx (x=1-2) input polarity"]
        pub const fn set_bkcmpp(mut self, idx: usize, val: vals::Bkinp) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0xa + idx * 0x1));
            self.bits |= (val.to_bits() as u32 & 0x1) << (0xa + idx * 0x1);
            self
        }
        #[inline(always)]
        #[doc = "TIM_BRK_CMPx (x=1-2) input polarity"]
        pub const fn bkcmpp(self, idx: usize) -> vals::Bkinp {
            assert!(idx < 2);
            let val = ((self.bits >> (0xa + idx * 0x1)) & 0x1) as _;
            unsafe { vals::Bkinp::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "etr_in source selection"]
        pub const fn set_etrsel(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0xe);
            self.bits |= (val as u32 & 0xf) << 0xe;
            self
        }
        #[inline(always)]
        #[doc = "etr_in source selection"]
        pub const fn etrsel(self) -> u8 {
            ((self.bits >> 0xe) & 0xf) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "alternate function register 1"]
    pub struct Af1Gp16 {
        bits: u32,
    }
    impl Default for Af1Gp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Af1Gp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "etr_in source selection"]
        pub const fn set_etrsel(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0xe);
            self.bits |= (val as u32 & 0xf) << 0xe;
            self
        }
        #[inline(always)]
        #[doc = "etr_in source selection"]
        pub const fn etrsel(self) -> u8 {
            ((self.bits >> 0xe) & 0xf) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "alternate function register 2"]
    pub struct Af2Adv {
        bits: u32,
    }
    impl Default for Af2Adv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Af2Adv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "TIMx_BKIN2 input enable"]
        pub const fn set_bk2ine(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TIMx_BKIN2 input enable"]
        pub const fn bk2ine(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TIM_BRK2_CMPx (x=1-8) enable"]
        pub const fn set_bk2cmpe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x1 + idx * 0x8));
            self.bits |= if val { 1 << (0x1 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TIM_BRK2_CMPx (x=1-8) enable"]
        pub const fn bk2cmpe(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x1 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "BRK2 DFSDM1_BREAK1 enable"]
        pub const fn set_bk2df1bk1e(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "BRK2 DFSDM1_BREAK1 enable"]
        pub const fn bk2df1bk1e(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TIMx_BK2IN input polarity"]
        pub const fn set_bk2inp(mut self, val: vals::Bkinp) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
            self
        }
        #[inline(always)]
        #[doc = "TIMx_BK2IN input polarity"]
        pub const fn bk2inp(self) -> vals::Bkinp {
            let val = ((self.bits >> 0x9) & 0x1) as _;
            unsafe { vals::Bkinp::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "TIM_BRK2_CMPx (x=1-4) input polarity"]
        pub const fn set_bk2cmpp(mut self, idx: usize, val: vals::Bkinp) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0xa + idx * 0x1));
            self.bits |= (val.to_bits() as u32 & 0x1) << (0xa + idx * 0x1);
            self
        }
        #[inline(always)]
        #[doc = "TIM_BRK2_CMPx (x=1-4) input polarity"]
        pub const fn bk2cmpp(self, idx: usize) -> vals::Bkinp {
            assert!(idx < 2);
            let val = ((self.bits >> (0xa + idx * 0x1)) & 0x1) as _;
            unsafe { vals::Bkinp::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "auto-reload register"]
    pub struct ArrCore {
        bits: u32,
    }
    impl Default for ArrCore {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl ArrCore {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Auto-reload value"]
        pub const fn set_arr(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Auto-reload value"]
        pub const fn arr(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "break and dead-time register"]
    pub struct Bdtr1chCmp {
        bits: u32,
    }
    impl Default for Bdtr1chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bdtr1chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Dead-time generator setup"]
        pub const fn set_dtg(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Dead-time generator setup"]
        pub const fn dtg(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Lock configuration"]
        pub const fn set_lock(mut self, val: vals::Lock) -> Self {
            self.bits &= !(0x3 << 0x8);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Lock configuration"]
        pub const fn lock(self) -> vals::Lock {
            let val = ((self.bits >> 0x8) & 0x3) as _;
            unsafe { vals::Lock::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Off-state selection for Idle mode"]
        pub const fn set_ossi(mut self, val: vals::Ossi) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
            self
        }
        #[inline(always)]
        #[doc = "Off-state selection for Idle mode"]
        pub const fn ossi(self) -> vals::Ossi {
            let val = ((self.bits >> 0xa) & 0x1) as _;
            unsafe { vals::Ossi::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Off-state selection for Run mode"]
        pub const fn set_ossr(mut self, val: vals::Ossr) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
            self
        }
        #[inline(always)]
        #[doc = "Off-state selection for Run mode"]
        pub const fn ossr(self) -> vals::Ossr {
            let val = ((self.bits >> 0xb) & 0x1) as _;
            unsafe { vals::Ossr::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Break x (x=1) enable"]
        pub const fn set_bke(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0xc + idx * 0xc));
            self.bits |= if val { 1 << (0xc + idx * 0xc) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1) enable"]
        pub const fn bke(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0xc + idx * 0xc)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break x (x=1) polarity"]
        pub const fn set_bkp(mut self, idx: usize, val: vals::Bkp) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0xd + idx * 0xc));
            self.bits |= (val.to_bits() as u32 & 0x1) << (0xd + idx * 0xc);
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1) polarity"]
        pub const fn bkp(self, idx: usize) -> vals::Bkp {
            assert!(idx < 1);
            let val = ((self.bits >> (0xd + idx * 0xc)) & 0x1) as _;
            unsafe { vals::Bkp::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Automatic output enable"]
        pub const fn set_aoe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Automatic output enable"]
        pub const fn aoe(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Main output enable"]
        pub const fn set_moe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= if val { 1 << 0xf } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Main output enable"]
        pub const fn moe(self) -> bool {
            ((self.bits >> 0xf) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break x (x=1) filter"]
        pub const fn set_bkf(mut self, idx: usize, val: vals::FilterValue) -> Self {
            assert!(idx < 1);
            self.bits &= !(0xf << (0x10 + idx * 0x4));
            self.bits |= (val.to_bits() as u32 & 0xf) << (0x10 + idx * 0x4);
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1) filter"]
        pub const fn bkf(self, idx: usize) -> vals::FilterValue {
            assert!(idx < 1);
            let val = ((self.bits >> (0x10 + idx * 0x4)) & 0xf) as _;
            unsafe { vals::FilterValue::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "break and dead-time register"]
    pub struct BdtrAdv {
        bits: u32,
    }
    impl Default for BdtrAdv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl BdtrAdv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Dead-time generator setup"]
        pub const fn set_dtg(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Dead-time generator setup"]
        pub const fn dtg(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Lock configuration"]
        pub const fn set_lock(mut self, val: vals::Lock) -> Self {
            self.bits &= !(0x3 << 0x8);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Lock configuration"]
        pub const fn lock(self) -> vals::Lock {
            let val = ((self.bits >> 0x8) & 0x3) as _;
            unsafe { vals::Lock::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Off-state selection for Idle mode"]
        pub const fn set_ossi(mut self, val: vals::Ossi) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
            self
        }
        #[inline(always)]
        #[doc = "Off-state selection for Idle mode"]
        pub const fn ossi(self) -> vals::Ossi {
            let val = ((self.bits >> 0xa) & 0x1) as _;
            unsafe { vals::Ossi::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Off-state selection for Run mode"]
        pub const fn set_ossr(mut self, val: vals::Ossr) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
            self
        }
        #[inline(always)]
        #[doc = "Off-state selection for Run mode"]
        pub const fn ossr(self) -> vals::Ossr {
            let val = ((self.bits >> 0xb) & 0x1) as _;
            unsafe { vals::Ossr::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Break x (x=1,2) enable"]
        pub const fn set_bke(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0xc + idx * 0xc));
            self.bits |= if val { 1 << (0xc + idx * 0xc) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1,2) enable"]
        pub const fn bke(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0xc + idx * 0xc)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break x (x=1,2) polarity"]
        pub const fn set_bkp(mut self, idx: usize, val: vals::Bkp) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0xd + idx * 0xc));
            self.bits |= (val.to_bits() as u32 & 0x1) << (0xd + idx * 0xc);
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1,2) polarity"]
        pub const fn bkp(self, idx: usize) -> vals::Bkp {
            assert!(idx < 2);
            let val = ((self.bits >> (0xd + idx * 0xc)) & 0x1) as _;
            unsafe { vals::Bkp::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Automatic output enable"]
        pub const fn set_aoe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Automatic output enable"]
        pub const fn aoe(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Main output enable"]
        pub const fn set_moe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= if val { 1 << 0xf } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Main output enable"]
        pub const fn moe(self) -> bool {
            ((self.bits >> 0xf) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break x (x=1,2) filter"]
        pub const fn set_bkf(mut self, idx: usize, val: vals::FilterValue) -> Self {
            assert!(idx < 2);
            self.bits &= !(0xf << (0x10 + idx * 0x4));
            self.bits |= (val.to_bits() as u32 & 0xf) << (0x10 + idx * 0x4);
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1,2) filter"]
        pub const fn bkf(self, idx: usize) -> vals::FilterValue {
            assert!(idx < 2);
            let val = ((self.bits >> (0x10 + idx * 0x4)) & 0xf) as _;
            unsafe { vals::FilterValue::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare enable register"]
    pub struct Ccer1ch {
        bits: u32,
    }
    impl Default for Ccer1ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ccer1ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output enable"]
        pub const fn set_cce(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x0 + idx * 0x4));
            self.bits |= if val { 1 << (0x0 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output enable"]
        pub const fn cce(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x0 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        pub const fn set_ccp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x1 + idx * 0x4));
            self.bits |= if val { 1 << (0x1 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        pub const fn ccp(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x1 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        pub const fn set_ccnp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x3 + idx * 0x4));
            self.bits |= if val { 1 << (0x3 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        pub const fn ccnp(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x3 + idx * 0x4)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare enable register"]
    pub struct Ccer1chCmp {
        bits: u32,
    }
    impl Default for Ccer1chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ccer1chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output enable"]
        pub const fn set_cce(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x0 + idx * 0x4));
            self.bits |= if val { 1 << (0x0 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output enable"]
        pub const fn cce(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x0 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        pub const fn set_ccp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x1 + idx * 0x4));
            self.bits |= if val { 1 << (0x1 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        pub const fn ccp(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x1 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        pub const fn set_ccne(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x2 + idx * 0x4));
            self.bits |= if val { 1 << (0x2 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        pub const fn ccne(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x2 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        pub const fn set_ccnp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x3 + idx * 0x4));
            self.bits |= if val { 1 << (0x3 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        pub const fn ccnp(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x3 + idx * 0x4)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare enable register"]
    pub struct Ccer2ch {
        bits: u32,
    }
    impl Default for Ccer2ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ccer2ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        pub const fn set_cce(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x0 + idx * 0x4));
            self.bits |= if val { 1 << (0x0 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        pub const fn cce(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x0 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        pub const fn set_ccp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x1 + idx * 0x4));
            self.bits |= if val { 1 << (0x1 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        pub const fn ccp(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x1 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        pub const fn set_ccnp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x3 + idx * 0x4));
            self.bits |= if val { 1 << (0x3 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        pub const fn ccnp(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x3 + idx * 0x4)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare enable register"]
    pub struct Ccer2chCmp {
        bits: u32,
    }
    impl Default for Ccer2chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ccer2chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        pub const fn set_cce(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x0 + idx * 0x4));
            self.bits |= if val { 1 << (0x0 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        pub const fn cce(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x0 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        pub const fn set_ccp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x1 + idx * 0x4));
            self.bits |= if val { 1 << (0x1 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        pub const fn ccp(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x1 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        pub const fn set_ccne(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x2 + idx * 0x4));
            self.bits |= if val { 1 << (0x2 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        pub const fn ccne(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x2 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        pub const fn set_ccnp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x3 + idx * 0x4));
            self.bits |= if val { 1 << (0x3 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        pub const fn ccnp(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x3 + idx * 0x4)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare enable register"]
    pub struct CcerAdv {
        bits: u32,
    }
    impl Default for CcerAdv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl CcerAdv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-6) output enable"]
        pub const fn set_cce(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 6);
            self.bits &= !(0x1 << (0x0 + idx * 0x4));
            self.bits |= if val { 1 << (0x0 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-6) output enable"]
        pub const fn cce(self, idx: usize) -> bool {
            assert!(idx < 6);
            ((self.bits >> (0x0 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-6) output Polarity"]
        pub const fn set_ccp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 6);
            self.bits &= !(0x1 << (0x1 + idx * 0x4));
            self.bits |= if val { 1 << (0x1 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-6) output Polarity"]
        pub const fn ccp(self, idx: usize) -> bool {
            assert!(idx < 6);
            ((self.bits >> (0x1 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-3) complementary output enable"]
        pub const fn set_ccne(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 3);
            self.bits &= !(0x1 << (0x2 + idx * 0x4));
            self.bits |= if val { 1 << (0x2 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-3) complementary output enable"]
        pub const fn ccne(self, idx: usize) -> bool {
            assert!(idx < 3);
            ((self.bits >> (0x2 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        pub const fn set_ccnp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x3 + idx * 0x4));
            self.bits |= if val { 1 << (0x3 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        pub const fn ccnp(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x3 + idx * 0x4)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare enable register"]
    pub struct CcerGp16 {
        bits: u32,
    }
    impl Default for CcerGp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl CcerGp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) output enable"]
        pub const fn set_cce(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x0 + idx * 0x4));
            self.bits |= if val { 1 << (0x0 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) output enable"]
        pub const fn cce(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x0 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        pub const fn set_ccp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x1 + idx * 0x4));
            self.bits |= if val { 1 << (0x1 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        pub const fn ccp(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x1 + idx * 0x4)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        pub const fn set_ccnp(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x3 + idx * 0x4));
            self.bits |= if val { 1 << (0x3 + idx * 0x4) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        pub const fn ccnp(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x3 + idx * 0x4)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare mode register 3"]
    pub struct Ccmr3Adv {
        bits: u32,
    }
    impl Default for Ccmr3Adv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ccmr3Adv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Output compare x (x=5,6) fast enable"]
        pub const fn set_ocfe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x2 + idx * 0x8));
            self.bits |= if val { 1 << (0x2 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output compare x (x=5,6) fast enable"]
        pub const fn ocfe(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x2 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output compare x (x=5,6) preload enable"]
        pub const fn set_ocpe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x3 + idx * 0x8));
            self.bits |= if val { 1 << (0x3 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output compare x (x=5,6) preload enable"]
        pub const fn ocpe(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x3 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output compare x (x=5,6) mode"]
        pub const fn set_ocm(mut self, idx: usize, val: vals::Ocm) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x7 << (0x4 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0x7) << (0x4 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Output compare x (x=5,6) mode"]
        pub const fn ocm(self, idx: usize) -> vals::Ocm {
            assert!(idx < 2);
            let val = ((self.bits >> (0x4 + idx * 0x8)) & 0x7) as _;
            unsafe { vals::Ocm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Output compare x (x=5,6) clear enable"]
        pub const fn set_occe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x7 + idx * 0x8));
            self.bits |= if val { 1 << (0x7 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output compare x (x=5,6) clear enable"]
        pub const fn occe(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x7 + idx * 0x8)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare mode register x (x=1) (input mode)"]
    pub struct CcmrInput1ch {
        bits: u32,
    }
    impl Default for CcmrInput1ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl CcmrInput1ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare y selection"]
        pub const fn set_ccs(mut self, idx: usize, val: vals::CcmrInputCcs) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x3 << (0x0 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare y selection"]
        pub const fn ccs(self, idx: usize) -> vals::CcmrInputCcs {
            assert!(idx < 1);
            let val = ((self.bits >> (0x0 + idx * 0x8)) & 0x3) as _;
            unsafe { vals::CcmrInputCcs::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Input capture y prescaler"]
        pub const fn set_icpsc(mut self, idx: usize, val: u8) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x3 << (0x2 + idx * 0x8));
            self.bits |= (val as u32 & 0x3) << (0x2 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Input capture y prescaler"]
        pub const fn icpsc(self, idx: usize) -> u8 {
            assert!(idx < 1);
            ((self.bits >> (0x2 + idx * 0x8)) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Input capture y filter"]
        pub const fn set_icf(mut self, idx: usize, val: vals::FilterValue) -> Self {
            assert!(idx < 1);
            self.bits &= !(0xf << (0x4 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0xf) << (0x4 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Input capture y filter"]
        pub const fn icf(self, idx: usize) -> vals::FilterValue {
            assert!(idx < 1);
            let val = ((self.bits >> (0x4 + idx * 0x8)) & 0xf) as _;
            unsafe { vals::FilterValue::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare mode register x (x=1) (input mode)"]
    pub struct CcmrInput2ch {
        bits: u32,
    }
    impl Default for CcmrInput2ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl CcmrInput2ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare y selection"]
        pub const fn set_ccs(mut self, idx: usize, val: vals::CcmrInputCcs) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x3 << (0x0 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare y selection"]
        pub const fn ccs(self, idx: usize) -> vals::CcmrInputCcs {
            assert!(idx < 2);
            let val = ((self.bits >> (0x0 + idx * 0x8)) & 0x3) as _;
            unsafe { vals::CcmrInputCcs::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Input capture y prescaler"]
        pub const fn set_icpsc(mut self, idx: usize, val: u8) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x3 << (0x2 + idx * 0x8));
            self.bits |= (val as u32 & 0x3) << (0x2 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Input capture y prescaler"]
        pub const fn icpsc(self, idx: usize) -> u8 {
            assert!(idx < 2);
            ((self.bits >> (0x2 + idx * 0x8)) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Input capture y filter"]
        pub const fn set_icf(mut self, idx: usize, val: vals::FilterValue) -> Self {
            assert!(idx < 2);
            self.bits &= !(0xf << (0x4 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0xf) << (0x4 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Input capture y filter"]
        pub const fn icf(self, idx: usize) -> vals::FilterValue {
            assert!(idx < 2);
            let val = ((self.bits >> (0x4 + idx * 0x8)) & 0xf) as _;
            unsafe { vals::FilterValue::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare mode register x (x=1) (output mode)"]
    pub struct CcmrOutput1ch {
        bits: u32,
    }
    impl Default for CcmrOutput1ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl CcmrOutput1ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare y selection"]
        pub const fn set_ccs(mut self, idx: usize, val: vals::CcmrOutputCcs) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x3 << (0x0 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare y selection"]
        pub const fn ccs(self, idx: usize) -> vals::CcmrOutputCcs {
            assert!(idx < 1);
            let val = ((self.bits >> (0x0 + idx * 0x8)) & 0x3) as _;
            unsafe { vals::CcmrOutputCcs::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Output compare y fast enable"]
        pub const fn set_ocfe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x2 + idx * 0x8));
            self.bits |= if val { 1 << (0x2 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output compare y fast enable"]
        pub const fn ocfe(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x2 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output compare y preload enable"]
        pub const fn set_ocpe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x3 + idx * 0x8));
            self.bits |= if val { 1 << (0x3 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output compare y preload enable"]
        pub const fn ocpe(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x3 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output compare y mode"]
        pub const fn set_ocm(mut self, idx: usize, val: vals::Ocm) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x7 << (0x4 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0x7) << (0x4 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Output compare y mode"]
        pub const fn ocm(self, idx: usize) -> vals::Ocm {
            assert!(idx < 1);
            let val = ((self.bits >> (0x4 + idx * 0x8)) & 0x7) as _;
            unsafe { vals::Ocm::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare mode register x (x=1) (output mode)"]
    pub struct CcmrOutput2ch {
        bits: u32,
    }
    impl Default for CcmrOutput2ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl CcmrOutput2ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare y selection"]
        pub const fn set_ccs(mut self, idx: usize, val: vals::CcmrOutputCcs) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x3 << (0x0 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare y selection"]
        pub const fn ccs(self, idx: usize) -> vals::CcmrOutputCcs {
            assert!(idx < 2);
            let val = ((self.bits >> (0x0 + idx * 0x8)) & 0x3) as _;
            unsafe { vals::CcmrOutputCcs::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Output compare y fast enable"]
        pub const fn set_ocfe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x2 + idx * 0x8));
            self.bits |= if val { 1 << (0x2 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output compare y fast enable"]
        pub const fn ocfe(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x2 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output compare y preload enable"]
        pub const fn set_ocpe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x3 + idx * 0x8));
            self.bits |= if val { 1 << (0x3 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output compare y preload enable"]
        pub const fn ocpe(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x3 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output compare y mode"]
        pub const fn set_ocm(mut self, idx: usize, val: vals::Ocm) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x7 << (0x4 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0x7) << (0x4 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Output compare y mode"]
        pub const fn ocm(self, idx: usize) -> vals::Ocm {
            assert!(idx < 2);
            let val = ((self.bits >> (0x4 + idx * 0x8)) & 0x7) as _;
            unsafe { vals::Ocm::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare mode register x (x=1-2) (output mode)"]
    pub struct CcmrOutputGp16 {
        bits: u32,
    }
    impl Default for CcmrOutputGp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl CcmrOutputGp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/Compare y selection"]
        pub const fn set_ccs(mut self, idx: usize, val: vals::CcmrOutputCcs) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x3 << (0x0 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare y selection"]
        pub const fn ccs(self, idx: usize) -> vals::CcmrOutputCcs {
            assert!(idx < 2);
            let val = ((self.bits >> (0x0 + idx * 0x8)) & 0x3) as _;
            unsafe { vals::CcmrOutputCcs::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Output compare y fast enable"]
        pub const fn set_ocfe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x2 + idx * 0x8));
            self.bits |= if val { 1 << (0x2 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output compare y fast enable"]
        pub const fn ocfe(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x2 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output compare y preload enable"]
        pub const fn set_ocpe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x3 + idx * 0x8));
            self.bits |= if val { 1 << (0x3 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output compare y preload enable"]
        pub const fn ocpe(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x3 + idx * 0x8)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output compare y mode"]
        pub const fn set_ocm(mut self, idx: usize, val: vals::Ocm) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x7 << (0x4 + idx * 0x8));
            self.bits |= (val.to_bits() as u32 & 0x7) << (0x4 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Output compare y mode"]
        pub const fn ocm(self, idx: usize) -> vals::Ocm {
            assert!(idx < 2);
            let val = ((self.bits >> (0x4 + idx * 0x8)) & 0x7) as _;
            unsafe { vals::Ocm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Output compare y clear enable"]
        pub const fn set_occe(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x7 + idx * 0x8));
            self.bits |= if val { 1 << (0x7 + idx * 0x8) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output compare y clear enable"]
        pub const fn occe(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x7 + idx * 0x8)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare register 5"]
    pub struct Ccr5Adv {
        bits: u32,
    }
    impl Default for Ccr5Adv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ccr5Adv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "capture/compare x (x=1-4,6) value"]
        pub const fn set_ccr(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "capture/compare x (x=1-4,6) value"]
        pub const fn ccr(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
        #[inline(always)]
        #[doc = "Group channel 5 and channel x (x=1-3)"]
        pub const fn set_gc5c(mut self, idx: usize, val: vals::Gc5c) -> Self {
            assert!(idx < 3);
            self.bits &= !(0x1 << (0x1d + idx * 0x1));
            self.bits |= (val.to_bits() as u32 & 0x1) << (0x1d + idx * 0x1);
            self
        }
        #[inline(always)]
        #[doc = "Group channel 5 and channel x (x=1-3)"]
        pub const fn gc5c(self, idx: usize) -> vals::Gc5c {
            assert!(idx < 3);
            let val = ((self.bits >> (0x1d + idx * 0x1)) & 0x1) as _;
            unsafe { vals::Gc5c::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "capture/compare register x (x=1-4,6)"]
    pub struct Ccr1ch {
        bits: u32,
    }
    impl Default for Ccr1ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ccr1ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "capture/compare x (x=1-4,6) value"]
        pub const fn set_ccr(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "capture/compare x (x=1-4,6) value"]
        pub const fn ccr(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "counter"]
    pub struct CntCore {
        bits: u32,
    }
    impl Default for CntCore {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl CntCore {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "counter value"]
        pub const fn set_cnt(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "counter value"]
        pub const fn cnt(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
        #[inline(always)]
        #[doc = "UIF copy"]
        pub const fn set_uifcpy(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1f);
            self.bits |= if val { 1 << 0x1f } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "UIF copy"]
        pub const fn uifcpy(self) -> bool {
            ((self.bits >> 0x1f) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 1"]
    pub struct Cr11ch {
        bits: u32,
    }
    impl Default for Cr11ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr11ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Counter enable"]
        pub const fn set_cen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Counter enable"]
        pub const fn cen(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update disable"]
        pub const fn set_udis(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update disable"]
        pub const fn udis(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update request source"]
        pub const fn set_urs(mut self, val: vals::Urs) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Update request source"]
        pub const fn urs(self) -> vals::Urs {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Urs::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "One-pulse mode enbaled"]
        pub const fn set_opm(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "One-pulse mode enbaled"]
        pub const fn opm(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Auto-reload preload enable"]
        pub const fn set_arpe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Auto-reload preload enable"]
        pub const fn arpe(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Clock division"]
        pub const fn set_ckd(mut self, val: vals::Ckd) -> Self {
            self.bits &= !(0x3 << 0x8);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Clock division"]
        pub const fn ckd(self) -> vals::Ckd {
            let val = ((self.bits >> 0x8) & 0x3) as _;
            unsafe { vals::Ckd::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "UIF status bit remapping enable"]
        pub const fn set_uifremap(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "UIF status bit remapping enable"]
        pub const fn uifremap(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 1"]
    pub struct Cr1Core {
        bits: u32,
    }
    impl Default for Cr1Core {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr1Core {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Counter enable"]
        pub const fn set_cen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Counter enable"]
        pub const fn cen(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update disable"]
        pub const fn set_udis(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update disable"]
        pub const fn udis(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update request source"]
        pub const fn set_urs(mut self, val: vals::Urs) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Update request source"]
        pub const fn urs(self) -> vals::Urs {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Urs::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "One-pulse mode enbaled"]
        pub const fn set_opm(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "One-pulse mode enbaled"]
        pub const fn opm(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Auto-reload preload enable"]
        pub const fn set_arpe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Auto-reload preload enable"]
        pub const fn arpe(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "UIF status bit remapping enable"]
        pub const fn set_uifremap(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "UIF status bit remapping enable"]
        pub const fn uifremap(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 1"]
    pub struct Cr1Gp16 {
        bits: u32,
    }
    impl Default for Cr1Gp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr1Gp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Counter enable"]
        pub const fn set_cen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Counter enable"]
        pub const fn cen(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update disable"]
        pub const fn set_udis(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update disable"]
        pub const fn udis(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update request source"]
        pub const fn set_urs(mut self, val: vals::Urs) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Update request source"]
        pub const fn urs(self) -> vals::Urs {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Urs::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "One-pulse mode enbaled"]
        pub const fn set_opm(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "One-pulse mode enbaled"]
        pub const fn opm(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Direction"]
        pub const fn set_dir(mut self, val: vals::Dir) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Direction"]
        pub const fn dir(self) -> vals::Dir {
            let val = ((self.bits >> 0x4) & 0x1) as _;
            unsafe { vals::Dir::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Center-aligned mode selection"]
        pub const fn set_cms(mut self, val: vals::Cms) -> Self {
            self.bits &= !(0x3 << 0x5);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x5;
            self
        }
        #[inline(always)]
        #[doc = "Center-aligned mode selection"]
        pub const fn cms(self) -> vals::Cms {
            let val = ((self.bits >> 0x5) & 0x3) as _;
            unsafe { vals::Cms::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Auto-reload preload enable"]
        pub const fn set_arpe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Auto-reload preload enable"]
        pub const fn arpe(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Clock division"]
        pub const fn set_ckd(mut self, val: vals::Ckd) -> Self {
            self.bits &= !(0x3 << 0x8);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Clock division"]
        pub const fn ckd(self) -> vals::Ckd {
            let val = ((self.bits >> 0x8) & 0x3) as _;
            unsafe { vals::Ckd::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "UIF status bit remapping enable"]
        pub const fn set_uifremap(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "UIF status bit remapping enable"]
        pub const fn uifremap(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 2"]
    pub struct Cr21chCmp {
        bits: u32,
    }
    impl Default for Cr21chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr21chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/compare preloaded control"]
        pub const fn set_ccpc(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare preloaded control"]
        pub const fn ccpc(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare control update selection"]
        pub const fn set_ccus(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare control update selection"]
        pub const fn ccus(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare DMA selection"]
        pub const fn set_ccds(mut self, val: vals::Ccds) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare DMA selection"]
        pub const fn ccds(self) -> vals::Ccds {
            let val = ((self.bits >> 0x3) & 0x1) as _;
            unsafe { vals::Ccds::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Output Idle state x (x=1)"]
        pub const fn set_ois(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x8 + idx * 0x2));
            self.bits |= if val { 1 << (0x8 + idx * 0x2) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output Idle state x (x=1)"]
        pub const fn ois(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x8 + idx * 0x2)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output Idle state x (x=1)"]
        pub const fn set_oisn(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x9 + idx * 0x2));
            self.bits |= if val { 1 << (0x9 + idx * 0x2) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output Idle state x (x=1)"]
        pub const fn oisn(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x9 + idx * 0x2)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 2"]
    pub struct Cr22ch {
        bits: u32,
    }
    impl Default for Cr22ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr22ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Master mode selection"]
        pub const fn set_mms(mut self, val: vals::Mms) -> Self {
            self.bits &= !(0x7 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Master mode selection"]
        pub const fn mms(self) -> vals::Mms {
            let val = ((self.bits >> 0x4) & 0x7) as _;
            unsafe { vals::Mms::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "TI1 selection"]
        pub const fn set_ti1s(mut self, val: vals::Ti1s) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "TI1 selection"]
        pub const fn ti1s(self) -> vals::Ti1s {
            let val = ((self.bits >> 0x7) & 0x1) as _;
            unsafe { vals::Ti1s::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 2"]
    pub struct Cr22chCmp {
        bits: u32,
    }
    impl Default for Cr22chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr22chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/compare preloaded control"]
        pub const fn set_ccpc(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare preloaded control"]
        pub const fn ccpc(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare control update selection"]
        pub const fn set_ccus(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare control update selection"]
        pub const fn ccus(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare DMA selection"]
        pub const fn set_ccds(mut self, val: vals::Ccds) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare DMA selection"]
        pub const fn ccds(self) -> vals::Ccds {
            let val = ((self.bits >> 0x3) & 0x1) as _;
            unsafe { vals::Ccds::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Master mode selection"]
        pub const fn set_mms(mut self, val: vals::Mms) -> Self {
            self.bits &= !(0x7 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Master mode selection"]
        pub const fn mms(self) -> vals::Mms {
            let val = ((self.bits >> 0x4) & 0x7) as _;
            unsafe { vals::Mms::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "TI1 selection"]
        pub const fn set_ti1s(mut self, val: vals::Ti1s) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "TI1 selection"]
        pub const fn ti1s(self) -> vals::Ti1s {
            let val = ((self.bits >> 0x7) & 0x1) as _;
            unsafe { vals::Ti1s::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Output Idle state x (x=1,2)"]
        pub const fn set_ois(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x8 + idx * 0x2));
            self.bits |= if val { 1 << (0x8 + idx * 0x2) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output Idle state x (x=1,2)"]
        pub const fn ois(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x8 + idx * 0x2)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output Idle state x (x=1)"]
        pub const fn set_oisn(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x9 + idx * 0x2));
            self.bits |= if val { 1 << (0x9 + idx * 0x2) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output Idle state x (x=1)"]
        pub const fn oisn(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x9 + idx * 0x2)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 2"]
    pub struct Cr2Adv {
        bits: u32,
    }
    impl Default for Cr2Adv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr2Adv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/compare preloaded control"]
        pub const fn set_ccpc(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare preloaded control"]
        pub const fn ccpc(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare control update selection"]
        pub const fn set_ccus(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare control update selection"]
        pub const fn ccus(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare DMA selection"]
        pub const fn set_ccds(mut self, val: vals::Ccds) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare DMA selection"]
        pub const fn ccds(self) -> vals::Ccds {
            let val = ((self.bits >> 0x3) & 0x1) as _;
            unsafe { vals::Ccds::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Master mode selection"]
        pub const fn set_mms(mut self, val: vals::Mms) -> Self {
            self.bits &= !(0x7 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Master mode selection"]
        pub const fn mms(self) -> vals::Mms {
            let val = ((self.bits >> 0x4) & 0x7) as _;
            unsafe { vals::Mms::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "TI1 selection"]
        pub const fn set_ti1s(mut self, val: vals::Ti1s) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "TI1 selection"]
        pub const fn ti1s(self) -> vals::Ti1s {
            let val = ((self.bits >> 0x7) & 0x1) as _;
            unsafe { vals::Ti1s::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Output Idle state x (x=1-6)"]
        pub const fn set_ois(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 6);
            self.bits &= !(0x1 << (0x8 + idx * 0x2));
            self.bits |= if val { 1 << (0x8 + idx * 0x2) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output Idle state x (x=1-6)"]
        pub const fn ois(self, idx: usize) -> bool {
            assert!(idx < 6);
            ((self.bits >> (0x8 + idx * 0x2)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output Idle state x N x (x=1-4)"]
        pub const fn set_oisn(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x9 + idx * 0x2));
            self.bits |= if val { 1 << (0x9 + idx * 0x2) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output Idle state x N x (x=1-4)"]
        pub const fn oisn(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x9 + idx * 0x2)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Master mode selection 2"]
        pub const fn set_mms2(mut self, val: vals::Mms2) -> Self {
            self.bits &= !(0xf << 0x14);
            self.bits |= (val.to_bits() as u32 & 0xf) << 0x14;
            self
        }
        #[inline(always)]
        #[doc = "Master mode selection 2"]
        pub const fn mms2(self) -> vals::Mms2 {
            let val = ((self.bits >> 0x14) & 0xf) as _;
            unsafe { vals::Mms2::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 2"]
    pub struct Cr2Basic {
        bits: u32,
    }
    impl Default for Cr2Basic {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr2Basic {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Master mode selection"]
        pub const fn set_mms(mut self, val: vals::Mms) -> Self {
            self.bits &= !(0x7 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Master mode selection"]
        pub const fn mms(self) -> vals::Mms {
            let val = ((self.bits >> 0x4) & 0x7) as _;
            unsafe { vals::Mms::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register 2"]
    pub struct Cr2Gp16 {
        bits: u32,
    }
    impl Default for Cr2Gp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr2Gp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Capture/compare DMA selection"]
        pub const fn set_ccds(mut self, val: vals::Ccds) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare DMA selection"]
        pub const fn ccds(self) -> vals::Ccds {
            let val = ((self.bits >> 0x3) & 0x1) as _;
            unsafe { vals::Ccds::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Master mode selection"]
        pub const fn set_mms(mut self, val: vals::Mms) -> Self {
            self.bits &= !(0x7 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Master mode selection"]
        pub const fn mms(self) -> vals::Mms {
            let val = ((self.bits >> 0x4) & 0x7) as _;
            unsafe { vals::Mms::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "TI1 selection"]
        pub const fn set_ti1s(mut self, val: vals::Ti1s) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "TI1 selection"]
        pub const fn ti1s(self) -> vals::Ti1s {
            let val = ((self.bits >> 0x7) & 0x1) as _;
            unsafe { vals::Ti1s::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA control register"]
    pub struct Dcr1chCmp {
        bits: u32,
    }
    impl Default for Dcr1chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dcr1chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DMA base address"]
        pub const fn set_dba(mut self, val: u8) -> Self {
            self.bits &= !(0x1f << 0x0);
            self.bits |= (val as u32 & 0x1f) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DMA base address"]
        pub const fn dba(self) -> u8 {
            ((self.bits >> 0x0) & 0x1f) as _
        }
        #[inline(always)]
        #[doc = "DMA burst length"]
        pub const fn set_dbl(mut self, val: u8) -> Self {
            self.bits &= !(0x1f << 0x8);
            self.bits |= (val as u32 & 0x1f) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "DMA burst length"]
        pub const fn dbl(self) -> u8 {
            ((self.bits >> 0x8) & 0x1f) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA/Interrupt enable register"]
    pub struct Dier1ch {
        bits: u32,
    }
    impl Default for Dier1ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dier1ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn set_uie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn uie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        pub const fn set_ccie(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        pub const fn ccie(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA/Interrupt enable register"]
    pub struct Dier1chCmp {
        bits: u32,
    }
    impl Default for Dier1chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dier1chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn set_uie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn uie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        pub const fn set_ccie(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        pub const fn ccie(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "COM interrupt enable"]
        pub const fn set_comie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "COM interrupt enable"]
        pub const fn comie(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break interrupt enable"]
        pub const fn set_bie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break interrupt enable"]
        pub const fn bie(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update DMA request enable"]
        pub const fn set_ude(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update DMA request enable"]
        pub const fn ude(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        pub const fn set_ccde(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x9 + idx * 0x1));
            self.bits |= if val { 1 << (0x9 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        pub const fn ccde(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x9 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA/Interrupt enable register"]
    pub struct Dier2ch {
        bits: u32,
    }
    impl Default for Dier2ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dier2ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn set_uie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn uie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) interrupt enable"]
        pub const fn set_ccie(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) interrupt enable"]
        pub const fn ccie(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger interrupt enable"]
        pub const fn set_tie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger interrupt enable"]
        pub const fn tie(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA/Interrupt enable register"]
    pub struct Dier2chCmp {
        bits: u32,
    }
    impl Default for Dier2chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dier2chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn set_uie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn uie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        pub const fn set_ccie(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        pub const fn ccie(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "COM interrupt enable"]
        pub const fn set_comie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "COM interrupt enable"]
        pub const fn comie(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger interrupt enable"]
        pub const fn set_tie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger interrupt enable"]
        pub const fn tie(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break interrupt enable"]
        pub const fn set_bie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break interrupt enable"]
        pub const fn bie(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update DMA request enable"]
        pub const fn set_ude(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update DMA request enable"]
        pub const fn ude(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        pub const fn set_ccde(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x9 + idx * 0x1));
            self.bits |= if val { 1 << (0x9 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        pub const fn ccde(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x9 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "COM DMA request enable"]
        pub const fn set_comde(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= if val { 1 << 0xd } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "COM DMA request enable"]
        pub const fn comde(self) -> bool {
            ((self.bits >> 0xd) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger DMA request enable"]
        pub const fn set_tde(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger DMA request enable"]
        pub const fn tde(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA/Interrupt enable register"]
    pub struct DierAdv {
        bits: u32,
    }
    impl Default for DierAdv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl DierAdv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn set_uie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn uie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        pub const fn set_ccie(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        pub const fn ccie(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "COM interrupt enable"]
        pub const fn set_comie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "COM interrupt enable"]
        pub const fn comie(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger interrupt enable"]
        pub const fn set_tie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger interrupt enable"]
        pub const fn tie(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break interrupt enable"]
        pub const fn set_bie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break interrupt enable"]
        pub const fn bie(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update DMA request enable"]
        pub const fn set_ude(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update DMA request enable"]
        pub const fn ude(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        pub const fn set_ccde(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x9 + idx * 0x1));
            self.bits |= if val { 1 << (0x9 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        pub const fn ccde(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x9 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "COM DMA request enable"]
        pub const fn set_comde(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= if val { 1 << 0xd } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "COM DMA request enable"]
        pub const fn comde(self) -> bool {
            ((self.bits >> 0xd) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger DMA request enable"]
        pub const fn set_tde(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger DMA request enable"]
        pub const fn tde(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA/Interrupt enable register"]
    pub struct DierBasicNoCr2 {
        bits: u32,
    }
    impl Default for DierBasicNoCr2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl DierBasicNoCr2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn set_uie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn uie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update DMA request enable"]
        pub const fn set_ude(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update DMA request enable"]
        pub const fn ude(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA/Interrupt enable register"]
    pub struct DierCore {
        bits: u32,
    }
    impl Default for DierCore {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl DierCore {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn set_uie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn uie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA/Interrupt enable register"]
    pub struct DierGp16 {
        bits: u32,
    }
    impl Default for DierGp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl DierGp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn set_uie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt enable"]
        pub const fn uie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        pub const fn set_ccie(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        pub const fn ccie(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger interrupt enable"]
        pub const fn set_tie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger interrupt enable"]
        pub const fn tie(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Update DMA request enable"]
        pub const fn set_ude(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update DMA request enable"]
        pub const fn ude(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        pub const fn set_ccde(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x9 + idx * 0x1));
            self.bits |= if val { 1 << (0x9 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        pub const fn ccde(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x9 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger DMA request enable"]
        pub const fn set_tde(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger DMA request enable"]
        pub const fn tde(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "DMA address for full transfer"]
    pub struct DmarGp16 {
        bits: u32,
    }
    impl Default for DmarGp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl DmarGp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DMA register for burst accesses"]
        pub const fn set_dmab(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DMA register for burst accesses"]
        pub const fn dmab(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "event generation register"]
    pub struct Egr1ch {
        bits: u32,
    }
    impl Default for Egr1ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Egr1ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn set_ug(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn ug(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1) generation"]
        pub const fn set_ccg(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1) generation"]
        pub const fn ccg(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "event generation register"]
    pub struct Egr1chCmp {
        bits: u32,
    }
    impl Default for Egr1chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Egr1chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn set_ug(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn ug(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1) generation"]
        pub const fn set_ccg(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1) generation"]
        pub const fn ccg(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare control update generation"]
        pub const fn set_comg(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare control update generation"]
        pub const fn comg(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break x (x=1) generation"]
        pub const fn set_bg(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x7 + idx * 0x1));
            self.bits |= if val { 1 << (0x7 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1) generation"]
        pub const fn bg(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x7 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "event generation register"]
    pub struct Egr2ch {
        bits: u32,
    }
    impl Default for Egr2ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Egr2ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn set_ug(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn ug(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-2) generation"]
        pub const fn set_ccg(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-2) generation"]
        pub const fn ccg(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger generation"]
        pub const fn set_tg(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger generation"]
        pub const fn tg(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "event generation register"]
    pub struct Egr2chCmp {
        bits: u32,
    }
    impl Default for Egr2chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Egr2chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn set_ug(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn ug(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1,2) generation"]
        pub const fn set_ccg(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1,2) generation"]
        pub const fn ccg(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare control update generation"]
        pub const fn set_comg(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare control update generation"]
        pub const fn comg(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger generation"]
        pub const fn set_tg(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger generation"]
        pub const fn tg(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break x (x=1) generation"]
        pub const fn set_bg(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x7 + idx * 0x1));
            self.bits |= if val { 1 << (0x7 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1) generation"]
        pub const fn bg(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x7 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "event generation register"]
    pub struct EgrAdv {
        bits: u32,
    }
    impl Default for EgrAdv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl EgrAdv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn set_ug(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn ug(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-4) generation"]
        pub const fn set_ccg(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-4) generation"]
        pub const fn ccg(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare control update generation"]
        pub const fn set_comg(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare control update generation"]
        pub const fn comg(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger generation"]
        pub const fn set_tg(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger generation"]
        pub const fn tg(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break x (x=1-2) generation"]
        pub const fn set_bg(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x7 + idx * 0x1));
            self.bits |= if val { 1 << (0x7 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1-2) generation"]
        pub const fn bg(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x7 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "event generation register"]
    pub struct EgrCore {
        bits: u32,
    }
    impl Default for EgrCore {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl EgrCore {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn set_ug(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn ug(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "event generation register"]
    pub struct EgrGp16 {
        bits: u32,
    }
    impl Default for EgrGp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl EgrGp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn set_ug(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update generation"]
        pub const fn ug(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-4) generation"]
        pub const fn set_ccg(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-4) generation"]
        pub const fn ccg(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger generation"]
        pub const fn set_tg(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger generation"]
        pub const fn tg(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "repetition counter register"]
    pub struct Rcr1chCmp {
        bits: u32,
    }
    impl Default for Rcr1chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Rcr1chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Repetition counter value"]
        pub const fn set_rep(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Repetition counter value"]
        pub const fn rep(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "repetition counter register"]
    pub struct RcrAdv {
        bits: u32,
    }
    impl Default for RcrAdv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl RcrAdv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Repetition counter value"]
        pub const fn set_rep(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Repetition counter value"]
        pub const fn rep(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "slave mode control register"]
    pub struct Smcr2ch {
        bits: u32,
    }
    impl Default for Smcr2ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Smcr2ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Master/Slave mode"]
        pub const fn set_msm(mut self, val: vals::Msm) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "Master/Slave mode"]
        pub const fn msm(self) -> vals::Msm {
            let val = ((self.bits >> 0x7) & 0x1) as _;
            unsafe { vals::Msm::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "slave mode control register"]
    pub struct SmcrGp16 {
        bits: u32,
    }
    impl Default for SmcrGp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl SmcrGp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Master/Slave mode"]
        pub const fn set_msm(mut self, val: vals::Msm) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
            self
        }
        #[inline(always)]
        #[doc = "Master/Slave mode"]
        pub const fn msm(self) -> vals::Msm {
            let val = ((self.bits >> 0x7) & 0x1) as _;
            unsafe { vals::Msm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "External trigger filter"]
        pub const fn set_etf(mut self, val: vals::FilterValue) -> Self {
            self.bits &= !(0xf << 0x8);
            self.bits |= (val.to_bits() as u32 & 0xf) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "External trigger filter"]
        pub const fn etf(self) -> vals::FilterValue {
            let val = ((self.bits >> 0x8) & 0xf) as _;
            unsafe { vals::FilterValue::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "External trigger prescaler"]
        pub const fn set_etps(mut self, val: vals::Etps) -> Self {
            self.bits &= !(0x3 << 0xc);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0xc;
            self
        }
        #[inline(always)]
        #[doc = "External trigger prescaler"]
        pub const fn etps(self) -> vals::Etps {
            let val = ((self.bits >> 0xc) & 0x3) as _;
            unsafe { vals::Etps::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "External clock mode 2 enable"]
        pub const fn set_ece(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "External clock mode 2 enable"]
        pub const fn ece(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "External trigger polarity"]
        pub const fn set_etp(mut self, val: vals::Etp) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
            self
        }
        #[inline(always)]
        #[doc = "External trigger polarity"]
        pub const fn etp(self) -> vals::Etp {
            let val = ((self.bits >> 0xf) & 0x1) as _;
            unsafe { vals::Etp::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "status register"]
    pub struct Sr1ch {
        bits: u32,
    }
    impl Default for Sr1ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sr1ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn set_uif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn uif(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        pub const fn set_ccif(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        pub const fn ccif(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        pub const fn set_ccof(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x9 + idx * 0x1));
            self.bits |= if val { 1 << (0x9 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        pub const fn ccof(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x9 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "status register"]
    pub struct Sr1chCmp {
        bits: u32,
    }
    impl Default for Sr1chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sr1chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn set_uif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn uif(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        pub const fn set_ccif(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        pub const fn ccif(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "COM interrupt flag"]
        pub const fn set_comif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "COM interrupt flag"]
        pub const fn comif(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break x (x=1) interrupt flag"]
        pub const fn set_bif(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x7 + idx * 0x1));
            self.bits |= if val { 1 << (0x7 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1) interrupt flag"]
        pub const fn bif(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x7 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        pub const fn set_ccof(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x9 + idx * 0x1));
            self.bits |= if val { 1 << (0x9 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        pub const fn ccof(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x9 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "status register"]
    pub struct Sr2ch {
        bits: u32,
    }
    impl Default for Sr2ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sr2ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn set_uif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn uif(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-2) interrupt flag"]
        pub const fn set_ccif(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-2) interrupt flag"]
        pub const fn ccif(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger interrupt flag"]
        pub const fn set_tif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger interrupt flag"]
        pub const fn tif(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) overcapture flag"]
        pub const fn set_ccof(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x9 + idx * 0x1));
            self.bits |= if val { 1 << (0x9 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-2) overcapture flag"]
        pub const fn ccof(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x9 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "status register"]
    pub struct Sr2chCmp {
        bits: u32,
    }
    impl Default for Sr2chCmp {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sr2chCmp {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn set_uif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn uif(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1,2) interrupt flag"]
        pub const fn set_ccif(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1,2) interrupt flag"]
        pub const fn ccif(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "COM interrupt flag"]
        pub const fn set_comif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "COM interrupt flag"]
        pub const fn comif(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger interrupt flag"]
        pub const fn set_tif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger interrupt flag"]
        pub const fn tif(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break x (x=1) interrupt flag"]
        pub const fn set_bif(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 1);
            self.bits &= !(0x1 << (0x7 + idx * 0x1));
            self.bits |= if val { 1 << (0x7 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1) interrupt flag"]
        pub const fn bif(self, idx: usize) -> bool {
            assert!(idx < 1);
            ((self.bits >> (0x7 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1,2) overcapture flag"]
        pub const fn set_ccof(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x9 + idx * 0x1));
            self.bits |= if val { 1 << (0x9 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1,2) overcapture flag"]
        pub const fn ccof(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x9 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "status register"]
    pub struct SrAdv {
        bits: u32,
    }
    impl Default for SrAdv {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl SrAdv {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn set_uif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn uif(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        pub const fn set_ccif(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        pub const fn ccif(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "COM interrupt flag"]
        pub const fn set_comif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "COM interrupt flag"]
        pub const fn comif(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger interrupt flag"]
        pub const fn set_tif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger interrupt flag"]
        pub const fn tif(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Break x (x=1,2) interrupt flag"]
        pub const fn set_bif(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 2);
            self.bits &= !(0x1 << (0x7 + idx * 0x1));
            self.bits |= if val { 1 << (0x7 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Break x (x=1,2) interrupt flag"]
        pub const fn bif(self, idx: usize) -> bool {
            assert!(idx < 2);
            ((self.bits >> (0x7 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        pub const fn set_ccof(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x9 + idx * 0x1));
            self.bits |= if val { 1 << (0x9 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        pub const fn ccof(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x9 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "System break interrupt flag"]
        pub const fn set_sbif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= if val { 1 << 0xd } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "System break interrupt flag"]
        pub const fn sbif(self) -> bool {
            ((self.bits >> 0xd) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare 5 interrupt flag"]
        pub const fn set_ccif5(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x10);
            self.bits |= if val { 1 << 0x10 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare 5 interrupt flag"]
        pub const fn ccif5(self) -> bool {
            ((self.bits >> 0x10) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare 6 interrupt flag"]
        pub const fn set_ccif6(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x11);
            self.bits |= if val { 1 << 0x11 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare 6 interrupt flag"]
        pub const fn ccif6(self) -> bool {
            ((self.bits >> 0x11) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "status register"]
    pub struct SrCore {
        bits: u32,
    }
    impl Default for SrCore {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl SrCore {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn set_uif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn uif(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "status register"]
    pub struct SrGp16 {
        bits: u32,
    }
    impl Default for SrGp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl SrGp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn set_uif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Update interrupt flag"]
        pub const fn uif(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        pub const fn set_ccif(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x1 + idx * 0x1));
            self.bits |= if val { 1 << (0x1 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        pub const fn ccif(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x1 + idx * 0x1)) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Trigger interrupt flag"]
        pub const fn set_tif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Trigger interrupt flag"]
        pub const fn tif(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        pub const fn set_ccof(mut self, idx: usize, val: bool) -> Self {
            assert!(idx < 4);
            self.bits &= !(0x1 << (0x9 + idx * 0x1));
            self.bits |= if val { 1 << (0x9 + idx * 0x1) } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        pub const fn ccof(self, idx: usize) -> bool {
            assert!(idx < 4);
            ((self.bits >> (0x9 + idx * 0x1)) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "input selection register"]
    pub struct Tisel1ch {
        bits: u32,
    }
    impl Default for Tisel1ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tisel1ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Selects TIM_TIx (x=1) input"]
        pub const fn set_tisel(mut self, idx: usize, val: u8) -> Self {
            assert!(idx < 1);
            self.bits &= !(0xf << (0x0 + idx * 0x8));
            self.bits |= (val as u32 & 0xf) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Selects TIM_TIx (x=1) input"]
        pub const fn tisel(self, idx: usize) -> u8 {
            assert!(idx < 1);
            ((self.bits >> (0x0 + idx * 0x8)) & 0xf) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "input selection register"]
    pub struct Tisel2ch {
        bits: u32,
    }
    impl Default for Tisel2ch {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Tisel2ch {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Selects TIM_TIx (x=1-2) input"]
        pub const fn set_tisel(mut self, idx: usize, val: u8) -> Self {
            assert!(idx < 2);
            self.bits &= !(0xf << (0x0 + idx * 0x8));
            self.bits |= (val as u32 & 0xf) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Selects TIM_TIx (x=1-2) input"]
        pub const fn tisel(self, idx: usize) -> u8 {
            assert!(idx < 2);
            ((self.bits >> (0x0 + idx * 0x8)) & 0xf) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "input selection register"]
    pub struct TiselGp16 {
        bits: u32,
    }
    impl Default for TiselGp16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl TiselGp16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Selects TIM_TIx (x=1-4) input"]
        pub const fn set_tisel(mut self, idx: usize, val: u8) -> Self {
            assert!(idx < 4);
            self.bits &= !(0xf << (0x0 + idx * 0x8));
            self.bits |= (val as u32 & 0xf) << (0x0 + idx * 0x8);
            self
        }
        #[inline(always)]
        #[doc = "Selects TIM_TIx (x=1-4) input"]
        pub const fn tisel(self, idx: usize) -> u8 {
            assert!(idx < 4);
            ((self.bits >> (0x0 + idx * 0x8)) & 0xf) as _
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum Bkinp {
        #[doc = "input polarity is not inverted (active low if BKxP = 0, active high if BKxP = 1)"]
        NotInverted = 0x0,
        #[doc = "input polarity is inverted (active high if BKxP = 0, active low if BKxP = 1)"]
        Inverted = 0x1,
    }
    impl Bkinp {
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
    pub enum Bkp {
        #[doc = "Break input tim_brk is active low"]
        ActiveLow = 0x0,
        #[doc = "Break input tim_brk is active high"]
        ActiveHigh = 0x1,
    }
    impl Bkp {
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
    pub enum Ccds {
        #[doc = "CCx DMA request sent when CCx event occurs"]
        OnCompare = 0x0,
        #[doc = "CCx DMA request sent when update event occurs"]
        OnUpdate = 0x1,
    }
    impl Ccds {
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
    pub enum CcmrInputCcs {
        #[doc = "CCx channel is configured as input, normal mapping: ICx mapped to TIx"]
        Ti4 = 0x1,
        #[doc = "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)"]
        Ti3 = 0x2,
        #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
        Trc = 0x3,
    }
    impl CcmrInputCcs {
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
    pub enum CcmrOutputCcs {
        #[doc = "CCx channel is configured as output"]
        Output = 0x0,
    }
    impl CcmrOutputCcs {
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
    pub enum Ckd {
        #[doc = "t_DTS = t_CK_INT"]
        Div1 = 0x0,
        #[doc = "t_DTS = 2 × t_CK_INT"]
        Div2 = 0x1,
        #[doc = "t_DTS = 4 × t_CK_INT"]
        Div4 = 0x2,
    }
    impl Ckd {
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
    pub enum Cms {
        #[doc = "The counter counts up or down depending on the direction bit"]
        EdgeAligned = 0x0,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
        CenterAligned1 = 0x1,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
        CenterAligned2 = 0x2,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
        CenterAligned3 = 0x3,
    }
    impl Cms {
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
    pub enum Dir {
        #[doc = "Counter used as upcounter"]
        Up = 0x0,
        #[doc = "Counter used as downcounter"]
        Down = 0x1,
    }
    impl Dir {
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
    pub enum Etp {
        #[doc = "ETR is noninverted, active at high level or rising edge"]
        NotInverted = 0x0,
        #[doc = "ETR is inverted, active at low level or falling edge"]
        Inverted = 0x1,
    }
    impl Etp {
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
    pub enum Etps {
        #[doc = "Prescaler OFF"]
        Div1 = 0x0,
        #[doc = "ETRP frequency divided by 2"]
        Div2 = 0x1,
        #[doc = "ETRP frequency divided by 4"]
        Div4 = 0x2,
        #[doc = "ETRP frequency divided by 8"]
        Div8 = 0x3,
    }
    impl Etps {
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
    pub enum FilterValue {
        #[doc = "No filter, sampling is done at fDTS"]
        NoFilter = 0x0,
        #[doc = "fSAMPLING=fCK_INT, N=2"]
        FckIntN2 = 0x1,
        #[doc = "fSAMPLING=fCK_INT, N=4"]
        FckIntN4 = 0x2,
        #[doc = "fSAMPLING=fCK_INT, N=8"]
        FckIntN8 = 0x3,
        #[doc = "fSAMPLING=fDTS/2, N=6"]
        FdtsDiv2N6 = 0x4,
        #[doc = "fSAMPLING=fDTS/2, N=8"]
        FdtsDiv2N8 = 0x5,
        #[doc = "fSAMPLING=fDTS/4, N=6"]
        FdtsDiv4N6 = 0x6,
        #[doc = "fSAMPLING=fDTS/4, N=8"]
        FdtsDiv4N8 = 0x7,
        #[doc = "fSAMPLING=fDTS/8, N=6"]
        FdtsDiv8N6 = 0x8,
        #[doc = "fSAMPLING=fDTS/8, N=8"]
        FdtsDiv8N8 = 0x9,
        #[doc = "fSAMPLING=fDTS/16, N=5"]
        FdtsDiv16N5 = 0xa,
        #[doc = "fSAMPLING=fDTS/16, N=6"]
        FdtsDiv16N6 = 0xb,
        #[doc = "fSAMPLING=fDTS/16, N=8"]
        FdtsDiv16N8 = 0xc,
        #[doc = "fSAMPLING=fDTS/32, N=5"]
        FdtsDiv32N5 = 0xd,
        #[doc = "fSAMPLING=fDTS/32, N=6"]
        FdtsDiv32N6 = 0xe,
        #[doc = "fSAMPLING=fDTS/32, N=8"]
        FdtsDiv32N8 = 0xf,
    }
    impl FilterValue {
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
    pub enum Gc5c {
        #[doc = "No effect of TIM_OC5REF on TIM_OCxREFC (x=1-3)"]
        NoEffect = 0x0,
        #[doc = "TIM_OCxREFC is the logical AND of TIM_OCxREF and TIM_OC5REF"]
        LogicalAnd = 0x1,
    }
    impl Gc5c {
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
    pub enum Lock {
        #[doc = "No bit is write protected"]
        Disabled = 0x0,
        #[doc = "DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKBID/BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written"]
        Level1 = 0x1,
        #[doc = "LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
        Level2 = 0x2,
        #[doc = "LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
        Level3 = 0x3,
    }
    impl Lock {
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
    pub enum Mms {
        #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
        Reset = 0x0,
        #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
        Enable = 0x1,
        #[doc = "The update event is selected as trigger output"]
        Update = 0x2,
        #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
        ComparePulse = 0x3,
        #[doc = "OC1REF signal is used as trigger output"]
        CompareOc1 = 0x4,
        #[doc = "OC2REF signal is used as trigger output"]
        CompareOc2 = 0x5,
        #[doc = "OC3REF signal is used as trigger output"]
        CompareOc3 = 0x6,
        #[doc = "OC4REF signal is used as trigger output"]
        CompareOc4 = 0x7,
    }
    impl Mms {
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
    pub enum Mms2 {
        #[doc = "The UG bit from the TIMx_EGR register is used as TRGO2"]
        Reset = 0x0,
        #[doc = "The counter enable signal, CNT_EN, is used as TRGO2"]
        Enable = 0x1,
        #[doc = "The update event is selected as TRGO2"]
        Update = 0x2,
        #[doc = "TRGO2 send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
        ComparePulse = 0x3,
        #[doc = "OC1REF signal is used as TRGO2"]
        CompareOc1 = 0x4,
        #[doc = "OC2REF signal is used as TRGO2"]
        CompareOc2 = 0x5,
        #[doc = "OC3REF signal is used as TRGO2"]
        CompareOc3 = 0x6,
        #[doc = "OC4REF signal is used as TRGO2"]
        CompareOc4 = 0x7,
        #[doc = "OC5REF signal is used as TRGO2"]
        CompareOc5 = 0x8,
        #[doc = "OC6REF signal is used as TRGO2"]
        CompareOc6 = 0x9,
        #[doc = "OC4REF rising or falling edges generate pulses on TRGO2"]
        ComparePulseOc4 = 0xa,
        #[doc = "OC6REF rising or falling edges generate pulses on TRGO2"]
        ComparePulseOc6 = 0xb,
        #[doc = "OC4REF or OC6REF rising edges generate pulses on TRGO2"]
        ComparePulseOc4OrOc6Rising = 0xc,
        #[doc = "OC4REF rising or OC6REF falling edges generate pulses on TRGO2"]
        ComparePulseOc4RisingOrOc6Falling = 0xd,
        #[doc = "OC5REF or OC6REF rising edges generate pulses on TRGO2"]
        ComparePulseOc5OrOc6Rising = 0xe,
        #[doc = "OC5REF rising or OC6REF falling edges generate pulses on TRGO2"]
        ComparePulseOc5RisingOrOc6Falling = 0xf,
    }
    impl Mms2 {
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
    pub enum Msm {
        #[doc = "No action"]
        NoSync = 0x0,
        #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
        Sync = 0x1,
    }
    impl Msm {
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
    pub enum Ocm {
        #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
        Frozen = 0x0,
        #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
        ActiveOnMatch = 0x1,
        #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
        InactiveOnMatch = 0x2,
        #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
        Toggle = 0x3,
        #[doc = "OCyREF is forced low"]
        ForceInactive = 0x4,
        #[doc = "OCyREF is forced high"]
        ForceActive = 0x5,
        #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
        PwmMode1 = 0x6,
        #[doc = "Inversely to PwmMode1"]
        PwmMode2 = 0x7,
    }
    impl Ocm {
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
    pub enum Ossi {
        #[doc = "When inactive, OC/OCN outputs are disabled"]
        Disabled = 0x0,
        #[doc = "When inactive, OC/OCN outputs are forced to idle level"]
        IdleLevel = 0x1,
    }
    impl Ossi {
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
    pub enum Ossr {
        #[doc = "When inactive, OC/OCN outputs are disabled"]
        Disabled = 0x0,
        #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level"]
        IdleLevel = 0x1,
    }
    impl Ossr {
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
    pub enum Sms {
        #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
        Disabled = 0x0,
        #[doc = "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
        EncoderMode1 = 0x1,
        #[doc = "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
        EncoderMode2 = 0x2,
        #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
        EncoderMode3 = 0x3,
        #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
        ResetMode = 0x4,
        #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
        GatedMode = 0x5,
        #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
        TriggerMode = 0x6,
        #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
        ExtClockMode = 0x7,
        #[doc = "Rising edge of the selected trigger input (tim_trgi) reinitializes the counter, generates an update of the registers and starts the counter."]
        CombinedResetTrigger = 0x8,
    }
    impl Sms {
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
    pub enum Ti1s {
        #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
        Normal = 0x0,
        #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
        Xor = 0x1,
    }
    impl Ti1s {
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
    pub enum Ts {
        #[doc = "Internal Trigger 0"]
        Itr0 = 0x0,
        #[doc = "Internal Trigger 1"]
        Itr1 = 0x1,
        #[doc = "Internal Trigger 2"]
        Itr2 = 0x2,
        #[doc = "Internal Trigger 3"]
        Itr3 = 0x3,
        #[doc = "TI1 Edge Detector"]
        Ti1fEd = 0x4,
        #[doc = "Filtered Timer Input 1"]
        Ti1fp1 = 0x5,
        #[doc = "Filtered Timer Input 2"]
        Ti2fp2 = 0x6,
        #[doc = "External Trigger input"]
        Etrf = 0x7,
        #[doc = "Internal Trigger 4"]
        Itr4 = 0x8,
        #[doc = "Internal Trigger 5"]
        Itr5 = 0x9,
        #[doc = "Internal Trigger 6"]
        Itr6 = 0xa,
        #[doc = "Internal Trigger 7"]
        Itr7 = 0xb,
        #[doc = "Internal Trigger 8"]
        Itr8 = 0xc,
        #[doc = "Internal Trigger 9"]
        Itr9 = 0xd,
        #[doc = "Internal Trigger 10"]
        Itr10 = 0xe,
        #[doc = "Internal Trigger 11"]
        Itr11 = 0xf,
        #[doc = "Internal Trigger 12"]
        Itr12 = 0x10,
        #[doc = "Internal Trigger 13"]
        Itr13 = 0x11,
        #[doc = "Internal Trigger 14"]
        Itr14 = 0x12,
        #[doc = "Internal Trigger 15"]
        Itr15 = 0x13,
    }
    impl Ts {
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
    pub enum Urs {
        #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
        AnyEvent = 0x0,
        #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
        CounterOnly = 0x1,
    }
    impl Urs {
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
