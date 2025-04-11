
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "1-channel timers"]
pub struct Tim1ch {
    ptr: *mut u8,
}
impl Tim1ch {
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
    pub const fn cr1(&self) -> utils::Reg<Cr11chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr11chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<Dier1chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<Dier1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<Sr1chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<Sr1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<Egr1chBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<Egr1chBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<CcmrInput1chBits, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrInput1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<CcmrOutput1chBits, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrOutput1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<Ccer1chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<Ccer1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<CntCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<CntCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn arr(&self) -> utils::Reg<ArrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<ArrCoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<Ccr1chBits, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<Ccr1chBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn tisel(&self) -> utils::Reg<Tisel1chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<Tisel1chBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn cr1(&self) -> utils::Reg<Cr11chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr11chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr21chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr21chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<Dier1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<Dier1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<Sr1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<Sr1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<Egr1chCmpBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<Egr1chCmpBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<CcmrInput1chBits, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrInput1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<CcmrOutput1chBits, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrOutput1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<Ccer1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<Ccer1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<CntCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<CntCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn arr(&self) -> utils::Reg<ArrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<ArrCoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "repetition counter register"]
    pub const fn rcr(&self) -> utils::Reg<Rcr1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<Rcr1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<Ccr1chBits, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<Ccr1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "break and dead-time register"]
    pub const fn bdtr(&self) -> utils::Reg<Bdtr1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<Bdtr1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA control register"]
    pub const fn dcr(&self) -> utils::Reg<Dcr1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<Dcr1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA address for full transfer"]
    pub const fn dmar(&self) -> utils::Reg<DmarGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<DmarGp16Bits, utils::RW>>::from_ptr(ptr)
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
    pub const fn af1(&self) -> utils::Reg<Af11chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<Af11chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<Tisel1chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<Tisel1chBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn cr1(&self) -> utils::Reg<Cr11chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr11chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr22chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr22chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "slave mode control register"]
    pub const fn smcr(&self) -> utils::Reg<Smcr2chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<Smcr2chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<Dier2chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<Dier2chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<Sr2chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<Sr2chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<Egr2chBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<Egr2chBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<CcmrInput2chBits, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrInput2chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<CcmrOutput2chBits, utils::RW> {
        assert!(idx < 1);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrOutput2chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<Ccer2chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<Ccer2chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<CntCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<CntCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn arr(&self) -> utils::Reg<ArrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<ArrCoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1-2)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<Ccr1chBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<Ccr1chBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn tisel(&self) -> utils::Reg<Tisel2chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<Tisel2chBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn cr1(&self) -> utils::Reg<Cr11chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr11chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr22chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr22chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "slave mode control register"]
    pub const fn smcr(&self) -> utils::Reg<Smcr2chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<Smcr2chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<Dier2chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<Dier2chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<Sr2chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<Sr2chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<Egr2chCmpBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<Egr2chCmpBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<CcmrInput1chBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrInput1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<CcmrOutput1chBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrOutput1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<Ccer2chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<Ccer2chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<CntCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<CntCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn arr(&self) -> utils::Reg<ArrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<ArrCoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "repetition counter register"]
    pub const fn rcr(&self) -> utils::Reg<Rcr1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<Rcr1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1-2)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<Ccr1chBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<Ccr1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "break and dead-time register"]
    pub const fn bdtr(&self) -> utils::Reg<Bdtr1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<Bdtr1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA control register"]
    pub const fn dcr(&self) -> utils::Reg<Dcr1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<Dcr1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA address for full transfer"]
    pub const fn dmar(&self) -> utils::Reg<DmarGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<DmarGp16Bits, utils::RW>>::from_ptr(ptr)
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
    pub const fn af1(&self) -> utils::Reg<Af11chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<Af11chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<Tisel2chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<Tisel2chBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn cr1(&self) -> utils::Reg<Cr1Gp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr1Gp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr2AdvBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr2AdvBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "slave mode control register"]
    pub const fn smcr(&self) -> utils::Reg<SmcrGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<SmcrGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<DierAdvBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<DierAdvBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrAdvBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<SrAdvBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<EgrAdvBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<EgrAdvBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<CcmrInput2chBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrInput2chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<CcmrOutputGp16Bits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrOutputGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<CcerAdvBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<CcerAdvBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<CntCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<CntCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn arr(&self) -> utils::Reg<ArrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<ArrCoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "repetition counter register"]
    pub const fn rcr(&self) -> utils::Reg<RcrAdvBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<RcrAdvBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1-4)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<Ccr1chBits, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<Ccr1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "break and dead-time register"]
    pub const fn bdtr(&self) -> utils::Reg<BdtrAdvBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<BdtrAdvBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA control register"]
    pub const fn dcr(&self) -> utils::Reg<Dcr1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<Dcr1chCmpBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn ccmr3(&self) -> utils::Reg<Ccmr3AdvBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x54);
            <utils::Reg<Ccmr3AdvBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register 5"]
    pub const fn ccr5(&self) -> utils::Reg<Ccr5AdvBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x58);
            <utils::Reg<Ccr5AdvBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register 6"]
    pub const fn ccr6(&self) -> utils::Reg<Ccr1chBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c);
            <utils::Reg<Ccr1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "alternate function register 1"]
    pub const fn af1(&self) -> utils::Reg<Af1AdvBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<Af1AdvBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "alternate function register 2"]
    pub const fn af2(&self) -> utils::Reg<Af2AdvBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x64);
            <utils::Reg<Af2AdvBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<TiselGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<TiselGp16Bits, utils::RW>>::from_ptr(ptr)
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
    pub const fn cr1(&self) -> utils::Reg<Cr1CoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr1CoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr2BasicBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr2BasicBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<DierBasicNoCr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<DierBasicNoCr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<SrCoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<EgrCoreBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<EgrCoreBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<CntCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<CntCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn arr(&self) -> utils::Reg<ArrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<ArrCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn cr1(&self) -> utils::Reg<Cr1CoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr1CoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<DierBasicNoCr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<DierBasicNoCr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<SrCoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<EgrCoreBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<EgrCoreBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<CntCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<CntCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn arr(&self) -> utils::Reg<ArrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<ArrCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn cr1(&self) -> utils::Reg<Cr1CoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr1CoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<DierCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<DierCoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<SrCoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<EgrCoreBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<EgrCoreBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<CntCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<CntCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn arr(&self) -> utils::Reg<ArrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<ArrCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn cr1(&self) -> utils::Reg<Cr1Gp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr1Gp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr2Gp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr2Gp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "slave mode control register"]
    pub const fn smcr(&self) -> utils::Reg<SmcrGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<SmcrGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<DierGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<DierGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<SrGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<EgrGp16Bits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<EgrGp16Bits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<CcmrInput2chBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrInput2chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<CcmrOutputGp16Bits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrOutputGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<CcerGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<CcerGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "counter"]
    pub const fn cnt(&self) -> utils::Reg<CntCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<CntCoreBits, utils::RW>>::from_ptr(ptr)
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
    pub const fn arr(&self) -> utils::Reg<ArrCoreBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<ArrCoreBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare register x (x=1-4)"]
    pub const fn ccr(&self, idx: usize) -> utils::Reg<Ccr1chBits, utils::RW> {
        assert!(idx < 4);
        unsafe {
            let ptr = self.ptr.add(0x34 + idx * 0x4);
            <utils::Reg<Ccr1chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA control register"]
    pub const fn dcr(&self) -> utils::Reg<Dcr1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<Dcr1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA address for full transfer"]
    pub const fn dmar(&self) -> utils::Reg<DmarGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<DmarGp16Bits, utils::RW>>::from_ptr(ptr)
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
    pub const fn af1(&self) -> utils::Reg<Af1Gp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<Af1Gp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<TiselGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<TiselGp16Bits, utils::RW>>::from_ptr(ptr)
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
    pub const fn cr1(&self) -> utils::Reg<Cr1Gp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr1Gp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "control register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr2Gp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr2Gp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "slave mode control register"]
    pub const fn smcr(&self) -> utils::Reg<SmcrGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<SmcrGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA/Interrupt enable register"]
    pub const fn dier(&self) -> utils::Reg<DierGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<DierGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "status register"]
    pub const fn sr(&self) -> utils::Reg<SrGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<SrGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "event generation register"]
    pub const fn egr(&self) -> utils::Reg<EgrGp16Bits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<EgrGp16Bits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    pub const fn ccmr_input(&self, idx: usize) -> utils::Reg<CcmrInput2chBits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrInput2chBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    pub const fn ccmr_output(&self, idx: usize) -> utils::Reg<CcmrOutputGp16Bits, utils::RW> {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x18 + idx * 0x4);
            <utils::Reg<CcmrOutputGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "capture/compare enable register"]
    pub const fn ccer(&self) -> utils::Reg<CcerGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<CcerGp16Bits, utils::RW>>::from_ptr(ptr)
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
    pub const fn dcr(&self) -> utils::Reg<Dcr1chCmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<Dcr1chCmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DMA address for full transfer"]
    pub const fn dmar(&self) -> utils::Reg<DmarGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<DmarGp16Bits, utils::RW>>::from_ptr(ptr)
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
    pub const fn af1(&self) -> utils::Reg<Af1Gp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<Af1Gp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "input selection register"]
    pub const fn tisel(&self) -> utils::Reg<TiselGp16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<TiselGp16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "alternate function register 1"]
pub struct Af11chCmpBits {
    bits: u32,
}
impl Default for Af11chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Af11chCmpBits {
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
    pub const fn set_bkinp(mut self, val: BkinpVal) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "TIMx_BKIN input polarity"]
    pub const fn bkinp(self) -> BkinpVal {
        let val = ((self.bits >> 0x9) & 0x1) as _;
        unsafe { BkinpVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "TIM_BRK_CMPx (x=1-2) input polarity"]
    pub const fn set_bkcmpp(mut self, idx: usize, val: BkinpVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0xa + idx * 0x1));
        self.bits |= (val.to_bits() as u32 & 0x1) << (0xa + idx * 0x1);
        self
    }
    #[inline(always)]
    #[doc = "TIM_BRK_CMPx (x=1-2) input polarity"]
    pub const fn bkcmpp(self, idx: usize) -> BkinpVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0xa + idx * 0x1)) & 0x1) as _;
        unsafe { BkinpVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "alternate function register 1"]
pub struct Af1AdvBits {
    bits: u32,
}
impl Default for Af1AdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Af1AdvBits {
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
    pub const fn set_bkinp(mut self, val: BkinpVal) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "TIMx_BKIN input polarity"]
    pub const fn bkinp(self) -> BkinpVal {
        let val = ((self.bits >> 0x9) & 0x1) as _;
        unsafe { BkinpVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "TIM_BRK_CMPx (x=1-2) input polarity"]
    pub const fn set_bkcmpp(mut self, idx: usize, val: BkinpVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0xa + idx * 0x1));
        self.bits |= (val.to_bits() as u32 & 0x1) << (0xa + idx * 0x1);
        self
    }
    #[inline(always)]
    #[doc = "TIM_BRK_CMPx (x=1-2) input polarity"]
    pub const fn bkcmpp(self, idx: usize) -> BkinpVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0xa + idx * 0x1)) & 0x1) as _;
        unsafe { BkinpVal::from_bits_unchecked(val) }
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
pub struct Af1Gp16Bits {
    bits: u32,
}
impl Default for Af1Gp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Af1Gp16Bits {
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
pub struct Af2AdvBits {
    bits: u32,
}
impl Default for Af2AdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Af2AdvBits {
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
    pub const fn set_bk2inp(mut self, val: BkinpVal) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "TIMx_BK2IN input polarity"]
    pub const fn bk2inp(self) -> BkinpVal {
        let val = ((self.bits >> 0x9) & 0x1) as _;
        unsafe { BkinpVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "TIM_BRK2_CMPx (x=1-4) input polarity"]
    pub const fn set_bk2cmpp(mut self, idx: usize, val: BkinpVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0xa + idx * 0x1));
        self.bits |= (val.to_bits() as u32 & 0x1) << (0xa + idx * 0x1);
        self
    }
    #[inline(always)]
    #[doc = "TIM_BRK2_CMPx (x=1-4) input polarity"]
    pub const fn bk2cmpp(self, idx: usize) -> BkinpVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0xa + idx * 0x1)) & 0x1) as _;
        unsafe { BkinpVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "auto-reload register"]
pub struct ArrCoreBits {
    bits: u32,
}
impl Default for ArrCoreBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ArrCoreBits {
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
pub struct Bdtr1chCmpBits {
    bits: u32,
}
impl Default for Bdtr1chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Bdtr1chCmpBits {
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
    pub const fn set_lock(mut self, val: LockVal) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Lock configuration"]
    pub const fn lock(self) -> LockVal {
        let val = ((self.bits >> 0x8) & 0x3) as _;
        unsafe { LockVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Off-state selection for Idle mode"]
    pub const fn set_ossi(mut self, val: OssiVal) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "Off-state selection for Idle mode"]
    pub const fn ossi(self) -> OssiVal {
        let val = ((self.bits >> 0xa) & 0x1) as _;
        unsafe { OssiVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Off-state selection for Run mode"]
    pub const fn set_ossr(mut self, val: OssrVal) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "Off-state selection for Run mode"]
    pub const fn ossr(self) -> OssrVal {
        let val = ((self.bits >> 0xb) & 0x1) as _;
        unsafe { OssrVal::from_bits_unchecked(val) }
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
    pub const fn set_bkp(mut self, idx: usize, val: BkpVal) -> Self {
        assert!(idx < 1);
        self.bits &= !(0x1 << (0xd + idx * 0xc));
        self.bits |= (val.to_bits() as u32 & 0x1) << (0xd + idx * 0xc);
        self
    }
    #[inline(always)]
    #[doc = "Break x (x=1) polarity"]
    pub const fn bkp(self, idx: usize) -> BkpVal {
        assert!(idx < 1);
        let val = ((self.bits >> (0xd + idx * 0xc)) & 0x1) as _;
        unsafe { BkpVal::from_bits_unchecked(val) }
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
    pub const fn set_bkf(mut self, idx: usize, val: FilterValueVal) -> Self {
        assert!(idx < 1);
        self.bits &= !(0xf << (0x10 + idx * 0x4));
        self.bits |= (val.to_bits() as u32 & 0xf) << (0x10 + idx * 0x4);
        self
    }
    #[inline(always)]
    #[doc = "Break x (x=1) filter"]
    pub const fn bkf(self, idx: usize) -> FilterValueVal {
        assert!(idx < 1);
        let val = ((self.bits >> (0x10 + idx * 0x4)) & 0xf) as _;
        unsafe { FilterValueVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "break and dead-time register"]
pub struct BdtrAdvBits {
    bits: u32,
}
impl Default for BdtrAdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl BdtrAdvBits {
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
    pub const fn set_lock(mut self, val: LockVal) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Lock configuration"]
    pub const fn lock(self) -> LockVal {
        let val = ((self.bits >> 0x8) & 0x3) as _;
        unsafe { LockVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Off-state selection for Idle mode"]
    pub const fn set_ossi(mut self, val: OssiVal) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "Off-state selection for Idle mode"]
    pub const fn ossi(self) -> OssiVal {
        let val = ((self.bits >> 0xa) & 0x1) as _;
        unsafe { OssiVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Off-state selection for Run mode"]
    pub const fn set_ossr(mut self, val: OssrVal) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "Off-state selection for Run mode"]
    pub const fn ossr(self) -> OssrVal {
        let val = ((self.bits >> 0xb) & 0x1) as _;
        unsafe { OssrVal::from_bits_unchecked(val) }
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
    pub const fn set_bkp(mut self, idx: usize, val: BkpVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x1 << (0xd + idx * 0xc));
        self.bits |= (val.to_bits() as u32 & 0x1) << (0xd + idx * 0xc);
        self
    }
    #[inline(always)]
    #[doc = "Break x (x=1,2) polarity"]
    pub const fn bkp(self, idx: usize) -> BkpVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0xd + idx * 0xc)) & 0x1) as _;
        unsafe { BkpVal::from_bits_unchecked(val) }
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
    pub const fn set_bkf(mut self, idx: usize, val: FilterValueVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0xf << (0x10 + idx * 0x4));
        self.bits |= (val.to_bits() as u32 & 0xf) << (0x10 + idx * 0x4);
        self
    }
    #[inline(always)]
    #[doc = "Break x (x=1,2) filter"]
    pub const fn bkf(self, idx: usize) -> FilterValueVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0x10 + idx * 0x4)) & 0xf) as _;
        unsafe { FilterValueVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "capture/compare enable register"]
pub struct Ccer1chBits {
    bits: u32,
}
impl Default for Ccer1chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ccer1chBits {
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
pub struct Ccer1chCmpBits {
    bits: u32,
}
impl Default for Ccer1chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ccer1chCmpBits {
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
pub struct Ccer2chBits {
    bits: u32,
}
impl Default for Ccer2chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ccer2chBits {
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
pub struct Ccer2chCmpBits {
    bits: u32,
}
impl Default for Ccer2chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ccer2chCmpBits {
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
pub struct CcerAdvBits {
    bits: u32,
}
impl Default for CcerAdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CcerAdvBits {
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
pub struct CcerGp16Bits {
    bits: u32,
}
impl Default for CcerGp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CcerGp16Bits {
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
pub struct Ccmr3AdvBits {
    bits: u32,
}
impl Default for Ccmr3AdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ccmr3AdvBits {
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
    pub const fn set_ocm(mut self, idx: usize, val: OcmVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x7 << (0x4 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0x7) << (0x4 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Output compare x (x=5,6) mode"]
    pub const fn ocm(self, idx: usize) -> OcmVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0x4 + idx * 0x8)) & 0x7) as _;
        unsafe { OcmVal::from_bits_unchecked(val) }
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
pub struct CcmrInput1chBits {
    bits: u32,
}
impl Default for CcmrInput1chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CcmrInput1chBits {
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
    pub const fn set_ccs(mut self, idx: usize, val: CcmrInputCcsVal) -> Self {
        assert!(idx < 1);
        self.bits &= !(0x3 << (0x0 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Capture/Compare y selection"]
    pub const fn ccs(self, idx: usize) -> CcmrInputCcsVal {
        assert!(idx < 1);
        let val = ((self.bits >> (0x0 + idx * 0x8)) & 0x3) as _;
        unsafe { CcmrInputCcsVal::from_bits_unchecked(val) }
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
    pub const fn set_icf(mut self, idx: usize, val: FilterValueVal) -> Self {
        assert!(idx < 1);
        self.bits &= !(0xf << (0x4 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0xf) << (0x4 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Input capture y filter"]
    pub const fn icf(self, idx: usize) -> FilterValueVal {
        assert!(idx < 1);
        let val = ((self.bits >> (0x4 + idx * 0x8)) & 0xf) as _;
        unsafe { FilterValueVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "capture/compare mode register x (x=1) (input mode)"]
pub struct CcmrInput2chBits {
    bits: u32,
}
impl Default for CcmrInput2chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CcmrInput2chBits {
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
    pub const fn set_ccs(mut self, idx: usize, val: CcmrInputCcsVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x3 << (0x0 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Capture/Compare y selection"]
    pub const fn ccs(self, idx: usize) -> CcmrInputCcsVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0x0 + idx * 0x8)) & 0x3) as _;
        unsafe { CcmrInputCcsVal::from_bits_unchecked(val) }
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
    pub const fn set_icf(mut self, idx: usize, val: FilterValueVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0xf << (0x4 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0xf) << (0x4 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Input capture y filter"]
    pub const fn icf(self, idx: usize) -> FilterValueVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0x4 + idx * 0x8)) & 0xf) as _;
        unsafe { FilterValueVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "capture/compare mode register x (x=1) (output mode)"]
pub struct CcmrOutput1chBits {
    bits: u32,
}
impl Default for CcmrOutput1chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CcmrOutput1chBits {
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
    pub const fn set_ccs(mut self, idx: usize, val: CcmrOutputCcsVal) -> Self {
        assert!(idx < 1);
        self.bits &= !(0x3 << (0x0 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Capture/Compare y selection"]
    pub const fn ccs(self, idx: usize) -> CcmrOutputCcsVal {
        assert!(idx < 1);
        let val = ((self.bits >> (0x0 + idx * 0x8)) & 0x3) as _;
        unsafe { CcmrOutputCcsVal::from_bits_unchecked(val) }
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
    pub const fn set_ocm(mut self, idx: usize, val: OcmVal) -> Self {
        assert!(idx < 1);
        self.bits &= !(0x7 << (0x4 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0x7) << (0x4 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Output compare y mode"]
    pub const fn ocm(self, idx: usize) -> OcmVal {
        assert!(idx < 1);
        let val = ((self.bits >> (0x4 + idx * 0x8)) & 0x7) as _;
        unsafe { OcmVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "capture/compare mode register x (x=1) (output mode)"]
pub struct CcmrOutput2chBits {
    bits: u32,
}
impl Default for CcmrOutput2chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CcmrOutput2chBits {
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
    pub const fn set_ccs(mut self, idx: usize, val: CcmrOutputCcsVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x3 << (0x0 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Capture/Compare y selection"]
    pub const fn ccs(self, idx: usize) -> CcmrOutputCcsVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0x0 + idx * 0x8)) & 0x3) as _;
        unsafe { CcmrOutputCcsVal::from_bits_unchecked(val) }
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
    pub const fn set_ocm(mut self, idx: usize, val: OcmVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x7 << (0x4 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0x7) << (0x4 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Output compare y mode"]
    pub const fn ocm(self, idx: usize) -> OcmVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0x4 + idx * 0x8)) & 0x7) as _;
        unsafe { OcmVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "capture/compare mode register x (x=1-2) (output mode)"]
pub struct CcmrOutputGp16Bits {
    bits: u32,
}
impl Default for CcmrOutputGp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CcmrOutputGp16Bits {
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
    pub const fn set_ccs(mut self, idx: usize, val: CcmrOutputCcsVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x3 << (0x0 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0x3) << (0x0 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Capture/Compare y selection"]
    pub const fn ccs(self, idx: usize) -> CcmrOutputCcsVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0x0 + idx * 0x8)) & 0x3) as _;
        unsafe { CcmrOutputCcsVal::from_bits_unchecked(val) }
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
    pub const fn set_ocm(mut self, idx: usize, val: OcmVal) -> Self {
        assert!(idx < 2);
        self.bits &= !(0x7 << (0x4 + idx * 0x8));
        self.bits |= (val.to_bits() as u32 & 0x7) << (0x4 + idx * 0x8);
        self
    }
    #[inline(always)]
    #[doc = "Output compare y mode"]
    pub const fn ocm(self, idx: usize) -> OcmVal {
        assert!(idx < 2);
        let val = ((self.bits >> (0x4 + idx * 0x8)) & 0x7) as _;
        unsafe { OcmVal::from_bits_unchecked(val) }
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
pub struct Ccr5AdvBits {
    bits: u32,
}
impl Default for Ccr5AdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ccr5AdvBits {
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
    pub const fn set_gc5c(mut self, idx: usize, val: Gc5cVal) -> Self {
        assert!(idx < 3);
        self.bits &= !(0x1 << (0x1d + idx * 0x1));
        self.bits |= (val.to_bits() as u32 & 0x1) << (0x1d + idx * 0x1);
        self
    }
    #[inline(always)]
    #[doc = "Group channel 5 and channel x (x=1-3)"]
    pub const fn gc5c(self, idx: usize) -> Gc5cVal {
        assert!(idx < 3);
        let val = ((self.bits >> (0x1d + idx * 0x1)) & 0x1) as _;
        unsafe { Gc5cVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "capture/compare register x (x=1-4,6)"]
pub struct Ccr1chBits {
    bits: u32,
}
impl Default for Ccr1chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ccr1chBits {
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
pub struct CntCoreBits {
    bits: u32,
}
impl Default for CntCoreBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CntCoreBits {
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
pub struct Cr11chBits {
    bits: u32,
}
impl Default for Cr11chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr11chBits {
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
    pub const fn set_urs(mut self, val: UrsVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Update request source"]
    pub const fn urs(self) -> UrsVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { UrsVal::from_bits_unchecked(val) }
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
    pub const fn set_ckd(mut self, val: CkdVal) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Clock division"]
    pub const fn ckd(self) -> CkdVal {
        let val = ((self.bits >> 0x8) & 0x3) as _;
        unsafe { CkdVal::from_bits_unchecked(val) }
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
pub struct Cr1CoreBits {
    bits: u32,
}
impl Default for Cr1CoreBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr1CoreBits {
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
    pub const fn set_urs(mut self, val: UrsVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Update request source"]
    pub const fn urs(self) -> UrsVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { UrsVal::from_bits_unchecked(val) }
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
pub struct Cr1Gp16Bits {
    bits: u32,
}
impl Default for Cr1Gp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr1Gp16Bits {
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
    pub const fn set_urs(mut self, val: UrsVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Update request source"]
    pub const fn urs(self) -> UrsVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { UrsVal::from_bits_unchecked(val) }
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
    pub const fn set_dir(mut self, val: DirVal) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Direction"]
    pub const fn dir(self) -> DirVal {
        let val = ((self.bits >> 0x4) & 0x1) as _;
        unsafe { DirVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Center-aligned mode selection"]
    pub const fn set_cms(mut self, val: CmsVal) -> Self {
        self.bits &= !(0x3 << 0x5);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Center-aligned mode selection"]
    pub const fn cms(self) -> CmsVal {
        let val = ((self.bits >> 0x5) & 0x3) as _;
        unsafe { CmsVal::from_bits_unchecked(val) }
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
    pub const fn set_ckd(mut self, val: CkdVal) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Clock division"]
    pub const fn ckd(self) -> CkdVal {
        let val = ((self.bits >> 0x8) & 0x3) as _;
        unsafe { CkdVal::from_bits_unchecked(val) }
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
pub struct Cr21chCmpBits {
    bits: u32,
}
impl Default for Cr21chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr21chCmpBits {
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
    pub const fn set_ccds(mut self, val: CcdsVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Capture/compare DMA selection"]
    pub const fn ccds(self) -> CcdsVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { CcdsVal::from_bits_unchecked(val) }
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
pub struct Cr22chBits {
    bits: u32,
}
impl Default for Cr22chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr22chBits {
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
    pub const fn set_mms(mut self, val: MmsVal) -> Self {
        self.bits &= !(0x7 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Master mode selection"]
    pub const fn mms(self) -> MmsVal {
        let val = ((self.bits >> 0x4) & 0x7) as _;
        unsafe { MmsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "TI1 selection"]
    pub const fn set_ti1s(mut self, val: Ti1sVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "TI1 selection"]
    pub const fn ti1s(self) -> Ti1sVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { Ti1sVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register 2"]
pub struct Cr22chCmpBits {
    bits: u32,
}
impl Default for Cr22chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr22chCmpBits {
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
    pub const fn set_ccds(mut self, val: CcdsVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Capture/compare DMA selection"]
    pub const fn ccds(self) -> CcdsVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { CcdsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Master mode selection"]
    pub const fn set_mms(mut self, val: MmsVal) -> Self {
        self.bits &= !(0x7 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Master mode selection"]
    pub const fn mms(self) -> MmsVal {
        let val = ((self.bits >> 0x4) & 0x7) as _;
        unsafe { MmsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "TI1 selection"]
    pub const fn set_ti1s(mut self, val: Ti1sVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "TI1 selection"]
    pub const fn ti1s(self) -> Ti1sVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { Ti1sVal::from_bits_unchecked(val) }
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
pub struct Cr2AdvBits {
    bits: u32,
}
impl Default for Cr2AdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr2AdvBits {
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
    pub const fn set_ccds(mut self, val: CcdsVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Capture/compare DMA selection"]
    pub const fn ccds(self) -> CcdsVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { CcdsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Master mode selection"]
    pub const fn set_mms(mut self, val: MmsVal) -> Self {
        self.bits &= !(0x7 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Master mode selection"]
    pub const fn mms(self) -> MmsVal {
        let val = ((self.bits >> 0x4) & 0x7) as _;
        unsafe { MmsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "TI1 selection"]
    pub const fn set_ti1s(mut self, val: Ti1sVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "TI1 selection"]
    pub const fn ti1s(self) -> Ti1sVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { Ti1sVal::from_bits_unchecked(val) }
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
    pub const fn set_mms2(mut self, val: Mms2Val) -> Self {
        self.bits &= !(0xf << 0x14);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Master mode selection 2"]
    pub const fn mms2(self) -> Mms2Val {
        let val = ((self.bits >> 0x14) & 0xf) as _;
        unsafe { Mms2Val::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register 2"]
pub struct Cr2BasicBits {
    bits: u32,
}
impl Default for Cr2BasicBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr2BasicBits {
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
    pub const fn set_mms(mut self, val: MmsVal) -> Self {
        self.bits &= !(0x7 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Master mode selection"]
    pub const fn mms(self) -> MmsVal {
        let val = ((self.bits >> 0x4) & 0x7) as _;
        unsafe { MmsVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "control register 2"]
pub struct Cr2Gp16Bits {
    bits: u32,
}
impl Default for Cr2Gp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr2Gp16Bits {
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
    pub const fn set_ccds(mut self, val: CcdsVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Capture/compare DMA selection"]
    pub const fn ccds(self) -> CcdsVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { CcdsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Master mode selection"]
    pub const fn set_mms(mut self, val: MmsVal) -> Self {
        self.bits &= !(0x7 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Master mode selection"]
    pub const fn mms(self) -> MmsVal {
        let val = ((self.bits >> 0x4) & 0x7) as _;
        unsafe { MmsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "TI1 selection"]
    pub const fn set_ti1s(mut self, val: Ti1sVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "TI1 selection"]
    pub const fn ti1s(self) -> Ti1sVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { Ti1sVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DMA control register"]
pub struct Dcr1chCmpBits {
    bits: u32,
}
impl Default for Dcr1chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dcr1chCmpBits {
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
pub struct Dier1chBits {
    bits: u32,
}
impl Default for Dier1chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dier1chBits {
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
pub struct Dier1chCmpBits {
    bits: u32,
}
impl Default for Dier1chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dier1chCmpBits {
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
pub struct Dier2chBits {
    bits: u32,
}
impl Default for Dier2chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dier2chBits {
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
pub struct Dier2chCmpBits {
    bits: u32,
}
impl Default for Dier2chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dier2chCmpBits {
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
pub struct DierAdvBits {
    bits: u32,
}
impl Default for DierAdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DierAdvBits {
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
pub struct DierBasicNoCr2Bits {
    bits: u32,
}
impl Default for DierBasicNoCr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DierBasicNoCr2Bits {
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
pub struct DierCoreBits {
    bits: u32,
}
impl Default for DierCoreBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DierCoreBits {
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
pub struct DierGp16Bits {
    bits: u32,
}
impl Default for DierGp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DierGp16Bits {
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
pub struct DmarGp16Bits {
    bits: u32,
}
impl Default for DmarGp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmarGp16Bits {
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
pub struct Egr1chBits {
    bits: u32,
}
impl Default for Egr1chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Egr1chBits {
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
pub struct Egr1chCmpBits {
    bits: u32,
}
impl Default for Egr1chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Egr1chCmpBits {
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
pub struct Egr2chBits {
    bits: u32,
}
impl Default for Egr2chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Egr2chBits {
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
pub struct Egr2chCmpBits {
    bits: u32,
}
impl Default for Egr2chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Egr2chCmpBits {
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
pub struct EgrAdvBits {
    bits: u32,
}
impl Default for EgrAdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl EgrAdvBits {
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
pub struct EgrCoreBits {
    bits: u32,
}
impl Default for EgrCoreBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl EgrCoreBits {
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
pub struct EgrGp16Bits {
    bits: u32,
}
impl Default for EgrGp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl EgrGp16Bits {
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
pub struct Rcr1chCmpBits {
    bits: u32,
}
impl Default for Rcr1chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Rcr1chCmpBits {
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
pub struct RcrAdvBits {
    bits: u32,
}
impl Default for RcrAdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl RcrAdvBits {
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
pub struct Smcr2chBits {
    bits: u32,
}
impl Default for Smcr2chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Smcr2chBits {
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
    pub const fn set_msm(mut self, val: MsmVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "Master/Slave mode"]
    pub const fn msm(self) -> MsmVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { MsmVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "slave mode control register"]
pub struct SmcrGp16Bits {
    bits: u32,
}
impl Default for SmcrGp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SmcrGp16Bits {
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
    pub const fn set_msm(mut self, val: MsmVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "Master/Slave mode"]
    pub const fn msm(self) -> MsmVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { MsmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "External trigger filter"]
    pub const fn set_etf(mut self, val: FilterValueVal) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "External trigger filter"]
    pub const fn etf(self) -> FilterValueVal {
        let val = ((self.bits >> 0x8) & 0xf) as _;
        unsafe { FilterValueVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "External trigger prescaler"]
    pub const fn set_etps(mut self, val: EtpsVal) -> Self {
        self.bits &= !(0x3 << 0xc);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "External trigger prescaler"]
    pub const fn etps(self) -> EtpsVal {
        let val = ((self.bits >> 0xc) & 0x3) as _;
        unsafe { EtpsVal::from_bits_unchecked(val) }
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
    pub const fn set_etp(mut self, val: EtpVal) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
        self
    }
    #[inline(always)]
    #[doc = "External trigger polarity"]
    pub const fn etp(self) -> EtpVal {
        let val = ((self.bits >> 0xf) & 0x1) as _;
        unsafe { EtpVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "status register"]
pub struct Sr1chBits {
    bits: u32,
}
impl Default for Sr1chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Sr1chBits {
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
pub struct Sr1chCmpBits {
    bits: u32,
}
impl Default for Sr1chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Sr1chCmpBits {
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
pub struct Sr2chBits {
    bits: u32,
}
impl Default for Sr2chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Sr2chBits {
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
pub struct Sr2chCmpBits {
    bits: u32,
}
impl Default for Sr2chCmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Sr2chCmpBits {
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
pub struct SrAdvBits {
    bits: u32,
}
impl Default for SrAdvBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SrAdvBits {
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
pub struct SrCoreBits {
    bits: u32,
}
impl Default for SrCoreBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SrCoreBits {
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
pub struct SrGp16Bits {
    bits: u32,
}
impl Default for SrGp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SrGp16Bits {
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
pub struct Tisel1chBits {
    bits: u32,
}
impl Default for Tisel1chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Tisel1chBits {
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
pub struct Tisel2chBits {
    bits: u32,
}
impl Default for Tisel2chBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Tisel2chBits {
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
pub struct TiselGp16Bits {
    bits: u32,
}
impl Default for TiselGp16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl TiselGp16Bits {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BkinpVal {
    #[doc = "input polarity is not inverted (active low if BKxP = 0, active high if BKxP = 1)"]
    NotInverted = 0x0,
    #[doc = "input polarity is inverted (active high if BKxP = 0, active low if BKxP = 1)"]
    Inverted = 0x1,
}
impl BkinpVal {
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
pub enum BkpVal {
    #[doc = "Break input tim_brk is active low"]
    ActiveLow = 0x0,
    #[doc = "Break input tim_brk is active high"]
    ActiveHigh = 0x1,
}
impl BkpVal {
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
pub enum CcdsVal {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    OnCompare = 0x0,
    #[doc = "CCx DMA request sent when update event occurs"]
    OnUpdate = 0x1,
}
impl CcdsVal {
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
pub enum CcmrInputCcsVal {
    #[doc = "CCx channel is configured as input, normal mapping: ICx mapped to TIx"]
    Ti4 = 0x1,
    #[doc = "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)"]
    Ti3 = 0x2,
    #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
    Trc = 0x3,
}
impl CcmrInputCcsVal {
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
pub enum CcmrOutputCcsVal {
    #[doc = "CCx channel is configured as output"]
    Output = 0x0,
}
impl CcmrOutputCcsVal {
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
pub enum CkdVal {
    #[doc = "t_DTS = t_CK_INT"]
    Div1 = 0x0,
    #[doc = "t_DTS = 2 × t_CK_INT"]
    Div2 = 0x1,
    #[doc = "t_DTS = 4 × t_CK_INT"]
    Div4 = 0x2,
}
impl CkdVal {
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
pub enum CmsVal {
    #[doc = "The counter counts up or down depending on the direction bit"]
    EdgeAligned = 0x0,
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
    CenterAligned1 = 0x1,
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
    CenterAligned2 = 0x2,
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
    CenterAligned3 = 0x3,
}
impl CmsVal {
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
    #[doc = "Counter used as upcounter"]
    Up = 0x0,
    #[doc = "Counter used as downcounter"]
    Down = 0x1,
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
pub enum EtpVal {
    #[doc = "ETR is noninverted, active at high level or rising edge"]
    NotInverted = 0x0,
    #[doc = "ETR is inverted, active at low level or falling edge"]
    Inverted = 0x1,
}
impl EtpVal {
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
pub enum EtpsVal {
    #[doc = "Prescaler OFF"]
    Div1 = 0x0,
    #[doc = "ETRP frequency divided by 2"]
    Div2 = 0x1,
    #[doc = "ETRP frequency divided by 4"]
    Div4 = 0x2,
    #[doc = "ETRP frequency divided by 8"]
    Div8 = 0x3,
}
impl EtpsVal {
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
pub enum FilterValueVal {
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
impl FilterValueVal {
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
pub enum Gc5cVal {
    #[doc = "No effect of TIM_OC5REF on TIM_OCxREFC (x=1-3)"]
    NoEffect = 0x0,
    #[doc = "TIM_OCxREFC is the logical AND of TIM_OCxREF and TIM_OC5REF"]
    LogicalAnd = 0x1,
}
impl Gc5cVal {
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
pub enum LockVal {
    #[doc = "No bit is write protected"]
    Disabled = 0x0,
    #[doc = "DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKBID/BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written"]
    Level1 = 0x1,
    #[doc = "LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
    Level2 = 0x2,
    #[doc = "LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
    Level3 = 0x3,
}
impl LockVal {
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
pub enum MmsVal {
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
impl MmsVal {
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
pub enum Mms2Val {
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
impl Mms2Val {
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
pub enum MsmVal {
    #[doc = "No action"]
    NoSync = 0x0,
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    Sync = 0x1,
}
impl MsmVal {
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
pub enum OcmVal {
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
impl OcmVal {
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
pub enum OssiVal {
    #[doc = "When inactive, OC/OCN outputs are disabled"]
    Disabled = 0x0,
    #[doc = "When inactive, OC/OCN outputs are forced to idle level"]
    IdleLevel = 0x1,
}
impl OssiVal {
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
pub enum OssrVal {
    #[doc = "When inactive, OC/OCN outputs are disabled"]
    Disabled = 0x0,
    #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level"]
    IdleLevel = 0x1,
}
impl OssrVal {
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
pub enum SmsVal {
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
impl SmsVal {
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
pub enum Ti1sVal {
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    Normal = 0x0,
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    Xor = 0x1,
}
impl Ti1sVal {
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
pub enum TsVal {
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
impl TsVal {
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
pub enum UrsVal {
    #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    AnyEvent = 0x0,
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    CounterOnly = 0x1,
}
impl UrsVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
