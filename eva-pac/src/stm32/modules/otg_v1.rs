
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "USB OTG core by Synopsys (more docs at <https://www.intel.com/content/www/us/en/programmable/hps/agilex5/index_frames.html>)"]
pub struct Otg {
    ptr: *mut u8,
}
impl Otg {
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
    #[doc = "Control and status register"]
    pub const fn gotgctl(&self) -> utils::Reg<GotgctlBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<GotgctlBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt register"]
    pub const fn gotgint(&self) -> utils::Reg<GotgintBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<GotgintBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB configuration register"]
    pub const fn gahbcfg(&self) -> utils::Reg<GahbcfgBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<GahbcfgBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "USB configuration register"]
    pub const fn gusbcfg(&self) -> utils::Reg<GusbcfgBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<GusbcfgBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Reset register"]
    pub const fn grstctl(&self) -> utils::Reg<GrstctlBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<GrstctlBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Core interrupt register"]
    pub const fn gintsts(&self) -> utils::Reg<GintstsBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<GintstsBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt mask register"]
    pub const fn gintmsk(&self) -> utils::Reg<GintmskBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<GintmskBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Receive status debug read register"]
    pub const fn grxstsr(&self) -> utils::Reg<GrxstsBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<GrxstsBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Status read and pop register"]
    pub const fn grxstsp(&self) -> utils::Reg<GrxstsBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<GrxstsBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Receive FIFO size register"]
    pub const fn grxfsiz(&self) -> utils::Reg<GrxfsizBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<GrxfsizBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Endpoint 0 transmit FIFO size register (device mode)"]
    pub const fn dieptxf0(&self) -> utils::Reg<FsizBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<FsizBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Non-periodic transmit FIFO size register (host mode)"]
    pub const fn hnptxfsiz(&self) -> utils::Reg<FsizBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<FsizBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Non-periodic transmit FIFO/queue status register (host mode)"]
    pub const fn hnptxsts(&self) -> utils::Reg<HnptxstsBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<HnptxstsBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "OTG I2C access register"]
    pub const fn gi2cctl(&self) -> utils::Reg<Gi2cctlBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<Gi2cctlBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "General core configuration register, for core_id 0x0000_1xxx"]
    pub const fn gccfg_v1(&self) -> utils::Reg<GccfgV1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<GccfgV1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "General core configuration register, for core_id 0x0000_[23]xxx"]
    pub const fn gccfg_v2(&self) -> utils::Reg<GccfgV2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<GccfgV2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "General core configuration register, for core_id 0x0000_5xxx"]
    pub const fn gccfg_v3(&self) -> utils::Reg<GccfgV3Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<GccfgV3Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Core ID register"]
    pub const fn cid(&self) -> utils::Reg<CidBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c);
            <utils::Reg<CidBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Synopsis ID Register"]
    pub const fn snpsid(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "User HW Config 1 Register"]
    pub const fn hwcfg1(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "User HW Config 2 Register"]
    pub const fn hwcfg2(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "User HW Config 3 Register"]
    pub const fn hwcfg3(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "User HW Config 4 Register"]
    pub const fn hwcfg4(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "OTG core LPM configuration register"]
    pub const fn glpmcfg(&self) -> utils::Reg<GlpmcfgBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x54);
            <utils::Reg<GlpmcfgBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Global PowerDn Register"]
    pub const fn gpwrdn(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x58);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Global DFIFO SW Config Register"]
    pub const fn gdfifocfg(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "ADP (Attach Detection Protocol) Control Register"]
    pub const fn adpctl(&self) -> utils::Reg<AdpctlBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<AdpctlBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host periodic transmit FIFO size register"]
    pub const fn hptxfsiz(&self) -> utils::Reg<FsizBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x100);
            <utils::Reg<FsizBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device IN endpoint transmit FIFO size register"]
    pub const fn dieptxf(&self, idx: usize) -> utils::Reg<FsizBits, utils::RW> {
        assert!(idx < 7);
        unsafe {
            let ptr = self.ptr.add(0x104 + idx * 0x4);
            <utils::Reg<FsizBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host configuration register"]
    pub const fn hcfg(&self) -> utils::Reg<HcfgBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x400);
            <utils::Reg<HcfgBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host frame interval register"]
    pub const fn hfir(&self) -> utils::Reg<HfirBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x404);
            <utils::Reg<HfirBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host frame number/frame time remaining register"]
    pub const fn hfnum(&self) -> utils::Reg<HfnumBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x408);
            <utils::Reg<HfnumBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Periodic transmit FIFO/queue status register"]
    pub const fn hptxsts(&self) -> utils::Reg<HptxstsBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x410);
            <utils::Reg<HptxstsBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host all channels interrupt register"]
    pub const fn haint(&self) -> utils::Reg<HaintBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x414);
            <utils::Reg<HaintBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host all channels interrupt mask register"]
    pub const fn haintmsk(&self) -> utils::Reg<HaintmskBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x418);
            <utils::Reg<HaintmskBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host Frame Scheduling List Register"]
    pub const fn hflbaddr(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x41c);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host port control and status register"]
    pub const fn hprt(&self) -> utils::Reg<HprtBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x440);
            <utils::Reg<HprtBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host channel characteristics register"]
    pub const fn hcchar(&self, idx: usize) -> utils::Reg<HccharBits, utils::RW> {
        assert!(idx < 12);
        unsafe {
            let ptr = self.ptr.add(0x500 + idx * 0x20);
            <utils::Reg<HccharBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host channel split control register"]
    pub const fn hcsplt(&self, idx: usize) -> utils::Reg<u32, utils::RW> {
        assert!(idx < 12);
        unsafe {
            let ptr = self.ptr.add(0x504 + idx * 0x20);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host channel interrupt register"]
    pub const fn hcint(&self, idx: usize) -> utils::Reg<HcintBits, utils::RW> {
        assert!(idx < 12);
        unsafe {
            let ptr = self.ptr.add(0x508 + idx * 0x20);
            <utils::Reg<HcintBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host channel mask register"]
    pub const fn hcintmsk(&self, idx: usize) -> utils::Reg<HcintmskBits, utils::RW> {
        assert!(idx < 12);
        unsafe {
            let ptr = self.ptr.add(0x50c + idx * 0x20);
            <utils::Reg<HcintmskBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host channel transfer size register"]
    pub const fn hctsiz(&self, idx: usize) -> utils::Reg<HctsizBits, utils::RW> {
        assert!(idx < 12);
        unsafe {
            let ptr = self.ptr.add(0x510 + idx * 0x20);
            <utils::Reg<HctsizBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host channel DMA address register (config for scatter/gather)"]
    pub const fn hcdma(&self, idx: usize) -> utils::Reg<HcdmaBits, utils::RW> {
        assert!(idx < 12);
        unsafe {
            let ptr = self.ptr.add(0x514 + idx * 0x20);
            <utils::Reg<HcdmaBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Host channel DMA address register (address for current transfer; debug)"]
    pub const fn hcdmab(&self, idx: usize) -> utils::Reg<u32, utils::RW> {
        assert!(idx < 12);
        unsafe {
            let ptr = self.ptr.add(0x51c + idx * 0x20);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device configuration register"]
    pub const fn dcfg(&self) -> utils::Reg<DcfgBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x800);
            <utils::Reg<DcfgBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device control register"]
    pub const fn dctl(&self) -> utils::Reg<DctlBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x804);
            <utils::Reg<DctlBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device status register"]
    pub const fn dsts(&self) -> utils::Reg<DstsBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x808);
            <utils::Reg<DstsBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device IN endpoint common interrupt mask register"]
    pub const fn diepmsk(&self) -> utils::Reg<DiepmskBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x810);
            <utils::Reg<DiepmskBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device OUT endpoint common interrupt mask register"]
    pub const fn doepmsk(&self) -> utils::Reg<DoepmskBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x814);
            <utils::Reg<DoepmskBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device all endpoints interrupt register"]
    pub const fn daint(&self) -> utils::Reg<DaintBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x818);
            <utils::Reg<DaintBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "All endpoints interrupt mask register"]
    pub const fn daintmsk(&self) -> utils::Reg<DaintmskBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x81c);
            <utils::Reg<DaintmskBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device VBUS discharge time register"]
    pub const fn dvbusdis(&self) -> utils::Reg<DvbusdisBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x828);
            <utils::Reg<DvbusdisBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device VBUS pulsing time register"]
    pub const fn dvbuspulse(&self) -> utils::Reg<DvbuspulseBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x82c);
            <utils::Reg<DvbuspulseBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device IN endpoint FIFO empty interrupt mask register"]
    pub const fn diepempmsk(&self) -> utils::Reg<DiepempmskBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x834);
            <utils::Reg<DiepempmskBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device IN endpoint control register"]
    pub const fn diepctl(&self, idx: usize) -> utils::Reg<DiepctlBits, utils::RW> {
        assert!(idx < 16);
        unsafe {
            let ptr = self.ptr.add(0x900 + idx * 0x20);
            <utils::Reg<DiepctlBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device IN endpoint interrupt register"]
    pub const fn diepint(&self, idx: usize) -> utils::Reg<DiepintBits, utils::RW> {
        assert!(idx < 16);
        unsafe {
            let ptr = self.ptr.add(0x908 + idx * 0x20);
            <utils::Reg<DiepintBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device IN endpoint transfer size register"]
    pub const fn dieptsiz(&self, idx: usize) -> utils::Reg<DieptsizBits, utils::RW> {
        assert!(idx < 16);
        unsafe {
            let ptr = self.ptr.add(0x910 + idx * 0x20);
            <utils::Reg<DieptsizBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device IN endpoint transmit FIFO status register"]
    pub const fn dtxfsts(&self, idx: usize) -> utils::Reg<DtxfstsBits, utils::RO> {
        assert!(idx < 16);
        unsafe {
            let ptr = self.ptr.add(0x918 + idx * 0x20);
            <utils::Reg<DtxfstsBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device OUT endpoint control register"]
    pub const fn doepctl(&self, idx: usize) -> utils::Reg<DoepctlBits, utils::RW> {
        assert!(idx < 16);
        unsafe {
            let ptr = self.ptr.add(0xb00 + idx * 0x20);
            <utils::Reg<DoepctlBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device OUT endpoint interrupt register"]
    pub const fn doepint(&self, idx: usize) -> utils::Reg<DoepintBits, utils::RW> {
        assert!(idx < 16);
        unsafe {
            let ptr = self.ptr.add(0xb08 + idx * 0x20);
            <utils::Reg<DoepintBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device OUT endpoint transfer size register"]
    pub const fn doeptsiz(&self, idx: usize) -> utils::Reg<DoeptsizBits, utils::RW> {
        assert!(idx < 16);
        unsafe {
            let ptr = self.ptr.add(0xb10 + idx * 0x20);
            <utils::Reg<DoeptsizBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device OUT/IN endpoint DMA address register"]
    pub const fn doepdma(&self, idx: usize) -> utils::Reg<u32, utils::RW> {
        assert!(idx < 16);
        unsafe {
            let ptr = self.ptr.add(0xb14 + idx * 0x20);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Power and clock gating control register"]
    pub const fn pcgcctl(&self) -> utils::Reg<PcgcctlBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xe00);
            <utils::Reg<PcgcctlBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Device endpoint / host channel FIFO register"]
    pub const fn fifo(&self, idx: usize) -> utils::Reg<FifoBits, utils::RW> {
        assert!(idx < 16);
        unsafe {
            let ptr = self.ptr.add(0x1000 + idx * 0x1000);
            <utils::Reg<FifoBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "ADP (Attach Detection Protocol) Control Register"]
pub struct AdpctlBits {
    bits: u32,
}
impl Default for AdpctlBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl AdpctlBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Probe Discharge time (times for TADP_DSCHG)"]
    pub const fn set_prb_dschg(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Probe Discharge time (times for TADP_DSCHG)"]
    pub const fn prb_dschg(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Probe Delta (resolution for RTIM)"]
    pub const fn set_prb_delta(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x2);
        self.bits |= (val as u32 & 0x3) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Probe Delta (resolution for RTIM)"]
    pub const fn prb_delta(self) -> u8 {
        ((self.bits >> 0x2) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Probe Period (TADP_PRD)"]
    pub const fn set_prb_per(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x4);
        self.bits |= (val as u32 & 0x3) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Probe Period (TADP_PRD)"]
    pub const fn prb_per(self) -> u8 {
        ((self.bits >> 0x4) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Probe Period (TADP_PRD)"]
    pub const fn set_rtim(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x6);
        self.bits |= (val as u32 & 0x7ff) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Probe Period (TADP_PRD)"]
    pub const fn rtim(self) -> u16 {
        ((self.bits >> 0x6) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "Enable Probe"]
    pub const fn set_enaprb(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable Probe"]
    pub const fn enaprb(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enable Sense"]
    pub const fn set_enasns(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable Sense"]
    pub const fn enasns(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADP Reset"]
    pub const fn set_adpres(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADP Reset"]
    pub const fn adpres(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADP Enable"]
    pub const fn set_adpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADP Enable"]
    pub const fn adpen(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADP Probe Interrupt Enable"]
    pub const fn set_adp_prb_int(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADP Probe Interrupt Enable"]
    pub const fn adp_prb_int(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADP Sense Interrupt Enable"]
    pub const fn set_adp_sns_int(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADP Sense Interrupt Enable"]
    pub const fn adp_sns_int(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADP Timeout Interrupt Enable"]
    pub const fn set_adp_tmout_int(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADP Timeout Interrupt Enable"]
    pub const fn adp_tmout_int(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADP Probe Interrupt Mask"]
    pub const fn set_adp_prb_msk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADP Probe Interrupt Mask"]
    pub const fn adp_prb_msk(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ADP Timeout Interrupt Mask"]
    pub const fn set_adp_tmout_msk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ADP Timeout Interrupt Mask"]
    pub const fn adp_tmout_msk(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Access Request"]
    pub const fn set_ar(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Access Request"]
    pub const fn ar(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Core ID register"]
pub struct CidBits {
    bits: u32,
}
impl Default for CidBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CidBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Product ID field"]
    pub const fn set_product_id(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Product ID field"]
    pub const fn product_id(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device all endpoints interrupt register"]
pub struct DaintBits {
    bits: u32,
}
impl Default for DaintBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DaintBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "IN endpoint interrupt bits"]
    pub const fn set_iepint(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "IN endpoint interrupt bits"]
    pub const fn iepint(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "OUT endpoint interrupt bits"]
    pub const fn set_oepint(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "OUT endpoint interrupt bits"]
    pub const fn oepint(self) -> u16 {
        ((self.bits >> 0x10) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "All endpoints interrupt mask register"]
pub struct DaintmskBits {
    bits: u32,
}
impl Default for DaintmskBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DaintmskBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "IN EP interrupt mask bits"]
    pub const fn set_iepm(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "IN EP interrupt mask bits"]
    pub const fn iepm(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "OUT EP interrupt mask bits"]
    pub const fn set_oepm(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "OUT EP interrupt mask bits"]
    pub const fn oepm(self) -> u16 {
        ((self.bits >> 0x10) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device configuration register"]
pub struct DcfgBits {
    bits: u32,
}
impl Default for DcfgBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DcfgBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Device speed"]
    pub const fn set_dspd(mut self, val: DspdVal) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Device speed"]
    pub const fn dspd(self) -> DspdVal {
        let val = ((self.bits >> 0x0) & 0x3) as _;
        unsafe { DspdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Non-zero-length status OUT handshake"]
    pub const fn set_nzlsohsk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Non-zero-length status OUT handshake"]
    pub const fn nzlsohsk(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Device address"]
    pub const fn set_dad(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x4);
        self.bits |= (val as u32 & 0x7f) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Device address"]
    pub const fn dad(self) -> u8 {
        ((self.bits >> 0x4) & 0x7f) as _
    }
    #[inline(always)]
    #[doc = "Periodic frame interval"]
    pub const fn set_pfivl(mut self, val: PfivlVal) -> Self {
        self.bits &= !(0x3 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "Periodic frame interval"]
    pub const fn pfivl(self) -> PfivlVal {
        let val = ((self.bits >> 0xb) & 0x3) as _;
        unsafe { PfivlVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Transceiver delay"]
    pub const fn set_xcvrdly(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transceiver delay"]
    pub const fn xcvrdly(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device control register"]
pub struct DctlBits {
    bits: u32,
}
impl Default for DctlBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DctlBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Remote wakeup signaling"]
    pub const fn set_rwusig(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Remote wakeup signaling"]
    pub const fn rwusig(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Soft disconnect"]
    pub const fn set_sdis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Soft disconnect"]
    pub const fn sdis(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Global IN NAK status"]
    pub const fn set_ginsts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Global IN NAK status"]
    pub const fn ginsts(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Global OUT NAK status"]
    pub const fn set_gonsts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Global OUT NAK status"]
    pub const fn gonsts(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Test control"]
    pub const fn set_tctl(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x4);
        self.bits |= (val as u32 & 0x7) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Test control"]
    pub const fn tctl(self) -> u8 {
        ((self.bits >> 0x4) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Set global IN NAK"]
    pub const fn set_sginak(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Set global IN NAK"]
    pub const fn sginak(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear global IN NAK"]
    pub const fn set_cginak(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear global IN NAK"]
    pub const fn cginak(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Set global OUT NAK"]
    pub const fn set_sgonak(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Set global OUT NAK"]
    pub const fn sgonak(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear global OUT NAK"]
    pub const fn set_cgonak(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear global OUT NAK"]
    pub const fn cgonak(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Power-on programming done"]
    pub const fn set_poprgdne(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Power-on programming done"]
    pub const fn poprgdne(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device endpoint control register"]
pub struct DiepctlBits {
    bits: u32,
}
impl Default for DiepctlBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DiepctlBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "MPSIZ"]
    pub const fn set_mpsiz(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x0);
        self.bits |= (val as u32 & 0x7ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "MPSIZ"]
    pub const fn mpsiz(self) -> u16 {
        ((self.bits >> 0x0) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "USBAEP"]
    pub const fn set_usbaep(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USBAEP"]
    pub const fn usbaep(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EONUM/DPID"]
    pub const fn set_eonum_dpid(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EONUM/DPID"]
    pub const fn eonum_dpid(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "NAKSTS"]
    pub const fn set_naksts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "NAKSTS"]
    pub const fn naksts(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EPTYP"]
    pub const fn set_eptyp(mut self, val: EptypVal) -> Self {
        self.bits &= !(0x3 << 0x12);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x12;
        self
    }
    #[inline(always)]
    #[doc = "EPTYP"]
    pub const fn eptyp(self) -> EptypVal {
        let val = ((self.bits >> 0x12) & 0x3) as _;
        unsafe { EptypVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "SNPM"]
    pub const fn set_snpm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SNPM"]
    pub const fn snpm(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "STALL"]
    pub const fn set_stall(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "STALL"]
    pub const fn stall(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TXFNUM"]
    pub const fn set_txfnum(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x16);
        self.bits |= (val as u32 & 0xf) << 0x16;
        self
    }
    #[inline(always)]
    #[doc = "TXFNUM"]
    pub const fn txfnum(self) -> u8 {
        ((self.bits >> 0x16) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "CNAK"]
    pub const fn set_cnak(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CNAK"]
    pub const fn cnak(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SNAK"]
    pub const fn set_snak(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SNAK"]
    pub const fn snak(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SD0PID/SEVNFRM"]
    pub const fn set_sd0pid_sevnfrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SD0PID/SEVNFRM"]
    pub const fn sd0pid_sevnfrm(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SODDFRM/SD1PID"]
    pub const fn set_soddfrm_sd1pid(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SODDFRM/SD1PID"]
    pub const fn soddfrm_sd1pid(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EPDIS"]
    pub const fn set_epdis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EPDIS"]
    pub const fn epdis(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EPENA"]
    pub const fn set_epena(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EPENA"]
    pub const fn epena(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device IN endpoint FIFO empty interrupt mask register"]
pub struct DiepempmskBits {
    bits: u32,
}
impl Default for DiepempmskBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DiepempmskBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "IN EP Tx FIFO empty interrupt mask bits"]
    pub const fn set_ineptxfem(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "IN EP Tx FIFO empty interrupt mask bits"]
    pub const fn ineptxfem(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device endpoint interrupt register"]
pub struct DiepintBits {
    bits: u32,
}
impl Default for DiepintBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DiepintBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "XFRC"]
    pub const fn set_xfrc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "XFRC"]
    pub const fn xfrc(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EPDISD"]
    pub const fn set_epdisd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EPDISD"]
    pub const fn epdisd(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TOC"]
    pub const fn set_toc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TOC"]
    pub const fn toc(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ITTXFE"]
    pub const fn set_ittxfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ITTXFE"]
    pub const fn ittxfe(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "INEPNE"]
    pub const fn set_inepne(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "INEPNE"]
    pub const fn inepne(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TXFE"]
    pub const fn set_txfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TXFE"]
    pub const fn txfe(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device IN endpoint common interrupt mask register"]
pub struct DiepmskBits {
    bits: u32,
}
impl Default for DiepmskBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DiepmskBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transfer completed interrupt mask"]
    pub const fn set_xfrcm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer completed interrupt mask"]
    pub const fn xfrcm(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Endpoint disabled interrupt mask"]
    pub const fn set_epdm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Endpoint disabled interrupt mask"]
    pub const fn epdm(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timeout condition mask (Non-isochronous endpoints)"]
    pub const fn set_tom(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timeout condition mask (Non-isochronous endpoints)"]
    pub const fn tom(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IN token received when TxFIFO empty mask"]
    pub const fn set_ittxfemsk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IN token received when TxFIFO empty mask"]
    pub const fn ittxfemsk(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IN token received with EP mismatch mask"]
    pub const fn set_inepnmm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IN token received with EP mismatch mask"]
    pub const fn inepnmm(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IN endpoint NAK effective mask"]
    pub const fn set_inepnem(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IN endpoint NAK effective mask"]
    pub const fn inepnem(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device endpoint transfer size register"]
pub struct DieptsizBits {
    bits: u32,
}
impl Default for DieptsizBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DieptsizBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transfer size"]
    pub const fn set_xfrsiz(mut self, val: u32) -> Self {
        self.bits &= !(0x7ffff << 0x0);
        self.bits |= (val as u32 & 0x7ffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Transfer size"]
    pub const fn xfrsiz(self) -> u32 {
        ((self.bits >> 0x0) & 0x7ffff) as _
    }
    #[inline(always)]
    #[doc = "Packet count"]
    pub const fn set_pktcnt(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x13);
        self.bits |= (val as u32 & 0x3ff) << 0x13;
        self
    }
    #[inline(always)]
    #[doc = "Packet count"]
    pub const fn pktcnt(self) -> u16 {
        ((self.bits >> 0x13) & 0x3ff) as _
    }
    #[inline(always)]
    #[doc = "Multi count"]
    pub const fn set_mcnt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x1d);
        self.bits |= (val as u32 & 0x3) << 0x1d;
        self
    }
    #[inline(always)]
    #[doc = "Multi count"]
    pub const fn mcnt(self) -> u8 {
        ((self.bits >> 0x1d) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device endpoint control register"]
pub struct DoepctlBits {
    bits: u32,
}
impl Default for DoepctlBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DoepctlBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "MPSIZ"]
    pub const fn set_mpsiz(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x0);
        self.bits |= (val as u32 & 0x7ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "MPSIZ"]
    pub const fn mpsiz(self) -> u16 {
        ((self.bits >> 0x0) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "USBAEP"]
    pub const fn set_usbaep(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USBAEP"]
    pub const fn usbaep(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EONUM/DPID"]
    pub const fn set_eonum_dpid(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EONUM/DPID"]
    pub const fn eonum_dpid(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "NAKSTS"]
    pub const fn set_naksts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "NAKSTS"]
    pub const fn naksts(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EPTYP"]
    pub const fn set_eptyp(mut self, val: EptypVal) -> Self {
        self.bits &= !(0x3 << 0x12);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x12;
        self
    }
    #[inline(always)]
    #[doc = "EPTYP"]
    pub const fn eptyp(self) -> EptypVal {
        let val = ((self.bits >> 0x12) & 0x3) as _;
        unsafe { EptypVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "SNPM"]
    pub const fn set_snpm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SNPM"]
    pub const fn snpm(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "STALL"]
    pub const fn set_stall(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "STALL"]
    pub const fn stall(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CNAK"]
    pub const fn set_cnak(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CNAK"]
    pub const fn cnak(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SNAK"]
    pub const fn set_snak(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SNAK"]
    pub const fn snak(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SD0PID/SEVNFRM"]
    pub const fn set_sd0pid_sevnfrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SD0PID/SEVNFRM"]
    pub const fn sd0pid_sevnfrm(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SODDFRM"]
    pub const fn set_soddfrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SODDFRM"]
    pub const fn soddfrm(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EPDIS"]
    pub const fn set_epdis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EPDIS"]
    pub const fn epdis(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EPENA"]
    pub const fn set_epena(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EPENA"]
    pub const fn epena(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device endpoint interrupt register"]
pub struct DoepintBits {
    bits: u32,
}
impl Default for DoepintBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DoepintBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "XFRC"]
    pub const fn set_xfrc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "XFRC"]
    pub const fn xfrc(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "EPDISD"]
    pub const fn set_epdisd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "EPDISD"]
    pub const fn epdisd(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "STUP"]
    pub const fn set_stup(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "STUP"]
    pub const fn stup(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "OTEPDIS"]
    pub const fn set_otepdis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "OTEPDIS"]
    pub const fn otepdis(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "B2BSTUP"]
    pub const fn set_b2bstup(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "B2BSTUP"]
    pub const fn b2bstup(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device OUT endpoint common interrupt mask register"]
pub struct DoepmskBits {
    bits: u32,
}
impl Default for DoepmskBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DoepmskBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transfer completed interrupt mask"]
    pub const fn set_xfrcm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer completed interrupt mask"]
    pub const fn xfrcm(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Endpoint disabled interrupt mask"]
    pub const fn set_epdm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Endpoint disabled interrupt mask"]
    pub const fn epdm(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SETUP phase done mask"]
    pub const fn set_stupm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SETUP phase done mask"]
    pub const fn stupm(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "OUT token received when endpoint disabled mask"]
    pub const fn set_otepdm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "OUT token received when endpoint disabled mask"]
    pub const fn otepdm(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device OUT endpoint transfer size register"]
pub struct DoeptsizBits {
    bits: u32,
}
impl Default for DoeptsizBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DoeptsizBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transfer size"]
    pub const fn set_xfrsiz(mut self, val: u32) -> Self {
        self.bits &= !(0x7ffff << 0x0);
        self.bits |= (val as u32 & 0x7ffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Transfer size"]
    pub const fn xfrsiz(self) -> u32 {
        ((self.bits >> 0x0) & 0x7ffff) as _
    }
    #[inline(always)]
    #[doc = "Packet count"]
    pub const fn set_pktcnt(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x13);
        self.bits |= (val as u32 & 0x3ff) << 0x13;
        self
    }
    #[inline(always)]
    #[doc = "Packet count"]
    pub const fn pktcnt(self) -> u16 {
        ((self.bits >> 0x13) & 0x3ff) as _
    }
    #[inline(always)]
    #[doc = "Received data PID/SETUP packet count"]
    pub const fn set_rxdpid_stupcnt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x1d);
        self.bits |= (val as u32 & 0x3) << 0x1d;
        self
    }
    #[inline(always)]
    #[doc = "Received data PID/SETUP packet count"]
    pub const fn rxdpid_stupcnt(self) -> u8 {
        ((self.bits >> 0x1d) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device status register"]
pub struct DstsBits {
    bits: u32,
}
impl Default for DstsBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DstsBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Suspend status"]
    pub const fn set_suspsts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Suspend status"]
    pub const fn suspsts(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enumerated speed"]
    pub const fn set_enumspd(mut self, val: DspdVal) -> Self {
        self.bits &= !(0x3 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Enumerated speed"]
    pub const fn enumspd(self) -> DspdVal {
        let val = ((self.bits >> 0x1) & 0x3) as _;
        unsafe { DspdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Erratic error"]
    pub const fn set_eerr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Erratic error"]
    pub const fn eerr(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Frame number of the received SOF"]
    pub const fn set_fnsof(mut self, val: u16) -> Self {
        self.bits &= !(0x3fff << 0x8);
        self.bits |= (val as u32 & 0x3fff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Frame number of the received SOF"]
    pub const fn fnsof(self) -> u16 {
        ((self.bits >> 0x8) & 0x3fff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device IN endpoint transmit FIFO status register"]
pub struct DtxfstsBits {
    bits: u32,
}
impl Default for DtxfstsBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DtxfstsBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "IN endpoint TxFIFO space available"]
    pub const fn set_ineptfsav(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "IN endpoint TxFIFO space available"]
    pub const fn ineptfsav(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device VBUS discharge time register"]
pub struct DvbusdisBits {
    bits: u32,
}
impl Default for DvbusdisBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DvbusdisBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Device VBUS discharge time"]
    pub const fn set_vbusdt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Device VBUS discharge time"]
    pub const fn vbusdt(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Device VBUS pulsing time register"]
pub struct DvbuspulseBits {
    bits: u32,
}
impl Default for DvbuspulseBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DvbuspulseBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Device VBUS pulsing time"]
    pub const fn set_dvbusp(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Device VBUS pulsing time"]
    pub const fn dvbusp(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "FIFO register"]
pub struct FifoBits {
    bits: u32,
}
impl Default for FifoBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FifoBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Data"]
    pub const fn set_data(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Data"]
    pub const fn data(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "FIFO size register"]
pub struct FsizBits {
    bits: u32,
}
impl Default for FsizBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FsizBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "RAM start address"]
    pub const fn set_sa(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "RAM start address"]
    pub const fn sa(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "FIFO depth"]
    pub const fn set_fd(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "FIFO depth"]
    pub const fn fd(self) -> u16 {
        ((self.bits >> 0x10) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "AHB configuration register"]
pub struct GahbcfgBits {
    bits: u32,
}
impl Default for GahbcfgBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GahbcfgBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Global interrupt mask"]
    pub const fn set_gint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Global interrupt mask"]
    pub const fn gint(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Burst length/type"]
    pub const fn set_hbstlen(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x1);
        self.bits |= (val as u32 & 0xf) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Burst length/type"]
    pub const fn hbstlen(self) -> u8 {
        ((self.bits >> 0x1) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "DMA enable"]
    pub const fn set_dmaen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA enable"]
    pub const fn dmaen(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TxFIFO empty level"]
    pub const fn set_txfelvl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TxFIFO empty level"]
    pub const fn txfelvl(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Periodic TxFIFO empty level"]
    pub const fn set_ptxfelvl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Periodic TxFIFO empty level"]
    pub const fn ptxfelvl(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "General core configuration register"]
pub struct GccfgV1Bits {
    bits: u32,
}
impl Default for GccfgV1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GccfgV1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Power down"]
    pub const fn set_pwrdwn(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Power down"]
    pub const fn pwrdwn(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enable the VBUS \"A\" sensing device"]
    pub const fn set_vbusasen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable the VBUS \"A\" sensing device"]
    pub const fn vbusasen(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enable the VBUS \"B\" sensing device"]
    pub const fn set_vbusbsen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable the VBUS \"B\" sensing device"]
    pub const fn vbusbsen(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SOF output enable"]
    pub const fn set_sofouten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SOF output enable"]
    pub const fn sofouten(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VBUS sensing disable"]
    pub const fn set_novbussens(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VBUS sensing disable"]
    pub const fn novbussens(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "General core configuration register"]
pub struct GccfgV2Bits {
    bits: u32,
}
impl Default for GccfgV2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GccfgV2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Data contact detection (DCD) status"]
    pub const fn set_dcdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data contact detection (DCD) status"]
    pub const fn dcdet(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Primary detection (PD) status"]
    pub const fn set_pdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Primary detection (PD) status"]
    pub const fn pdet(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Secondary detection (SD) status"]
    pub const fn set_sdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Secondary detection (SD) status"]
    pub const fn sdet(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DM pull-up detection status"]
    pub const fn set_ps2det(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DM pull-up detection status"]
    pub const fn ps2det(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Power down"]
    pub const fn set_pwrdwn(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Power down"]
    pub const fn pwrdwn(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Battery charging detector (BCD) enable"]
    pub const fn set_bcden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Battery charging detector (BCD) enable"]
    pub const fn bcden(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data contact detection (DCD) mode enable"]
    pub const fn set_dcden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data contact detection (DCD) mode enable"]
    pub const fn dcden(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Primary detection (PD) mode enable"]
    pub const fn set_pden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Primary detection (PD) mode enable"]
    pub const fn pden(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Secondary detection (SD) mode enable"]
    pub const fn set_sden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Secondary detection (SD) mode enable"]
    pub const fn sden(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB VBUS detection enable"]
    pub const fn set_vbden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB VBUS detection enable"]
    pub const fn vbden(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Internal high-speed PHY enable."]
    pub const fn set_phyhsen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Internal high-speed PHY enable."]
    pub const fn phyhsen(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "OTG general core configuration register."]
pub struct GccfgV3Bits {
    bits: u32,
}
impl Default for GccfgV3Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GccfgV3Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Charger detection, result of the current mode (primary or secondary)."]
    pub const fn set_chgdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Charger detection, result of the current mode (primary or secondary)."]
    pub const fn chgdet(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Single-Ended DP indicator This bit gives the voltage level on DP (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard)."]
    pub const fn set_fsvplus(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Single-Ended DP indicator This bit gives the voltage level on DP (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard)."]
    pub const fn fsvplus(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Single-Ended DM indicator This bit gives the voltage level on DM (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard)."]
    pub const fn set_fsvminus(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Single-Ended DM indicator This bit gives the voltage level on DM (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard)."]
    pub const fn fsvminus(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VBUS session indicator Indicates if VBUS is above VBUS session threshold."]
    pub const fn set_sessvld(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VBUS session indicator Indicates if VBUS is above VBUS session threshold."]
    pub const fn sessvld(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host CDP behavior enable."]
    pub const fn set_hcdpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host CDP behavior enable."]
    pub const fn hcdpen(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host CDP port voltage detector enable on DP."]
    pub const fn set_hcdpdeten(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host CDP port voltage detector enable on DP."]
    pub const fn hcdpdeten(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host CDP port Voltage source enable on DM."]
    pub const fn set_hvdmsrcen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host CDP port Voltage source enable on DM."]
    pub const fn hvdmsrcen(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data Contact Detection enable."]
    pub const fn set_dcden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data Contact Detection enable."]
    pub const fn dcden(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Primary detection enable."]
    pub const fn set_pden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Primary detection enable."]
    pub const fn pden(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VBUS detection enable Enables VBUS Sensing Comparators in order to detect VBUS presence and/or perform OTG operation."]
    pub const fn set_vbden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VBUS detection enable Enables VBUS Sensing Comparators in order to detect VBUS presence and/or perform OTG operation."]
    pub const fn vbden(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Secondary detection enable."]
    pub const fn set_sden(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Secondary detection enable."]
    pub const fn sden(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Software override value of the VBUS B-session detection."]
    pub const fn set_vbvaloval(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Software override value of the VBUS B-session detection."]
    pub const fn vbvaloval(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enables a software override of the VBUS B-session detection."]
    pub const fn set_vbvaloven(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enables a software override of the VBUS B-session detection."]
    pub const fn vbvaloven(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force host mode pull-downs If the ID pin functions are enabled, the host mode pull-downs on DP and DM activate automatically. However, whenever that is not the case, yet host mode is required, this bit must be used to force the pull-downs active."]
    pub const fn set_forcehostpd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force host mode pull-downs If the ID pin functions are enabled, the host mode pull-downs on DP and DM activate automatically. However, whenever that is not the case, yet host mode is required, this bit must be used to force the pull-downs active."]
    pub const fn forcehostpd(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "I2C access register"]
pub struct Gi2cctlBits {
    bits: u32,
}
impl Default for Gi2cctlBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Gi2cctlBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "I2C Read/Write Data"]
    pub const fn set_rwdata(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "I2C Read/Write Data"]
    pub const fn rwdata(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "I2C Register Address"]
    pub const fn set_regaddr(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "I2C Register Address"]
    pub const fn regaddr(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "I2C Address"]
    pub const fn set_addr(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x10);
        self.bits |= (val as u32 & 0x7f) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "I2C Address"]
    pub const fn addr(self) -> u8 {
        ((self.bits >> 0x10) & 0x7f) as _
    }
    #[inline(always)]
    #[doc = "I2C Enable"]
    pub const fn set_i2cen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C Enable"]
    pub const fn i2cen(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C ACK"]
    pub const fn set_ack(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C ACK"]
    pub const fn ack(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C Device Address"]
    pub const fn set_i2cdevadr(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x1a);
        self.bits |= (val as u32 & 0x3) << 0x1a;
        self
    }
    #[inline(always)]
    #[doc = "I2C Device Address"]
    pub const fn i2cdevadr(self) -> u8 {
        ((self.bits >> 0x1a) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "I2C DatSe0 USB mode"]
    pub const fn set_i2cdatse0(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C DatSe0 USB mode"]
    pub const fn i2cdatse0(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Read/Write Indicator"]
    pub const fn set_rw(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Read/Write Indicator"]
    pub const fn rw(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "I2C Busy/Done"]
    pub const fn set_bsydne(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "I2C Busy/Done"]
    pub const fn bsydne(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt mask register"]
pub struct GintmskBits {
    bits: u32,
}
impl Default for GintmskBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GintmskBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Mode mismatch interrupt mask"]
    pub const fn set_mmism(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mode mismatch interrupt mask"]
    pub const fn mmism(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "OTG interrupt mask"]
    pub const fn set_otgint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "OTG interrupt mask"]
    pub const fn otgint(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Start of frame mask"]
    pub const fn set_sofm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Start of frame mask"]
    pub const fn sofm(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive FIFO non-empty mask"]
    pub const fn set_rxflvlm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive FIFO non-empty mask"]
    pub const fn rxflvlm(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Non-periodic TxFIFO empty mask"]
    pub const fn set_nptxfem(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Non-periodic TxFIFO empty mask"]
    pub const fn nptxfem(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Global non-periodic IN NAK effective mask"]
    pub const fn set_ginakeffm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Global non-periodic IN NAK effective mask"]
    pub const fn ginakeffm(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Global OUT NAK effective mask"]
    pub const fn set_gonakeffm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Global OUT NAK effective mask"]
    pub const fn gonakeffm(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Early suspend mask"]
    pub const fn set_esuspm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Early suspend mask"]
    pub const fn esuspm(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB suspend mask"]
    pub const fn set_usbsuspm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB suspend mask"]
    pub const fn usbsuspm(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB reset mask"]
    pub const fn set_usbrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB reset mask"]
    pub const fn usbrst(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enumeration done mask"]
    pub const fn set_enumdnem(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enumeration done mask"]
    pub const fn enumdnem(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Isochronous OUT packet dropped interrupt mask"]
    pub const fn set_isoodrpm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Isochronous OUT packet dropped interrupt mask"]
    pub const fn isoodrpm(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "End of periodic frame interrupt mask"]
    pub const fn set_eopfm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "End of periodic frame interrupt mask"]
    pub const fn eopfm(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Endpoint mismatch interrupt mask"]
    pub const fn set_epmism(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Endpoint mismatch interrupt mask"]
    pub const fn epmism(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IN endpoints interrupt mask"]
    pub const fn set_iepint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IN endpoints interrupt mask"]
    pub const fn iepint(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "OUT endpoints interrupt mask"]
    pub const fn set_oepint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "OUT endpoints interrupt mask"]
    pub const fn oepint(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Incomplete isochronous IN transfer mask"]
    pub const fn set_iisoixfrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Incomplete isochronous IN transfer mask"]
    pub const fn iisoixfrm(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Incomplete periodic transfer mask (host mode) / Incomplete isochronous OUT transfer mask (device mode)"]
    pub const fn set_ipxfrm_iisooxfrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Incomplete periodic transfer mask (host mode) / Incomplete isochronous OUT transfer mask (device mode)"]
    pub const fn ipxfrm_iisooxfrm(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data fetch suspended mask"]
    pub const fn set_fsuspm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data fetch suspended mask"]
    pub const fn fsuspm(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Reset detected interrupt mask"]
    pub const fn set_rstde(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Reset detected interrupt mask"]
    pub const fn rstde(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host port interrupt mask"]
    pub const fn set_prtim(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host port interrupt mask"]
    pub const fn prtim(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host channels interrupt mask"]
    pub const fn set_hcim(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host channels interrupt mask"]
    pub const fn hcim(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Periodic TxFIFO empty mask"]
    pub const fn set_ptxfem(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Periodic TxFIFO empty mask"]
    pub const fn ptxfem(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LPM interrupt mask"]
    pub const fn set_lpmintm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LPM interrupt mask"]
    pub const fn lpmintm(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Connector ID status change mask"]
    pub const fn set_cidschgm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Connector ID status change mask"]
    pub const fn cidschgm(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Disconnect detected interrupt mask"]
    pub const fn set_discint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Disconnect detected interrupt mask"]
    pub const fn discint(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Session request/new session detected interrupt mask"]
    pub const fn set_srqim(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Session request/new session detected interrupt mask"]
    pub const fn srqim(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Resume/remote wakeup detected interrupt mask"]
    pub const fn set_wuim(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Resume/remote wakeup detected interrupt mask"]
    pub const fn wuim(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Core interrupt register"]
pub struct GintstsBits {
    bits: u32,
}
impl Default for GintstsBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GintstsBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Current mode of operation"]
    pub const fn set_cmod(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Current mode of operation"]
    pub const fn cmod(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mode mismatch interrupt"]
    pub const fn set_mmis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mode mismatch interrupt"]
    pub const fn mmis(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "OTG interrupt"]
    pub const fn set_otgint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "OTG interrupt"]
    pub const fn otgint(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Start of frame"]
    pub const fn set_sof(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Start of frame"]
    pub const fn sof(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "RxFIFO non-empty"]
    pub const fn set_rxflvl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RxFIFO non-empty"]
    pub const fn rxflvl(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Non-periodic TxFIFO empty"]
    pub const fn set_nptxfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Non-periodic TxFIFO empty"]
    pub const fn nptxfe(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Global IN non-periodic NAK effective"]
    pub const fn set_ginakeff(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Global IN non-periodic NAK effective"]
    pub const fn ginakeff(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Global OUT NAK effective"]
    pub const fn set_goutnakeff(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Global OUT NAK effective"]
    pub const fn goutnakeff(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Early suspend"]
    pub const fn set_esusp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Early suspend"]
    pub const fn esusp(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB suspend"]
    pub const fn set_usbsusp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB suspend"]
    pub const fn usbsusp(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB reset"]
    pub const fn set_usbrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "USB reset"]
    pub const fn usbrst(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Enumeration done"]
    pub const fn set_enumdne(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enumeration done"]
    pub const fn enumdne(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Isochronous OUT packet dropped interrupt"]
    pub const fn set_isoodrp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Isochronous OUT packet dropped interrupt"]
    pub const fn isoodrp(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "End of periodic frame interrupt"]
    pub const fn set_eopf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "End of periodic frame interrupt"]
    pub const fn eopf(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IN endpoint interrupt"]
    pub const fn set_iepint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IN endpoint interrupt"]
    pub const fn iepint(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "OUT endpoint interrupt"]
    pub const fn set_oepint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "OUT endpoint interrupt"]
    pub const fn oepint(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Incomplete isochronous IN transfer"]
    pub const fn set_iisoixfr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Incomplete isochronous IN transfer"]
    pub const fn iisoixfr(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Incomplete periodic transfer (host mode) / Incomplete isochronous OUT transfer (device mode)"]
    pub const fn set_ipxfr_incompisoout(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Incomplete periodic transfer (host mode) / Incomplete isochronous OUT transfer (device mode)"]
    pub const fn ipxfr_incompisoout(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data fetch suspended"]
    pub const fn set_datafsusp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data fetch suspended"]
    pub const fn datafsusp(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Reset detected"]
    pub const fn set_resetdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Reset detected"]
    pub const fn resetdet(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host port interrupt"]
    pub const fn set_hprtint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host port interrupt"]
    pub const fn hprtint(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host channels interrupt"]
    pub const fn set_hcint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host channels interrupt"]
    pub const fn hcint(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Periodic TxFIFO empty"]
    pub const fn set_ptxfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Periodic TxFIFO empty"]
    pub const fn ptxfe(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Connector ID status change"]
    pub const fn set_cidschg(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Connector ID status change"]
    pub const fn cidschg(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Disconnect detected interrupt"]
    pub const fn set_discint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Disconnect detected interrupt"]
    pub const fn discint(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Session request/new session detected interrupt"]
    pub const fn set_srqint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Session request/new session detected interrupt"]
    pub const fn srqint(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Resume/remote wakeup detected interrupt"]
    pub const fn set_wkupint(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Resume/remote wakeup detected interrupt"]
    pub const fn wkupint(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Core LPM configuration register"]
pub struct GlpmcfgBits {
    bits: u32,
}
impl Default for GlpmcfgBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GlpmcfgBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "LPM support enable"]
    pub const fn set_lpmen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LPM support enable"]
    pub const fn lpmen(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LPM token acknowledge enable"]
    pub const fn set_lpmack(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LPM token acknowledge enable"]
    pub const fn lpmack(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Best effort service latency"]
    pub const fn set_besl(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x2);
        self.bits |= (val as u32 & 0xf) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Best effort service latency"]
    pub const fn besl(self) -> u8 {
        ((self.bits >> 0x2) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "bRemoteWake value"]
    pub const fn set_remwake(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "bRemoteWake value"]
    pub const fn remwake(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "L1 Shallow Sleep enable"]
    pub const fn set_l1ssen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "L1 Shallow Sleep enable"]
    pub const fn l1ssen(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "BESL threshold"]
    pub const fn set_beslthrs(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "BESL threshold"]
    pub const fn beslthrs(self) -> u8 {
        ((self.bits >> 0x8) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "L1 deep sleep enable"]
    pub const fn set_l1dsen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "L1 deep sleep enable"]
    pub const fn l1dsen(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LPM response"]
    pub const fn set_lpmrst(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0xd);
        self.bits |= (val as u32 & 0x3) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "LPM response"]
    pub const fn lpmrst(self) -> u8 {
        ((self.bits >> 0xd) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Port sleep status"]
    pub const fn set_slpsts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port sleep status"]
    pub const fn slpsts(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Sleep State Resume OK"]
    pub const fn set_l1rsmok(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Sleep State Resume OK"]
    pub const fn l1rsmok(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LPM Channel Index"]
    pub const fn set_lpmchidx(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x11);
        self.bits |= (val as u32 & 0xf) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "LPM Channel Index"]
    pub const fn lpmchidx(self) -> u8 {
        ((self.bits >> 0x11) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "LPM retry count"]
    pub const fn set_lpmrcnt(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x15);
        self.bits |= (val as u32 & 0x7) << 0x15;
        self
    }
    #[inline(always)]
    #[doc = "LPM retry count"]
    pub const fn lpmrcnt(self) -> u8 {
        ((self.bits >> 0x15) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Send LPM transaction"]
    pub const fn set_sndlpm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Send LPM transaction"]
    pub const fn sndlpm(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LPM retry count status"]
    pub const fn set_lpmrcntsts(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x19);
        self.bits |= (val as u32 & 0x7) << 0x19;
        self
    }
    #[inline(always)]
    #[doc = "LPM retry count status"]
    pub const fn lpmrcntsts(self) -> u8 {
        ((self.bits >> 0x19) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Enable best effort service latency"]
    pub const fn set_enbesl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enable best effort service latency"]
    pub const fn enbesl(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Control and status register"]
pub struct GotgctlBits {
    bits: u32,
}
impl Default for GotgctlBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GotgctlBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Session request success"]
    pub const fn set_srqscs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Session request success"]
    pub const fn srqscs(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Session request"]
    pub const fn set_srq(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Session request"]
    pub const fn srq(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VBUS valid override enable"]
    pub const fn set_vbvaloen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VBUS valid override enable"]
    pub const fn vbvaloen(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "VBUS valid override value"]
    pub const fn set_vbvaloval(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "VBUS valid override value"]
    pub const fn vbvaloval(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "A-peripheral session valid override enable"]
    pub const fn set_avaloen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "A-peripheral session valid override enable"]
    pub const fn avaloen(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "A-peripheral session valid override value"]
    pub const fn set_avaloval(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "A-peripheral session valid override value"]
    pub const fn avaloval(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "B-peripheral session valid override enable"]
    pub const fn set_bvaloen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "B-peripheral session valid override enable"]
    pub const fn bvaloen(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "B-peripheral session valid override value"]
    pub const fn set_bvaloval(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "B-peripheral session valid override value"]
    pub const fn bvaloval(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host negotiation success"]
    pub const fn set_hngscs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host negotiation success"]
    pub const fn hngscs(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HNP request"]
    pub const fn set_hnprq(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HNP request"]
    pub const fn hnprq(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host set HNP enable"]
    pub const fn set_hshnpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host set HNP enable"]
    pub const fn hshnpen(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Device HNP enabled"]
    pub const fn set_dhnpen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Device HNP enabled"]
    pub const fn dhnpen(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Embedded host enable"]
    pub const fn set_ehen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Embedded host enable"]
    pub const fn ehen(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Connector ID status"]
    pub const fn set_cidsts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Connector ID status"]
    pub const fn cidsts(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Long/short debounce time"]
    pub const fn set_dbct(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Long/short debounce time"]
    pub const fn dbct(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "A-session valid"]
    pub const fn set_asvld(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "A-session valid"]
    pub const fn asvld(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "B-session valid"]
    pub const fn set_bsvld(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "B-session valid"]
    pub const fn bsvld(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt register"]
pub struct GotgintBits {
    bits: u32,
}
impl Default for GotgintBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GotgintBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Session end detected"]
    pub const fn set_sedet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Session end detected"]
    pub const fn sedet(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Session request success status change"]
    pub const fn set_srsschg(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Session request success status change"]
    pub const fn srsschg(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host negotiation success status change"]
    pub const fn set_hnsschg(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host negotiation success status change"]
    pub const fn hnsschg(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host negotiation detected"]
    pub const fn set_hngdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host negotiation detected"]
    pub const fn hngdet(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "A-device timeout change"]
    pub const fn set_adtochg(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "A-device timeout change"]
    pub const fn adtochg(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Debounce done"]
    pub const fn set_dbcdne(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Debounce done"]
    pub const fn dbcdne(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ID input pin changed"]
    pub const fn set_idchng(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ID input pin changed"]
    pub const fn idchng(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Reset register"]
pub struct GrstctlBits {
    bits: u32,
}
impl Default for GrstctlBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GrstctlBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Core soft reset"]
    pub const fn set_csrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Core soft reset"]
    pub const fn csrst(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HCLK soft reset"]
    pub const fn set_hsrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HCLK soft reset"]
    pub const fn hsrst(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Host frame counter reset"]
    pub const fn set_fcrst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Host frame counter reset"]
    pub const fn fcrst(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "RxFIFO flush"]
    pub const fn set_rxfflsh(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RxFIFO flush"]
    pub const fn rxfflsh(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TxFIFO flush"]
    pub const fn set_txfflsh(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TxFIFO flush"]
    pub const fn txfflsh(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TxFIFO number"]
    pub const fn set_txfnum(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0x6);
        self.bits |= (val as u32 & 0x1f) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "TxFIFO number"]
    pub const fn txfnum(self) -> u8 {
        ((self.bits >> 0x6) & 0x1f) as _
    }
    #[inline(always)]
    #[doc = "DMA request signal enabled for USB OTG HS"]
    pub const fn set_dmareq(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA request signal enabled for USB OTG HS"]
    pub const fn dmareq(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "AHB master idle"]
    pub const fn set_ahbidl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "AHB master idle"]
    pub const fn ahbidl(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Receive FIFO size register"]
pub struct GrxfsizBits {
    bits: u32,
}
impl Default for GrxfsizBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GrxfsizBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "RxFIFO depth"]
    pub const fn set_rxfd(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "RxFIFO depth"]
    pub const fn rxfd(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Status read and pop register"]
pub struct GrxstsBits {
    bits: u32,
}
impl Default for GrxstsBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GrxstsBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Endpoint number (device mode) / Channel number (host mode)"]
    pub const fn set_epnum(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x0);
        self.bits |= (val as u32 & 0xf) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Endpoint number (device mode) / Channel number (host mode)"]
    pub const fn epnum(self) -> u8 {
        ((self.bits >> 0x0) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Byte count"]
    pub const fn set_bcnt(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x4);
        self.bits |= (val as u32 & 0x7ff) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Byte count"]
    pub const fn bcnt(self) -> u16 {
        ((self.bits >> 0x4) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "Data PID"]
    pub const fn set_dpid(mut self, val: DpidVal) -> Self {
        self.bits &= !(0x3 << 0xf);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xf;
        self
    }
    #[inline(always)]
    #[doc = "Data PID"]
    pub const fn dpid(self) -> DpidVal {
        let val = ((self.bits >> 0xf) & 0x3) as _;
        unsafe { DpidVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Packet status (device mode)"]
    pub const fn set_pktstsd(mut self, val: PktstsdVal) -> Self {
        self.bits &= !(0xf << 0x11);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Packet status (device mode)"]
    pub const fn pktstsd(self) -> PktstsdVal {
        let val = ((self.bits >> 0x11) & 0xf) as _;
        unsafe { PktstsdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Packet status (host mode)"]
    pub const fn set_pktstsh(mut self, val: PktstshVal) -> Self {
        self.bits &= !(0xf << 0x11);
        self.bits |= (val.to_bits() as u32 & 0xf) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Packet status (host mode)"]
    pub const fn pktstsh(self) -> PktstshVal {
        let val = ((self.bits >> 0x11) & 0xf) as _;
        unsafe { PktstshVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Frame number (device mode)"]
    pub const fn set_frmnum(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x15);
        self.bits |= (val as u32 & 0xf) << 0x15;
        self
    }
    #[inline(always)]
    #[doc = "Frame number (device mode)"]
    pub const fn frmnum(self) -> u8 {
        ((self.bits >> 0x15) & 0xf) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "USB configuration register"]
pub struct GusbcfgBits {
    bits: u32,
}
impl Default for GusbcfgBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GusbcfgBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "FS timeout calibration"]
    pub const fn set_tocal(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x0);
        self.bits |= (val as u32 & 0x7) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "FS timeout calibration"]
    pub const fn tocal(self) -> u8 {
        ((self.bits >> 0x0) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Full-speed internal serial transceiver enable"]
    pub const fn set_physel(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Full-speed internal serial transceiver enable"]
    pub const fn physel(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "SRP-capable"]
    pub const fn set_srpcap(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "SRP-capable"]
    pub const fn srpcap(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "HNP-capable"]
    pub const fn set_hnpcap(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "HNP-capable"]
    pub const fn hnpcap(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "USB turnaround time"]
    pub const fn set_trdt(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0xa);
        self.bits |= (val as u32 & 0xf) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "USB turnaround time"]
    pub const fn trdt(self) -> u8 {
        ((self.bits >> 0xa) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "PHY Low-power clock select"]
    pub const fn set_phylpcs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Low-power clock select"]
    pub const fn phylpcs(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPI FS/LS select"]
    pub const fn set_ulpifsls(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPI FS/LS select"]
    pub const fn ulpifsls(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPI Auto-resume"]
    pub const fn set_ulpiar(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPI Auto-resume"]
    pub const fn ulpiar(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPI Clock SuspendM"]
    pub const fn set_ulpicsm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPI Clock SuspendM"]
    pub const fn ulpicsm(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPI External VBUS Drive"]
    pub const fn set_ulpievbusd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPI External VBUS Drive"]
    pub const fn ulpievbusd(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPI external VBUS indicator"]
    pub const fn set_ulpievbusi(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPI external VBUS indicator"]
    pub const fn ulpievbusi(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TermSel DLine pulsing selection"]
    pub const fn set_tsdps(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TermSel DLine pulsing selection"]
    pub const fn tsdps(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Indicator complement"]
    pub const fn set_pcci(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Indicator complement"]
    pub const fn pcci(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Indicator pass through"]
    pub const fn set_ptci(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Indicator pass through"]
    pub const fn ptci(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ULPI interface protect disable"]
    pub const fn set_ulpiipd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ULPI interface protect disable"]
    pub const fn ulpiipd(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force host mode"]
    pub const fn set_fhmod(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force host mode"]
    pub const fn fhmod(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Force device mode"]
    pub const fn set_fdmod(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Force device mode"]
    pub const fn fdmod(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Corrupt Tx packet"]
    pub const fn set_ctxpkt(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Corrupt Tx packet"]
    pub const fn ctxpkt(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host all channels interrupt register"]
pub struct HaintBits {
    bits: u32,
}
impl Default for HaintBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HaintBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Channel interrupts"]
    pub const fn set_haint(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Channel interrupts"]
    pub const fn haint(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host all channels interrupt mask register"]
pub struct HaintmskBits {
    bits: u32,
}
impl Default for HaintmskBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HaintmskBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Channel interrupt mask"]
    pub const fn set_haintm(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Channel interrupt mask"]
    pub const fn haintm(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host channel characteristics register"]
pub struct HccharBits {
    bits: u32,
}
impl Default for HccharBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HccharBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Maximum packet size"]
    pub const fn set_mpsiz(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x0);
        self.bits |= (val as u32 & 0x7ff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Maximum packet size"]
    pub const fn mpsiz(self) -> u16 {
        ((self.bits >> 0x0) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "Endpoint number"]
    pub const fn set_epnum(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0xb);
        self.bits |= (val as u32 & 0xf) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "Endpoint number"]
    pub const fn epnum(self) -> u8 {
        ((self.bits >> 0xb) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Endpoint direction"]
    pub const fn set_epdir(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Endpoint direction"]
    pub const fn epdir(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Low-speed device"]
    pub const fn set_lsdev(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Low-speed device"]
    pub const fn lsdev(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Endpoint type"]
    pub const fn set_eptyp(mut self, val: EptypVal) -> Self {
        self.bits &= !(0x3 << 0x12);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x12;
        self
    }
    #[inline(always)]
    #[doc = "Endpoint type"]
    pub const fn eptyp(self) -> EptypVal {
        let val = ((self.bits >> 0x12) & 0x3) as _;
        unsafe { EptypVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Multicount"]
    pub const fn set_mcnt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x14);
        self.bits |= (val as u32 & 0x3) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Multicount"]
    pub const fn mcnt(self) -> u8 {
        ((self.bits >> 0x14) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Device address"]
    pub const fn set_dad(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x16);
        self.bits |= (val as u32 & 0x7f) << 0x16;
        self
    }
    #[inline(always)]
    #[doc = "Device address"]
    pub const fn dad(self) -> u8 {
        ((self.bits >> 0x16) & 0x7f) as _
    }
    #[inline(always)]
    #[doc = "Odd frame (request iso/interrupt transaction to be performed on odd micro-frame)"]
    pub const fn set_oddfrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Odd frame (request iso/interrupt transaction to be performed on odd micro-frame)"]
    pub const fn oddfrm(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Channel disable"]
    pub const fn set_chdis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= if val { 1 << 0x1e } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Channel disable"]
    pub const fn chdis(self) -> bool {
        ((self.bits >> 0x1e) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Channel enable"]
    pub const fn set_chena(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Channel enable"]
    pub const fn chena(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host channel DMA config register"]
pub struct HcdmaBits {
    bits: u32,
}
impl Default for HcdmaBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HcdmaBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "QTD list base address"]
    pub const fn set_qtdaddr(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "QTD list base address"]
    pub const fn qtdaddr(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
    #[inline(always)]
    #[doc = "Current QTD (transfer descriptor) index"]
    pub const fn set_cqtd(mut self, val: u8) -> Self {
        self.bits &= !(0x3f << 0x3);
        self.bits |= (val as u32 & 0x3f) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Current QTD (transfer descriptor) index"]
    pub const fn cqtd(self) -> u8 {
        ((self.bits >> 0x3) & 0x3f) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host configuration register"]
pub struct HcfgBits {
    bits: u32,
}
impl Default for HcfgBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HcfgBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "FS/LS PHY clock select"]
    pub const fn set_fslspcs(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "FS/LS PHY clock select"]
    pub const fn fslspcs(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "FS- and LS-only support"]
    pub const fn set_fslss(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FS- and LS-only support"]
    pub const fn fslss(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Descriptor DMA-mode enable (qtd)"]
    pub const fn set_descdma(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Descriptor DMA-mode enable (qtd)"]
    pub const fn descdma(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Frame list length"]
    pub const fn set_frlistlen(mut self, val: FrlistlenVal) -> Self {
        self.bits &= !(0x3 << 0x18);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Frame list length"]
    pub const fn frlistlen(self) -> FrlistlenVal {
        let val = ((self.bits >> 0x18) & 0x3) as _;
        unsafe { FrlistlenVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Period scheduling enable"]
    pub const fn set_perschedena(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Period scheduling enable"]
    pub const fn perschedena(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host channel interrupt register"]
pub struct HcintBits {
    bits: u32,
}
impl Default for HcintBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HcintBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transfer completed"]
    pub const fn set_xfrc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer completed"]
    pub const fn xfrc(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Channel halted"]
    pub const fn set_chh(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Channel halted"]
    pub const fn chh(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "STALL response received interrupt"]
    pub const fn set_stall(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "STALL response received interrupt"]
    pub const fn stall(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "NAK response received interrupt"]
    pub const fn set_nak(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "NAK response received interrupt"]
    pub const fn nak(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ACK response received/transmitted interrupt"]
    pub const fn set_ack(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ACK response received/transmitted interrupt"]
    pub const fn ack(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transaction error"]
    pub const fn set_txerr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transaction error"]
    pub const fn txerr(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Babble error"]
    pub const fn set_bberr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Babble error"]
    pub const fn bberr(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Frame overrun"]
    pub const fn set_frmor(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Frame overrun"]
    pub const fn frmor(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data toggle error"]
    pub const fn set_dterr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data toggle error"]
    pub const fn dterr(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host channel mask register"]
pub struct HcintmskBits {
    bits: u32,
}
impl Default for HcintmskBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HcintmskBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transfer completed mask"]
    pub const fn set_xfrcm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transfer completed mask"]
    pub const fn xfrcm(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Channel halted mask"]
    pub const fn set_chhm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Channel halted mask"]
    pub const fn chhm(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "STALL response received interrupt mask"]
    pub const fn set_stallm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "STALL response received interrupt mask"]
    pub const fn stallm(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "NAK response received interrupt mask"]
    pub const fn set_nakm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "NAK response received interrupt mask"]
    pub const fn nakm(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "ACK response received/transmitted interrupt mask"]
    pub const fn set_ackm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "ACK response received/transmitted interrupt mask"]
    pub const fn ackm(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Response received interrupt mask"]
    pub const fn set_nyet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Response received interrupt mask"]
    pub const fn nyet(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transaction error mask"]
    pub const fn set_txerrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transaction error mask"]
    pub const fn txerrm(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Babble error mask"]
    pub const fn set_bberrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Babble error mask"]
    pub const fn bberrm(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Frame overrun mask"]
    pub const fn set_frmorm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Frame overrun mask"]
    pub const fn frmorm(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Data toggle error mask"]
    pub const fn set_dterrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Data toggle error mask"]
    pub const fn dterrm(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host channel transfer size register"]
pub struct HctsizBits {
    bits: u32,
}
impl Default for HctsizBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HctsizBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transfer size for non-isochronuous/interrupt pipes"]
    pub const fn set_xfrsiz(mut self, val: u32) -> Self {
        self.bits &= !(0x7ffff << 0x0);
        self.bits |= (val as u32 & 0x7ffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Transfer size for non-isochronuous/interrupt pipes"]
    pub const fn xfrsiz(self) -> u32 {
        ((self.bits >> 0x0) & 0x7ffff) as _
    }
    #[inline(always)]
    #[doc = "Schedule info for isochronuous & interrupt pipes (xfrsiz[7:0])"]
    pub const fn set_schedinfo(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Schedule info for isochronuous & interrupt pipes (xfrsiz[7:0])"]
    pub const fn schedinfo(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "NTD descriptor list length for isochronuous & interrupt pipes (xfrsiz[15:8], note val+1 is actual length)"]
    pub const fn set_ntdl(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x8);
        self.bits |= (val as u32 & 0xff) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "NTD descriptor list length for isochronuous & interrupt pipes (xfrsiz[15:8], note val+1 is actual length)"]
    pub const fn ntdl(self) -> u8 {
        ((self.bits >> 0x8) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Packet count"]
    pub const fn set_pktcnt(mut self, val: u16) -> Self {
        self.bits &= !(0x3ff << 0x13);
        self.bits |= (val as u32 & 0x3ff) << 0x13;
        self
    }
    #[inline(always)]
    #[doc = "Packet count"]
    pub const fn pktcnt(self) -> u16 {
        ((self.bits >> 0x13) & 0x3ff) as _
    }
    #[inline(always)]
    #[doc = "Data PID"]
    pub const fn set_dpid(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x1d);
        self.bits |= (val as u32 & 0x3) << 0x1d;
        self
    }
    #[inline(always)]
    #[doc = "Data PID"]
    pub const fn dpid(self) -> u8 {
        ((self.bits >> 0x1d) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Do Ping"]
    pub const fn set_doping(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Do Ping"]
    pub const fn doping(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host frame interval register"]
pub struct HfirBits {
    bits: u32,
}
impl Default for HfirBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HfirBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Frame interval"]
    pub const fn set_frivl(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Frame interval"]
    pub const fn frivl(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Dynamic Loading Control"]
    pub const fn set_rldctrl(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Dynamic Loading Control"]
    pub const fn rldctrl(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host frame number/frame time remaining register"]
pub struct HfnumBits {
    bits: u32,
}
impl Default for HfnumBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HfnumBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Frame number"]
    pub const fn set_frnum(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Frame number"]
    pub const fn frnum(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Frame time remaining"]
    pub const fn set_ftrem(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Frame time remaining"]
    pub const fn ftrem(self) -> u16 {
        ((self.bits >> 0x10) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Non-periodic transmit FIFO/queue status register"]
pub struct HnptxstsBits {
    bits: u32,
}
impl Default for HnptxstsBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HnptxstsBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Non-periodic TxFIFO space available"]
    pub const fn set_nptxfsav(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Non-periodic TxFIFO space available"]
    pub const fn nptxfsav(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Non-periodic transmit request queue space available"]
    pub const fn set_nptqxsav(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Non-periodic transmit request queue space available"]
    pub const fn nptqxsav(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Top of the non-periodic transmit request queue"]
    pub const fn set_nptxqtop(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x18);
        self.bits |= (val as u32 & 0x7f) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Top of the non-periodic transmit request queue"]
    pub const fn nptxqtop(self) -> u8 {
        ((self.bits >> 0x18) & 0x7f) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Host port control and status register"]
pub struct HprtBits {
    bits: u32,
}
impl Default for HprtBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HprtBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Port connect status"]
    pub const fn set_pcsts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port connect status"]
    pub const fn pcsts(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port connect detected"]
    pub const fn set_pcdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port connect detected"]
    pub const fn pcdet(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port enable (W1C)"]
    pub const fn set_pena(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port enable (W1C)"]
    pub const fn pena(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port enable/disable change"]
    pub const fn set_penchng(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port enable/disable change"]
    pub const fn penchng(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port overcurrent active"]
    pub const fn set_poca(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port overcurrent active"]
    pub const fn poca(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port overcurrent change"]
    pub const fn set_pocchng(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port overcurrent change"]
    pub const fn pocchng(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port resume"]
    pub const fn set_pres(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port resume"]
    pub const fn pres(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port suspend"]
    pub const fn set_psusp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port suspend"]
    pub const fn psusp(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port reset"]
    pub const fn set_prst(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port reset"]
    pub const fn prst(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port line status"]
    pub const fn set_plsts(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0xa);
        self.bits |= (val as u32 & 0x3) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "Port line status"]
    pub const fn plsts(self) -> u8 {
        ((self.bits >> 0xa) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Port power"]
    pub const fn set_ppwr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Port power"]
    pub const fn ppwr(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Port test control"]
    pub const fn set_ptctl(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0xd);
        self.bits |= (val as u32 & 0xf) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Port test control"]
    pub const fn ptctl(self) -> u8 {
        ((self.bits >> 0xd) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Port speed"]
    pub const fn set_pspd(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x11);
        self.bits |= (val as u32 & 0x3) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Port speed"]
    pub const fn pspd(self) -> u8 {
        ((self.bits >> 0x11) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Periodic transmit FIFO/queue status register"]
pub struct HptxstsBits {
    bits: u32,
}
impl Default for HptxstsBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HptxstsBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Periodic transmit data FIFO space available"]
    pub const fn set_ptxfsavl(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Periodic transmit data FIFO space available"]
    pub const fn ptxfsavl(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Periodic transmit request queue space available"]
    pub const fn set_ptxqsav(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x10);
        self.bits |= (val as u32 & 0xff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Periodic transmit request queue space available"]
    pub const fn ptxqsav(self) -> u8 {
        ((self.bits >> 0x10) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Top of the periodic transmit request queue"]
    pub const fn set_ptxqtop(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x18);
        self.bits |= (val as u32 & 0xff) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "Top of the periodic transmit request queue"]
    pub const fn ptxqtop(self) -> u8 {
        ((self.bits >> 0x18) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Power and clock gating control register"]
pub struct PcgcctlBits {
    bits: u32,
}
impl Default for PcgcctlBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PcgcctlBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Stop PHY clock"]
    pub const fn set_stppclk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Stop PHY clock"]
    pub const fn stppclk(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Gate HCLK"]
    pub const fn set_gatehclk(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Gate HCLK"]
    pub const fn gatehclk(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PHY Suspended"]
    pub const fn set_physusp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PHY Suspended"]
    pub const fn physusp(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DpidVal {
    Data0 = 0x0,

    Data2 = 0x1,

    Data1 = 0x2,

    Mdata = 0x3,
}
impl DpidVal {
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
pub enum DspdVal {
    #[doc = "High speed"]
    HighSpeed = 0x0,
    #[doc = "Full speed using external ULPI PHY"]
    FullSpeedExternal = 0x1,
    #[doc = "Full speed using internal embedded PHY"]
    FullSpeedInternal = 0x3,
}
impl DspdVal {
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
pub enum EptypVal {
    Control = 0x0,

    Isochronous = 0x1,

    Bulk = 0x2,

    Interrupt = 0x3,
}
impl EptypVal {
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
pub enum FrlistlenVal {
    #[doc = "Length = 8"]
    Len8 = 0x0,
    #[doc = "Length = 16"]
    Len16 = 0x1,
    #[doc = "Length = 32"]
    Len32 = 0x2,
    #[doc = "Length = 64"]
    Len64 = 0x3,
}
impl FrlistlenVal {
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
pub enum PfivlVal {
    #[doc = "80% of the frame interval"]
    FrameInterval80 = 0x0,
    #[doc = "85% of the frame interval"]
    FrameInterval85 = 0x1,
    #[doc = "90% of the frame interval"]
    FrameInterval90 = 0x2,
    #[doc = "95% of the frame interval"]
    FrameInterval95 = 0x3,
}
impl PfivlVal {
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
pub enum PktstsdVal {
    #[doc = "Global OUT NAK (triggers an interrupt)"]
    OutNak = 0x1,
    #[doc = "OUT data packet received"]
    OutDataRx = 0x2,
    #[doc = "OUT transfer completed (triggers an interrupt)"]
    OutDataDone = 0x3,
    #[doc = "SETUP transaction completed (triggers an interrupt)"]
    SetupDataDone = 0x4,
    #[doc = "SETUP data packet received"]
    SetupDataRx = 0x6,
}
impl PktstsdVal {
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
pub enum PktstshVal {
    #[doc = "IN data packet received"]
    InDataRx = 0x2,
    #[doc = "IN transfer completed (triggers an interrupt)"]
    InDataDone = 0x3,
    #[doc = "Data toggle error (triggers an interrupt)"]
    DataToggleErr = 0x5,
    #[doc = "Channel halted (triggers an interrupt)"]
    ChannelHalted = 0x7,
}
impl PktstshVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
