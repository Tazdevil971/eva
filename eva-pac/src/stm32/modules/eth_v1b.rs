
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Ethernet Peripheral"]
pub struct Eth {
    ptr: *mut u8,
}
impl Eth {
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
    #[doc = "Ethernet: media access control (MAC)"]
    pub const fn ethernet_mac(&self) -> EthernetMac {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <EthernetMac>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet: Precision Time Protocol (PTP)"]
    pub const fn ethernet_ptp(&self) -> EthernetPtp {
        unsafe {
            let ptr = self.ptr.add(0x700);
            <EthernetPtp>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet: DMA mode register (DMA)"]
    pub const fn ethernet_dma(&self) -> EthernetDma {
        unsafe {
            let ptr = self.ptr.add(0x1000);
            <EthernetDma>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Ethernet: DMA controller operation"]
pub struct EthernetDma {
    ptr: *mut u8,
}
impl EthernetDma {
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
    #[doc = "Ethernet DMA bus mode register"]
    pub const fn dmabmr(&self) -> utils::Reg<DmabmrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<DmabmrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA transmit poll demand register"]
    pub const fn dmatpdr(&self) -> utils::Reg<DmatpdrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<DmatpdrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "EHERNET DMA receive poll demand register"]
    pub const fn dmarpdr(&self) -> utils::Reg<DmarpdrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<DmarpdrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA receive descriptor list address register"]
    pub const fn dmardlar(&self) -> utils::Reg<DmardlarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<DmardlarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA transmit descriptor list address register"]
    pub const fn dmatdlar(&self) -> utils::Reg<DmatdlarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<DmatdlarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA status register"]
    pub const fn dmasr(&self) -> utils::Reg<DmasrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<DmasrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA operation mode register"]
    pub const fn dmaomr(&self) -> utils::Reg<DmaomrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<DmaomrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA interrupt enable register"]
    pub const fn dmaier(&self) -> utils::Reg<DmaierBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<DmaierBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
    pub const fn dmamfbocr(&self) -> utils::Reg<DmamfbocrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<DmamfbocrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA receive status watchdog timer register"]
    pub const fn dmarswtr(&self) -> utils::Reg<DmarswtrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<DmarswtrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA current host transmit descriptor register"]
    pub const fn dmachtdr(&self) -> utils::Reg<DmachtdrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<DmachtdrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA current host receive descriptor register"]
    pub const fn dmachrdr(&self) -> utils::Reg<DmachrdrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<DmachrdrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA current host transmit buffer address register"]
    pub const fn dmachtbar(&self) -> utils::Reg<DmachtbarBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<DmachtbarBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet DMA current host receive buffer address register"]
    pub const fn dmachrbar(&self) -> utils::Reg<DmachrbarBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x54);
            <utils::Reg<DmachrbarBits, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Ethernet: media access control (MAC)"]
pub struct EthernetMac {
    ptr: *mut u8,
}
impl EthernetMac {
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
    #[doc = "Ethernet MAC configuration register"]
    pub const fn maccr(&self) -> utils::Reg<MaccrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<MaccrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC frame filter register"]
    pub const fn macffr(&self) -> utils::Reg<MacffrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<MacffrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC hash table high register"]
    pub const fn machthr(&self) -> utils::Reg<MachthrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<MachthrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC hash table low register"]
    pub const fn machtlr(&self) -> utils::Reg<MachtlrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<MachtlrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC MII address register"]
    pub const fn macmiiar(&self) -> utils::Reg<MacmiiarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<MacmiiarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC MII data register"]
    pub const fn macmiidr(&self) -> utils::Reg<MacmiidrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<MacmiidrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC flow control register"]
    pub const fn macfcr(&self) -> utils::Reg<MacfcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<MacfcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC VLAN tag register"]
    pub const fn macvlantr(&self) -> utils::Reg<MacvlantrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<MacvlantrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC remote wakeup frame filter register"]
    pub const fn macrwuffr(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC PMT control and status register"]
    pub const fn macpmtcsr(&self) -> utils::Reg<MacpmtcsrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<MacpmtcsrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC debug register"]
    pub const fn macdbgr(&self) -> utils::Reg<MacdbgrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<MacdbgrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC interrupt status register"]
    pub const fn macsr(&self) -> utils::Reg<MacsrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<MacsrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC interrupt mask register"]
    pub const fn macimr(&self) -> utils::Reg<MacimrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c);
            <utils::Reg<MacimrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 0 high register"]
    pub const fn maca0hr(&self) -> utils::Reg<Maca0hrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<Maca0hrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 0 low register"]
    pub const fn maca0lr(&self) -> utils::Reg<Maca0lrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<Maca0lrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 1/2/3 high register"]
    pub const fn macahr(&self, idx: usize) -> utils::Reg<MacahrBits, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x48 + idx * 0x8);
            <utils::Reg<MacahrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 1/2/3 low register"]
    pub const fn macalr(&self, idx: usize) -> utils::Reg<MacalrBits, utils::RW> {
        assert!(idx < 3);
        unsafe {
            let ptr = self.ptr.add(0x4c + idx * 0x8);
            <utils::Reg<MacalrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MMC control register"]
    pub const fn mmccr(&self) -> utils::Reg<MmccrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x100);
            <utils::Reg<MmccrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MMC receive interrupt register"]
    pub const fn mmcrir(&self) -> utils::Reg<MmcrirBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x104);
            <utils::Reg<MmcrirBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MMC transmit interrupt register"]
    pub const fn mmctir(&self) -> utils::Reg<MmctirBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x108);
            <utils::Reg<MmctirBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MMC receive interrupt mask register"]
    pub const fn mmcrimr(&self) -> utils::Reg<MmcrimrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10c);
            <utils::Reg<MmcrimrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MMC transmit interrupt mask register"]
    pub const fn mmctimr(&self) -> utils::Reg<MmctimrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x110);
            <utils::Reg<MmctimrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
    pub const fn mmctgfsccr(&self) -> utils::Reg<MmctgfsccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x14c);
            <utils::Reg<MmctgfsccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
    pub const fn mmctgfmsccr(&self) -> utils::Reg<MmctgfmsccrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x150);
            <utils::Reg<MmctgfmsccrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MMC transmitted good frames counter register"]
    pub const fn mmctgfcr(&self) -> utils::Reg<MmctgfcrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x168);
            <utils::Reg<MmctgfcrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MMC received frames with CRC error counter register"]
    pub const fn mmcrfcecr(&self) -> utils::Reg<MmcrfcecrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x194);
            <utils::Reg<MmcrfcecrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet MMC received frames with alignment error counter register"]
    pub const fn mmcrfaecr(&self) -> utils::Reg<MmcrfaecrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x198);
            <utils::Reg<MmcrfaecrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "MMC received good unicast frames counter register"]
    pub const fn mmcrgufcr(&self) -> utils::Reg<MmcrgufcrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x1c4);
            <utils::Reg<MmcrgufcrBits, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Ethernet: Precision time protocol"]
pub struct EthernetPtp {
    ptr: *mut u8,
}
impl EthernetPtp {
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
    #[doc = "Ethernet PTP time stamp control register"]
    pub const fn ptptscr(&self) -> utils::Reg<PtptscrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<PtptscrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet PTP subsecond increment register"]
    pub const fn ptpssir(&self) -> utils::Reg<PtpssirBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<PtpssirBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet PTP time stamp high register"]
    pub const fn ptptshr(&self) -> utils::Reg<PtptshrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<PtptshrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet PTP time stamp low register"]
    pub const fn ptptslr(&self) -> utils::Reg<PtptslrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<PtptslrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet PTP time stamp high update register"]
    pub const fn ptptshur(&self) -> utils::Reg<PtptshurBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<PtptshurBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet PTP time stamp low update register"]
    pub const fn ptptslur(&self) -> utils::Reg<PtptslurBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<PtptslurBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet PTP time stamp addend register"]
    pub const fn ptptsar(&self) -> utils::Reg<PtptsarBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<PtptsarBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet PTP target time high register"]
    pub const fn ptptthr(&self) -> utils::Reg<PtptthrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<PtptthrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet PTP target time low register"]
    pub const fn ptpttlr(&self) -> utils::Reg<PtpttlrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<PtpttlrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet PTP time stamp status register"]
    pub const fn ptptssr(&self) -> utils::Reg<PtptssrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<PtptssrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Ethernet PTP PPS control register"]
    pub const fn ptpppscr(&self) -> utils::Reg<PtpppscrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<PtpppscrBits, utils::RO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA bus mode register"]
pub struct DmabmrBits {
    bits: u32,
}
impl Default for DmabmrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmabmrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Software reset"]
    pub const fn set_sr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Software reset"]
    pub const fn sr(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA arbitration"]
    pub const fn set_da(mut self, val: DaVal) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "DMA arbitration"]
    pub const fn da(self) -> DaVal {
        let val = ((self.bits >> 0x1) & 0x1) as _;
        unsafe { DaVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Descriptor skip length"]
    pub const fn set_dsl(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0x2);
        self.bits |= (val as u32 & 0x1f) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Descriptor skip length"]
    pub const fn dsl(self) -> u8 {
        ((self.bits >> 0x2) & 0x1f) as _
    }
    #[inline(always)]
    #[doc = "Enhanced descriptor format enable"]
    pub const fn set_edfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Enhanced descriptor format enable"]
    pub const fn edfe(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Programmable burst length"]
    pub const fn set_pbl(mut self, val: PblVal) -> Self {
        self.bits &= !(0x3f << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x3f) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Programmable burst length"]
    pub const fn pbl(self) -> PblVal {
        let val = ((self.bits >> 0x8) & 0x3f) as _;
        unsafe { PblVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Rx-Tx priority ratio"]
    pub const fn set_pm(mut self, val: PriorityRxOverTxVal) -> Self {
        self.bits &= !(0x3 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Rx-Tx priority ratio"]
    pub const fn pm(self) -> PriorityRxOverTxVal {
        let val = ((self.bits >> 0xe) & 0x3) as _;
        unsafe { PriorityRxOverTxVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Fixed burst"]
    pub const fn set_fb(mut self, val: FbVal) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Fixed burst"]
    pub const fn fb(self) -> FbVal {
        let val = ((self.bits >> 0x10) & 0x1) as _;
        unsafe { FbVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Rx DMA PBL"]
    pub const fn set_rdp(mut self, val: RdpVal) -> Self {
        self.bits &= !(0x3f << 0x11);
        self.bits |= (val.to_bits() as u32 & 0x3f) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Rx DMA PBL"]
    pub const fn rdp(self) -> RdpVal {
        let val = ((self.bits >> 0x11) & 0x3f) as _;
        unsafe { RdpVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Use separate PBL"]
    pub const fn set_usp(mut self, val: UspVal) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x17;
        self
    }
    #[inline(always)]
    #[doc = "Use separate PBL"]
    pub const fn usp(self) -> UspVal {
        let val = ((self.bits >> 0x17) & 0x1) as _;
        unsafe { UspVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "4xPBL mode"]
    pub const fn set_fpm(mut self, val: FpmVal) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "4xPBL mode"]
    pub const fn fpm(self) -> FpmVal {
        let val = ((self.bits >> 0x18) & 0x1) as _;
        unsafe { FpmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Address-aligned beats"]
    pub const fn set_aab(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Address-aligned beats"]
    pub const fn aab(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mixed burst"]
    pub const fn set_mb(mut self, val: MbVal) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1a;
        self
    }
    #[inline(always)]
    #[doc = "Mixed burst"]
    pub const fn mb(self) -> MbVal {
        let val = ((self.bits >> 0x1a) & 0x1) as _;
        unsafe { MbVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA current host receive buffer address register"]
pub struct DmachrbarBits {
    bits: u32,
}
impl Default for DmachrbarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmachrbarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Host receive buffer address pointer"]
    pub const fn set_hrbap(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Host receive buffer address pointer"]
    pub const fn hrbap(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA current host receive descriptor register"]
pub struct DmachrdrBits {
    bits: u32,
}
impl Default for DmachrdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmachrdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Host receive descriptor address pointer"]
    pub const fn set_hrdap(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Host receive descriptor address pointer"]
    pub const fn hrdap(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub struct DmachtbarBits {
    bits: u32,
}
impl Default for DmachtbarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmachtbarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Host transmit buffer address pointer"]
    pub const fn set_htbap(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Host transmit buffer address pointer"]
    pub const fn htbap(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub struct DmachtdrBits {
    bits: u32,
}
impl Default for DmachtdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmachtdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Host transmit descriptor address pointer"]
    pub const fn set_htdap(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Host transmit descriptor address pointer"]
    pub const fn htdap(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA interrupt enable register"]
pub struct DmaierBits {
    bits: u32,
}
impl Default for DmaierBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmaierBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transmit interrupt enable"]
    pub const fn set_tie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit interrupt enable"]
    pub const fn tie(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit process stopped interrupt enable"]
    pub const fn set_tpsie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit process stopped interrupt enable"]
    pub const fn tpsie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit buffer unavailable interrupt enable"]
    pub const fn set_tbuie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit buffer unavailable interrupt enable"]
    pub const fn tbuie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit jabber timeout interrupt enable"]
    pub const fn set_tjtie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit jabber timeout interrupt enable"]
    pub const fn tjtie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive overflow interrupt enable"]
    pub const fn set_roie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive overflow interrupt enable"]
    pub const fn roie(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit underflow interrupt enable"]
    pub const fn set_tuie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit underflow interrupt enable"]
    pub const fn tuie(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive interrupt enable"]
    pub const fn set_rie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive interrupt enable"]
    pub const fn rie(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive buffer unavailable interrupt enable"]
    pub const fn set_rbuie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive buffer unavailable interrupt enable"]
    pub const fn rbuie(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive process stopped interrupt enable"]
    pub const fn set_rpsie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive process stopped interrupt enable"]
    pub const fn rpsie(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive watchdog timeout interrupt enable"]
    pub const fn set_rwtie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive watchdog timeout interrupt enable"]
    pub const fn rwtie(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Early transmit interrupt enable"]
    pub const fn set_etie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Early transmit interrupt enable"]
    pub const fn etie(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Fatal bus error interrupt enable"]
    pub const fn set_fbeie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Fatal bus error interrupt enable"]
    pub const fn fbeie(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Early receive interrupt enable"]
    pub const fn set_erie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Early receive interrupt enable"]
    pub const fn erie(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Abnormal interrupt summary enable"]
    pub const fn set_aise(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Abnormal interrupt summary enable"]
    pub const fn aise(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Normal interrupt summary enable"]
    pub const fn set_nise(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Normal interrupt summary enable"]
    pub const fn nise(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub struct DmamfbocrBits {
    bits: u32,
}
impl Default for DmamfbocrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmamfbocrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Missed frames by the controller"]
    pub const fn set_mfc(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Missed frames by the controller"]
    pub const fn mfc(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Overflow bit for missed frame counter"]
    pub const fn set_omfc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overflow bit for missed frame counter"]
    pub const fn omfc(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Missed frames by the application"]
    pub const fn set_mfa(mut self, val: u16) -> Self {
        self.bits &= !(0x7ff << 0x11);
        self.bits |= (val as u32 & 0x7ff) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Missed frames by the application"]
    pub const fn mfa(self) -> u16 {
        ((self.bits >> 0x11) & 0x7ff) as _
    }
    #[inline(always)]
    #[doc = "Overflow bit for FIFO overflow counter"]
    pub const fn set_ofoc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overflow bit for FIFO overflow counter"]
    pub const fn ofoc(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA operation mode register"]
pub struct DmaomrBits {
    bits: u32,
}
impl Default for DmaomrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmaomrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Start/stop receive"]
    pub const fn set_sr(mut self, val: DmaomrSrVal) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Start/stop receive"]
    pub const fn sr(self) -> DmaomrSrVal {
        let val = ((self.bits >> 0x1) & 0x1) as _;
        unsafe { DmaomrSrVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Operate on second frame"]
    pub const fn set_osf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Operate on second frame"]
    pub const fn osf(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive threshold control"]
    pub const fn set_rtc(mut self, val: RtcVal) -> Self {
        self.bits &= !(0x3 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Receive threshold control"]
    pub const fn rtc(self) -> RtcVal {
        let val = ((self.bits >> 0x3) & 0x3) as _;
        unsafe { RtcVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Forward undersized good frames"]
    pub const fn set_fugf(mut self, val: FugfVal) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Forward undersized good frames"]
    pub const fn fugf(self) -> FugfVal {
        let val = ((self.bits >> 0x6) & 0x1) as _;
        unsafe { FugfVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Forward error frames"]
    pub const fn set_fef(mut self, val: FefVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "Forward error frames"]
    pub const fn fef(self) -> FefVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { FefVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Start/stop transmission"]
    pub const fn set_st(mut self, val: StVal) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Start/stop transmission"]
    pub const fn st(self) -> StVal {
        let val = ((self.bits >> 0xd) & 0x1) as _;
        unsafe { StVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Transmit threshold control"]
    pub const fn set_ttc(mut self, val: TtcVal) -> Self {
        self.bits &= !(0x7 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Transmit threshold control"]
    pub const fn ttc(self) -> TtcVal {
        let val = ((self.bits >> 0xe) & 0x7) as _;
        unsafe { TtcVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Flush transmit FIFO"]
    pub const fn set_ftf(mut self, val: FtfVal) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Flush transmit FIFO"]
    pub const fn ftf(self) -> FtfVal {
        let val = ((self.bits >> 0x14) & 0x1) as _;
        unsafe { FtfVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Transmit store and forward"]
    pub const fn set_tsf(mut self, val: TsfVal) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x15;
        self
    }
    #[inline(always)]
    #[doc = "Transmit store and forward"]
    pub const fn tsf(self) -> TsfVal {
        let val = ((self.bits >> 0x15) & 0x1) as _;
        unsafe { TsfVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Disable flushing of received frames"]
    pub const fn set_dfrf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Disable flushing of received frames"]
    pub const fn dfrf(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive store and forward"]
    pub const fn set_rsf(mut self, val: RsfVal) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x19;
        self
    }
    #[inline(always)]
    #[doc = "Receive store and forward"]
    pub const fn rsf(self) -> RsfVal {
        let val = ((self.bits >> 0x19) & 0x1) as _;
        unsafe { RsfVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Dropping of TCP/IP checksum error frames disable"]
    pub const fn set_dtcefd(mut self, val: DtcefdVal) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1a;
        self
    }
    #[inline(always)]
    #[doc = "Dropping of TCP/IP checksum error frames disable"]
    pub const fn dtcefd(self) -> DtcefdVal {
        let val = ((self.bits >> 0x1a) & 0x1) as _;
        unsafe { DtcefdVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA receive descriptor list address register"]
pub struct DmardlarBits {
    bits: u32,
}
impl Default for DmardlarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmardlarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Start of receive list"]
    pub const fn set_srl(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Start of receive list"]
    pub const fn srl(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "EHERNET DMA receive poll demand register"]
pub struct DmarpdrBits {
    bits: u32,
}
impl Default for DmarpdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmarpdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Receive poll demand"]
    pub const fn set_rpd(mut self, val: RpdVal) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val.to_bits() as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Receive poll demand"]
    pub const fn rpd(self) -> RpdVal {
        let val = ((self.bits >> 0x0) & 0xffffffff) as _;
        unsafe { RpdVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA receive status watchdog timer register"]
pub struct DmarswtrBits {
    bits: u32,
}
impl Default for DmarswtrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmarswtrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Receive status watchdog timer count"]
    pub const fn set_rswtc(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Receive status watchdog timer count"]
    pub const fn rswtc(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA status register"]
pub struct DmasrBits {
    bits: u32,
}
impl Default for DmasrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmasrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transmit status"]
    pub const fn set_ts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit status"]
    pub const fn ts(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit process stopped status"]
    pub const fn set_tpss(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit process stopped status"]
    pub const fn tpss(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit buffer unavailable status"]
    pub const fn set_tbus(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit buffer unavailable status"]
    pub const fn tbus(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit jabber timeout status"]
    pub const fn set_tjts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit jabber timeout status"]
    pub const fn tjts(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive overflow status"]
    pub const fn set_ros(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive overflow status"]
    pub const fn ros(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmit underflow status"]
    pub const fn set_tus(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit underflow status"]
    pub const fn tus(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive status"]
    pub const fn set_rs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive status"]
    pub const fn rs(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive buffer unavailable status"]
    pub const fn set_rbus(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive buffer unavailable status"]
    pub const fn rbus(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive process stopped status"]
    pub const fn set_rpss(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive process stopped status"]
    pub const fn rpss(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PWTS"]
    pub const fn set_pwts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PWTS"]
    pub const fn pwts(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Early transmit status"]
    pub const fn set_ets(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Early transmit status"]
    pub const fn ets(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Fatal bus error status"]
    pub const fn set_fbes(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Fatal bus error status"]
    pub const fn fbes(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Early receive status"]
    pub const fn set_ers(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Early receive status"]
    pub const fn ers(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Abnormal interrupt summary"]
    pub const fn set_ais(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Abnormal interrupt summary"]
    pub const fn ais(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Normal interrupt summary"]
    pub const fn set_nis(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Normal interrupt summary"]
    pub const fn nis(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive process state"]
    pub const fn set_rps(mut self, val: RpsVal) -> Self {
        self.bits &= !(0x7 << 0x11);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Receive process state"]
    pub const fn rps(self) -> RpsVal {
        let val = ((self.bits >> 0x11) & 0x7) as _;
        unsafe { RpsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Transmit process state"]
    pub const fn set_tps(mut self, val: TpsVal) -> Self {
        self.bits &= !(0x7 << 0x14);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Transmit process state"]
    pub const fn tps(self) -> TpsVal {
        let val = ((self.bits >> 0x14) & 0x7) as _;
        unsafe { TpsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Error bits status"]
    pub const fn set_ebs(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0x17);
        self.bits |= (val as u32 & 0x7) << 0x17;
        self
    }
    #[inline(always)]
    #[doc = "Error bits status"]
    pub const fn ebs(self) -> u8 {
        ((self.bits >> 0x17) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "MMC status"]
    pub const fn set_mmcs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1b);
        self.bits |= if val { 1 << 0x1b } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MMC status"]
    pub const fn mmcs(self) -> bool {
        ((self.bits >> 0x1b) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "PMT status"]
    pub const fn set_pmts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1c);
        self.bits |= if val { 1 << 0x1c } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PMT status"]
    pub const fn pmts(self) -> bool {
        ((self.bits >> 0x1c) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Time stamp trigger status"]
    pub const fn set_tsts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1d);
        self.bits |= if val { 1 << 0x1d } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Time stamp trigger status"]
    pub const fn tsts(self) -> bool {
        ((self.bits >> 0x1d) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub struct DmatdlarBits {
    bits: u32,
}
impl Default for DmatdlarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmatdlarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Start of transmit list"]
    pub const fn set_stl(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Start of transmit list"]
    pub const fn stl(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet DMA transmit poll demand register"]
pub struct DmatpdrBits {
    bits: u32,
}
impl Default for DmatpdrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DmatpdrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transmit poll demand"]
    pub const fn set_tpd(mut self, val: TpdVal) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val.to_bits() as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Transmit poll demand"]
    pub const fn tpd(self) -> TpdVal {
        let val = ((self.bits >> 0x0) & 0xffffffff) as _;
        unsafe { TpdVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC address 0 high register"]
pub struct Maca0hrBits {
    bits: u32,
}
impl Default for Maca0hrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Maca0hrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 0 high"]
    pub const fn set_maca0h(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 0 high"]
    pub const fn maca0h(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "Always 1"]
    pub const fn set_mo(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Always 1"]
    pub const fn mo(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC address 0 low register"]
pub struct Maca0lrBits {
    bits: u32,
}
impl Default for Maca0lrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Maca0lrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 0 low"]
    pub const fn set_maca0l(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 0 low"]
    pub const fn maca0l(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC address 1/2/3 high register"]
pub struct MacahrBits {
    bits: u32,
}
impl Default for MacahrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacahrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 1/2/3 high"]
    pub const fn set_macah(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 1/2/3 high"]
    pub const fn macah(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "MBC"]
    pub const fn set_mbc(mut self, val: u8) -> Self {
        self.bits &= !(0x3f << 0x18);
        self.bits |= (val as u32 & 0x3f) << 0x18;
        self
    }
    #[inline(always)]
    #[doc = "MBC"]
    pub const fn mbc(self) -> u8 {
        ((self.bits >> 0x18) & 0x3f) as _
    }
    #[inline(always)]
    #[doc = "SA"]
    pub const fn set_sa(mut self, val: MacahrSaVal) -> Self {
        self.bits &= !(0x1 << 0x1e);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1e;
        self
    }
    #[inline(always)]
    #[doc = "SA"]
    pub const fn sa(self) -> MacahrSaVal {
        let val = ((self.bits >> 0x1e) & 0x1) as _;
        unsafe { MacahrSaVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "AE"]
    pub const fn set_ae(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "AE"]
    pub const fn ae(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC address 1/2/3 low register"]
pub struct MacalrBits {
    bits: u32,
}
impl Default for MacalrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacalrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 1/2/3 low"]
    pub const fn set_macal(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Ethernet MAC address 1/2/3 low"]
    pub const fn macal(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC configuration register"]
pub struct MaccrBits {
    bits: u32,
}
impl Default for MaccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MaccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Receiver enable"]
    pub const fn set_re(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receiver enable"]
    pub const fn re(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmitter enable"]
    pub const fn set_te(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmitter enable"]
    pub const fn te(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Deferral check"]
    pub const fn set_dc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Deferral check"]
    pub const fn dc(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Back-off limit"]
    pub const fn set_bl(mut self, val: BlVal) -> Self {
        self.bits &= !(0x3 << 0x5);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Back-off limit"]
    pub const fn bl(self) -> BlVal {
        let val = ((self.bits >> 0x5) & 0x3) as _;
        unsafe { BlVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Automatic pad/CRC stripping"]
    pub const fn set_apcs(mut self, val: ApcsVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "Automatic pad/CRC stripping"]
    pub const fn apcs(self) -> ApcsVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { ApcsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Retry disable"]
    pub const fn set_rd(mut self, val: RdVal) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "Retry disable"]
    pub const fn rd(self) -> RdVal {
        let val = ((self.bits >> 0x9) & 0x1) as _;
        unsafe { RdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "IPv4 checksum offload"]
    pub const fn set_ipco(mut self, val: IpcoVal) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "IPv4 checksum offload"]
    pub const fn ipco(self) -> IpcoVal {
        let val = ((self.bits >> 0xa) & 0x1) as _;
        unsafe { IpcoVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Duplex mode"]
    pub const fn set_dm(mut self, val: DmVal) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "Duplex mode"]
    pub const fn dm(self) -> DmVal {
        let val = ((self.bits >> 0xb) & 0x1) as _;
        unsafe { DmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Loopback mode"]
    pub const fn set_lm(mut self, val: LmVal) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "Loopback mode"]
    pub const fn lm(self) -> LmVal {
        let val = ((self.bits >> 0xc) & 0x1) as _;
        unsafe { LmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Receive own disable"]
    pub const fn set_rod(mut self, val: RodVal) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Receive own disable"]
    pub const fn rod(self) -> RodVal {
        let val = ((self.bits >> 0xd) & 0x1) as _;
        unsafe { RodVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Fast Ethernet speed"]
    pub const fn set_fes(mut self, val: FesVal) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Fast Ethernet speed"]
    pub const fn fes(self) -> FesVal {
        let val = ((self.bits >> 0xe) & 0x1) as _;
        unsafe { FesVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Carrier sense disable"]
    pub const fn set_csd(mut self, val: CsdVal) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Carrier sense disable"]
    pub const fn csd(self) -> CsdVal {
        let val = ((self.bits >> 0x10) & 0x1) as _;
        unsafe { CsdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Interframe gap"]
    pub const fn set_ifg(mut self, val: IfgVal) -> Self {
        self.bits &= !(0x7 << 0x11);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Interframe gap"]
    pub const fn ifg(self) -> IfgVal {
        let val = ((self.bits >> 0x11) & 0x7) as _;
        unsafe { IfgVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Jabber disable"]
    pub const fn set_jd(mut self, val: JdVal) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x16;
        self
    }
    #[inline(always)]
    #[doc = "Jabber disable"]
    pub const fn jd(self) -> JdVal {
        let val = ((self.bits >> 0x16) & 0x1) as _;
        unsafe { JdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Watchdog disable"]
    pub const fn set_wd(mut self, val: WdVal) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x17;
        self
    }
    #[inline(always)]
    #[doc = "Watchdog disable"]
    pub const fn wd(self) -> WdVal {
        let val = ((self.bits >> 0x17) & 0x1) as _;
        unsafe { WdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "CRC stripping for type frames"]
    pub const fn set_cstf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CRC stripping for type frames"]
    pub const fn cstf(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC debug register"]
pub struct MacdbgrBits {
    bits: u32,
}
impl Default for MacdbgrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacdbgrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "MAC MII receive protocol engine active"]
    pub const fn set_mmrpea(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MAC MII receive protocol engine active"]
    pub const fn mmrpea(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MAC small FIFO read/write controllers status"]
    pub const fn set_msfrwcs(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x1);
        self.bits |= (val as u32 & 0x3) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "MAC small FIFO read/write controllers status"]
    pub const fn msfrwcs(self) -> u8 {
        ((self.bits >> 0x1) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Rx FIFO write controller active"]
    pub const fn set_rfwra(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Rx FIFO write controller active"]
    pub const fn rfwra(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Rx FIFO read controller status"]
    pub const fn set_rfrcs(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x5);
        self.bits |= (val as u32 & 0x3) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Rx FIFO read controller status"]
    pub const fn rfrcs(self) -> u8 {
        ((self.bits >> 0x5) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Rx FIFO fill level"]
    pub const fn set_rffl(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x8);
        self.bits |= (val as u32 & 0x3) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Rx FIFO fill level"]
    pub const fn rffl(self) -> u8 {
        ((self.bits >> 0x8) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "MAC MII transmit engine active"]
    pub const fn set_mmtea(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MAC MII transmit engine active"]
    pub const fn mmtea(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MAC transmit frame controller status"]
    pub const fn set_mtfcs(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x11);
        self.bits |= (val as u32 & 0x3) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "MAC transmit frame controller status"]
    pub const fn mtfcs(self) -> u8 {
        ((self.bits >> 0x11) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "MAC transmitter in pause"]
    pub const fn set_mtp(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MAC transmitter in pause"]
    pub const fn mtp(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tx FIFO read status"]
    pub const fn set_tfrs(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x14);
        self.bits |= (val as u32 & 0x3) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Tx FIFO read status"]
    pub const fn tfrs(self) -> u8 {
        ((self.bits >> 0x14) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Tx FIFO write active"]
    pub const fn set_tfwa(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tx FIFO write active"]
    pub const fn tfwa(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tx FIFO not empty"]
    pub const fn set_tfne(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tx FIFO not empty"]
    pub const fn tfne(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tx FIFO full"]
    pub const fn set_tff(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tx FIFO full"]
    pub const fn tff(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC flow control register"]
pub struct MacfcrBits {
    bits: u32,
}
impl Default for MacfcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacfcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Flow control busy/back pressure activate"]
    pub const fn set_fcb(mut self, val: FcbVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Flow control busy/back pressure activate"]
    pub const fn fcb(self) -> FcbVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { FcbVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Transmit flow control enable"]
    pub const fn set_tfce(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmit flow control enable"]
    pub const fn tfce(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Receive flow control enable"]
    pub const fn set_rfce(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive flow control enable"]
    pub const fn rfce(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Unicast pause frame detect"]
    pub const fn set_upfd(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Unicast pause frame detect"]
    pub const fn upfd(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Pause low threshold"]
    pub const fn set_plt(mut self, val: PltVal) -> Self {
        self.bits &= !(0x3 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Pause low threshold"]
    pub const fn plt(self) -> PltVal {
        let val = ((self.bits >> 0x4) & 0x3) as _;
        unsafe { PltVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Zero-quanta pause disable"]
    pub const fn set_zqpd(mut self, val: ZqpdVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "Zero-quanta pause disable"]
    pub const fn zqpd(self) -> ZqpdVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { ZqpdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Pause time"]
    pub const fn set_pt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Pause time"]
    pub const fn pt(self) -> u16 {
        ((self.bits >> 0x10) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC frame filter register"]
pub struct MacffrBits {
    bits: u32,
}
impl Default for MacffrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacffrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Promiscuous mode"]
    pub const fn set_pm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Promiscuous mode"]
    pub const fn pm(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Hash unicast"]
    pub const fn set_hu(mut self, val: HuVal) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Hash unicast"]
    pub const fn hu(self) -> HuVal {
        let val = ((self.bits >> 0x1) & 0x1) as _;
        unsafe { HuVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Hash multicast"]
    pub const fn set_hm(mut self, val: HmVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Hash multicast"]
    pub const fn hm(self) -> HmVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { HmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Destination address unique filtering"]
    pub const fn set_daif(mut self, val: DaifVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Destination address unique filtering"]
    pub const fn daif(self) -> DaifVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { DaifVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Pass all multicast"]
    pub const fn set_pam(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Pass all multicast"]
    pub const fn pam(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Broadcast frames disable"]
    pub const fn set_bfd(mut self, val: BfdVal) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Broadcast frames disable"]
    pub const fn bfd(self) -> BfdVal {
        let val = ((self.bits >> 0x5) & 0x1) as _;
        unsafe { BfdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Pass control frames"]
    pub const fn set_pcf(mut self, val: PcfVal) -> Self {
        self.bits &= !(0x3 << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Pass control frames"]
    pub const fn pcf(self) -> PcfVal {
        let val = ((self.bits >> 0x6) & 0x3) as _;
        unsafe { PcfVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Source address inverse filtering"]
    pub const fn set_saif(mut self, val: SaifVal) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "Source address inverse filtering"]
    pub const fn saif(self) -> SaifVal {
        let val = ((self.bits >> 0x7) & 0x1) as _;
        unsafe { SaifVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Source address filter"]
    pub const fn set_saf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Source address filter"]
    pub const fn saf(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Hash or perfect filter"]
    pub const fn set_hpf(mut self, val: HpfVal) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "Hash or perfect filter"]
    pub const fn hpf(self) -> HpfVal {
        let val = ((self.bits >> 0x9) & 0x1) as _;
        unsafe { HpfVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Receive all"]
    pub const fn set_ra(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Receive all"]
    pub const fn ra(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC hash table high register"]
pub struct MachthrBits {
    bits: u32,
}
impl Default for MachthrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MachthrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Upper 32 bits of hash table"]
    pub const fn set_hth(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Upper 32 bits of hash table"]
    pub const fn hth(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC hash table low register"]
pub struct MachtlrBits {
    bits: u32,
}
impl Default for MachtlrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MachtlrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Lower 32 bits of hash table"]
    pub const fn set_htl(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Lower 32 bits of hash table"]
    pub const fn htl(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC interrupt mask register"]
pub struct MacimrBits {
    bits: u32,
}
impl Default for MacimrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacimrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "PMT interrupt mask"]
    pub const fn set_pmtim(mut self, val: PmtimVal) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "PMT interrupt mask"]
    pub const fn pmtim(self) -> PmtimVal {
        let val = ((self.bits >> 0x3) & 0x1) as _;
        unsafe { PmtimVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Time stamp trigger interrupt mask"]
    pub const fn set_tstim(mut self, val: TstimVal) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "Time stamp trigger interrupt mask"]
    pub const fn tstim(self) -> TstimVal {
        let val = ((self.bits >> 0x9) & 0x1) as _;
        unsafe { TstimVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC MII address register"]
pub struct MacmiiarBits {
    bits: u32,
}
impl Default for MacmiiarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacmiiarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "MII busy"]
    pub const fn set_mb(mut self, val: MbProgressVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "MII busy"]
    pub const fn mb(self) -> MbProgressVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { MbProgressVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "MII write"]
    pub const fn set_mw(mut self, val: MwVal) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "MII write"]
    pub const fn mw(self) -> MwVal {
        let val = ((self.bits >> 0x1) & 0x1) as _;
        unsafe { MwVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clock range"]
    pub const fn set_cr(mut self, val: CrVal) -> Self {
        self.bits &= !(0x7 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Clock range"]
    pub const fn cr(self) -> CrVal {
        let val = ((self.bits >> 0x2) & 0x7) as _;
        unsafe { CrVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "MII register - select the desired MII register in the PHY device"]
    pub const fn set_mr(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0x6);
        self.bits |= (val as u32 & 0x1f) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "MII register - select the desired MII register in the PHY device"]
    pub const fn mr(self) -> u8 {
        ((self.bits >> 0x6) & 0x1f) as _
    }
    #[inline(always)]
    #[doc = "PHY address - select which of possible 32 PHYs is being accessed"]
    pub const fn set_pa(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0xb);
        self.bits |= (val as u32 & 0x1f) << 0xb;
        self
    }
    #[inline(always)]
    #[doc = "PHY address - select which of possible 32 PHYs is being accessed"]
    pub const fn pa(self) -> u8 {
        ((self.bits >> 0xb) & 0x1f) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC MII data register"]
pub struct MacmiidrBits {
    bits: u32,
}
impl Default for MacmiidrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacmiidrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "MII data read from/written to the PHY"]
    pub const fn set_md(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "MII data read from/written to the PHY"]
    pub const fn md(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC PMT control and status register"]
pub struct MacpmtcsrBits {
    bits: u32,
}
impl Default for MacpmtcsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacpmtcsrBits {
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
    pub const fn set_pd(mut self, val: PdVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Power down"]
    pub const fn pd(self) -> PdVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { PdVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Magic packet enable"]
    pub const fn set_mpe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Magic packet enable"]
    pub const fn mpe(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wakeup frame enable"]
    pub const fn set_wfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wakeup frame enable"]
    pub const fn wfe(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Magic packet received"]
    pub const fn set_mpr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Magic packet received"]
    pub const fn mpr(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wakeup frame received"]
    pub const fn set_wfr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wakeup frame received"]
    pub const fn wfr(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Global unicast"]
    pub const fn set_gu(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Global unicast"]
    pub const fn gu(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wakeup frame filter register pointer reset"]
    pub const fn set_wffrpr(mut self, val: WffrprVal) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1f;
        self
    }
    #[inline(always)]
    #[doc = "Wakeup frame filter register pointer reset"]
    pub const fn wffrpr(self) -> WffrprVal {
        let val = ((self.bits >> 0x1f) & 0x1) as _;
        unsafe { WffrprVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC interrupt status register"]
pub struct MacsrBits {
    bits: u32,
}
impl Default for MacsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "PMT status"]
    pub const fn set_pmts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "PMT status"]
    pub const fn pmts(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MMC status"]
    pub const fn set_mmcs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MMC status"]
    pub const fn mmcs(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MMC receive status"]
    pub const fn set_mmcrs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MMC receive status"]
    pub const fn mmcrs(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MMC transmit status"]
    pub const fn set_mmcts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MMC transmit status"]
    pub const fn mmcts(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Time stamp trigger status"]
    pub const fn set_tsts(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Time stamp trigger status"]
    pub const fn tsts(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MAC VLAN tag register"]
pub struct MacvlantrBits {
    bits: u32,
}
impl Default for MacvlantrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MacvlantrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "VLAN tag identifier (for receive frames)"]
    pub const fn set_vlanti(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "VLAN tag identifier (for receive frames)"]
    pub const fn vlanti(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
    #[inline(always)]
    #[doc = "12-bit VLAN tag comparison"]
    pub const fn set_vlantc(mut self, val: VlantcVal) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "12-bit VLAN tag comparison"]
    pub const fn vlantc(self) -> VlantcVal {
        let val = ((self.bits >> 0x10) & 0x1) as _;
        unsafe { VlantcVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MMC control register"]
pub struct MmccrBits {
    bits: u32,
}
impl Default for MmccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Counter reset"]
    pub const fn set_cr(mut self, val: CounterResetVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Counter reset"]
    pub const fn cr(self) -> CounterResetVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { CounterResetVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Counter stop rollover"]
    pub const fn set_csr(mut self, val: CsrVal) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Counter stop rollover"]
    pub const fn csr(self) -> CsrVal {
        let val = ((self.bits >> 0x1) & 0x1) as _;
        unsafe { CsrVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Reset on read"]
    pub const fn set_ror(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Reset on read"]
    pub const fn ror(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MMC counter freeze"]
    pub const fn set_mcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "MMC counter freeze"]
    pub const fn mcf(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "MMC counter preset"]
    pub const fn set_mcp(mut self, val: McpVal) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "MMC counter preset"]
    pub const fn mcp(self) -> McpVal {
        let val = ((self.bits >> 0x4) & 0x1) as _;
        unsafe { McpVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "MMC counter Full-Half preset"]
    pub const fn set_mcfhp(mut self, val: McfhpVal) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "MMC counter Full-Half preset"]
    pub const fn mcfhp(self) -> McfhpVal {
        let val = ((self.bits >> 0x5) & 0x1) as _;
        unsafe { McfhpVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MMC received frames with alignment error counter register"]
pub struct MmcrfaecrBits {
    bits: u32,
}
impl Default for MmcrfaecrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmcrfaecrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "RFAEC"]
    pub const fn set_rfaec(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "RFAEC"]
    pub const fn rfaec(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MMC received frames with CRC error counter register"]
pub struct MmcrfcecrBits {
    bits: u32,
}
impl Default for MmcrfcecrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmcrfcecrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "RFCFC"]
    pub const fn set_rfcfc(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "RFCFC"]
    pub const fn rfcfc(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "MMC received good unicast frames counter register"]
pub struct MmcrgufcrBits {
    bits: u32,
}
impl Default for MmcrgufcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmcrgufcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "RGUFC"]
    pub const fn set_rgufc(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "RGUFC"]
    pub const fn rgufc(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MMC receive interrupt mask register"]
pub struct MmcrimrBits {
    bits: u32,
}
impl Default for MmcrimrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmcrimrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Received frame CRC error mask"]
    pub const fn set_rfcem(mut self, val: RfcemVal) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Received frame CRC error mask"]
    pub const fn rfcem(self) -> RfcemVal {
        let val = ((self.bits >> 0x5) & 0x1) as _;
        unsafe { RfcemVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Received frames alignment error mask"]
    pub const fn set_rfaem(mut self, val: RfaemVal) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Received frames alignment error mask"]
    pub const fn rfaem(self) -> RfaemVal {
        let val = ((self.bits >> 0x6) & 0x1) as _;
        unsafe { RfaemVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Received good Unicast frames mask"]
    pub const fn set_rgufm(mut self, val: RgufmVal) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Received good Unicast frames mask"]
    pub const fn rgufm(self) -> RgufmVal {
        let val = ((self.bits >> 0x11) & 0x1) as _;
        unsafe { RgufmVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MMC receive interrupt register"]
pub struct MmcrirBits {
    bits: u32,
}
impl Default for MmcrirBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmcrirBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Received frames CRC error status"]
    pub const fn set_rfces(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Received frames CRC error status"]
    pub const fn rfces(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Received frames alignment error status"]
    pub const fn set_rfaes(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Received frames alignment error status"]
    pub const fn rfaes(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Received good Unicast frames status"]
    pub const fn set_rgufs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Received good Unicast frames status"]
    pub const fn rgufs(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MMC transmitted good frames counter register"]
pub struct MmctgfcrBits {
    bits: u32,
}
impl Default for MmctgfcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmctgfcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "HTL"]
    pub const fn set_tgfc(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "HTL"]
    pub const fn tgfc(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
pub struct MmctgfmsccrBits {
    bits: u32,
}
impl Default for MmctgfmsccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmctgfmsccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TGFMSCC"]
    pub const fn set_tgfmscc(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "TGFMSCC"]
    pub const fn tgfmscc(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
pub struct MmctgfsccrBits {
    bits: u32,
}
impl Default for MmctgfsccrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmctgfsccrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transmitted good frames single collision counter"]
    pub const fn set_tgfscc(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Transmitted good frames single collision counter"]
    pub const fn tgfscc(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MMC transmit interrupt mask register"]
pub struct MmctimrBits {
    bits: u32,
}
impl Default for MmctimrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmctimrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transmitted good frames single collision mask"]
    pub const fn set_tgfscm(mut self, val: TgfscmVal) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Transmitted good frames single collision mask"]
    pub const fn tgfscm(self) -> TgfscmVal {
        let val = ((self.bits >> 0xe) & 0x1) as _;
        unsafe { TgfscmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Transmitted good frames more than single collision mask"]
    pub const fn set_tgfmscm(mut self, val: TgfmscmVal) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
        self
    }
    #[inline(always)]
    #[doc = "Transmitted good frames more than single collision mask"]
    pub const fn tgfmscm(self) -> TgfmscmVal {
        let val = ((self.bits >> 0xf) & 0x1) as _;
        unsafe { TgfmscmVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Transmitted good frames mask"]
    pub const fn set_tgfm(mut self, val: TgfmVal) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Transmitted good frames mask"]
    pub const fn tgfm(self) -> TgfmVal {
        let val = ((self.bits >> 0x10) & 0x1) as _;
        unsafe { TgfmVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet MMC transmit interrupt register"]
pub struct MmctirBits {
    bits: u32,
}
impl Default for MmctirBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl MmctirBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Transmitted good frames single collision status"]
    pub const fn set_tgfscs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmitted good frames single collision status"]
    pub const fn tgfscs(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmitted good frames more than single collision status"]
    pub const fn set_tgfmscs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmitted good frames more than single collision status"]
    pub const fn tgfmscs(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Transmitted good frames status"]
    pub const fn set_tgfs(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Transmitted good frames status"]
    pub const fn tgfs(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP PPS control register"]
pub struct PtpppscrBits {
    bits: u32,
}
impl Default for PtpppscrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtpppscrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TSSO"]
    pub const fn set_tsso(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSO"]
    pub const fn tsso(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSTTR"]
    pub const fn set_tsttr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSTTR"]
    pub const fn tsttr(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP subsecond increment register"]
pub struct PtpssirBits {
    bits: u32,
}
impl Default for PtpssirBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtpssirBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "STSSI"]
    pub const fn set_stssi(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "STSSI"]
    pub const fn stssi(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP time stamp addend register"]
pub struct PtptsarBits {
    bits: u32,
}
impl Default for PtptsarBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtptsarBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TSA"]
    pub const fn set_tsa(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "TSA"]
    pub const fn tsa(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP time stamp control register"]
pub struct PtptscrBits {
    bits: u32,
}
impl Default for PtptscrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtptscrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TSE"]
    pub const fn set_tse(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSE"]
    pub const fn tse(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSFCU"]
    pub const fn set_tsfcu(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSFCU"]
    pub const fn tsfcu(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSSTI"]
    pub const fn set_tssti(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSTI"]
    pub const fn tssti(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSSTU"]
    pub const fn set_tsstu(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSTU"]
    pub const fn tsstu(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSITE"]
    pub const fn set_tsite(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSITE"]
    pub const fn tsite(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TTSARU"]
    pub const fn set_ttsaru(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TTSARU"]
    pub const fn ttsaru(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSSARFE"]
    pub const fn set_tssarfe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSARFE"]
    pub const fn tssarfe(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSSSR"]
    pub const fn set_tsssr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSSR"]
    pub const fn tsssr(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSPTPPSV2E"]
    pub const fn set_tsptppsv2e(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSPTPPSV2E"]
    pub const fn tsptppsv2e(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSSPTPOEFE"]
    pub const fn set_tssptpoefe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSPTPOEFE"]
    pub const fn tssptpoefe(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSSIPV6FE"]
    pub const fn set_tssipv6fe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSIPV6FE"]
    pub const fn tssipv6fe(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSSIPV4FE"]
    pub const fn set_tssipv4fe(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSIPV4FE"]
    pub const fn tssipv4fe(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSSEME"]
    pub const fn set_tsseme(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xe);
        self.bits |= if val { 1 << 0xe } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSEME"]
    pub const fn tsseme(self) -> bool {
        ((self.bits >> 0xe) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSSMRME"]
    pub const fn set_tssmrme(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xf);
        self.bits |= if val { 1 << 0xf } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSMRME"]
    pub const fn tssmrme(self) -> bool {
        ((self.bits >> 0xf) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSCNT"]
    pub const fn set_tscnt(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x10);
        self.bits |= (val as u32 & 0x3) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "TSCNT"]
    pub const fn tscnt(self) -> u8 {
        ((self.bits >> 0x10) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "TSPFFMAE"]
    pub const fn set_tspffmae(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSPFFMAE"]
    pub const fn tspffmae(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP time stamp high register"]
pub struct PtptshrBits {
    bits: u32,
}
impl Default for PtptshrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtptshrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "STS"]
    pub const fn set_sts(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "STS"]
    pub const fn sts(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP time stamp high update register"]
pub struct PtptshurBits {
    bits: u32,
}
impl Default for PtptshurBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtptshurBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TSUS"]
    pub const fn set_tsus(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "TSUS"]
    pub const fn tsus(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP time stamp low register"]
pub struct PtptslrBits {
    bits: u32,
}
impl Default for PtptslrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtptslrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "STSS"]
    pub const fn set_stss(mut self, val: u32) -> Self {
        self.bits &= !(0x7fffffff << 0x0);
        self.bits |= (val as u32 & 0x7fffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "STSS"]
    pub const fn stss(self) -> u32 {
        ((self.bits >> 0x0) & 0x7fffffff) as _
    }
    #[inline(always)]
    #[doc = "STPNS"]
    pub const fn set_stpns(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "STPNS"]
    pub const fn stpns(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP time stamp low update register"]
pub struct PtptslurBits {
    bits: u32,
}
impl Default for PtptslurBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtptslurBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TSUSS"]
    pub const fn set_tsuss(mut self, val: u32) -> Self {
        self.bits &= !(0x7fffffff << 0x0);
        self.bits |= (val as u32 & 0x7fffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "TSUSS"]
    pub const fn tsuss(self) -> u32 {
        ((self.bits >> 0x0) & 0x7fffffff) as _
    }
    #[inline(always)]
    #[doc = "TSUPNS"]
    pub const fn set_tsupns(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1f);
        self.bits |= if val { 1 << 0x1f } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSUPNS"]
    pub const fn tsupns(self) -> bool {
        ((self.bits >> 0x1f) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP time stamp status register"]
pub struct PtptssrBits {
    bits: u32,
}
impl Default for PtptssrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtptssrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TSSO"]
    pub const fn set_tsso(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSO"]
    pub const fn tsso(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TSSO"]
    pub const fn set_tsttr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TSSO"]
    pub const fn tsttr(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP target time high register"]
pub struct PtptthrBits {
    bits: u32,
}
impl Default for PtptthrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtptthrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "0"]
    pub const fn set_ttsh(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "0"]
    pub const fn ttsh(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Ethernet PTP target time low register"]
pub struct PtpttlrBits {
    bits: u32,
}
impl Default for PtpttlrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl PtpttlrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TTSL"]
    pub const fn set_ttsl(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "TTSL"]
    pub const fn ttsl(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ApcsVal {
    #[doc = "MAC passes all incoming frames unmodified"]
    Disabled = 0x0,
    #[doc = "MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes"]
    Strip = 0x1,
}
impl ApcsVal {
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
pub enum BfdVal {
    #[doc = "Address filters pass all received broadcast frames"]
    Enabled = 0x0,
    #[doc = "Address filters filter all incoming broadcast frames"]
    Disabled = 0x1,
}
impl BfdVal {
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
pub enum BlVal {
    #[doc = "For retransmission n, wait up to 2^min(n, 10) time slots"]
    Bl10 = 0x0,
    #[doc = "For retransmission n, wait up to 2^min(n, 8) time slots"]
    Bl8 = 0x1,
    #[doc = "For retransmission n, wait up to 2^min(n, 4) time slots"]
    Bl4 = 0x2,
    #[doc = "For retransmission n, wait up to 2^min(n, 1) time slots"]
    Bl1 = 0x3,
}
impl BlVal {
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
pub enum CrVal {
    #[doc = "60-100MHz HCLK/42"]
    Cr60100 = 0x0,
    #[doc = "100-150 MHz HCLK/62"]
    Cr100150 = 0x1,
    #[doc = "20-35MHz HCLK/16"]
    Cr2035 = 0x2,
    #[doc = "35-60MHz HCLK/16"]
    Cr3560 = 0x3,
    #[doc = "150-168MHz HCLK/102"]
    Cr150168 = 0x4,
}
impl CrVal {
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
pub enum CsdVal {
    #[doc = "Errors generated due to loss of carrier"]
    Enabled = 0x0,
    #[doc = "No error generated due to loss of carrier"]
    Disabled = 0x1,
}
impl CsdVal {
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
pub enum CsrVal {
    #[doc = "Counters roll over to zero after reaching the maximum value"]
    Rollover = 0x0,
    #[doc = "Counters do not roll over to zero after reaching the maximum value"]
    NotRollover = 0x1,
}
impl CsrVal {
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
pub enum CounterResetVal {
    #[doc = "Reset all counters. Cleared automatically"]
    Reset = 0x1,
}
impl CounterResetVal {
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
pub enum DaVal {
    #[doc = "Round-robin with Rx:Tx priority given by PM"]
    RoundRobin = 0x0,
    #[doc = "Rx has priority over Tx"]
    RxPriority = 0x1,
}
impl DaVal {
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
pub enum DaifVal {
    #[doc = "Normal filtering of frames"]
    Normal = 0x0,
    #[doc = "Address check block operates in inverse filtering mode for the DA address comparison"]
    Invert = 0x1,
}
impl DaifVal {
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
pub enum DmVal {
    #[doc = "MAC operates in half-duplex mode"]
    HalfDuplex = 0x0,
    #[doc = "MAC operates in full-duplex mode"]
    FullDuplex = 0x1,
}
impl DmVal {
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
pub enum DmaomrSrVal {
    #[doc = "Reception is stopped after transfer of the current frame"]
    Stopped = 0x0,
    #[doc = "Reception is placed in the Running state"]
    Started = 0x1,
}
impl DmaomrSrVal {
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
pub enum DtcefdVal {
    #[doc = "Drop frames with errors only in the receive checksum offload engine"]
    Enabled = 0x0,
    #[doc = "Do not drop frames that only have errors in the receive checksum offload engine"]
    Disabled = 0x1,
}
impl DtcefdVal {
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
pub enum FbVal {
    #[doc = "AHB uses SINGLE and INCR burst transfers"]
    Variable = 0x0,
    #[doc = "AHB uses only fixed burst transfers"]
    Fixed = 0x1,
}
impl FbVal {
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
pub enum FcbVal {
    #[doc = "In half duplex only, deasserts back pressure"]
    DisableBackPressure = 0x0,
    #[doc = "In full duplex, initiate a Pause control frame. In half duplex, assert back pressure"]
    PauseOrBackPressure = 0x1,
}
impl FcbVal {
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
pub enum FefVal {
    #[doc = "Rx FIFO drops frames with error status"]
    Drop = 0x0,
    #[doc = "All frames except runt error frames are forwarded to the DMA"]
    Forward = 0x1,
}
impl FefVal {
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
pub enum FesVal {
    #[doc = "10 Mbit/s"]
    Fes10 = 0x0,
    #[doc = "100 Mbit/s"]
    Fes100 = 0x1,
}
impl FesVal {
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
pub enum FpmVal {
    #[doc = "PBL values used as-is"]
    X1 = 0x0,
    #[doc = "PBL values multiplied by 4"]
    X4 = 0x1,
}
impl FpmVal {
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
pub enum FtfVal {
    #[doc = "Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    Flush = 0x1,
}
impl FtfVal {
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
pub enum FugfVal {
    #[doc = "Rx FIFO drops all frames of less than 64 bytes"]
    Drop = 0x0,
    #[doc = "Rx FIFO forwards undersized frames"]
    Forward = 0x1,
}
impl FugfVal {
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
pub enum HmVal {
    #[doc = "MAC performs a perfect destination address filtering for multicast frames"]
    Perfect = 0x0,
    #[doc = "MAC performs destination address filtering of received multicast frames according to the hash table"]
    Hash = 0x1,
}
impl HmVal {
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
pub enum HpfVal {
    #[doc = "If HM or HU is set, only frames that match the Hash filter are passed"]
    HashOnly = 0x0,
    #[doc = "If HM or HU is set, frames that match either the perfect filter or the hash filter are passed"]
    HashOrPerfect = 0x1,
}
impl HpfVal {
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
pub enum HuVal {
    #[doc = "MAC performs a perfect destination address filtering for unicast frames"]
    Perfect = 0x0,
    #[doc = "MAC performs destination address filtering of received unicast frames according to the hash table"]
    Hash = 0x1,
}
impl HuVal {
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
pub enum IfgVal {
    #[doc = "96 bit times"]
    Ifg96 = 0x0,
    #[doc = "88 bit times"]
    Ifg88 = 0x1,
    #[doc = "80 bit times"]
    Ifg80 = 0x2,
    #[doc = "72 bit times"]
    Ifg72 = 0x3,
    #[doc = "64 bit times"]
    Ifg64 = 0x4,
    #[doc = "56 bit times"]
    Ifg56 = 0x5,
    #[doc = "48 bit times"]
    Ifg48 = 0x6,
    #[doc = "40 bit times"]
    Ifg40 = 0x7,
}
impl IfgVal {
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
pub enum IpcoVal {
    #[doc = "IPv4 checksum offload disabled"]
    Disabled = 0x0,
    #[doc = "IPv4 checksums are checked in received frames"]
    Offload = 0x1,
}
impl IpcoVal {
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
pub enum JdVal {
    #[doc = "Jabber enabled, transmit frames up to 2048 bytes"]
    Enabled = 0x0,
    #[doc = "Jabber disabled, transmit frames up to 16384 bytes"]
    Disabled = 0x1,
}
impl JdVal {
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
pub enum LmVal {
    #[doc = "Normal mode"]
    Normal = 0x0,
    #[doc = "MAC operates in loopback mode at the MII"]
    Loopback = 0x1,
}
impl LmVal {
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
pub enum MacahrSaVal {
    #[doc = "This address is used for comparison with DA fields of the received frame"]
    Destination = 0x0,
    #[doc = "This address is used for comparison with SA fields of received frames"]
    Source = 0x1,
}
impl MacahrSaVal {
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
pub enum MbVal {
    #[doc = "Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below"]
    Normal = 0x0,
    #[doc = "If FB is low, start all bursts greater than 16 with INCR (undefined burst)"]
    Mixed = 0x1,
}
impl MbVal {
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
pub enum MbProgressVal {
    #[doc = "This bit is set to 1 by the application to indicate that a read or write access is in progress"]
    Busy = 0x1,
}
impl MbProgressVal {
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
pub enum McfhpVal {
    #[doc = "When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0"]
    AlmostHalf = 0x0,
    #[doc = "When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0"]
    AlmostFull = 0x1,
}
impl McfhpVal {
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
pub enum McpVal {
    #[doc = "MMC counters will be preset to almost full or almost half. Cleared automatically"]
    Preset = 0x1,
}
impl McpVal {
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
pub enum MwVal {
    #[doc = "Read operation"]
    Read = 0x0,
    #[doc = "Write operation"]
    Write = 0x1,
}
impl MwVal {
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
pub enum PblVal {
    #[doc = "Maximum of 1 beat per DMA transaction"]
    Pbl1 = 0x1,
    #[doc = "Maximum of 2 beats per DMA transaction"]
    Pbl2 = 0x2,
    #[doc = "Maximum of 4 beats per DMA transaction"]
    Pbl4 = 0x4,
    #[doc = "Maximum of 8 beats per DMA transaction"]
    Pbl8 = 0x8,
    #[doc = "Maximum of 16 beats per DMA transaction"]
    Pbl16 = 0x10,
    #[doc = "Maximum of 32 beats per DMA transaction"]
    Pbl32 = 0x20,
}
impl PblVal {
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
pub enum PcfVal {
    #[doc = "MAC prevents all control frames from reaching the application"]
    PreventAll = 0x0,
    #[doc = "MAC forwards all control frames to application except Pause"]
    ForwardAllExceptPause = 0x1,
    #[doc = "MAC forwards all control frames to application even if they fail the address filter"]
    ForwardAll = 0x2,
    #[doc = "MAC forwards control frames that pass the address filter"]
    ForwardAllFiltered = 0x3,
}
impl PcfVal {
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
pub enum PdVal {
    #[doc = "All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    Enabled = 0x1,
}
impl PdVal {
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
pub enum PltVal {
    #[doc = "Pause time minus 4 slot times"]
    Plt4 = 0x0,
    #[doc = "Pause time minus 28 slot times"]
    Plt28 = 0x1,
    #[doc = "Pause time minus 144 slot times"]
    Plt144 = 0x2,
    #[doc = "Pause time minus 256 slot times"]
    Plt256 = 0x3,
}
impl PltVal {
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
pub enum PmtimVal {
    #[doc = "PMT Status interrupt generation enabled"]
    Unmasked = 0x0,
    #[doc = "PMT Status interrupt generation disabled"]
    Masked = 0x1,
}
impl PmtimVal {
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
pub enum PriorityRxOverTxVal {
    #[doc = "RxDMA priority over TxDMA is 1:1"]
    OneToOne = 0x0,
    #[doc = "RxDMA priority over TxDMA is 2:1"]
    TwoToOne = 0x1,
    #[doc = "RxDMA priority over TxDMA is 3:1"]
    ThreeToOne = 0x2,
    #[doc = "RxDMA priority over TxDMA is 4:1"]
    FourToOne = 0x3,
}
impl PriorityRxOverTxVal {
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
pub enum RdVal {
    #[doc = "MAC attempts retries based on the settings of BL"]
    Enabled = 0x0,
    #[doc = "MAC attempts only 1 transmission"]
    Disabled = 0x1,
}
impl RdVal {
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
pub enum RdpVal {
    #[doc = "1 beat per RxDMA transaction"]
    Rdp1 = 0x1,
    #[doc = "2 beats per RxDMA transaction"]
    Rdp2 = 0x2,
    #[doc = "4 beats per RxDMA transaction"]
    Rdp4 = 0x4,
    #[doc = "8 beats per RxDMA transaction"]
    Rdp8 = 0x8,
    #[doc = "16 beats per RxDMA transaction"]
    Rdp16 = 0x10,
    #[doc = "32 beats per RxDMA transaction"]
    Rdp32 = 0x20,
}
impl RdpVal {
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
pub enum RfaemVal {
    #[doc = "Received-alignment-error counter half-full interrupt enabled"]
    Unmasked = 0x0,
    #[doc = "Received-alignment-error counter half-full interrupt disabled"]
    Masked = 0x1,
}
impl RfaemVal {
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
pub enum RfcemVal {
    #[doc = "Received-crc-error counter half-full interrupt enabled"]
    Unmasked = 0x0,
    #[doc = "Received-crc-error counter half-full interrupt disabled"]
    Masked = 0x1,
}
impl RfcemVal {
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
pub enum RgufmVal {
    #[doc = "Received-good-unicast counter half-full interrupt enabled"]
    Unmasked = 0x0,
    #[doc = "Received-good-unicast counter half-full interrupt disabled"]
    Masked = 0x1,
}
impl RgufmVal {
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
pub enum RodVal {
    #[doc = "MAC receives all packets from PHY while transmitting"]
    Enabled = 0x0,
    #[doc = "MAC disables reception of frames in half-duplex mode"]
    Disabled = 0x1,
}
impl RodVal {
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
#[repr(u32)]
pub enum RpdVal {
    #[doc = "Poll the receive descriptor list"]
    Poll = 0x0,
}
impl RpdVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self as u32
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RpsVal {
    #[doc = "Stopped, reset or Stop Receive command issued"]
    Stopped = 0x0,
    #[doc = "Running, fetching receive transfer descriptor"]
    RunningFetching = 0x1,
    #[doc = "Running, waiting for receive packet"]
    RunningWaiting = 0x3,
    #[doc = "Suspended, receive descriptor unavailable"]
    Suspended = 0x4,
    #[doc = "Running, writing data to host memory buffer"]
    RunningWriting = 0x7,
}
impl RpsVal {
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
pub enum RsfVal {
    #[doc = "Rx FIFO operates in cut-through mode, subject to RTC bits"]
    CutThrough = 0x0,
    #[doc = "Frames are read from Rx FIFO after complete frame has been written"]
    StoreForward = 0x1,
}
impl RsfVal {
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
pub enum RtcVal {
    #[doc = "64 bytes"]
    Rtc64 = 0x0,
    #[doc = "32 bytes"]
    Rtc32 = 0x1,
    #[doc = "96 bytes"]
    Rtc96 = 0x2,
    #[doc = "128 bytes"]
    Rtc128 = 0x3,
}
impl RtcVal {
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
pub enum SaifVal {
    #[doc = "Source address filter operates normally"]
    Normal = 0x0,
    #[doc = "Source address filter operation inverted"]
    Invert = 0x1,
}
impl SaifVal {
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
pub enum StVal {
    #[doc = "Transmission is placed in the Stopped state"]
    Stopped = 0x0,
    #[doc = "Transmission is placed in Running state"]
    Started = 0x1,
}
impl StVal {
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
pub enum TgfmVal {
    #[doc = "Transmitted-good counter half-full interrupt enabled"]
    Unmasked = 0x0,
    #[doc = "Transmitted-good counter half-full interrupt disabled"]
    Masked = 0x1,
}
impl TgfmVal {
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
pub enum TgfmscmVal {
    #[doc = "Transmitted-good-multiple-collision half-full interrupt enabled"]
    Unmasked = 0x0,
    #[doc = "Transmitted-good-multiple-collision half-full interrupt disabled"]
    Masked = 0x1,
}
impl TgfmscmVal {
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
pub enum TgfscmVal {
    #[doc = "Transmitted-good-single-collision half-full interrupt enabled"]
    Unmasked = 0x0,
    #[doc = "Transmitted-good-single-collision half-full interrupt disabled"]
    Masked = 0x1,
}
impl TgfscmVal {
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
#[repr(u32)]
pub enum TpdVal {
    #[doc = "Poll the transmit descriptor list"]
    Poll = 0x0,
}
impl TpdVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self as u32
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TpsVal {
    #[doc = "Stopped, Reset or Stop Transmit command issued"]
    Stopped = 0x0,
    #[doc = "Running, fetching transmit transfer descriptor"]
    RunningFetching = 0x1,
    #[doc = "Running, waiting for status"]
    RunningWaiting = 0x2,
    #[doc = "Running, reading data from host memory buffer"]
    RunningReading = 0x3,
    #[doc = "Suspended, transmit descriptor unavailable or transmit buffer underflow"]
    Suspended = 0x6,
    #[doc = "Running, closing transmit descriptor"]
    Running = 0x7,
}
impl TpsVal {
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
pub enum TsfVal {
    #[doc = "Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    CutThrough = 0x0,
    #[doc = "Transmission starts when a full frame is in the Tx FIFO"]
    StoreForward = 0x1,
}
impl TsfVal {
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
pub enum TstimVal {
    #[doc = "Time stamp interrupt generation enabled"]
    Unmasked = 0x0,
    #[doc = "Time stamp interrupt generation disabled"]
    Masked = 0x1,
}
impl TstimVal {
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
pub enum TtcVal {
    #[doc = "64 bytes"]
    Ttc64 = 0x0,
    #[doc = "128 bytes"]
    Ttc128 = 0x1,
    #[doc = "192 bytes"]
    Ttc192 = 0x2,
    #[doc = "256 bytes"]
    Ttc256 = 0x3,
    #[doc = "40 bytes"]
    Ttc40 = 0x4,
    #[doc = "32 bytes"]
    Ttc32 = 0x5,
    #[doc = "24 bytes"]
    Ttc24 = 0x6,
    #[doc = "16 bytes"]
    Ttc16 = 0x7,
}
impl TtcVal {
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
pub enum UspVal {
    #[doc = "PBL value used for both Rx and Tx DMA"]
    Combined = 0x0,
    #[doc = "RxDMA uses RDP value, TxDMA uses PBL value"]
    Separate = 0x1,
}
impl UspVal {
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
pub enum VlantcVal {
    #[doc = "Full 16 bit VLAN identifiers are used for comparison and filtering"]
    Vlantc16 = 0x0,
    #[doc = "12 bit VLAN identifies are used for comparison and filtering"]
    Vlantc12 = 0x1,
}
impl VlantcVal {
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
pub enum WdVal {
    #[doc = "Watchdog enabled, receive frames limited to 2048 bytes"]
    Enabled = 0x0,
    #[doc = "Watchdog disabled, receive frames may be up to to 16384 bytes"]
    Disabled = 0x1,
}
impl WdVal {
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
pub enum WffrprVal {
    #[doc = "Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    Reset = 0x1,
}
impl WffrprVal {
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
pub enum ZqpdVal {
    #[doc = "Normal operation with automatic zero-quanta pause control frame generation"]
    Enabled = 0x0,
    #[doc = "Automatic generation of zero-quanta pause control frames is disabled"]
    Disabled = 0x1,
}
impl ZqpdVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
