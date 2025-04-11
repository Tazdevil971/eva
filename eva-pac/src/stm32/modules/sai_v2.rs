
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
pub struct Ch {
    ptr: *mut u8,
}
impl Ch {
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
    #[doc = "Configuration register 1"]
    pub const fn cr1(&self) -> utils::Reg<Cr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<Cr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Configuration register 2"]
    pub const fn cr2(&self) -> utils::Reg<Cr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<Cr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
    pub const fn frcr(&self) -> utils::Reg<FrcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<FrcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
    pub const fn slotr(&self) -> utils::Reg<SlotrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<SlotrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt mask register 2"]
    pub const fn im(&self) -> utils::Reg<ImBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<ImBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Status register"]
    pub const fn sr(&self) -> utils::Reg<SrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<SrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Clear flag register"]
    pub const fn clrfr(&self) -> utils::Reg<ClrfrBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<ClrfrBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Data register"]
    pub const fn dr(&self) -> utils::Reg<DrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<DrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Serial audio interface"]
pub struct Sai {
    ptr: *mut u8,
}
impl Sai {
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
    #[doc = "Global configuration register"]
    pub const fn gcr(&self) -> utils::Reg<GcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<GcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    pub const fn ch(&self, idx: usize) -> Ch {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x4 + idx * 0x20);
            <Ch>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Clear flag register"]
pub struct ClrfrBits {
    bits: u32,
}
impl Default for ClrfrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ClrfrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0."]
    pub const fn set_covrudr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0."]
    pub const fn covrudr(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0."]
    pub const fn set_cmutedet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0."]
    pub const fn cmutedet(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE[1] = 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0."]
    pub const fn set_cwckcfg(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE[1] = 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0."]
    pub const fn cwckcfg(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0."]
    pub const fn set_ccnrdy(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0."]
    pub const fn ccnrdy(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0."]
    pub const fn set_cafsdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0."]
    pub const fn cafsdet(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0."]
    pub const fn set_clfsdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0."]
    pub const fn clfsdet(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Configuration register 1"]
pub struct Cr1Bits {
    bits: u32,
}
impl Default for Cr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr1Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "SAIx audio block mode immediately"]
    pub const fn set_mode(mut self, val: ModeVal) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "SAIx audio block mode immediately"]
    pub const fn mode(self) -> ModeVal {
        let val = ((self.bits >> 0x0) & 0x3) as _;
        unsafe { ModeVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
    pub const fn set_prtcfg(mut self, val: PrtcfgVal) -> Self {
        self.bits &= !(0x3 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
    pub const fn prtcfg(self) -> PrtcfgVal {
        let val = ((self.bits >> 0x2) & 0x3) as _;
        unsafe { PrtcfgVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG[1:0]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP[1:0] bits, DS[1:0] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
    pub const fn set_ds(mut self, val: DsVal) -> Self {
        self.bits &= !(0x7 << 0x5);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x5;
        self
    }
    #[inline(always)]
    #[doc = "Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG[1:0]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP[1:0] bits, DS[1:0] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
    pub const fn ds(self) -> DsVal {
        let val = ((self.bits >> 0x5) & 0x7) as _;
        unsafe { DsVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
    pub const fn set_lsbfirst(mut self, val: LsbfirstVal) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
    pub const fn lsbfirst(self) -> LsbfirstVal {
        let val = ((self.bits >> 0x8) & 0x1) as _;
        unsafe { LsbfirstVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
    pub const fn set_ckstr(mut self, val: CkstrVal) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
    pub const fn ckstr(self) -> CkstrVal {
        let val = ((self.bits >> 0x9) & 0x1) as _;
        unsafe { CkstrVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled."]
    pub const fn set_syncen(mut self, val: SyncenVal) -> Self {
        self.bits &= !(0x3 << 0xa);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xa;
        self
    }
    #[inline(always)]
    #[doc = "Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled."]
    pub const fn syncen(self) -> SyncenVal {
        let val = ((self.bits >> 0xa) & 0x3) as _;
        unsafe { SyncenVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details."]
    pub const fn set_mono(mut self, val: MonoVal) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xc;
        self
    }
    #[inline(always)]
    #[doc = "Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details."]
    pub const fn mono(self) -> MonoVal {
        let val = ((self.bits >> 0xc) & 0x1) as _;
        unsafe { MonoVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
    pub const fn set_outdriv(mut self, val: OutdrivVal) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
    pub const fn outdriv(self) -> OutdrivVal {
        let val = ((self.bits >> 0xd) & 0x1) as _;
        unsafe { OutdrivVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit."]
    pub const fn set_saien(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit."]
    pub const fn saien(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE[1:0] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
    pub const fn set_dmaen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE[1:0] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
    pub const fn dmaen(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "No fixed divider between MCLK and FS"]
    pub const fn set_nodiv(mut self, val: NodivVal) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x13;
        self
    }
    #[inline(always)]
    #[doc = "No fixed divider between MCLK and FS"]
    pub const fn nodiv(self) -> NodivVal {
        let val = ((self.bits >> 0x13) & 0x1) as _;
        unsafe { NodivVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:"]
    pub const fn set_mckdiv(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x14);
        self.bits |= (val as u32 & 0xf) << 0x14;
        self
    }
    #[inline(always)]
    #[doc = "Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:"]
    pub const fn mckdiv(self) -> u8 {
        ((self.bits >> 0x14) & 0xf) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Configuration register 2"]
pub struct Cr2Bits {
    bits: u32,
}
impl Default for Cr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Cr2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "FIFO threshold. This bit is set and cleared by software."]
    pub const fn set_fth(mut self, val: FthVal) -> Self {
        self.bits &= !(0x7 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "FIFO threshold. This bit is set and cleared by software."]
    pub const fn fth(self) -> FthVal {
        let val = ((self.bits >> 0x0) & 0x7) as _;
        unsafe { FthVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled."]
    pub const fn set_fflush(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled."]
    pub const fn fflush(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section: Output data line management on an inactive slot for more details."]
    pub const fn set_tris(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section: Output data line management on an inactive slot for more details."]
    pub const fn tris(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    pub const fn set_mute(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    pub const fn mute(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIXEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    pub const fn set_muteval(mut self, val: MutevalVal) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIXEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    pub const fn muteval(self) -> MutevalVal {
        let val = ((self.bits >> 0x6) & 0x1) as _;
        unsafe { MutevalVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET will be set and an interrupt will be generated if bit MUTEDETIE is set. Refer to Section: Mute mode for more details."]
    pub const fn set_mutecnt(mut self, val: u8) -> Self {
        self.bits &= !(0x3f << 0x7);
        self.bits |= (val as u32 & 0x3f) << 0x7;
        self
    }
    #[inline(always)]
    #[doc = "Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET will be set and an interrupt will be generated if bit MUTEDETIE is set. Refer to Section: Mute mode for more details."]
    pub const fn mutecnt(self) -> u8 {
        ((self.bits >> 0x7) & 0x3f) as _
    }
    #[inline(always)]
    #[doc = "Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm."]
    pub const fn set_cpl(mut self, val: CplVal) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm."]
    pub const fn cpl(self) -> CplVal {
        let val = ((self.bits >> 0xd) & 0x1) as _;
        unsafe { CplVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that will be used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE[0]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section: Companding mode for more details. Note: Companding mode is applicable only when TDM is selected."]
    pub const fn set_comp(mut self, val: CompVal) -> Self {
        self.bits &= !(0x3 << 0xe);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0xe;
        self
    }
    #[inline(always)]
    #[doc = "Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that will be used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE[0]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section: Companding mode for more details. Note: Companding mode is applicable only when TDM is selected."]
    pub const fn comp(self) -> CompVal {
        let val = ((self.bits >> 0xe) & 0x3) as _;
        unsafe { CompVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Data register"]
pub struct DrBits {
    bits: u32,
}
impl Default for DrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl DrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
    pub const fn set_data(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
    pub const fn data(self) -> u32 {
        ((self.bits >> 0x0) & 0xffffffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
pub struct FrcrBits {
    bits: u32,
}
impl Default for FrcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl FrcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL[7:0] + 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block will behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT[4:0] of SAI_xSLOTR register (NBSLOT[3:0] = 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC97 or SPDIF audio block configuration."]
    pub const fn set_frl(mut self, val: u8) -> Self {
        self.bits &= !(0xff << 0x0);
        self.bits |= (val as u32 & 0xff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL[7:0] + 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block will behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT[4:0] of SAI_xSLOTR register (NBSLOT[3:0] = 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC97 or SPDIF audio block configuration."]
    pub const fn frl(self) -> u8 {
        ((self.bits >> 0x0) & 0xff) as _
    }
    #[inline(always)]
    #[doc = "Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL[6:0] + 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
    pub const fn set_fsall(mut self, val: u8) -> Self {
        self.bits &= !(0x7f << 0x8);
        self.bits |= (val as u32 & 0x7f) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL[6:0] + 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
    pub const fn fsall(self) -> u8 {
        ((self.bits >> 0x8) & 0x7f) as _
    }
    #[inline(always)]
    #[doc = "Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots will be dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC97 or SPDIF audio block configuration. It must be configured when the audio block is disabled."]
    pub const fn set_fsdef(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots will be dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC97 or SPDIF audio block configuration. It must be configured when the audio block is disabled."]
    pub const fn fsdef(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
    pub const fn set_fspol(mut self, val: FspolVal) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
    pub const fn fspol(self) -> FspolVal {
        let val = ((self.bits >> 0x11) & 0x1) as _;
        unsafe { FspolVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
    pub const fn set_fsoff(mut self, val: FsoffVal) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x12;
        self
    }
    #[inline(always)]
    #[doc = "Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
    pub const fn fsoff(self) -> FsoffVal {
        let val = ((self.bits >> 0x12) & 0x1) as _;
        unsafe { FsoffVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Global configuration register"]
pub struct GcrBits {
    bits: u32,
}
impl Default for GcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl GcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Synchronization inputs"]
    pub const fn set_syncin(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x0);
        self.bits |= (val as u32 & 0x3) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Synchronization inputs"]
    pub const fn syncin(self) -> u8 {
        ((self.bits >> 0x0) & 0x3) as _
    }
    #[inline(always)]
    #[doc = "Synchronization outputs These bits are set and cleared by software."]
    pub const fn set_syncout(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x4);
        self.bits |= (val as u32 & 0x3) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Synchronization outputs These bits are set and cleared by software."]
    pub const fn syncout(self) -> u8 {
        ((self.bits >> 0x4) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Interrupt mask register 2"]
pub struct ImBits {
    bits: u32,
}
impl Default for ImBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ImBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
    pub const fn set_ovrudrie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
    pub const fn ovrudrie(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
    pub const fn set_mutedetie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
    pub const fn mutedetie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE[1] = 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in TDM mode and is meaningless in other modes."]
    pub const fn set_wckcfgie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE[1] = 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in TDM mode and is meaningless in other modes."]
    pub const fn wckcfgie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interruption in receiver mode,"]
    pub const fn set_freqie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interruption in receiver mode,"]
    pub const fn freqie(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Codec not ready interrupt enable (AC97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interruption i generated. This bit has a meaning only if the AC97 mode is selected through PRTCFG[1:0] bits and the audio block is operates as a receiver."]
    pub const fn set_cnrdyie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Codec not ready interrupt enable (AC97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interruption i generated. This bit has a meaning only if the AC97 mode is selected through PRTCFG[1:0] bits and the audio block is operates as a receiver."]
    pub const fn cnrdyie(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt will be generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC97, SPDIF mode or when the audio block operates as a master."]
    pub const fn set_afsdetie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt will be generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC97, SPDIF mode or when the audio block operates as a master."]
    pub const fn afsdetie(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt will be generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC97, SPDIF mode or when the audio block operates as a master."]
    pub const fn set_lfsdetie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt will be generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC97, SPDIF mode or when the audio block operates as a master."]
    pub const fn lfsdetie(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
pub struct SlotrBits {
    bits: u32,
}
impl Default for SlotrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SlotrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
    pub const fn set_fboff(mut self, val: u8) -> Self {
        self.bits &= !(0x1f << 0x0);
        self.bits |= (val as u32 & 0x1f) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
    pub const fn fboff(self) -> u8 {
        ((self.bits >> 0x0) & 0x1f) as _
    }
    #[inline(always)]
    #[doc = "Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI will be undetermined. Refer to Section: Output data line management on an inactive slot for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
    pub const fn set_slotsz(mut self, val: SlotszVal) -> Self {
        self.bits &= !(0x3 << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI will be undetermined. Refer to Section: Output data line management on an inactive slot for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
    pub const fn slotsz(self) -> SlotszVal {
        let val = ((self.bits >> 0x6) & 0x3) as _;
        unsafe { SlotszVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
    pub const fn set_nbslot(mut self, val: u8) -> Self {
        self.bits &= !(0xf << 0x8);
        self.bits |= (val as u32 & 0xf) << 0x8;
        self
    }
    #[inline(always)]
    #[doc = "Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
    pub const fn nbslot(self) -> u8 {
        ((self.bits >> 0x8) & 0xf) as _
    }
    #[inline(always)]
    #[doc = "Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
    pub const fn set_sloten(mut self, val: SlotenVal) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val.to_bits() as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
    pub const fn sloten(self) -> SlotenVal {
        let val = ((self.bits >> 0x10) & 0xffff) as _;
        unsafe { SlotenVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Status register"]
pub struct SrBits {
    bits: u32,
}
impl Default for SrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl SrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register."]
    pub const fn set_ovrudr(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register."]
    pub const fn ovrudr(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register."]
    pub const fn set_mutedet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register."]
    pub const fn mutedet(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE[1] = 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register."]
    pub const fn set_wckcfg(mut self, val: WckcfgVal) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
        self
    }
    #[inline(always)]
    #[doc = "Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE[1] = 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register."]
    pub const fn wckcfg(self) -> WckcfgVal {
        let val = ((self.bits >> 0x2) & 0x1) as _;
        unsafe { WckcfgVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register."]
    pub const fn set_freq(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register."]
    pub const fn freq(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Codec not ready. This bit is read only. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register."]
    pub const fn set_cnrdy(mut self, val: CnrdyVal) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
        self
    }
    #[inline(always)]
    #[doc = "Codec not ready. This bit is read only. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register."]
    pub const fn cnrdy(self) -> CnrdyVal {
        let val = ((self.bits >> 0x4) & 0x1) as _;
        unsafe { CnrdyVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register."]
    pub const fn set_afsdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register."]
    pub const fn afsdet(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register"]
    pub const fn set_lfsdet(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register"]
    pub const fn lfsdet(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). If the SAI block is configured as transmitter: If SAI block is configured as receiver:"]
    pub const fn set_flvl(mut self, val: FlvlVal) -> Self {
        self.bits &= !(0x7 << 0x10);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). If the SAI block is configured as transmitter: If SAI block is configured as receiver:"]
    pub const fn flvl(self) -> FlvlVal {
        let val = ((self.bits >> 0x10) & 0x7) as _;
        unsafe { FlvlVal::from_bits_unchecked(val) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CkstrVal {
    #[doc = "Data strobing edge is falling edge of SCK"]
    FallingEdge = 0x0,
    #[doc = "Data strobing edge is rising edge of SCK"]
    RisingEdge = 0x1,
}
impl CkstrVal {
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
pub enum CnrdyVal {
    #[doc = "External AC’97 Codec is ready"]
    Ready = 0x0,
    #[doc = "External AC’97 Codec is not ready"]
    NotReady = 0x1,
}
impl CnrdyVal {
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
pub enum CompVal {
    #[doc = "No companding algorithm"]
    NoCompanding = 0x0,
    #[doc = "μ-Law algorithm"]
    MuLaw = 0x2,
    #[doc = "A-Law algorithm"]
    ALaw = 0x3,
}
impl CompVal {
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
pub enum CplVal {
    #[doc = "1’s complement representation"]
    OnesComplement = 0x0,
    #[doc = "2’s complement representation"]
    TwosComplement = 0x1,
}
impl CplVal {
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
pub enum DsVal {
    #[doc = "8 bits"]
    Bit8 = 0x2,
    #[doc = "10 bits"]
    Bit10 = 0x3,
    #[doc = "16 bits"]
    Bit16 = 0x4,
    #[doc = "20 bits"]
    Bit20 = 0x5,
    #[doc = "24 bits"]
    Bit24 = 0x6,
    #[doc = "32 bits"]
    Bit32 = 0x7,
}
impl DsVal {
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
pub enum FlvlVal {
    #[doc = "FIFO empty"]
    Empty = 0x0,
    #[doc = "FIFO <= 1⁄4 but not empty"]
    Quarter1 = 0x1,
    #[doc = "1⁄4 < FIFO <= 1⁄2"]
    Quarter2 = 0x2,
    #[doc = "1⁄2 < FIFO <= 3⁄4"]
    Quarter3 = 0x3,
    #[doc = "3⁄4 < FIFO but not full"]
    Quarter4 = 0x4,
    #[doc = "FIFO full"]
    Full = 0x5,
}
impl FlvlVal {
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
pub enum FsoffVal {
    #[doc = "FS is asserted on the first bit of the slot 0"]
    OnFirst = 0x0,
    #[doc = "FS is asserted one bit before the first bit of the slot 0"]
    BeforeFirst = 0x1,
}
impl FsoffVal {
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
pub enum FspolVal {
    #[doc = "FS is active low (falling edge)"]
    FallingEdge = 0x0,
    #[doc = "FS is active high (rising edge)"]
    RisingEdge = 0x1,
}
impl FspolVal {
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
pub enum FthVal {
    #[doc = "FIFO empty"]
    Empty = 0x0,
    #[doc = "1⁄4 FIFO"]
    Quarter1 = 0x1,
    #[doc = "1⁄2 FIFO"]
    Quarter2 = 0x2,
    #[doc = "3⁄4 FIFO"]
    Quarter3 = 0x3,
    #[doc = "FIFO full"]
    Full = 0x4,
}
impl FthVal {
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
pub enum LsbfirstVal {
    #[doc = "Data are transferred with MSB first"]
    MsbFirst = 0x0,
    #[doc = "Data are transferred with LSB first"]
    LsbFirst = 0x1,
}
impl LsbfirstVal {
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
pub enum ModeVal {
    #[doc = "Master transmitter"]
    MasterTx = 0x0,
    #[doc = "Master receiver"]
    MasterRx = 0x1,
    #[doc = "Slave transmitter"]
    SlaveTx = 0x2,
    #[doc = "Slave receiver"]
    SlaveRx = 0x3,
}
impl ModeVal {
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
pub enum MonoVal {
    #[doc = "Stereo mode"]
    Stereo = 0x0,
    #[doc = "Mono mode"]
    Mono = 0x1,
}
impl MonoVal {
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
pub enum MutevalVal {
    #[doc = "Bit value 0 is sent during the mute mode"]
    SendZero = 0x0,
    #[doc = "Last values are sent during the mute mode"]
    SendLast = 0x1,
}
impl MutevalVal {
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
pub enum NodivVal {
    #[doc = "MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value"]
    MasterClock = 0x0,
    #[doc = "MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL."]
    NoDiv = 0x1,
}
impl NodivVal {
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
pub enum OutdrivVal {
    #[doc = "Audio block output driven when SAIEN is set"]
    OnStart = 0x0,
    #[doc = "Audio block output driven immediately after the setting of this bit"]
    Immediately = 0x1,
}
impl OutdrivVal {
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
pub enum PrtcfgVal {
    #[doc = "Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol"]
    Free = 0x0,
    #[doc = "SPDIF protocol"]
    Spdif = 0x1,
    #[doc = "AC’97 protocol"]
    Ac97 = 0x2,
}
impl PrtcfgVal {
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
#[repr(u16)]
pub enum SlotenVal {
    #[doc = "Inactive slot"]
    Inactive = 0x0,
    #[doc = "Active slot"]
    Active = 0x1,
}
impl SlotenVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u16) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u16 {
        self as u16
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SlotszVal {
    #[doc = "The slot size is equivalent to the data size (specified in DS[3:0] in the SAI_xCR1 register)"]
    DataSize = 0x0,
    #[doc = "16-bit"]
    Bit16 = 0x1,
    #[doc = "32-bit"]
    Bit32 = 0x2,
}
impl SlotszVal {
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
pub enum SyncenVal {
    #[doc = "audio sub-block in asynchronous mode"]
    Asynchronous = 0x0,
    #[doc = "audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode"]
    Internal = 0x1,
    #[doc = "audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode"]
    External = 0x2,
}
impl SyncenVal {
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
pub enum WckcfgVal {
    #[doc = "Clock configuration is correct"]
    Correct = 0x0,
    #[doc = "Clock configuration does not respect the rule concerning the frame length specification"]
    Wrong = 0x1,
}
impl WckcfgVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
