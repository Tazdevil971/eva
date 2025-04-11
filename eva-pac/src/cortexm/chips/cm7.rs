#[allow(unused_imports)]
use super::utils;
#[path = "../modules/cm_regs.rs"]
pub mod cm_regs;
#[path = "../modules/scb.rs"]
pub mod scb;
#[path = "../modules/syst.rs"]
pub mod syst;
#[doc = "System Timer"]
pub const SYST: syst::Syst = unsafe { <syst::Syst>::from_addr(0xe000e010) };
#[doc = "System Control Block"]
pub const SCB: scb::Scb = unsafe { <scb::Scb>::from_addr(0xe000ed00) };
#[doc = "Fault mask register"]
pub mod faultmask {
    #[allow(unused_imports)]
    use super::*;
    pub unsafe fn read() -> cm_regs::FaultmaskBits {
        let value: u32;
        unsafe {
            ::core::arch::asm!(
                "mrs {}, FAULTMASK",
                out(reg) value,
                options(nomem, nostack, preserves_flags)
            );
            <cm_regs::FaultmaskBits>::from_bits_unchecked(value)
        }
    }
    pub unsafe fn write(value: cm_regs::FaultmaskBits) {
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
    pub unsafe fn read() -> cm_regs::PrimaskBits {
        let value: u32;
        unsafe {
            ::core::arch::asm!(
                "mrs {}, PRIMASK",
                out(reg) value,
                options(nomem, nostack, preserves_flags)
            );
            <cm_regs::PrimaskBits>::from_bits_unchecked(value)
        }
    }
    pub unsafe fn write(value: cm_regs::PrimaskBits) {
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
