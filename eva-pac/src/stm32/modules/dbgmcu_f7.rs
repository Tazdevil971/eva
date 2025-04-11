
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Debug support"]
pub struct Dbgmcu {
    ptr: *mut u8,
}
impl Dbgmcu {
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
    #[doc = "IDCODE"]
    pub const fn idcode(&self) -> utils::Reg<IdcodeBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<IdcodeBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control Register"]
    pub const fn cr(&self) -> utils::Reg<CrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<CrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Debug MCU APB1 Freeze register"]
    pub const fn apb1fzr(&self) -> utils::Reg<Apb1fzrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<Apb1fzrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Debug MCU APB2 Freeze register"]
    pub const fn apb2fzr(&self) -> utils::Reg<Apb2fzrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<Apb2fzrBits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Debug MCU APB1 Freeze register"]
pub struct Apb1fzrBits {
    bits: u32,
}
impl Default for Apb1fzrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Apb1fzrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TIM2"]
    pub const fn set_tim2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM2"]
    pub const fn tim2(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM3"]
    pub const fn set_tim3(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM3"]
    pub const fn tim3(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM4"]
    pub const fn set_tim4(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM4"]
    pub const fn tim4(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM5"]
    pub const fn set_tim5(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x3);
        self.bits |= if val { 1 << 0x3 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM5"]
    pub const fn tim5(self) -> bool {
        ((self.bits >> 0x3) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM6"]
    pub const fn set_tim6(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x4);
        self.bits |= if val { 1 << 0x4 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM6"]
    pub const fn tim6(self) -> bool {
        ((self.bits >> 0x4) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM7"]
    pub const fn set_tim7(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM7"]
    pub const fn tim7(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM12"]
    pub const fn set_tim12(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x6);
        self.bits |= if val { 1 << 0x6 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM12"]
    pub const fn tim12(self) -> bool {
        ((self.bits >> 0x6) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM13"]
    pub const fn set_tim13(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x7);
        self.bits |= if val { 1 << 0x7 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM13"]
    pub const fn tim13(self) -> bool {
        ((self.bits >> 0x7) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM14"]
    pub const fn set_tim14(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x8);
        self.bits |= if val { 1 << 0x8 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM14"]
    pub const fn tim14(self) -> bool {
        ((self.bits >> 0x8) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "LPTIM1"]
    pub const fn set_lptim1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x9);
        self.bits |= if val { 1 << 0x9 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "LPTIM1"]
    pub const fn lptim1(self) -> bool {
        ((self.bits >> 0x9) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "RTC"]
    pub const fn set_rtc(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xa);
        self.bits |= if val { 1 << 0xa } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "RTC"]
    pub const fn rtc(self) -> bool {
        ((self.bits >> 0xa) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "WWDG"]
    pub const fn set_wwdg(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xb);
        self.bits |= if val { 1 << 0xb } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "WWDG"]
    pub const fn wwdg(self) -> bool {
        ((self.bits >> 0xb) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "IWDG"]
    pub const fn set_iwdg(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xc);
        self.bits |= if val { 1 << 0xc } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "IWDG"]
    pub const fn iwdg(self) -> bool {
        ((self.bits >> 0xc) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN3"]
    pub const fn set_can3(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0xd);
        self.bits |= if val { 1 << 0xd } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN3"]
    pub const fn can3(self) -> bool {
        ((self.bits >> 0xd) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DBG_I2C1_SMBUS_TIMEOUT"]
    pub const fn set_dbg_i2c1_smbus_timeout(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x15);
        self.bits |= if val { 1 << 0x15 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DBG_I2C1_SMBUS_TIMEOUT"]
    pub const fn dbg_i2c1_smbus_timeout(self) -> bool {
        ((self.bits >> 0x15) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DBG_I2C2_SMBUS_TIMEOUT"]
    pub const fn set_dbg_i2c2_smbus_timeout(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x16);
        self.bits |= if val { 1 << 0x16 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DBG_I2C2_SMBUS_TIMEOUT"]
    pub const fn dbg_i2c2_smbus_timeout(self) -> bool {
        ((self.bits >> 0x16) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DBG_I2C3_SMBUS_TIMEOUT"]
    pub const fn set_dbg_i2c3_smbus_timeout(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x17);
        self.bits |= if val { 1 << 0x17 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DBG_I2C3_SMBUS_TIMEOUT"]
    pub const fn dbg_i2c3_smbus_timeout(self) -> bool {
        ((self.bits >> 0x17) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DBG_I2C4SMBUS_TIMEOUT"]
    pub const fn set_dbg_i2c4_smbus_timeout(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x18);
        self.bits |= if val { 1 << 0x18 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DBG_I2C4SMBUS_TIMEOUT"]
    pub const fn dbg_i2c4_smbus_timeout(self) -> bool {
        ((self.bits >> 0x18) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN1"]
    pub const fn set_can1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x19);
        self.bits |= if val { 1 << 0x19 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN1"]
    pub const fn can1(self) -> bool {
        ((self.bits >> 0x19) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "CAN2"]
    pub const fn set_can2(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1a);
        self.bits |= if val { 1 << 0x1a } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "CAN2"]
    pub const fn can2(self) -> bool {
        ((self.bits >> 0x1a) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Debug MCU APB2 Freeze register"]
pub struct Apb2fzrBits {
    bits: u32,
}
impl Default for Apb2fzrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Apb2fzrBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "TIM1 counter stopped when core is halted"]
    pub const fn set_tim1(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM1 counter stopped when core is halted"]
    pub const fn tim1(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM8 counter stopped when core is halted"]
    pub const fn set_tim8(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM8 counter stopped when core is halted"]
    pub const fn tim8(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM9 counter stopped when core is halted"]
    pub const fn set_tim9(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x10);
        self.bits |= if val { 1 << 0x10 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM9 counter stopped when core is halted"]
    pub const fn tim9(self) -> bool {
        ((self.bits >> 0x10) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM10 counter stopped when core is halted"]
    pub const fn set_tim10(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x11);
        self.bits |= if val { 1 << 0x11 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM10 counter stopped when core is halted"]
    pub const fn tim10(self) -> bool {
        ((self.bits >> 0x11) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TIM11 counter stopped when core is halted"]
    pub const fn set_tim11(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x12);
        self.bits |= if val { 1 << 0x12 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TIM11 counter stopped when core is halted"]
    pub const fn tim11(self) -> bool {
        ((self.bits >> 0x12) & 0x1) != 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "Control Register"]
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
    #[doc = "DBG_SLEEP"]
    pub const fn set_dbg_sleep(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x0);
        self.bits |= if val { 1 << 0x0 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DBG_SLEEP"]
    pub const fn dbg_sleep(self) -> bool {
        ((self.bits >> 0x0) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DBG_STOP"]
    pub const fn set_dbg_stop(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x1);
        self.bits |= if val { 1 << 0x1 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DBG_STOP"]
    pub const fn dbg_stop(self) -> bool {
        ((self.bits >> 0x1) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "DBG_STANDBY"]
    pub const fn set_dbg_standby(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x2);
        self.bits |= if val { 1 << 0x2 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "DBG_STANDBY"]
    pub const fn dbg_standby(self) -> bool {
        ((self.bits >> 0x2) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TRACE_IOEN"]
    pub const fn set_trace_ioen(mut self, val: bool) -> Self {
        self.bits &= !(0x1 << 0x5);
        self.bits |= if val { 1 << 0x5 } else { 0 };
        self
    }
    #[inline(always)]
    #[doc = "TRACE_IOEN"]
    pub const fn trace_ioen(self) -> bool {
        ((self.bits >> 0x5) & 0x1) != 0
    }
    #[inline(always)]
    #[doc = "TRACE_MODE"]
    pub const fn set_trace_mode(mut self, val: u8) -> Self {
        self.bits &= !(0x3 << 0x6);
        self.bits |= (val as u32 & 0x3) << 0x6;
        self
    }
    #[inline(always)]
    #[doc = "TRACE_MODE"]
    pub const fn trace_mode(self) -> u8 {
        ((self.bits >> 0x6) & 0x3) as _
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "IDCODE"]
pub struct IdcodeBits {
    bits: u32,
}
impl Default for IdcodeBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl IdcodeBits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DEV_ID"]
    pub const fn set_dev_id(mut self, val: u16) -> Self {
        self.bits &= !(0xfff << 0x0);
        self.bits |= (val as u32 & 0xfff) << 0x0;
        self
    }
    #[inline(always)]
    #[doc = "DEV_ID"]
    pub const fn dev_id(self) -> u16 {
        ((self.bits >> 0x0) & 0xfff) as _
    }
    #[inline(always)]
    #[doc = "REV_ID"]
    pub const fn set_rev_id(mut self, val: u16) -> Self {
        self.bits &= !(0xffff << 0x10);
        self.bits |= (val as u32 & 0xffff) << 0x10;
        self
    }
    #[inline(always)]
    #[doc = "REV_ID"]
    pub const fn rev_id(self) -> u16 {
        ((self.bits >> 0x10) & 0xffff) as _
    }
}
