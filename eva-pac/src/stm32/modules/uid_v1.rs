
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Device Factory programmed 96-bit unique device identifier"]
pub struct Uid {
    ptr: *mut u8,
}
impl Uid {
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
    #[doc = "Factory programmed 96-bit unique device identifier word 0"]
    pub const fn uid(&self, idx: usize) -> utils::Reg<u32, utils::RO> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x0 + idx * 0x4);
            <utils::Reg<u32, utils::RO>>::from_ptr(ptr)
        }
    }
}
