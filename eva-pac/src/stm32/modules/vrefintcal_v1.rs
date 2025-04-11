
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "VREFINT Factory Calibration"]
pub struct Vrefintcal {
    ptr: *mut u8,
}
impl Vrefintcal {
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
    #[doc = "Factory calibration"]
    pub const fn data(&self) -> utils::Reg<u16, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<u16, utils::RO>>::from_ptr(ptr)
        }
    }
}
