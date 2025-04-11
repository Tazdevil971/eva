
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Low power timer with Output Compare"]
pub struct Lptim {
    ptr: *mut u8,
}
impl Lptim {
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
    #[doc = "LPTIM interrupt and status register."]
    pub const fn isr(&self) -> utils::Reg<IsrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<IsrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "LPTIM interrupt clear register."]
    pub const fn icr(&self) -> utils::Reg<IcrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<IcrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "LPTIM interrupt enable register."]
    pub const fn ier(&self) -> utils::Reg<IerBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<IerBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "LPTIM configuration register."]
    pub const fn cfgr(&self) -> utils::Reg<CfgrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<CfgrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "LPTIM control register."]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "LPTIM compare register 1."]
    pub const fn cmp(&self) -> utils::Reg<CmpBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<CmpBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "LPTIM autoreload register."]
    pub const fn arr(&self) -> utils::Reg<ArrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<ArrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "LPTIM counter register."]
    pub const fn cnt(&self) -> utils::Reg<CntBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<CntBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "LPTIM option register."]
    pub const fn or(&self) -> utils::Reg<u32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<u32, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "LPTIM autoreload register."]
pub struct ArrBits {
    bits: u32,
}
impl Default for ArrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl ArrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx[15:0] value."]
    pub const fn set_arr(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx[15:0] value."]
    pub const fn arr(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "LPTIM configuration register."]
pub struct CfgrBits {
    bits: u32,
}
impl Default for CfgrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CfgrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Clock selector The CKSEL bit selects which clock source the LPTIM uses:."]
    pub const fn set_cksel(mut self, val: ClockSourceVal) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Clock selector The CKSEL bit selects which clock source the LPTIM uses:."]
    pub const fn cksel(self) -> ClockSourceVal {
        let val = ((self.bits >> 0x0) & 0x1) as _;
        unsafe { ClockSourceVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
    pub const fn set_ckpol(mut self, val: CkpolVal) -> Self {
        self.bits &= !(0x3 << 0x1);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x1;
        self
    }
    #[inline(always)]
    #[doc = "Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
    pub const fn ckpol(self) -> CkpolVal {
        let val = ((self.bits >> 0x1) & 0x3) as _;
        unsafe { CkpolVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    pub const fn set_ckflt(mut self, val: FilterVal) -> Self {
        self.bits &= !(0x3 << 0x3);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x3;
        self
    }
    #[inline(always)]
    #[doc = "Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    pub const fn ckflt(self) -> FilterVal {
        let val = ((self.bits >> 0x3) & 0x3) as _;
        unsafe { FilterVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    pub const fn set_trgflt(mut self, val: FilterVal) -> Self {
        self.bits &= !(0x3 << 0x6);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    pub const fn trgflt(self) -> FilterVal {
        let val = ((self.bits >> 0x6) & 0x3) as _;
        unsafe { FilterVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:."]
    pub const fn set_presc(mut self, val: PrescVal) -> Self {
        self.bits &= !(0x7 << 0x9);
        self.bits |= (val.to_bits() as u32 & 0x7) << 0x9;
        self
    }
    #[inline(always)]
    #[doc = "Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:."]
    pub const fn presc(self) -> PrescVal {
        let val = ((self.bits >> 0x9) & 0x7) as _;
        unsafe { PrescVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
    pub const fn set_trigsel(mut self, val: u8) -> Self {
        self.bits &= !(0x7 << 0xd);
        self.bits |= (val as u32 & 0x7) << 0xd;
        self
    }
    #[inline(always)]
    #[doc = "Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
    pub const fn trigsel(self) -> u8 {
        ((self.bits >> 0xd) & 0x7) as _
    }
    #[inline(always)]
    #[doc = "Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:."]
    pub const fn set_trigen(mut self, val: TrigenVal) -> Self {
        self.bits &= !(0x3 << 0x11);
        self.bits |= (val.to_bits() as u32 & 0x3) << 0x11;
        self
    }
    #[inline(always)]
    #[doc = "Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:."]
    pub const fn trigen(self) -> TrigenVal {
        let val = ((self.bits >> 0x11) & 0x3) as _;
        unsafe { TrigenVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Timeout enable The TIMOUT bit controls the Timeout feature."]
    pub const fn set_timout(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x13);
        self.bits |= if val { 1 << 0x13 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timeout enable The TIMOUT bit controls the Timeout feature."]
    pub const fn timout(self) -> bool {
        ((self.bits >> 0x13) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Waveform shape The WAVE bit controls the output shape."]
    pub const fn set_wave(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x14);
        self.bits |= if val { 1 << 0x14 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Waveform shape The WAVE bit controls the output shape."]
    pub const fn wave(self) -> bool {
        ((self.bits >> 0x14) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Waveform shape polarity The WAVEPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to."]
    pub const fn set_wavpol(mut self, val: WavpolVal) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x15;
        self
    }
    #[inline(always)]
    #[doc = "Waveform shape polarity The WAVEPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to."]
    pub const fn wavpol(self) -> WavpolVal {
        let val = ((self.bits >> 0x15) & 0x1) as _;
        unsafe { WavpolVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality."]
    pub const fn set_preload(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality."]
    pub const fn preload(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:."]
    pub const fn set_countmode(mut self, val: ClockSourceVal) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= (val.to_bits() as u32 & 0x1) << 0x17;
        self
    }
    #[inline(always)]
    #[doc = "counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:."]
    pub const fn countmode(self) -> ClockSourceVal {
        let val = ((self.bits >> 0x17) & 0x1) as _;
        unsafe { ClockSourceVal::from_bits_unchecked(val) }
    }
    #[inline(always)]
    #[doc = "Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn set_enc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn enc(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "LPTIM compare register 1."]
pub struct CmpBits {
    bits: u32,
}
impl Default for CmpBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CmpBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed. If LPTIM does not implement any channel: The compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on LPTIM output."]
    pub const fn set_cmp(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed. If LPTIM does not implement any channel: The compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on LPTIM output."]
    pub const fn cmp(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "LPTIM counter register."]
pub struct CntBits {
    bits: u32,
}
impl Default for CntBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl CntBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
    pub const fn set_cnt(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x0);
        self.bits |= (val as u32 & 0xffff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
    pub const fn cnt(self) -> u16 {
        ((self.bits >> 0x0) & 0xffff) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "LPTIM control register."]
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
    #[doc = "LPTIM enable The ENABLE bit is set and cleared by software."]
    pub const fn set_enable(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LPTIM enable The ENABLE bit is set and cleared by software."]
    pub const fn enable(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN[1:0] = ‘00’), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN[1:0] different than ‘00’), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
    pub const fn set_sngstrt(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN[1:0] = ‘00’), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN[1:0] different than ‘00’), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
    pub const fn sngstrt(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN[1:0] = ‘00’), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN[1:0] different than ‘00’), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
    pub const fn set_cntstrt(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN[1:0] = ‘00’), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN[1:0] different than ‘00’), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
    pub const fn cntstrt(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "LPTIM interrupt clear register."]
pub struct IcrBits {
    bits: u32,
}
impl Default for IcrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IcrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
    pub const fn set_cccf(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 1);
        self.bits &= !(0x1 << (0x0 + idx * 0x9));
        self.bits |= if val { 1 << (0x0 + idx * 0x9) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
    pub const fn cccf(self, idx: usize) -> bool {
        assert!(idx < 1);
        ((self.bits >> (0x0 + idx * 0x9)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register."]
    pub const fn set_arrmcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register."]
    pub const fn arrmcf(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register."]
    pub const fn set_exttrigcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register."]
    pub const fn exttrigcf(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register."]
    pub const fn set_cmpokcf(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 1);
        self.bits &= !(0x1 << (0x3 + idx * 0x10));
        self.bits |= if val { 1 << (0x3 + idx * 0x10) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register."]
    pub const fn cmpokcf(self, idx: usize) -> bool {
        assert!(idx < 1);
        ((self.bits >> (0x3 + idx * 0x10)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register."]
    pub const fn set_arrokcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register."]
    pub const fn arrokcf(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn set_upcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn upcf(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn set_downcf(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn downcf(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "LPTIM interrupt enable register."]
pub struct IerBits {
    bits: u32,
}
impl Default for IerBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IerBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Capture/compare 1 interrupt enable."]
    pub const fn set_ccie(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 1);
        self.bits &= !(0x1 << (0x0 + idx * 0x9));
        self.bits |= if val { 1 << (0x0 + idx * 0x9) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Capture/compare 1 interrupt enable."]
    pub const fn ccie(self, idx: usize) -> bool {
        assert!(idx < 1);
        ((self.bits >> (0x0 + idx * 0x9)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Autoreload match Interrupt Enable."]
    pub const fn set_arrmie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Autoreload match Interrupt Enable."]
    pub const fn arrmie(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "External trigger valid edge Interrupt Enable."]
    pub const fn set_exttrigie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "External trigger valid edge Interrupt Enable."]
    pub const fn exttrigie(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Compare register 1 update OK interrupt enable."]
    pub const fn set_cmpokie(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 1);
        self.bits &= !(0x1 << (0x3 + idx * 0x10));
        self.bits |= if val { 1 << (0x3 + idx * 0x10) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Compare register 1 update OK interrupt enable."]
    pub const fn cmpokie(self, idx: usize) -> bool {
        assert!(idx < 1);
        ((self.bits >> (0x3 + idx * 0x10)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Autoreload register update OK Interrupt Enable."]
    pub const fn set_arrokie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Autoreload register update OK Interrupt Enable."]
    pub const fn arrokie(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn set_upie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn upie(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn set_downie(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn downie(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "LPTIM interrupt and status register."]
pub struct IsrBits {
    bits: u32,
}
impl Default for IsrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IsrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "Compare 1 interrupt flag The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. The CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register."]
    pub const fn set_ccif(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 1);
        self.bits &= !(0x1 << (0x0 + idx * 0x9));
        self.bits |= if val { 1 << (0x0 + idx * 0x9) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Compare 1 interrupt flag The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. The CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register."]
    pub const fn ccif(self, idx: usize) -> bool {
        assert!(idx < 1);
        ((self.bits >> (0x0 + idx * 0x9)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
    pub const fn set_arrm(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
    pub const fn arrm(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
    pub const fn set_exttrig(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
    pub const fn exttrig(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Compare register 1 update OK CMP1OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR1 register has been successfully completed. CMP1OK flag can be cleared by writing 1 to the CMP1OKCF bit in the LPTIM_ICR register."]
    pub const fn set_cmpok(mut self, idx: usize, val: bool) -> Self {
        assert!(idx < 1);
        self.bits &= !(0x1 << (0x3 + idx * 0x10));
        self.bits |= if val { 1 << (0x3 + idx * 0x10) } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Compare register 1 update OK CMP1OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR1 register has been successfully completed. CMP1OK flag can be cleared by writing 1 to the CMP1OKCF bit in the LPTIM_ICR register."]
    pub const fn cmpok(self, idx: usize) -> bool {
        assert!(idx < 1);
        ((self.bits >> (0x3 + idx * 0x10)) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
    pub const fn set_arrok(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
    pub const fn arrok(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn set_up(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn up(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn set_down(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
    pub const fn down(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CkpolVal {
    #[doc = "the rising edge is the active edge used for counting. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active."]
    Rising = 0x0,
    #[doc = "the falling edge is the active edge used for counting. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active."]
    Falling = 0x1,
    #[doc = "both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active."]
    Both = 0x2,
}
impl CkpolVal {
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
pub enum ClockSourceVal {
    #[doc = "clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    Internal = 0x0,
    #[doc = "clocked by an external clock source through the LPTIM external Input1"]
    External = 0x1,
}
impl ClockSourceVal {
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
pub enum FilterVal {
    Count1 = 0x0,

    Count2 = 0x1,

    Count4 = 0x2,

    Count8 = 0x3,
}
impl FilterVal {
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
pub enum PrescVal {
    Div1 = 0x0,

    Div2 = 0x1,

    Div4 = 0x2,

    Div8 = 0x3,

    Div16 = 0x4,

    Div32 = 0x5,

    Div64 = 0x6,

    Div128 = 0x7,
}
impl PrescVal {
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
pub enum TrigenVal {
    #[doc = "software trigger (counting start is initiated by software)"]
    Software = 0x0,
    #[doc = "rising edge is the active edge"]
    RisingEdge = 0x1,
    #[doc = "falling edge is the active edge"]
    FallingEdge = 0x2,
    #[doc = "both edges are active edges"]
    BothEdge = 0x3,
}
impl TrigenVal {
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
pub enum WavpolVal {
    #[doc = "The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers."]
    Positive = 0x0,
    #[doc = "The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers."]
    Negative = 0x1,
}
impl WavpolVal {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { ::core::mem::transmute(bits) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
