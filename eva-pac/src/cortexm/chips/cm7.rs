#[allow(unused_imports)]
use super::utils;
#[path = "../modules/cm_regs.rs"]
pub mod cm_regs;
#[path = "../modules/scb.rs"]
pub mod scb;
#[doc = "System Control Block"]
pub const SCB: scb::Scb = unsafe { <scb::Scb>::from_ptr(0xe000ed00u64 as _) };
#[doc = "Fault mask register"]
pub mod faultmask {
    #[allow(unused_imports)]
    use super::*;
    pub unsafe fn read() -> cm_regs::fields::Faultmask {
        let value: u32;
        unsafe {
            ::core::arch::asm!(
                "mrs {}, FAULTMASK",
                out(reg) value,
                options(nomem, nostack, preserves_flags)
            );
            <cm_regs::fields::Faultmask>::from_bits_unchecked(value)
        }
    }
    pub unsafe fn write(value: cm_regs::fields::Faultmask) {
        let value = value.to_bits();
        unsafe {
            ::core::arch::asm!(
                "msr FAULTMASK, {}",
                in(reg) value,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}
#[doc = "Priority mask register"]
pub mod primask {
    #[allow(unused_imports)]
    use super::*;
    pub unsafe fn read() -> cm_regs::fields::Primask {
        let value: u32;
        unsafe {
            ::core::arch::asm!(
                "mrs {}, PRIMASK",
                out(reg) value,
                options(nomem, nostack, preserves_flags)
            );
            <cm_regs::fields::Primask>::from_bits_unchecked(value)
        }
    }
    pub unsafe fn write(value: cm_regs::fields::Primask) {
        let value = value.to_bits();
        unsafe {
            ::core::arch::asm!(
                "msr PRIMASK, {}",
                in(reg) value,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}
