#![doc = "System Control Block"]
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "System Control Block"]
pub struct Scb {
    ptr: *mut u8,
}
impl Scb {
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
    #[doc = "CPUID"]
    pub const fn cpuid(&self) -> utils::Reg<CpuidBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<CpuidBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt control and state register"]
    pub const fn icsr(&self) -> utils::Reg<IcsrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<IcsrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Vector table offset register"]
    pub const fn vtor(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Application interrupt and reset control register"]
    pub const fn aircr(&self) -> utils::Reg<AircrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<AircrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "System control register"]
    pub const fn scr(&self) -> utils::Reg<ScrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<ScrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Configuration and control register"]
    pub const fn ccr(&self) -> utils::Reg<CcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<CcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "System handler priority register 1"]
    pub const fn shpr1(&self) -> utils::Reg<Shpr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<Shpr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "System handler priority register 2"]
    pub const fn shpr2(&self) -> utils::Reg<Shpr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<Shpr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "System handler priority register 3"]
    pub const fn shpr3(&self) -> utils::Reg<Shpr3Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<Shpr3Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "System handler control and state register"]
    pub const fn shcrs(&self) -> utils::Reg<ShcrsBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<ShcrsBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Configurable fault status register"]
    pub const fn cfsr(&self) -> utils::Reg<CfsrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<CfsrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "HardFault status register"]
    pub const fn hfsr(&self) -> utils::Reg<HfsrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<HfsrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "MemManage fault address"]
    pub const fn mmfar(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "BusFault address"]
    pub const fn bfar(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Application interrupt and reset control register"]
pub struct AircrBits {
    bits: u32,
}
impl Default for AircrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl AircrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "System reset request"]
    pub const fn set_sysresetreq(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "System reset request"]
    pub const fn sysresetreq(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Interrupt priority grouping field"]
    pub const fn set_prigroup(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x8);
        self.bits |= (val as u32 & 0x7) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Interrupt priority grouping field"]
    pub const fn prigroup(self) -> u8 {
        ((self.bits >> 0x8) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Data endianness bit"]
    pub const fn set_endianness(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data endianness bit"]
    pub const fn endianness(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    pub const fn set_vectkey(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    pub const fn vectkey(self) -> u16 {
        ((self.bits >> 0x10) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Configuration and control register"]
pub struct CcrBits {
    bits: u32,
}
impl Default for CcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Indicates how the processor enters Thread mode"]
    pub const fn set_nonbasethrdena(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Indicates how the processor enters Thread mode"]
    pub const fn nonbasethrdena(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enables unprivileged software access to the STIR"]
    pub const fn set_usersetmpend(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enables unprivileged software access to the STIR"]
    pub const fn usersetmpend(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enables unaligned access traps"]
    pub const fn set_unalign_trp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enables unaligned access traps"]
    pub const fn unalign_trp(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    pub const fn set_div_0_trp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    pub const fn div_0_trp(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions"]
    pub const fn set_bfhfnmign(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions"]
    pub const fn bfhfnmign(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Always reads-as-one. It indicates stack alignment on exception entry is 8-byte aligned"]
    pub const fn set_stkalign(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Always reads-as-one. It indicates stack alignment on exception entry is 8-byte aligned"]
    pub const fn stkalign(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enables L1 data cache"]
    pub const fn set_dc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enables L1 data cache"]
    pub const fn dc(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enables L1 instruction cache"]
    pub const fn set_ic(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enables L1 instruction cache"]
    pub const fn ic(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Always reads-as-one. It indicates branch prediction is enabled"]
    pub const fn set_bp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Always reads-as-one. It indicates branch prediction is enabled"]
    pub const fn bp(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Configurable fault status register"]
pub struct CfsrBits {
    bits: u32,
}
impl Default for CfsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CfsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Instruction access violation flag"]
    pub const fn set_iaccviol(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Instruction access violation flag"]
    pub const fn iaccviol(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data access violation flag"]
    pub const fn set_daccviol(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data access violation flag"]
    pub const fn daccviol(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MemManage fault on unstacking for a return from exception"]
    pub const fn set_munstkerr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MemManage fault on unstacking for a return from exception"]
    pub const fn munstkerr(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MemManage fault on stacking for exception entry"]
    pub const fn set_mstkerr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MemManage fault on stacking for exception entry"]
    pub const fn mstkerr(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    pub const fn set_mlsperr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    pub const fn mlsperr(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MemManage Fault Address register (MMFAR) valid flag"]
    pub const fn set_mmarvalid(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MemManage Fault Address register (MMFAR) valid flag"]
    pub const fn mmarvalid(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Instruction bus error"]
    pub const fn set_ibuserr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Instruction bus error"]
    pub const fn ibuserr(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Precise data bus error"]
    pub const fn set_preciserr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Precise data bus error"]
    pub const fn preciserr(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Imprecise data bus error"]
    pub const fn set_impreciserr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Imprecise data bus error"]
    pub const fn impreciserr(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "BusFault on unstacking for a return from exception"]
    pub const fn set_unstkerr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "BusFault on unstacking for a return from exception"]
    pub const fn unstkerr(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "BusFault on stacking for exception entry"]
    pub const fn set_stkerr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "BusFault on stacking for exception entry"]
    pub const fn stkerr(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    pub const fn set_lsperr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    pub const fn lsperr(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "BusFault Address register (BFAR) valid flag"]
    pub const fn set_bfarvalid(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "BusFault Address register (BFAR) valid flag"]
    pub const fn bfarvalid(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Undefined instruction UsageFault"]
    pub const fn set_undefinstr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Undefined instruction UsageFault"]
    pub const fn undefinstr(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Invalid state UsageFault"]
    pub const fn set_invstate(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Invalid state UsageFault"]
    pub const fn invstate(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN"]
    pub const fn set_invpc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN"]
    pub const fn invpc(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "No coprocessor UsageFault"]
    pub const fn set_nocp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "No coprocessor UsageFault"]
    pub const fn nocp(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Unaligned access UsageFault"]
    pub const fn set_unaligned(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Unaligned access UsageFault"]
    pub const fn unaligned(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Divide by zero UsageFault"]
    pub const fn set_divbyzero(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Divide by zero UsageFault"]
    pub const fn divbyzero(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "CPUID"]
pub struct CpuidBits {
    bits: u32,
}
impl Default for CpuidBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CpuidBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Revision number, the p value in the rnpn product revision identifier"]
    pub const fn set_revision(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Revision number, the p value in the rnpn product revision identifier"]
    pub const fn revision(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Part number of the processor"]
    pub const fn set_part_no(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x4);
        self.bits |= (val as u32 & 0xfff) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Part number of the processor"]
    pub const fn part_no(self) -> u16 {
        ((self.bits >> 0x4) & 0xfff) as _
    }
    #[inline(always)]
    #[doc = "Reads as 0xF"]
    pub const fn set_constant(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x10);
        self.bits |= (val as u32 & 0xf) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Reads as 0xF"]
    pub const fn constant(self) -> u8 {
        ((self.bits >> 0x10) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Variant number, the r value in the rnpn product revision identifier"]
    pub const fn set_variant(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x14);
        self.bits |= (val as u32 & 0xf) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Variant number, the r value in the rnpn product revision identifier"]
    pub const fn variant(self) -> u8 {
        ((self.bits >> 0x14) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Implementer code"]
    pub const fn set_implementer(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Implementer code"]
    pub const fn implementer(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "HardFault status register"]
pub struct HfsrBits {
    bits: u32,
}
impl Default for HfsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HfsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Indicates a BusFault on a vector table read during exception processing"]
    pub const fn set_vecttbl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Indicates a BusFault on a vector table read during exception processing"]
    pub const fn vecttbl(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Indicates a forced hard fault"]
    pub const fn set_forced(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Indicates a forced hard fault"]
    pub const fn forced(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt control and state register"]
pub struct IcsrBits {
    bits: u32,
}
impl Default for IcsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IcsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Contains the active exception number"]
    pub const fn set_vectactive(mut self, val: u16) -> Self {
        self.bits &= !(0x1ff << 0x0);
        self.bits |= (val as u32 & 0x1ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Contains the active exception number"]
    pub const fn vectactive(self) -> u16 {
        ((self.bits >> 0x0) & 0x1ff) as _
    }
    #[inline(always)]
    #[doc = "Indicates whether there preempted active exceptions"]
    pub const fn set_rettobase(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Indicates whether there preempted active exceptions"]
    pub const fn rettobase(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Indicates the exception number of the highest priority pending enabled exception"]
    pub const fn set_vectpending(mut self, val: u16) -> Self {
        self.bits &= !(0x1ff << 0xc);
        self.bits |= (val as u32 & 0x1ff) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "Indicates the exception number of the highest priority pending enabled exception"]
    pub const fn vectpending(self) -> u16 {
        ((self.bits >> 0xc) & 0x1ff) as _
    }
    #[inline(always)]
    #[doc = "Interrupt pending flag, excluding NMI and Faults"]
    pub const fn set_isrpending(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Interrupt pending flag, excluding NMI and Faults"]
    pub const fn isrpending(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SysTick exception clear-pending bit"]
    pub const fn set_pendstclr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SysTick exception clear-pending bit"]
    pub const fn pendstclr(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SysTick exception set-pending bit"]
    pub const fn set_pendstset(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SysTick exception set-pending bit"]
    pub const fn pendstset(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PendSV clear-pending bit"]
    pub const fn set_pendsvclr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PendSV clear-pending bit"]
    pub const fn pendsvclr(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PendSV set-pending bit"]
    pub const fn set_pendsvset(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PendSV set-pending bit"]
    pub const fn pendsvset(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "NMI set-pending bit"]
    pub const fn set_nmipendset(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "NMI set-pending bit"]
    pub const fn nmipendset(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "System control register"]
pub struct ScrBits {
    bits: u32,
}
impl Default for ScrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ScrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
    pub const fn set_sleeponexit(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
    pub const fn sleeponexit(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Controls whether the processor uses sleep or deep sleep as its Low-power mode"]
    pub const fn set_sleepdeep(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Controls whether the processor uses sleep or deep sleep as its Low-power mode"]
    pub const fn sleepdeep(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Send Event on Pending bit"]
    pub const fn set_sevonpend(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Send Event on Pending bit"]
    pub const fn sevonpend(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "System handler control and state register"]
pub struct ShcrsBits {
    bits: u32,
}
impl Default for ShcrsBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ShcrsBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "MemManage exception active bit, reads as 1 if exception is active"]
    pub const fn set_memfaultact(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MemManage exception active bit, reads as 1 if exception is active"]
    pub const fn memfaultact(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "BusFault exception active bit, reads as 1 if exception is active"]
    pub const fn set_busfaultact(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "BusFault exception active bit, reads as 1 if exception is active"]
    pub const fn busfaultact(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UsageFault exception active bit, reads as 1 if exception is active"]
    pub const fn set_usgfaultact(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UsageFault exception active bit, reads as 1 if exception is active"]
    pub const fn usgfaultact(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SVCall active bit, reads as 1 if SVC call is active"]
    pub const fn set_svcallact(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SVCall active bit, reads as 1 if SVC call is active"]
    pub const fn svcallact(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Debug monitor active bit, reads as 1 if Debug monitor is active"]
    pub const fn set_monitoract(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Debug monitor active bit, reads as 1 if Debug monitor is active"]
    pub const fn monitoract(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PendSV exception active bit, reads as 1 if exception is active"]
    pub const fn set_pendsvact(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PendSV exception active bit, reads as 1 if exception is active"]
    pub const fn pendsvact(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SysTick exception active bit, reads as 1 if exception is active"]
    pub const fn set_systickact(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SysTick exception active bit, reads as 1 if exception is active"]
    pub const fn systickact(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UsageFault exception pending bit, reads as 1 if exception is pending"]
    pub const fn set_usgfaultpended(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UsageFault exception pending bit, reads as 1 if exception is pending"]
    pub const fn usgfaultpended(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MemManage exception pending bit, reads as 1 if exception is pending"]
    pub const fn set_memfaultpended(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MemManage exception pending bit, reads as 1 if exception is pending"]
    pub const fn memfaultpended(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "BusFault exception pending bit, reads as 1 if exception is pending"]
    pub const fn set_busfaultpended(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "BusFault exception pending bit, reads as 1 if exception is pending"]
    pub const fn busfaultpended(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SVCall pending bit, reads as 1 if exception is pending"]
    pub const fn set_svcallpended(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SVCall pending bit, reads as 1 if exception is pending"]
    pub const fn svcallpended(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MemManage enable bit, set to 1 to enable"]
    pub const fn set_memfaultena(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MemManage enable bit, set to 1 to enable"]
    pub const fn memfaultena(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "BusFault enable bit, set to 1 to enable"]
    pub const fn set_busfaultena(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "BusFault enable bit, set to 1 to enable"]
    pub const fn busfaultena(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "UsageFault enable bit, set to 1 to enable"]
    pub const fn set_usgfaultena(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "UsageFault enable bit, set to 1 to enable"]
    pub const fn usgfaultena(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "System handler priority register 1"]
pub struct Shpr1Bits {
    bits: u32,
}
impl Default for Shpr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Shpr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Priority of system handler 4, MemManage"]
    pub const fn set_pri_4(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Priority of system handler 4, MemManage"]
    pub const fn pri_4(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Priority of system handler 5, BusFault"]
    pub const fn set_pri_5(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Priority of system handler 5, BusFault"]
    pub const fn pri_5(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Priority of system handler 6, UsageFault"]
    pub const fn set_pri_6(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Priority of system handler 6, UsageFault"]
    pub const fn pri_6(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "System handler priority register 2"]
pub struct Shpr2Bits {
    bits: u32,
}
impl Default for Shpr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Shpr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Priority of system handler 11, SVCall"]
    pub const fn set_pri_11(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Priority of system handler 11, SVCall"]
    pub const fn pri_11(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "System handler priority register 3"]
pub struct Shpr3Bits {
    bits: u32,
}
impl Default for Shpr3Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Shpr3Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Priority of system handler 14, PendSV"]
    pub const fn set_pri_14(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Priority of system handler 14, PendSV"]
    pub const fn pri_14(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Priority of system handler 15, SysTick exception"]
    pub const fn set_pri_15(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Priority of system handler 15, SysTick exception"]
    pub const fn pri_15(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
