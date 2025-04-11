
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "DSI Host."]
pub struct Dsihost {
    ptr: *mut u8,
}
impl Dsihost {
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
    #[doc = "DSI Host Version Register."]
    pub const fn vr(&self) -> utils::Reg<VrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<VrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Control Register."]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI HOST Clock Control Register."]
    pub const fn ccr(&self) -> utils::Reg<CcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<CcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host LTDC VCID Register."]
    pub const fn lvcidr(&self) -> utils::Reg<LvcidrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<LvcidrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host LTDC Color Coding Register."]
    pub const fn lcolcr(&self) -> utils::Reg<LcolcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<LcolcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host LTDC Polarity Configuration Register."]
    pub const fn lpcr(&self) -> utils::Reg<LpcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<LpcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Low-Power mode Configuration Register."]
    pub const fn lpmcr(&self) -> utils::Reg<LpmcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<LpmcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Protocol Configuration Register."]
    pub const fn pcr(&self) -> utils::Reg<PcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<PcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Generic VCID Register."]
    pub const fn gvcidr(&self) -> utils::Reg<GvcidrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<GvcidrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host mode Configuration Register."]
    pub const fn mcr(&self) -> utils::Reg<McrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<McrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video mode Configuration Register."]
    pub const fn vmcr(&self) -> utils::Reg<VmcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<VmcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video Packet Configuration Register."]
    pub const fn vpcr(&self) -> utils::Reg<VpcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c);
            <utils::Reg<VpcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video Chunks Configuration Register."]
    pub const fn vccr(&self) -> utils::Reg<VccrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<VccrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video Null Packet Configuration Register."]
    pub const fn vnpcr(&self) -> utils::Reg<VnpcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<VnpcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video HSA Configuration Register."]
    pub const fn vhsacr(&self) -> utils::Reg<VhsacrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<VhsacrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video HBP Configuration Register."]
    pub const fn vhbpcr(&self) -> utils::Reg<VhbpcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<VhbpcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video Line Configuration Register."]
    pub const fn vlcr(&self) -> utils::Reg<VlcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<VlcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video VSA Configuration Register."]
    pub const fn vvsacr(&self) -> utils::Reg<VvsacrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x54);
            <utils::Reg<VvsacrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video VBP Configuration Register."]
    pub const fn vvbpcr(&self) -> utils::Reg<VvbpcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x58);
            <utils::Reg<VvbpcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video VFP Configuration Register."]
    pub const fn vvfpcr(&self) -> utils::Reg<VvfpcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c);
            <utils::Reg<VvfpcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video VA Configuration Register."]
    pub const fn vvacr(&self) -> utils::Reg<VvacrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<VvacrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host LTDC Command Configuration Register."]
    pub const fn lccr(&self) -> utils::Reg<LccrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x64);
            <utils::Reg<LccrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Command mode Configuration Register."]
    pub const fn cmcr(&self) -> utils::Reg<CmcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<CmcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Generic Header Configuration Register."]
    pub const fn ghcr(&self) -> utils::Reg<GhcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6c);
            <utils::Reg<GhcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Generic Payload Data Register."]
    pub const fn gpdr(&self) -> utils::Reg<GpdrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x70);
            <utils::Reg<GpdrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Generic Packet Status Register."]
    pub const fn gpsr(&self) -> utils::Reg<GpsrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x74);
            <utils::Reg<GpsrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Timeout Counter Configuration Register 0."]
    pub const fn tccr0(&self) -> utils::Reg<Tccr0Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x78);
            <utils::Reg<Tccr0Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Timeout Counter Configuration Register 1."]
    pub const fn tccr1(&self) -> utils::Reg<Tccr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7c);
            <utils::Reg<Tccr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Timeout Counter Configuration Register 2."]
    pub const fn tccr2(&self) -> utils::Reg<Tccr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x80);
            <utils::Reg<Tccr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Timeout Counter Configuration Register 3."]
    pub const fn tccr3(&self) -> utils::Reg<Tccr3Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x84);
            <utils::Reg<Tccr3Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Timeout Counter Configuration Register 4."]
    pub const fn tccr4(&self) -> utils::Reg<Tccr4Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x88);
            <utils::Reg<Tccr4Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Timeout Counter Configuration Register 5."]
    pub const fn tccr5(&self) -> utils::Reg<Tccr5Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8c);
            <utils::Reg<Tccr5Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Clock Lane Configuration Register."]
    pub const fn clcr(&self) -> utils::Reg<ClcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x94);
            <utils::Reg<ClcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Clock Lane Timer Configuration Register."]
    pub const fn cltcr(&self) -> utils::Reg<CltcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x98);
            <utils::Reg<CltcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Data Lane Timer Configuration Register."]
    pub const fn dltcr(&self) -> utils::Reg<DltcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x9c);
            <utils::Reg<DltcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host PHY Control Register."]
    pub const fn pctlr(&self) -> utils::Reg<PctlrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xa0);
            <utils::Reg<PctlrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host PHY Configuration Register."]
    pub const fn pconfr(&self) -> utils::Reg<PconfrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xa4);
            <utils::Reg<PconfrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host PHY ULPS Control Register."]
    pub const fn pucr(&self) -> utils::Reg<PucrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xa8);
            <utils::Reg<PucrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host PHY TX Triggers Configuration Register."]
    pub const fn pttcr(&self) -> utils::Reg<PttcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xac);
            <utils::Reg<PttcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host PHY Status Register."]
    pub const fn psr(&self) -> utils::Reg<PsrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0xb0);
            <utils::Reg<PsrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Interrupt & Status Register 0."]
    pub const fn isr0(&self) -> utils::Reg<Isr0Bits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0xbc);
            <utils::Reg<Isr0Bits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Interrupt & Status Register 1."]
    pub const fn isr1(&self) -> utils::Reg<Isr1Bits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0xc0);
            <utils::Reg<Isr1Bits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Interrupt Enable Register 0."]
    pub const fn ier0(&self) -> utils::Reg<Ier0Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc4);
            <utils::Reg<Ier0Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Interrupt Enable Register 1."]
    pub const fn ier1(&self) -> utils::Reg<Ier1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc8);
            <utils::Reg<Ier1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Force Interrupt Register 0."]
    pub const fn fir0(&self) -> utils::Reg<Fir0Bits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0xd8);
            <utils::Reg<Fir0Bits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Force Interrupt Register 1."]
    pub const fn fir1(&self) -> utils::Reg<Fir1Bits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0xdc);
            <utils::Reg<Fir1Bits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video Shadow Control Register."]
    pub const fn vscr(&self) -> utils::Reg<VscrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x100);
            <utils::Reg<VscrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host LTDC Current VCID Register."]
    pub const fn lcvcidr(&self) -> utils::Reg<LcvcidrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x10c);
            <utils::Reg<LcvcidrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host LTDC Current Color Coding Register."]
    pub const fn lcccr(&self) -> utils::Reg<LcccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x110);
            <utils::Reg<LcccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Low-Power mode Current Configuration Register."]
    pub const fn lpmccr(&self) -> utils::Reg<LpmccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x118);
            <utils::Reg<LpmccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video mode Current Configuration Register."]
    pub const fn vmccr(&self) -> utils::Reg<VmccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x138);
            <utils::Reg<VmccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video Packet Current Configuration Register."]
    pub const fn vpccr(&self) -> utils::Reg<VpccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x13c);
            <utils::Reg<VpccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video Chunks Current Configuration Register."]
    pub const fn vcccr(&self) -> utils::Reg<VcccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x140);
            <utils::Reg<VcccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video Null Packet Current Configuration Register."]
    pub const fn vnpccr(&self) -> utils::Reg<VnpccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x144);
            <utils::Reg<VnpccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video HSA Current Configuration Register."]
    pub const fn vhsaccr(&self) -> utils::Reg<VhsaccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x148);
            <utils::Reg<VhsaccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video HBP Current Configuration Register."]
    pub const fn vhbpccr(&self) -> utils::Reg<VhbpccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x14c);
            <utils::Reg<VhbpccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video Line Current Configuration Register."]
    pub const fn vlccr(&self) -> utils::Reg<VlccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x150);
            <utils::Reg<VlccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video VSA Current Configuration Register."]
    pub const fn vvsaccr(&self) -> utils::Reg<VvsaccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x154);
            <utils::Reg<VvsaccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video VBP Current Configuration Register."]
    pub const fn vvbpccr(&self) -> utils::Reg<VvbpccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x158);
            <utils::Reg<VvbpccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video VFP Current Configuration Register."]
    pub const fn vvfpccr(&self) -> utils::Reg<VvfpccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x15c);
            <utils::Reg<VvfpccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Host Video VA Current Configuration Register."]
    pub const fn vvaccr(&self) -> utils::Reg<VvaccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x160);
            <utils::Reg<VvaccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper Configuration Register."]
    pub const fn wcfgr(&self) -> utils::Reg<WcfgrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x400);
            <utils::Reg<WcfgrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper Control Register."]
    pub const fn wcr(&self) -> utils::Reg<WcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x404);
            <utils::Reg<WcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper Interrupt Enable Register."]
    pub const fn wier(&self) -> utils::Reg<WierBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x408);
            <utils::Reg<WierBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper Interrupt & Status Register."]
    pub const fn wisr(&self) -> utils::Reg<WisrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x40c);
            <utils::Reg<WisrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper Interrupt Flag Clear Register."]
    pub const fn wifcr(&self) -> utils::Reg<WifcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x410);
            <utils::Reg<WifcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper PHY Configuration Register 0."]
    pub const fn wpcr0(&self) -> utils::Reg<Wpcr0Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x418);
            <utils::Reg<Wpcr0Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper PHY Configuration Register 1."]
    pub const fn wpcr1(&self) -> utils::Reg<Wpcr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x41c);
            <utils::Reg<Wpcr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper PHY Configuration Register 2."]
    pub const fn wpcr2(&self) -> utils::Reg<Wpcr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x420);
            <utils::Reg<Wpcr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper PHY Configuration Register 3."]
    pub const fn wpcr3(&self) -> utils::Reg<Wpcr3Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x424);
            <utils::Reg<Wpcr3Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper PHY Configuration Register 4."]
    pub const fn wpcr4(&self) -> utils::Reg<Wpcr4Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x428);
            <utils::Reg<Wpcr4Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "DSI Wrapper Regulator and PLL Control Register."]
    pub const fn wrpcr(&self) -> utils::Reg<WrpcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x430);
            <utils::Reg<WrpcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI HOST Clock Control Register."]
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
    #[doc = "TX Escape Clock Division."]
    pub const fn set_txeckdiv(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "TX Escape Clock Division."]
    pub const fn txeckdiv(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Timeout Clock Division."]
    pub const fn set_tockdiv(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Timeout Clock Division."]
    pub const fn tockdiv(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Clock Lane Configuration Register."]
pub struct ClcrBits {
    bits: u32,
}
impl Default for ClcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ClcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "D-PHY Clock Control."]
    pub const fn set_dpcc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "D-PHY Clock Control."]
    pub const fn dpcc(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Automatic Clock lane Control."]
    pub const fn set_acr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Automatic Clock lane Control."]
    pub const fn acr(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Clock Lane Timer Configuration Register."]
pub struct CltcrBits {
    bits: u32,
}
impl Default for CltcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CltcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Low-Power to High-Speed Time."]
    pub const fn set_lp2hs_time(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x0);
        self.bits |= (val as u32 & 0x3ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Low-Power to High-Speed Time."]
    pub const fn lp2hs_time(self) -> u16 {
        ((self.bits >> 0x0) & 0x3ff) as _
    }
    #[inline(always)]
    #[doc = "High-Speed to Low-Power Time."]
    pub const fn set_hs2lp_time(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x10);
        self.bits |= (val as u32 & 0x3ff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "High-Speed to Low-Power Time."]
    pub const fn hs2lp_time(self) -> u16 {
        ((self.bits >> 0x10) & 0x3ff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Command mode Configuration Register."]
pub struct CmcrBits {
    bits: u32,
}
impl Default for CmcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CmcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Tearing Effect Acknowledge Request Enable."]
    pub const fn set_teare(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tearing Effect Acknowledge Request Enable."]
    pub const fn teare(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Request Enable."]
    pub const fn set_are(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Request Enable."]
    pub const fn are(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Short Write Zero parameters Transmission."]
    pub const fn set_gsw0tx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Short Write Zero parameters Transmission."]
    pub const fn gsw0tx(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Short Write One parameters Transmission."]
    pub const fn set_gsw1tx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Short Write One parameters Transmission."]
    pub const fn gsw1tx(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Short Write Two parameters Transmission."]
    pub const fn set_gsw2tx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Short Write Two parameters Transmission."]
    pub const fn gsw2tx(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Short Read Zero parameters Transmission."]
    pub const fn set_gsr0tx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Short Read Zero parameters Transmission."]
    pub const fn gsr0tx(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Short Read One parameters Transmission."]
    pub const fn set_gsr1tx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Short Read One parameters Transmission."]
    pub const fn gsr1tx(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Short Read Two parameters Transmission."]
    pub const fn set_gsr2tx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Short Read Two parameters Transmission."]
    pub const fn gsr2tx(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Long Write Transmission."]
    pub const fn set_glwtx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Long Write Transmission."]
    pub const fn glwtx(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DCS Short Write Zero parameter Transmission."]
    pub const fn set_dsw0tx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DCS Short Write Zero parameter Transmission."]
    pub const fn dsw0tx(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DCS Short Read One parameter Transmission."]
    pub const fn set_dsw1tx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DCS Short Read One parameter Transmission."]
    pub const fn dsw1tx(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DCS Short Read Zero parameter Transmission."]
    pub const fn set_dsr0tx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DCS Short Read Zero parameter Transmission."]
    pub const fn dsr0tx(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DCS Long Write Transmission."]
    pub const fn set_dlwtx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DCS Long Write Transmission."]
    pub const fn dlwtx(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Maximum Read Packet Size."]
    pub const fn set_mrdps(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Maximum Read Packet Size."]
    pub const fn mrdps(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Control Register."]
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
    #[doc = "Enable."]
    pub const fn set_en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable."]
    pub const fn en(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Data Lane Timer Configuration Register."]
pub struct DltcrBits {
    bits: u32,
}
impl Default for DltcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DltcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Maximum Read Time."]
    pub const fn set_mrd_time(mut self, val: u16) -> Self {
        self.bits &= !(0x7fff << 0x0);
        self.bits |= (val as u32 & 0x7fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Maximum Read Time."]
    pub const fn mrd_time(self) -> u16 {
        ((self.bits >> 0x0) & 0x7fff) as _
    }
    #[inline(always)]
    #[doc = "Low-Power To High-Speed Time."]
    pub const fn set_lp2hs_time(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Low-Power To High-Speed Time."]
    pub const fn lp2hs_time(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "High-Speed To Low-Power Time."]
    pub const fn set_hs2lp_time(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "High-Speed To Low-Power Time."]
    pub const fn hs2lp_time(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Force Interrupt Register 0."]
pub struct Fir0Bits {
    bits: u32,
}
impl Default for Fir0Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Fir0Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 0."]
    pub const fn set_fae0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 0."]
    pub const fn fae0(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 1."]
    pub const fn set_fae1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 1."]
    pub const fn fae1(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 2."]
    pub const fn set_fae2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 2."]
    pub const fn fae2(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 3."]
    pub const fn set_fae3(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 3."]
    pub const fn fae3(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 4."]
    pub const fn set_fae4(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 4."]
    pub const fn fae4(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 5."]
    pub const fn set_fae5(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 5."]
    pub const fn fae5(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 6."]
    pub const fn set_fae6(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 6."]
    pub const fn fae6(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 7."]
    pub const fn set_fae7(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 7."]
    pub const fn fae7(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 8."]
    pub const fn set_fae8(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 8."]
    pub const fn fae8(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 9."]
    pub const fn set_fae9(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 9."]
    pub const fn fae9(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 10."]
    pub const fn set_fae10(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 10."]
    pub const fn fae10(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 11."]
    pub const fn set_fae11(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 11."]
    pub const fn fae11(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 12."]
    pub const fn set_fae12(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 12."]
    pub const fn fae12(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 13."]
    pub const fn set_fae13(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 13."]
    pub const fn fae13(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 14."]
    pub const fn set_fae14(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 14."]
    pub const fn fae14(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 15."]
    pub const fn set_fae15(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Acknowledge Error 15."]
    pub const fn fae15(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force PHY Error 0."]
    pub const fn set_fpe0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force PHY Error 0."]
    pub const fn fpe0(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force PHY Error 1."]
    pub const fn set_fpe1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force PHY Error 1."]
    pub const fn fpe1(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force PHY Error 2."]
    pub const fn set_fpe2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force PHY Error 2."]
    pub const fn fpe2(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force PHY Error 3."]
    pub const fn set_fpe3(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force PHY Error 3."]
    pub const fn fpe3(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force PHY Error 4."]
    pub const fn set_fpe4(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force PHY Error 4."]
    pub const fn fpe4(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Force Interrupt Register 1."]
pub struct Fir1Bits {
    bits: u32,
}
impl Default for Fir1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Fir1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Force Timeout High-Speed Transmission."]
    pub const fn set_ftohstx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Timeout High-Speed Transmission."]
    pub const fn ftohstx(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Timeout Low-Power Reception."]
    pub const fn set_ftolprx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Timeout Low-Power Reception."]
    pub const fn ftolprx(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force ECC Single-bit Error."]
    pub const fn set_feccse(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force ECC Single-bit Error."]
    pub const fn feccse(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force ECC Multi-bit Error."]
    pub const fn set_feccme(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force ECC Multi-bit Error."]
    pub const fn feccme(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force CRC Error."]
    pub const fn set_fcrce(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force CRC Error."]
    pub const fn fcrce(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Packet Size Error."]
    pub const fn set_fpse(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Packet Size Error."]
    pub const fn fpse(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force EoTp Error."]
    pub const fn set_feotpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force EoTp Error."]
    pub const fn feotpe(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force LTDC Payload Write Error."]
    pub const fn set_flpwre(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force LTDC Payload Write Error."]
    pub const fn flpwre(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Generic Command Write Error."]
    pub const fn set_fgcwre(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Generic Command Write Error."]
    pub const fn fgcwre(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Generic Payload Write Error."]
    pub const fn set_fgpwre(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Generic Payload Write Error."]
    pub const fn fgpwre(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Generic Payload Transmit Error."]
    pub const fn set_fgptxe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Generic Payload Transmit Error."]
    pub const fn fgptxe(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Generic Payload Read Error."]
    pub const fn set_fgprde(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Generic Payload Read Error."]
    pub const fn fgprde(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force Generic Payload Receive Error."]
    pub const fn set_fgprxe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force Generic Payload Receive Error."]
    pub const fn fgprxe(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Generic Header Configuration Register."]
pub struct GhcrBits {
    bits: u32,
}
impl Default for GhcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GhcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Type."]
    pub const fn set_dt(mut self, val: u8) -> Self {
        self.bits &= !(0x3f << 0x0);
        self.bits |= (val as u32 & 0x3f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Type."]
    pub const fn dt(self) -> u8 {
        ((self.bits >> 0x0) & 0x3f) as _
    }
    #[inline(always)]
    #[doc = "Channel."]
    pub const fn set_vcid(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x6);
        self.bits |= (val as u32 & 0x3) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Channel."]
    pub const fn vcid(self) -> u8 {
        ((self.bits >> 0x6) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "WordCount LSB."]
    pub const fn set_wclsb(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "WordCount LSB."]
    pub const fn wclsb(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "WordCount MSB."]
    pub const fn set_wcmsb(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "WordCount MSB."]
    pub const fn wcmsb(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Generic Payload Data Register."]
pub struct GpdrBits {
    bits: u32,
}
impl Default for GpdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GpdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Payload Byte 1."]
    pub const fn set_data1(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Payload Byte 1."]
    pub const fn data1(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Payload Byte 2."]
    pub const fn set_data2(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Payload Byte 2."]
    pub const fn data2(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Payload Byte 3."]
    pub const fn set_data3(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Payload Byte 3."]
    pub const fn data3(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Payload Byte 4."]
    pub const fn set_data4(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Payload Byte 4."]
    pub const fn data4(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Generic Packet Status Register."]
pub struct GpsrBits {
    bits: u32,
}
impl Default for GpsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GpsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Command FIFO Empty."]
    pub const fn set_cmdfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command FIFO Empty."]
    pub const fn cmdfe(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Command FIFO Full."]
    pub const fn set_cmdff(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command FIFO Full."]
    pub const fn cmdff(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Payload Write FIFO Empty."]
    pub const fn set_pwrfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Payload Write FIFO Empty."]
    pub const fn pwrfe(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Payload Write FIFO Full."]
    pub const fn set_pwrff(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Payload Write FIFO Full."]
    pub const fn pwrff(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Payload Read FIFO Empty."]
    pub const fn set_prdfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Payload Read FIFO Empty."]
    pub const fn prdfe(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Payload Read FIFO Full."]
    pub const fn set_prdff(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Payload Read FIFO Full."]
    pub const fn prdff(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Read Command Busy."]
    pub const fn set_rcb(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Read Command Busy."]
    pub const fn rcb(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Generic VCID Register."]
pub struct GvcidrBits {
    bits: u32,
}
impl Default for GvcidrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GvcidrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Virtual Channel ID."]
    pub const fn set_vcid(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Virtual Channel ID."]
    pub const fn vcid(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Interrupt Enable Register 0."]
pub struct Ier0Bits {
    bits: u32,
}
impl Default for Ier0Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ier0Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 0 Interrupt Enable."]
    pub const fn set_ae0ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 0 Interrupt Enable."]
    pub const fn ae0ie(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 1 Interrupt Enable."]
    pub const fn set_ae1ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 1 Interrupt Enable."]
    pub const fn ae1ie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 2 Interrupt Enable."]
    pub const fn set_ae2ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 2 Interrupt Enable."]
    pub const fn ae2ie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 3 Interrupt Enable."]
    pub const fn set_ae3ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 3 Interrupt Enable."]
    pub const fn ae3ie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 4 Interrupt Enable."]
    pub const fn set_ae4ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 4 Interrupt Enable."]
    pub const fn ae4ie(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 5 Interrupt Enable."]
    pub const fn set_ae5ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 5 Interrupt Enable."]
    pub const fn ae5ie(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 6 Interrupt Enable."]
    pub const fn set_ae6ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 6 Interrupt Enable."]
    pub const fn ae6ie(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 7 Interrupt Enable."]
    pub const fn set_ae7ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 7 Interrupt Enable."]
    pub const fn ae7ie(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 8 Interrupt Enable."]
    pub const fn set_ae8ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 8 Interrupt Enable."]
    pub const fn ae8ie(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 9 Interrupt Enable."]
    pub const fn set_ae9ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 9 Interrupt Enable."]
    pub const fn ae9ie(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 10 Interrupt Enable."]
    pub const fn set_ae10ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 10 Interrupt Enable."]
    pub const fn ae10ie(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 11 Interrupt Enable."]
    pub const fn set_ae11ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 11 Interrupt Enable."]
    pub const fn ae11ie(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 12 Interrupt Enable."]
    pub const fn set_ae12ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 12 Interrupt Enable."]
    pub const fn ae12ie(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 13 Interrupt Enable."]
    pub const fn set_ae13ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 13 Interrupt Enable."]
    pub const fn ae13ie(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 14 Interrupt Enable."]
    pub const fn set_ae14ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 14 Interrupt Enable."]
    pub const fn ae14ie(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 15 Interrupt Enable."]
    pub const fn set_ae15ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 15 Interrupt Enable."]
    pub const fn ae15ie(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Error 0 Interrupt Enable."]
    pub const fn set_pe0ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Error 0 Interrupt Enable."]
    pub const fn pe0ie(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Error 1 Interrupt Enable."]
    pub const fn set_pe1ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Error 1 Interrupt Enable."]
    pub const fn pe1ie(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Error 2 Interrupt Enable."]
    pub const fn set_pe2ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Error 2 Interrupt Enable."]
    pub const fn pe2ie(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Error 3 Interrupt Enable."]
    pub const fn set_pe3ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Error 3 Interrupt Enable."]
    pub const fn pe3ie(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Error 4 Interrupt Enable."]
    pub const fn set_pe4ie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Error 4 Interrupt Enable."]
    pub const fn pe4ie(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Interrupt Enable Register 1."]
pub struct Ier1Bits {
    bits: u32,
}
impl Default for Ier1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Ier1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Timeout High-Speed Transmission Interrupt Enable."]
    pub const fn set_tohstxie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timeout High-Speed Transmission Interrupt Enable."]
    pub const fn tohstxie(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timeout Low-Power Reception Interrupt Enable."]
    pub const fn set_tolprxie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timeout Low-Power Reception Interrupt Enable."]
    pub const fn tolprxie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ECC Single-bit Error Interrupt Enable."]
    pub const fn set_eccseie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ECC Single-bit Error Interrupt Enable."]
    pub const fn eccseie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ECC Multi-bit Error Interrupt Enable."]
    pub const fn set_eccmeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ECC Multi-bit Error Interrupt Enable."]
    pub const fn eccmeie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CRC Error Interrupt Enable."]
    pub const fn set_crceie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CRC Error Interrupt Enable."]
    pub const fn crceie(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Packet Size Error Interrupt Enable."]
    pub const fn set_pseie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Packet Size Error Interrupt Enable."]
    pub const fn pseie(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EoTp Error Interrupt Enable."]
    pub const fn set_eotpeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EoTp Error Interrupt Enable."]
    pub const fn eotpeie(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LTDC Payload Write Error Interrupt Enable."]
    pub const fn set_lpwreie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LTDC Payload Write Error Interrupt Enable."]
    pub const fn lpwreie(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Command Write Error Interrupt Enable."]
    pub const fn set_gcwreie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Command Write Error Interrupt Enable."]
    pub const fn gcwreie(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Payload Write Error Interrupt Enable."]
    pub const fn set_gpwreie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Payload Write Error Interrupt Enable."]
    pub const fn gpwreie(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Payload Transmit Error Interrupt Enable."]
    pub const fn set_gptxeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Payload Transmit Error Interrupt Enable."]
    pub const fn gptxeie(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Payload Read Error Interrupt Enable."]
    pub const fn set_gprdeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Payload Read Error Interrupt Enable."]
    pub const fn gprdeie(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Payload Receive Error Interrupt Enable."]
    pub const fn set_gprxeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Payload Receive Error Interrupt Enable."]
    pub const fn gprxeie(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Interrupt & Status Register 0."]
pub struct Isr0Bits {
    bits: u32,
}
impl Default for Isr0Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Isr0Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 0."]
    pub const fn set_ae0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 0."]
    pub const fn ae0(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 1."]
    pub const fn set_ae1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 1."]
    pub const fn ae1(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 2."]
    pub const fn set_ae2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 2."]
    pub const fn ae2(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 3."]
    pub const fn set_ae3(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 3."]
    pub const fn ae3(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 4."]
    pub const fn set_ae4(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 4."]
    pub const fn ae4(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 5."]
    pub const fn set_ae5(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 5."]
    pub const fn ae5(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 6."]
    pub const fn set_ae6(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 6."]
    pub const fn ae6(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 7."]
    pub const fn set_ae7(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 7."]
    pub const fn ae7(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 8."]
    pub const fn set_ae8(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 8."]
    pub const fn ae8(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 9."]
    pub const fn set_ae9(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 9."]
    pub const fn ae9(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 10."]
    pub const fn set_ae10(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 10."]
    pub const fn ae10(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 11."]
    pub const fn set_ae11(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 11."]
    pub const fn ae11(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 12."]
    pub const fn set_ae12(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 12."]
    pub const fn ae12(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 13."]
    pub const fn set_ae13(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 13."]
    pub const fn ae13(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 14."]
    pub const fn set_ae14(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 14."]
    pub const fn ae14(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 15."]
    pub const fn set_ae15(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Acknowledge Error 15."]
    pub const fn ae15(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Error 0."]
    pub const fn set_pe0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Error 0."]
    pub const fn pe0(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Error 1."]
    pub const fn set_pe1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Error 1."]
    pub const fn pe1(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Error 2."]
    pub const fn set_pe2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Error 2."]
    pub const fn pe2(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Error 3."]
    pub const fn set_pe3(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Error 3."]
    pub const fn pe3(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Error 4."]
    pub const fn set_pe4(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Error 4."]
    pub const fn pe4(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Interrupt & Status Register 1."]
pub struct Isr1Bits {
    bits: u32,
}
impl Default for Isr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Isr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Timeout High-Speed Transmission."]
    pub const fn set_tohstx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timeout High-Speed Transmission."]
    pub const fn tohstx(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timeout Low-Power Reception."]
    pub const fn set_tolprx(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timeout Low-Power Reception."]
    pub const fn tolprx(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ECC Single-bit Error."]
    pub const fn set_eccse(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ECC Single-bit Error."]
    pub const fn eccse(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ECC Multi-bit Error."]
    pub const fn set_eccme(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ECC Multi-bit Error."]
    pub const fn eccme(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CRC Error."]
    pub const fn set_crce(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CRC Error."]
    pub const fn crce(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Packet Size Error."]
    pub const fn set_pse(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Packet Size Error."]
    pub const fn pse(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EoTp Error."]
    pub const fn set_eotpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EoTp Error."]
    pub const fn eotpe(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LTDC Payload Write Error."]
    pub const fn set_lpwre(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LTDC Payload Write Error."]
    pub const fn lpwre(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Command Write Error."]
    pub const fn set_gcwre(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Command Write Error."]
    pub const fn gcwre(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Payload Write Error."]
    pub const fn set_gpwre(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Payload Write Error."]
    pub const fn gpwre(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Payload Transmit Error."]
    pub const fn set_gptxe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Payload Transmit Error."]
    pub const fn gptxe(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Payload Read Error."]
    pub const fn set_gprde(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Payload Read Error."]
    pub const fn gprde(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Generic Payload Receive Error."]
    pub const fn set_gprxe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Generic Payload Receive Error."]
    pub const fn gprxe(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host LTDC Current Color Coding Register."]
pub struct LcccrBits {
    bits: u32,
}
impl Default for LcccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LcccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Color Coding."]
    pub const fn set_colc(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Color Coding."]
    pub const fn colc(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Loosely Packed Enable."]
    pub const fn set_lpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Loosely Packed Enable."]
    pub const fn lpe(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host LTDC Command Configuration Register."]
pub struct LccrBits {
    bits: u32,
}
impl Default for LccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Command Size."]
    pub const fn set_cmdsize(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Command Size."]
    pub const fn cmdsize(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host LTDC Color Coding Register."]
pub struct LcolcrBits {
    bits: u32,
}
impl Default for LcolcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LcolcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Color Coding."]
    pub const fn set_colc(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Color Coding."]
    pub const fn colc(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Loosely Packet Enable."]
    pub const fn set_lpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Loosely Packet Enable."]
    pub const fn lpe(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host LTDC Current VCID Register."]
pub struct LcvcidrBits {
    bits: u32,
}
impl Default for LcvcidrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LcvcidrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Virtual Channel ID."]
    pub const fn set_vcid(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Virtual Channel ID."]
    pub const fn vcid(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host LTDC Polarity Configuration Register."]
pub struct LpcrBits {
    bits: u32,
}
impl Default for LpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Data Enable Polarity."]
    pub const fn set_dep(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data Enable Polarity."]
    pub const fn dep(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VSYNC Polarity."]
    pub const fn set_vsp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VSYNC Polarity."]
    pub const fn vsp(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HSYNC Polarity."]
    pub const fn set_hsp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HSYNC Polarity."]
    pub const fn hsp(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Low-Power mode Current Configuration Register."]
pub struct LpmccrBits {
    bits: u32,
}
impl Default for LpmccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LpmccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "VACT Largest Packet Size."]
    pub const fn set_vlpsize(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "VACT Largest Packet Size."]
    pub const fn vlpsize(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Largest Packet Size."]
    pub const fn set_lpsize(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Largest Packet Size."]
    pub const fn lpsize(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Low-Power mode Configuration Register."]
pub struct LpmcrBits {
    bits: u32,
}
impl Default for LpmcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LpmcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "VACT Largest Packet Size."]
    pub const fn set_vlpsize(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "VACT Largest Packet Size."]
    pub const fn vlpsize(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Largest Packet Size."]
    pub const fn set_lpsize(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Largest Packet Size."]
    pub const fn lpsize(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host LTDC VCID Register."]
pub struct LvcidrBits {
    bits: u32,
}
impl Default for LvcidrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl LvcidrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Virtual Channel ID."]
    pub const fn set_vcid(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Virtual Channel ID."]
    pub const fn vcid(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host mode Configuration Register."]
pub struct McrBits {
    bits: u32,
}
impl Default for McrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl McrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Command mode."]
    pub const fn set_cmdm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Command mode."]
    pub const fn cmdm(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host PHY Configuration Register."]
pub struct PconfrBits {
    bits: u32,
}
impl Default for PconfrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PconfrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Number of Lanes."]
    pub const fn set_nl(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Number of Lanes."]
    pub const fn nl(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Stop Wait Time."]
    pub const fn set_sw_time(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Stop Wait Time."]
    pub const fn sw_time(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Protocol Configuration Register."]
pub struct PcrBits {
    bits: u32,
}
impl Default for PcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "EoTp Transmission Enable."]
    pub const fn set_ettxe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EoTp Transmission Enable."]
    pub const fn ettxe(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EoTp Reception Enable."]
    pub const fn set_etrxe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EoTp Reception Enable."]
    pub const fn etrxe(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Bus Turn Around Enable."]
    pub const fn set_btae(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Bus Turn Around Enable."]
    pub const fn btae(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ECC Reception Enable."]
    pub const fn set_eccrxe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ECC Reception Enable."]
    pub const fn eccrxe(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CRC Reception Enable."]
    pub const fn set_crcrxe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CRC Reception Enable."]
    pub const fn crcrxe(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host PHY Control Register."]
pub struct PctlrBits {
    bits: u32,
}
impl Default for PctlrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PctlrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Digital Enable."]
    pub const fn set_den(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Digital Enable."]
    pub const fn den(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clock Enable."]
    pub const fn set_cke(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clock Enable."]
    pub const fn cke(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host PHY Status Register."]
pub struct PsrBits {
    bits: u32,
}
impl Default for PsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "PHY Direction."]
    pub const fn set_pd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Direction."]
    pub const fn pd(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Stop State Clock lane."]
    pub const fn set_pssc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Stop State Clock lane."]
    pub const fn pssc(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPS Active Not Clock lane."]
    pub const fn set_uanc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPS Active Not Clock lane."]
    pub const fn uanc(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Stop State lane 0."]
    pub const fn set_pss0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Stop State lane 0."]
    pub const fn pss0(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPS Active Not lane 1."]
    pub const fn set_uan0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPS Active Not lane 1."]
    pub const fn uan0(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "RX ULPS Escape lane 0."]
    pub const fn set_rue0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RX ULPS Escape lane 0."]
    pub const fn rue0(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Stop State lane 1."]
    pub const fn set_pss1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Stop State lane 1."]
    pub const fn pss1(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPS Active Not lane 1."]
    pub const fn set_uan1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPS Active Not lane 1."]
    pub const fn uan1(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host PHY TX Triggers Configuration Register."]
pub struct PttcrBits {
    bits: u32,
}
impl Default for PttcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PttcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transmission Trigger."]
    pub const fn set_tx_trig(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Transmission Trigger."]
    pub const fn tx_trig(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host PHY ULPS Control Register."]
pub struct PucrBits {
    bits: u32,
}
impl Default for PucrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PucrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "ULPS Request on Clock Lane."]
    pub const fn set_urcl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPS Request on Clock Lane."]
    pub const fn urcl(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPS Exit on Clock Lane."]
    pub const fn set_uecl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPS Exit on Clock Lane."]
    pub const fn uecl(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPS Request on Data Lane."]
    pub const fn set_urdl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPS Request on Data Lane."]
    pub const fn urdl(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPS Exit on Data Lane."]
    pub const fn set_uedl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPS Exit on Data Lane."]
    pub const fn uedl(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Timeout Counter Configuration Register 0."]
pub struct Tccr0Bits {
    bits: u32,
}
impl Default for Tccr0Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Tccr0Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Low-power Reception Timeout Counter."]
    pub const fn set_lprx_tocnt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Low-power Reception Timeout Counter."]
    pub const fn lprx_tocnt(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "High-Speed Transmission Timeout Counter."]
    pub const fn set_hstx_tocnt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "High-Speed Transmission Timeout Counter."]
    pub const fn hstx_tocnt(self) -> u16 {
        ((self.bits >> 0x10) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Timeout Counter Configuration Register 1."]
pub struct Tccr1Bits {
    bits: u32,
}
impl Default for Tccr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Tccr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "High-Speed Read Timeout Counter."]
    pub const fn set_hsrd_tocnt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "High-Speed Read Timeout Counter."]
    pub const fn hsrd_tocnt(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Timeout Counter Configuration Register 2."]
pub struct Tccr2Bits {
    bits: u32,
}
impl Default for Tccr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Tccr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Low-Power Read Timeout Counter."]
    pub const fn set_lprd_tocnt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Read Timeout Counter."]
    pub const fn lprd_tocnt(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Timeout Counter Configuration Register 3."]
pub struct Tccr3Bits {
    bits: u32,
}
impl Default for Tccr3Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Tccr3Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "High-Speed Write Timeout Counter."]
    pub const fn set_hswr_tocnt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "High-Speed Write Timeout Counter."]
    pub const fn hswr_tocnt(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Presp mode."]
    pub const fn set_pm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Presp mode."]
    pub const fn pm(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Timeout Counter Configuration Register 4."]
pub struct Tccr4Bits {
    bits: u32,
}
impl Default for Tccr4Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Tccr4Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Low-Power Write Timeout Counter."]
    pub const fn set_lswr_tocnt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Write Timeout Counter."]
    pub const fn lswr_tocnt(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Timeout Counter Configuration Register 5."]
pub struct Tccr5Bits {
    bits: u32,
}
impl Default for Tccr5Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Tccr5Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Bus-Turn-Around Timeout Counter."]
    pub const fn set_bta_tocnt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Bus-Turn-Around Timeout Counter."]
    pub const fn bta_tocnt(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video Chunks Current Configuration Register."]
pub struct VcccrBits {
    bits: u32,
}
impl Default for VcccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VcccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Number of Chunks."]
    pub const fn set_numc(mut self, val: u16) -> Self {
        self.bits &= !(0x1fff << 0x0);
        self.bits |= (val as u32 & 0x1fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Number of Chunks."]
    pub const fn numc(self) -> u16 {
        ((self.bits >> 0x0) & 0x1fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video Chunks Configuration Register."]
pub struct VccrBits {
    bits: u32,
}
impl Default for VccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Number of Chunks."]
    pub const fn set_numc(mut self, val: u16) -> Self {
        self.bits &= !(0x1fff << 0x0);
        self.bits |= (val as u32 & 0x1fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Number of Chunks."]
    pub const fn numc(self) -> u16 {
        ((self.bits >> 0x0) & 0x1fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video HBP Current Configuration Register."]
pub struct VhbpccrBits {
    bits: u32,
}
impl Default for VhbpccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VhbpccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Horizontal Back-Porch duration."]
    pub const fn set_hbp(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Horizontal Back-Porch duration."]
    pub const fn hbp(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video HBP Configuration Register."]
pub struct VhbpcrBits {
    bits: u32,
}
impl Default for VhbpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VhbpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Horizontal Back-Porch duration."]
    pub const fn set_hbp(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Horizontal Back-Porch duration."]
    pub const fn hbp(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video HSA Current Configuration Register."]
pub struct VhsaccrBits {
    bits: u32,
}
impl Default for VhsaccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VhsaccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Horizontal Synchronism Active duration."]
    pub const fn set_hsa(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Horizontal Synchronism Active duration."]
    pub const fn hsa(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video HSA Configuration Register."]
pub struct VhsacrBits {
    bits: u32,
}
impl Default for VhsacrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VhsacrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Horizontal Synchronism Active duration."]
    pub const fn set_hsa(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Horizontal Synchronism Active duration."]
    pub const fn hsa(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video Line Current Configuration Register."]
pub struct VlccrBits {
    bits: u32,
}
impl Default for VlccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VlccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Horizontal Line duration."]
    pub const fn set_hline(mut self, val: u16) -> Self {
        self.bits &= !(0x7fff << 0x0);
        self.bits |= (val as u32 & 0x7fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Horizontal Line duration."]
    pub const fn hline(self) -> u16 {
        ((self.bits >> 0x0) & 0x7fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video Line Configuration Register."]
pub struct VlcrBits {
    bits: u32,
}
impl Default for VlcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VlcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Horizontal Line duration."]
    pub const fn set_hline(mut self, val: u16) -> Self {
        self.bits &= !(0x7fff << 0x0);
        self.bits |= (val as u32 & 0x7fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Horizontal Line duration."]
    pub const fn hline(self) -> u16 {
        ((self.bits >> 0x0) & 0x7fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video mode Current Configuration Register."]
pub struct VmccrBits {
    bits: u32,
}
impl Default for VmccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VmccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Video mode Type."]
    pub const fn set_vmt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Video mode Type."]
    pub const fn vmt(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Low-Power Vertical Sync time Enable."]
    pub const fn set_lpvsae(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Vertical Sync time Enable."]
    pub const fn lpvsae(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-power Vertical Back-Porch Enable."]
    pub const fn set_lpvbpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-power Vertical Back-Porch Enable."]
    pub const fn lpvbpe(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-power Vertical Front-Porch Enable."]
    pub const fn set_lpvfpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-power Vertical Front-Porch Enable."]
    pub const fn lpvfpe(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-Power Vertical Active Enable."]
    pub const fn set_lpvae(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Vertical Active Enable."]
    pub const fn lpvae(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-power Horizontal Back-Porch Enable."]
    pub const fn set_lphbpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-power Horizontal Back-Porch Enable."]
    pub const fn lphbpe(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-Power Horizontal Front-Porch Enable."]
    pub const fn set_lphfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Horizontal Front-Porch Enable."]
    pub const fn lphfe(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Frame BTA Acknowledge Enable."]
    pub const fn set_fbtaae(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Frame BTA Acknowledge Enable."]
    pub const fn fbtaae(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-Power Command Enable."]
    pub const fn set_lpce(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Command Enable."]
    pub const fn lpce(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video mode Configuration Register."]
pub struct VmcrBits {
    bits: u32,
}
impl Default for VmcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VmcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Video mode Type."]
    pub const fn set_vmt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Video mode Type."]
    pub const fn vmt(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Low-Power Vertical Sync Active Enable."]
    pub const fn set_lpvsae(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Vertical Sync Active Enable."]
    pub const fn lpvsae(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-power Vertical Back-Porch Enable."]
    pub const fn set_lpvbpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-power Vertical Back-Porch Enable."]
    pub const fn lpvbpe(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-power Vertical Front-porch Enable."]
    pub const fn set_lpvfpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-power Vertical Front-porch Enable."]
    pub const fn lpvfpe(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-Power Vertical Active Enable."]
    pub const fn set_lpvae(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Vertical Active Enable."]
    pub const fn lpvae(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-Power Horizontal Back-Porch Enable."]
    pub const fn set_lphbpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Horizontal Back-Porch Enable."]
    pub const fn lphbpe(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-Power Horizontal Front-Porch Enable."]
    pub const fn set_lphfpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Horizontal Front-Porch Enable."]
    pub const fn lphfpe(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Frame Bus-Turn-Around Acknowledge Enable."]
    pub const fn set_fbtaae(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Frame Bus-Turn-Around Acknowledge Enable."]
    pub const fn fbtaae(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-Power Command Enable."]
    pub const fn set_lpce(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-Power Command Enable."]
    pub const fn lpce(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Pattern Generator Enable."]
    pub const fn set_pge(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Pattern Generator Enable."]
    pub const fn pge(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Pattern Generator mode."]
    pub const fn set_pgm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Pattern Generator mode."]
    pub const fn pgm(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Pattern Generator Orientation."]
    pub const fn set_pgo(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Pattern Generator Orientation."]
    pub const fn pgo(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video Null Packet Current Configuration Register."]
pub struct VnpccrBits {
    bits: u32,
}
impl Default for VnpccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VnpccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Null Packet Size."]
    pub const fn set_npsize(mut self, val: u16) -> Self {
        self.bits &= !(0x1fff << 0x0);
        self.bits |= (val as u32 & 0x1fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Null Packet Size."]
    pub const fn npsize(self) -> u16 {
        ((self.bits >> 0x0) & 0x1fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video Null Packet Configuration Register."]
pub struct VnpcrBits {
    bits: u32,
}
impl Default for VnpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VnpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Null Packet Size."]
    pub const fn set_npsize(mut self, val: u16) -> Self {
        self.bits &= !(0x1fff << 0x0);
        self.bits |= (val as u32 & 0x1fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Null Packet Size."]
    pub const fn npsize(self) -> u16 {
        ((self.bits >> 0x0) & 0x1fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video Packet Current Configuration Register."]
pub struct VpccrBits {
    bits: u32,
}
impl Default for VpccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VpccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Video Packet Size."]
    pub const fn set_vpsize(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x0);
        self.bits |= (val as u32 & 0x3fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Video Packet Size."]
    pub const fn vpsize(self) -> u16 {
        ((self.bits >> 0x0) & 0x3fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video Packet Configuration Register."]
pub struct VpcrBits {
    bits: u32,
}
impl Default for VpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Video Packet Size."]
    pub const fn set_vpsize(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x0);
        self.bits |= (val as u32 & 0x3fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Video Packet Size."]
    pub const fn vpsize(self) -> u16 {
        ((self.bits >> 0x0) & 0x3fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Version Register."]
pub struct VrBits {
    bits: u32,
}
impl Default for VrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Version of the DSI Host."]
    pub const fn set_version(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Version of the DSI Host."]
    pub const fn version(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video Shadow Control Register."]
pub struct VscrBits {
    bits: u32,
}
impl Default for VscrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VscrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Enable."]
    pub const fn set_en(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable."]
    pub const fn en(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Update Register."]
    pub const fn set_ur(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Update Register."]
    pub const fn ur(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video VA Current Configuration Register."]
pub struct VvaccrBits {
    bits: u32,
}
impl Default for VvaccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VvaccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Vertical Active duration."]
    pub const fn set_va(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x0);
        self.bits |= (val as u32 & 0x3fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Active duration."]
    pub const fn va(self) -> u16 {
        ((self.bits >> 0x0) & 0x3fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video VA Configuration Register."]
pub struct VvacrBits {
    bits: u32,
}
impl Default for VvacrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VvacrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Vertical Active duration."]
    pub const fn set_va(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x0);
        self.bits |= (val as u32 & 0x3fff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Active duration."]
    pub const fn va(self) -> u16 {
        ((self.bits >> 0x0) & 0x3fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video VBP Current Configuration Register."]
pub struct VvbpccrBits {
    bits: u32,
}
impl Default for VvbpccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VvbpccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Vertical Back-Porch duration."]
    pub const fn set_vbp(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x0);
        self.bits |= (val as u32 & 0x3ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Back-Porch duration."]
    pub const fn vbp(self) -> u16 {
        ((self.bits >> 0x0) & 0x3ff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video VBP Configuration Register."]
pub struct VvbpcrBits {
    bits: u32,
}
impl Default for VvbpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VvbpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Vertical Back-Porch duration."]
    pub const fn set_vbp(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x0);
        self.bits |= (val as u32 & 0x3ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Back-Porch duration."]
    pub const fn vbp(self) -> u16 {
        ((self.bits >> 0x0) & 0x3ff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video VFP Current Configuration Register."]
pub struct VvfpccrBits {
    bits: u32,
}
impl Default for VvfpccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VvfpccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Vertical Front-Porch duration."]
    pub const fn set_vfp(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x0);
        self.bits |= (val as u32 & 0x3ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Front-Porch duration."]
    pub const fn vfp(self) -> u16 {
        ((self.bits >> 0x0) & 0x3ff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video VFP Configuration Register."]
pub struct VvfpcrBits {
    bits: u32,
}
impl Default for VvfpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VvfpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Vertical Front-Porch duration."]
    pub const fn set_vfp(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x0);
        self.bits |= (val as u32 & 0x3ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Front-Porch duration."]
    pub const fn vfp(self) -> u16 {
        ((self.bits >> 0x0) & 0x3ff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video VSA Current Configuration Register."]
pub struct VvsaccrBits {
    bits: u32,
}
impl Default for VvsaccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VvsaccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Vertical Synchronism Active duration."]
    pub const fn set_vsa(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x0);
        self.bits |= (val as u32 & 0x3ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Synchronism Active duration."]
    pub const fn vsa(self) -> u16 {
        ((self.bits >> 0x0) & 0x3ff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Host Video VSA Configuration Register."]
pub struct VvsacrBits {
    bits: u32,
}
impl Default for VvsacrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl VvsacrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Vertical Synchronism Active duration."]
    pub const fn set_vsa(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x0);
        self.bits |= (val as u32 & 0x3ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Vertical Synchronism Active duration."]
    pub const fn vsa(self) -> u16 {
        ((self.bits >> 0x0) & 0x3ff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper Configuration Register."]
pub struct WcfgrBits {
    bits: u32,
}
impl Default for WcfgrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WcfgrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DSI Mode."]
    pub const fn set_dsim(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DSI Mode."]
    pub const fn dsim(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Color Multiplexing."]
    pub const fn set_colmux(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x1);
        self.bits |= (val as u32 & 0x7) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Color Multiplexing."]
    pub const fn colmux(self) -> u8 {
        ((self.bits >> 0x1) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "TE Source."]
    pub const fn set_tesrc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TE Source."]
    pub const fn tesrc(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TE Polarity."]
    pub const fn set_tepol(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TE Polarity."]
    pub const fn tepol(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Automatic Refresh."]
    pub const fn set_ar(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Automatic Refresh."]
    pub const fn ar(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VSync Polarity."]
    pub const fn set_vspol(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VSync Polarity."]
    pub const fn vspol(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper Control Register."]
pub struct WcrBits {
    bits: u32,
}
impl Default for WcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Color Mode."]
    pub const fn set_colm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Color Mode."]
    pub const fn colm(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Shutdown."]
    pub const fn set_shtdn(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Shutdown."]
    pub const fn shtdn(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LTDC Enable."]
    pub const fn set_ltdcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LTDC Enable."]
    pub const fn ltdcen(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DSI Enable."]
    pub const fn set_dsien(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DSI Enable."]
    pub const fn dsien(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper Interrupt Enable Register."]
pub struct WierBits {
    bits: u32,
}
impl Default for WierBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WierBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Tearing Effect Interrupt Enable."]
    pub const fn set_teie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tearing Effect Interrupt Enable."]
    pub const fn teie(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "End of Refresh Interrupt Enable."]
    pub const fn set_erie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "End of Refresh Interrupt Enable."]
    pub const fn erie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLL Lock Interrupt Enable."]
    pub const fn set_plllie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLL Lock Interrupt Enable."]
    pub const fn plllie(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLL Unlock Interrupt Enable."]
    pub const fn set_plluie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLL Unlock Interrupt Enable."]
    pub const fn plluie(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Regulator Ready Interrupt Enable."]
    pub const fn set_rrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Regulator Ready Interrupt Enable."]
    pub const fn rrie(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper Interrupt Flag Clear Register."]
pub struct WifcrBits {
    bits: u32,
}
impl Default for WifcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WifcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clear Tearing Effect Interrupt Flag."]
    pub const fn set_cteif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear Tearing Effect Interrupt Flag."]
    pub const fn cteif(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear End of Refresh Interrupt Flag."]
    pub const fn set_cerif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear End of Refresh Interrupt Flag."]
    pub const fn cerif(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear PLL Lock Interrupt Flag."]
    pub const fn set_cplllif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear PLL Lock Interrupt Flag."]
    pub const fn cplllif(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear PLL Unlock Interrupt Flag."]
    pub const fn set_cplluif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear PLL Unlock Interrupt Flag."]
    pub const fn cplluif(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear Regulator Ready Interrupt Flag."]
    pub const fn set_crrif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear Regulator Ready Interrupt Flag."]
    pub const fn crrif(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper Interrupt & Status Register."]
pub struct WisrBits {
    bits: u32,
}
impl Default for WisrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WisrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Tearing Effect Interrupt Flag."]
    pub const fn set_teif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tearing Effect Interrupt Flag."]
    pub const fn teif(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "End of Refresh Interrupt Flag."]
    pub const fn set_erif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "End of Refresh Interrupt Flag."]
    pub const fn erif(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Busy Flag."]
    pub const fn set_busy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Busy Flag."]
    pub const fn busy(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLL Lock Status."]
    pub const fn set_pllls(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLL Lock Status."]
    pub const fn pllls(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLL Lock Interrupt Flag."]
    pub const fn set_plllif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLL Lock Interrupt Flag."]
    pub const fn plllif(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLL Unlock Interrupt Flag."]
    pub const fn set_plluif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLL Unlock Interrupt Flag."]
    pub const fn plluif(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Regulator Ready Status."]
    pub const fn set_rrs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Regulator Ready Status."]
    pub const fn rrs(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Regulator Ready Interrupt Flag."]
    pub const fn set_rrif(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Regulator Ready Interrupt Flag."]
    pub const fn rrif(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper PHY Configuration Register 0."]
pub struct Wpcr0Bits {
    bits: u32,
}
impl Default for Wpcr0Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Wpcr0Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Unit Interval multiplied by 4."]
    pub const fn set_uix4(mut self, val: u8) -> Self {
        self.bits &= !(0x3f << 0x0);
        self.bits |= (val as u32 & 0x3f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Unit Interval multiplied by 4."]
    pub const fn uix4(self) -> u8 {
        ((self.bits >> 0x0) & 0x3f) as _
    }
    #[inline(always)]
    #[doc = "Swap Clock Lane pins."]
    pub const fn set_swcl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Swap Clock Lane pins."]
    pub const fn swcl(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Swap Data Lane 0 pins."]
    pub const fn set_swdl0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Swap Data Lane 0 pins."]
    pub const fn swdl0(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Swap Data Lane 1 pins."]
    pub const fn set_swdl1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Swap Data Lane 1 pins."]
    pub const fn swdl1(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Invert Hight-Speed data signal on Clock Lane."]
    pub const fn set_hsicl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Invert Hight-Speed data signal on Clock Lane."]
    pub const fn hsicl(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Invert the Hight-Speed data signal on Data Lane 0."]
    pub const fn set_hsidl0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Invert the Hight-Speed data signal on Data Lane 0."]
    pub const fn hsidl0(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Invert the High-Speed data signal on Data Lane 1."]
    pub const fn set_hsidl1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Invert the High-Speed data signal on Data Lane 1."]
    pub const fn hsidl1(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force in TX Stop Mode the Clock Lane."]
    pub const fn set_ftxsmcl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force in TX Stop Mode the Clock Lane."]
    pub const fn ftxsmcl(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force in TX Stop Mode the Data Lanes."]
    pub const fn set_ftxsmdl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force in TX Stop Mode the Data Lanes."]
    pub const fn ftxsmdl(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Contention Detection OFF on Data Lanes."]
    pub const fn set_cdoffdl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Contention Detection OFF on Data Lanes."]
    pub const fn cdoffdl(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Turn Disable Data Lanes."]
    pub const fn set_tddl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Turn Disable Data Lanes."]
    pub const fn tddl(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Pull-Down Enable."]
    pub const fn set_pden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Pull-Down Enable."]
    pub const fn pden(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "custom time for tCLK-PREPARE Enable."]
    pub const fn set_tclkprepen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "custom time for tCLK-PREPARE Enable."]
    pub const fn tclkprepen(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "custom time for tCLK-ZERO Enable."]
    pub const fn set_tclkzeroen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "custom time for tCLK-ZERO Enable."]
    pub const fn tclkzeroen(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "custom time for tHS-PREPARE Enable."]
    pub const fn set_thsprepen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "custom time for tHS-PREPARE Enable."]
    pub const fn thsprepen(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "custom time for tHS-TRAIL Enable."]
    pub const fn set_thstrailen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "custom time for tHS-TRAIL Enable."]
    pub const fn thstrailen(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "custom time for tHS-ZERO Enable."]
    pub const fn set_thszeroen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "custom time for tHS-ZERO Enable."]
    pub const fn thszeroen(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "custom time for tLPX for Data lanes Enable."]
    pub const fn set_tlpxden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "custom time for tLPX for Data lanes Enable."]
    pub const fn tlpxden(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "custom time for tHS-EXIT Enable."]
    pub const fn set_thsexiten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "custom time for tHS-EXIT Enable."]
    pub const fn thsexiten(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "custom time for tLPX for Clock lane Enable."]
    pub const fn set_tlpxcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "custom time for tLPX for Clock lane Enable."]
    pub const fn tlpxcen(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "custom time for tCLK-POST Enable."]
    pub const fn set_tclkposten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "custom time for tCLK-POST Enable."]
    pub const fn tclkposten(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper PHY Configuration Register 1."]
pub struct Wpcr1Bits {
    bits: u32,
}
impl Default for Wpcr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Wpcr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "High-Speed Transmission Delay on Clock Lane."]
    pub const fn set_hstxdcl(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "High-Speed Transmission Delay on Clock Lane."]
    pub const fn hstxdcl(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "High-Speed Transmission Delay on Data Lanes."]
    pub const fn set_hstxdll(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x2);
        self.bits |= (val as u32 & 0x3) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "High-Speed Transmission Delay on Data Lanes."]
    pub const fn hstxdll(self) -> u8 {
        ((self.bits >> 0x2) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Low-Power transmission Slew Rate Compensation on Clock Lane."]
    pub const fn set_lpsrcl(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x6);
        self.bits |= (val as u32 & 0x3) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Low-Power transmission Slew Rate Compensation on Clock Lane."]
    pub const fn lpsrcl(self) -> u8 {
        ((self.bits >> 0x6) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Low-Power transmission Slew Rate Compensation on Data Lanes."]
    pub const fn set_lpsrdl(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Low-Power transmission Slew Rate Compensation on Data Lanes."]
    pub const fn lpsrdl(self) -> u8 {
        ((self.bits >> 0x8) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "SDD Control."]
    pub const fn set_sdcc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SDD Control."]
    pub const fn sdcc(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "High-Speed Transmission Slew Rate Control on Clock Lane."]
    pub const fn set_hstxsrccl(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "High-Speed Transmission Slew Rate Control on Clock Lane."]
    pub const fn hstxsrccl(self) -> u8 {
        ((self.bits >> 0x10) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "High-Speed Transmission Slew Rate Control on Data Lanes."]
    pub const fn set_hstxsrcdl(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x12);
        self.bits |= (val as u32 & 0x3) << 0x12;
        self
    }
    #[inline(always)]
    #[doc = "High-Speed Transmission Slew Rate Control on Data Lanes."]
    pub const fn hstxsrcdl(self) -> u8 {
        ((self.bits >> 0x12) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Forces LP Receiver in Low-Power Mode."]
    pub const fn set_flprxlpm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Forces LP Receiver in Low-Power Mode."]
    pub const fn flprxlpm(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-Power RX low-pass Filtering Tuning."]
    pub const fn set_lprxft(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x19);
        self.bits |= (val as u32 & 0x3) << 0x19;
        self
    }
    #[inline(always)]
    #[doc = "Low-Power RX low-pass Filtering Tuning."]
    pub const fn lprxft(self) -> u8 {
        ((self.bits >> 0x19) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper PHY Configuration Register 2."]
pub struct Wpcr2Bits {
    bits: u32,
}
impl Default for Wpcr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Wpcr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "tCLK-PREPARE."]
    pub const fn set_tclkprep(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "tCLK-PREPARE."]
    pub const fn tclkprep(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "tCLK-ZERO."]
    pub const fn set_tclkzeo(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "tCLK-ZERO."]
    pub const fn tclkzeo(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "tHS-PREPARE."]
    pub const fn set_thsprep(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "tHS-PREPARE."]
    pub const fn thsprep(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "tHSTRAIL."]
    pub const fn set_thstrail(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "tHSTRAIL."]
    pub const fn thstrail(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper PHY Configuration Register 3."]
pub struct Wpcr3Bits {
    bits: u32,
}
impl Default for Wpcr3Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Wpcr3Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "tHS-ZERO."]
    pub const fn set_thszero(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "tHS-ZERO."]
    pub const fn thszero(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "tLPX for Data lanes."]
    pub const fn set_tlpxd(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "tLPX for Data lanes."]
    pub const fn tlpxd(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "tHSEXIT."]
    pub const fn set_thsexit(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "tHSEXIT."]
    pub const fn thsexit(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "tLPXC for Clock lane."]
    pub const fn set_tlpxc(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "tLPXC for Clock lane."]
    pub const fn tlpxc(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper PHY Configuration Register 4."]
pub struct Wpcr4Bits {
    bits: u32,
}
impl Default for Wpcr4Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Wpcr4Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "tCLK-POST."]
    pub const fn set_tclkpost(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "tCLK-POST."]
    pub const fn tclkpost(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "DSI Wrapper Regulator and PLL Control Register."]
pub struct WrpcrBits {
    bits: u32,
}
impl Default for WrpcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl WrpcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "PLL Enable."]
    pub const fn set_pllen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PLL Enable."]
    pub const fn pllen(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PLL Loop Division Factor."]
    pub const fn set_ndiv(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x2);
        self.bits |= (val as u32 & 0x7f) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "PLL Loop Division Factor."]
    pub const fn ndiv(self) -> u8 {
        ((self.bits >> 0x2) & 0x7f) as _
    }
    #[inline(always)]
    #[doc = "PLL Input Division Factor."]
    pub const fn set_idf(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0xb);
        self.bits |= (val as u32 & 0xf) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "PLL Input Division Factor."]
    pub const fn idf(self) -> u8 {
        ((self.bits >> 0xb) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "PLL Output Division Factor."]
    pub const fn set_odf(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "PLL Output Division Factor."]
    pub const fn odf(self) -> u8 {
        ((self.bits >> 0x10) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Regulator Enable."]
    pub const fn set_regen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Regulator Enable."]
    pub const fn regen(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
}
