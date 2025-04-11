
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Low-power Universal synchronous asynchronous receiver transmitter"]
pub struct Lpuart {
    ptr: *mut u8,
}
impl Lpuart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "Control register 1"]
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 3"]
    pub const fn cr3(&self) -> utils::Reg<fields::Cr3, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Cr3, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Baud rate register"]
    pub const fn brr(&self) -> utils::Reg<fields::Brr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Brr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Request register"]
    pub const fn rqr(&self) -> utils::Reg<fields::Rqr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Rqr, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt & status register"]
    pub const fn isr(&self) -> utils::Reg<fields::Isr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<fields::Isr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt flag clear register"]
    pub const fn icr(&self) -> utils::Reg<fields::Icr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::Icr, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Receive data register"]
    pub const fn rdr(&self) -> utils::Reg<fields::Dr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::Dr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Transmit data register"]
    pub const fn tdr(&self) -> utils::Reg<fields::Dr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<fields::Dr, utils::WO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct Usart {
    ptr: *mut u8,
}
impl Usart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "Control register 1"]
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Cr2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 3"]
    pub const fn cr3(&self) -> utils::Reg<fields::Cr3, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Cr3, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Baud rate register"]
    pub const fn brr(&self) -> utils::Reg<fields::Brr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Brr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Guard time and prescaler register"]
    pub const fn gtpr(&self) -> utils::Reg<fields::Gtpr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Gtpr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Receiver timeout register"]
    pub const fn rtor(&self) -> utils::Reg<fields::Rtor, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Rtor, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Request register"]
    pub const fn rqr(&self) -> utils::Reg<fields::Rqr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Rqr, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt & status register"]
    pub const fn isr(&self) -> utils::Reg<fields::Isr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<fields::Isr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt flag clear register"]
    pub const fn icr(&self) -> utils::Reg<fields::Icr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::Icr, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Receive data register"]
    pub const fn rdr(&self) -> utils::Reg<fields::Dr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::Dr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Transmit data register"]
    pub const fn tdr(&self) -> utils::Reg<fields::Dr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<fields::Dr, utils::WO>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Baud rate register"]
    pub struct Brr {
        bits: u32,
    }
    impl Default for Brr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Brr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "USARTDIV"]
        pub const fn set_brr(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "USARTDIV"]
        pub const fn brr(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Control register 1"]
    pub struct Cr1 {
        bits: u32,
    }
    impl Default for Cr1 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr1 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "USART enable"]
        pub const fn set_ue(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "USART enable"]
        pub const fn ue(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "USART enable in Stop mode"]
        pub const fn set_uesm(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "USART enable in Stop mode"]
        pub const fn uesm(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
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
        #[doc = "IDLE interrupt enable"]
        pub const fn set_idleie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "IDLE interrupt enable"]
        pub const fn idleie(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "RXNE interrupt enable"]
        pub const fn set_rxneie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RXNE interrupt enable"]
        pub const fn rxneie(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transmission complete interrupt enable"]
        pub const fn set_tcie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transmission complete interrupt enable"]
        pub const fn tcie(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TXE interrupt enable"]
        pub const fn set_txeie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TXE interrupt enable"]
        pub const fn txeie(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "PE interrupt enable"]
        pub const fn set_peie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "PE interrupt enable"]
        pub const fn peie(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Parity selection"]
        pub const fn set_ps(mut self, val: vals::Ps) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
            self
        }
        #[inline(always)]
        #[doc = "Parity selection"]
        pub const fn ps(self) -> vals::Ps {
            let val = ((self.bits >> 0x9) & 0x1) as _;
            unsafe { vals::Ps::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Parity control enable"]
        pub const fn set_pce(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= if val { 1 << 0xa } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Parity control enable"]
        pub const fn pce(self) -> bool {
            ((self.bits >> 0xa) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Receiver wakeup method"]
        pub const fn set_wake(mut self, val: vals::Wake) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xb;
            self
        }
        #[inline(always)]
        #[doc = "Receiver wakeup method"]
        pub const fn wake(self) -> vals::Wake {
            let val = ((self.bits >> 0xb) & 0x1) as _;
            unsafe { vals::Wake::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Word length"]
        pub const fn set_m0(mut self, val: vals::M0) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xc;
            self
        }
        #[inline(always)]
        #[doc = "Word length"]
        pub const fn m0(self) -> vals::M0 {
            let val = ((self.bits >> 0xc) & 0x1) as _;
            unsafe { vals::M0::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Mute mode enable"]
        pub const fn set_mme(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= if val { 1 << 0xd } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Mute mode enable"]
        pub const fn mme(self) -> bool {
            ((self.bits >> 0xd) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Character match interrupt enable"]
        pub const fn set_cmie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Character match interrupt enable"]
        pub const fn cmie(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Oversampling mode"]
        pub const fn set_over8(mut self, val: vals::Over8) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
            self
        }
        #[inline(always)]
        #[doc = "Oversampling mode"]
        pub const fn over8(self) -> vals::Over8 {
            let val = ((self.bits >> 0xf) & 0x1) as _;
            unsafe { vals::Over8::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Driver Enable deassertion time"]
        pub const fn set_dedt(mut self, val: u8) -> Self {
            self.bits &= !(0x1f << 0x10);
            self.bits |= (val as u32 & 0x1f) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Driver Enable deassertion time"]
        pub const fn dedt(self) -> u8 {
            ((self.bits >> 0x10) & 0x1f) as _
        }
        #[inline(always)]
        #[doc = "Driver Enable assertion time"]
        pub const fn set_deat(mut self, val: u8) -> Self {
            self.bits &= !(0x1f << 0x15);
            self.bits |= (val as u32 & 0x1f) << 0x15;
            self
        }
        #[inline(always)]
        #[doc = "Driver Enable assertion time"]
        pub const fn deat(self) -> u8 {
            ((self.bits >> 0x15) & 0x1f) as _
        }
        #[inline(always)]
        #[doc = "Receiver timeout interrupt enable"]
        pub const fn set_rtoie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1a);
            self.bits |= if val { 1 << 0x1a } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Receiver timeout interrupt enable"]
        pub const fn rtoie(self) -> bool {
            ((self.bits >> 0x1a) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "End of Block interrupt enable"]
        pub const fn set_eobie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1b);
            self.bits |= if val { 1 << 0x1b } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "End of Block interrupt enable"]
        pub const fn eobie(self) -> bool {
            ((self.bits >> 0x1b) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Word length"]
        pub const fn set_m1(mut self, val: vals::M1) -> Self {
            self.bits &= !(0x1 << 0x1c);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1c;
            self
        }
        #[inline(always)]
        #[doc = "Word length"]
        pub const fn m1(self) -> vals::M1 {
            let val = ((self.bits >> 0x1c) & 0x1) as _;
            unsafe { vals::M1::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Control register 2"]
    pub struct Cr2 {
        bits: u32,
    }
    impl Default for Cr2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "7-bit Address Detection/4-bit Address Detection"]
        pub const fn set_addm(mut self, val: vals::Addm) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "7-bit Address Detection/4-bit Address Detection"]
        pub const fn addm(self) -> vals::Addm {
            let val = ((self.bits >> 0x4) & 0x1) as _;
            unsafe { vals::Addm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Line break detection length"]
        pub const fn set_lbdl(mut self, val: vals::Lbdl) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
            self
        }
        #[inline(always)]
        #[doc = "Line break detection length"]
        pub const fn lbdl(self) -> vals::Lbdl {
            let val = ((self.bits >> 0x5) & 0x1) as _;
            unsafe { vals::Lbdl::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "LIN break detection interrupt enable"]
        pub const fn set_lbdie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "LIN break detection interrupt enable"]
        pub const fn lbdie(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Last bit clock pulse"]
        pub const fn set_lbcl(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Last bit clock pulse"]
        pub const fn lbcl(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Clock phase"]
        pub const fn set_cpha(mut self, val: vals::Cpha) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x9;
            self
        }
        #[inline(always)]
        #[doc = "Clock phase"]
        pub const fn cpha(self) -> vals::Cpha {
            let val = ((self.bits >> 0x9) & 0x1) as _;
            unsafe { vals::Cpha::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clock polarity"]
        pub const fn set_cpol(mut self, val: vals::Cpol) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xa;
            self
        }
        #[inline(always)]
        #[doc = "Clock polarity"]
        pub const fn cpol(self) -> vals::Cpol {
            let val = ((self.bits >> 0xa) & 0x1) as _;
            unsafe { vals::Cpol::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clock enable"]
        pub const fn set_clken(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Clock enable"]
        pub const fn clken(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "STOP bits"]
        pub const fn set_stop(mut self, val: vals::Stop) -> Self {
            self.bits &= !(0x3 << 0xc);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0xc;
            self
        }
        #[inline(always)]
        #[doc = "STOP bits"]
        pub const fn stop(self) -> vals::Stop {
            let val = ((self.bits >> 0xc) & 0x3) as _;
            unsafe { vals::Stop::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "LIN mode enable"]
        pub const fn set_linen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "LIN mode enable"]
        pub const fn linen(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Swap TX/RX pins"]
        pub const fn set_swap(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= if val { 1 << 0xf } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Swap TX/RX pins"]
        pub const fn swap(self) -> bool {
            ((self.bits >> 0xf) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "RX pin active level inversion"]
        pub const fn set_rxinv(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x10);
            self.bits |= if val { 1 << 0x10 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RX pin active level inversion"]
        pub const fn rxinv(self) -> bool {
            ((self.bits >> 0x10) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "TX pin active level inversion"]
        pub const fn set_txinv(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x11);
            self.bits |= if val { 1 << 0x11 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "TX pin active level inversion"]
        pub const fn txinv(self) -> bool {
            ((self.bits >> 0x11) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Binary data inversion"]
        pub const fn set_datainv(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x12);
            self.bits |= if val { 1 << 0x12 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Binary data inversion"]
        pub const fn datainv(self) -> bool {
            ((self.bits >> 0x12) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Most significant bit first"]
        pub const fn set_msbfirst(mut self, val: vals::Msbfirst) -> Self {
            self.bits &= !(0x1 << 0x13);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x13;
            self
        }
        #[inline(always)]
        #[doc = "Most significant bit first"]
        pub const fn msbfirst(self) -> vals::Msbfirst {
            let val = ((self.bits >> 0x13) & 0x1) as _;
            unsafe { vals::Msbfirst::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Auto baud rate enable"]
        pub const fn set_abren(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x14);
            self.bits |= if val { 1 << 0x14 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Auto baud rate enable"]
        pub const fn abren(self) -> bool {
            ((self.bits >> 0x14) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Auto baud rate mode"]
        pub const fn set_abrmod(mut self, val: vals::Abrmod) -> Self {
            self.bits &= !(0x3 << 0x15);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x15;
            self
        }
        #[inline(always)]
        #[doc = "Auto baud rate mode"]
        pub const fn abrmod(self) -> vals::Abrmod {
            let val = ((self.bits >> 0x15) & 0x3) as _;
            unsafe { vals::Abrmod::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Receiver timeout enable"]
        pub const fn set_rtoen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x17);
            self.bits |= if val { 1 << 0x17 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Receiver timeout enable"]
        pub const fn rtoen(self) -> bool {
            ((self.bits >> 0x17) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Address of the USART node"]
        pub const fn set_add(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x18);
            self.bits |= (val as u32 & 0xff) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "Address of the USART node"]
        pub const fn add(self) -> u8 {
            ((self.bits >> 0x18) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Control register 3"]
    pub struct Cr3 {
        bits: u32,
    }
    impl Default for Cr3 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr3 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Error interrupt enable"]
        pub const fn set_eie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Error interrupt enable"]
        pub const fn eie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "IrDA mode enable"]
        pub const fn set_iren(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "IrDA mode enable"]
        pub const fn iren(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "IrDA low-power"]
        pub const fn set_irlp(mut self, val: vals::Irlp) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "IrDA low-power"]
        pub const fn irlp(self) -> vals::Irlp {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Irlp::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Half-duplex selection"]
        pub const fn set_hdsel(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Half-duplex selection"]
        pub const fn hdsel(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Smartcard NACK enable"]
        pub const fn set_nack(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Smartcard NACK enable"]
        pub const fn nack(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Smartcard mode enable"]
        pub const fn set_scen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Smartcard mode enable"]
        pub const fn scen(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "DMA enable receiver"]
        pub const fn set_dmar(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DMA enable receiver"]
        pub const fn dmar(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "DMA enable transmitter"]
        pub const fn set_dmat(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DMA enable transmitter"]
        pub const fn dmat(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "RTS enable"]
        pub const fn set_rtse(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "RTS enable"]
        pub const fn rtse(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CTS enable"]
        pub const fn set_ctse(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CTS enable"]
        pub const fn ctse(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CTS interrupt enable"]
        pub const fn set_ctsie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= if val { 1 << 0xa } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CTS interrupt enable"]
        pub const fn ctsie(self) -> bool {
            ((self.bits >> 0xa) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "One sample bit method enable"]
        pub const fn set_onebit(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "One sample bit method enable"]
        pub const fn onebit(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Overrun Disable"]
        pub const fn set_ovrdis(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= if val { 1 << 0xc } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Overrun Disable"]
        pub const fn ovrdis(self) -> bool {
            ((self.bits >> 0xc) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "DMA Disable on Reception Error"]
        pub const fn set_ddre(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= if val { 1 << 0xd } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "DMA Disable on Reception Error"]
        pub const fn ddre(self) -> bool {
            ((self.bits >> 0xd) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Driver enable mode"]
        pub const fn set_dem(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Driver enable mode"]
        pub const fn dem(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Driver enable polarity selection"]
        pub const fn set_dep(mut self, val: vals::Dep) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0xf;
            self
        }
        #[inline(always)]
        #[doc = "Driver enable polarity selection"]
        pub const fn dep(self) -> vals::Dep {
            let val = ((self.bits >> 0xf) & 0x1) as _;
            unsafe { vals::Dep::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Smartcard auto-retry count"]
        pub const fn set_scarcnt(mut self, val: u8) -> Self {
            self.bits &= !(0x7 << 0x11);
            self.bits |= (val as u32 & 0x7) << 0x11;
            self
        }
        #[inline(always)]
        #[doc = "Smartcard auto-retry count"]
        pub const fn scarcnt(self) -> u8 {
            ((self.bits >> 0x11) & 0x7) as _
        }
        #[inline(always)]
        #[doc = "Wakeup from Stop mode interrupt flag selection"]
        pub const fn set_wus(mut self, val: vals::Wus) -> Self {
            self.bits &= !(0x3 << 0x14);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x14;
            self
        }
        #[inline(always)]
        #[doc = "Wakeup from Stop mode interrupt flag selection"]
        pub const fn wus(self) -> vals::Wus {
            let val = ((self.bits >> 0x14) & 0x3) as _;
            unsafe { vals::Wus::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Wakeup from Stop mode interrupt enable"]
        pub const fn set_wufie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x16);
            self.bits |= if val { 1 << 0x16 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Wakeup from Stop mode interrupt enable"]
        pub const fn wufie(self) -> bool {
            ((self.bits >> 0x16) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Data register"]
    pub struct Dr {
        bits: u32,
    }
    impl Default for Dr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Data value"]
        pub const fn set_dr(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Data value"]
        pub const fn dr(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Guard time and prescaler register"]
    pub struct Gtpr {
        bits: u32,
    }
    impl Default for Gtpr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Gtpr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Prescaler value"]
        pub const fn set_psc(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Prescaler value"]
        pub const fn psc(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Guard time value"]
        pub const fn set_gt(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Guard time value"]
        pub const fn gt(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Interrupt flag clear register"]
    pub struct Icr {
        bits: u32,
    }
    impl Default for Icr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Icr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Parity error clear flag"]
        pub const fn set_pe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Parity error clear flag"]
        pub const fn pe(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Framing error clear flag"]
        pub const fn set_fe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Framing error clear flag"]
        pub const fn fe(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Noise error clear flag"]
        pub const fn set_ne(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Noise error clear flag"]
        pub const fn ne(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Overrun error clear flag"]
        pub const fn set_ore(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Overrun error clear flag"]
        pub const fn ore(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Idle line detected clear flag"]
        pub const fn set_idle(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Idle line detected clear flag"]
        pub const fn idle(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transmission complete clear flag"]
        pub const fn set_tc(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transmission complete clear flag"]
        pub const fn tc(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "LIN break detection clear flag"]
        pub const fn set_lbd(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "LIN break detection clear flag"]
        pub const fn lbd(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CTS clear flag"]
        pub const fn set_cts(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CTS clear flag"]
        pub const fn cts(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Receiver timeout clear flag"]
        pub const fn set_rtof(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Receiver timeout clear flag"]
        pub const fn rtof(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "End of block clear flag"]
        pub const fn set_eobf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= if val { 1 << 0xc } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "End of block clear flag"]
        pub const fn eobf(self) -> bool {
            ((self.bits >> 0xc) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Character match clear flag"]
        pub const fn set_cmf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x11);
            self.bits |= if val { 1 << 0x11 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Character match clear flag"]
        pub const fn cmf(self) -> bool {
            ((self.bits >> 0x11) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Wakeup from Stop mode clear flag"]
        pub const fn set_wuf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x14);
            self.bits |= if val { 1 << 0x14 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Wakeup from Stop mode clear flag"]
        pub const fn wuf(self) -> bool {
            ((self.bits >> 0x14) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Interrupt & status register"]
    pub struct Isr {
        bits: u32,
    }
    impl Default for Isr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Isr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Parity error"]
        pub const fn set_pe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Parity error"]
        pub const fn pe(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Framing error"]
        pub const fn set_fe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Framing error"]
        pub const fn fe(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Noise error flag"]
        pub const fn set_ne(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Noise error flag"]
        pub const fn ne(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Overrun error"]
        pub const fn set_ore(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Overrun error"]
        pub const fn ore(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Idle line detected"]
        pub const fn set_idle(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Idle line detected"]
        pub const fn idle(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Read data register not empty"]
        pub const fn set_rxne(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Read data register not empty"]
        pub const fn rxne(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transmission complete"]
        pub const fn set_tc(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transmission complete"]
        pub const fn tc(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transmit data register empty"]
        pub const fn set_txe(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transmit data register empty"]
        pub const fn txe(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "LIN break detection flag"]
        pub const fn set_lbd(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "LIN break detection flag"]
        pub const fn lbd(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CTS interrupt flag"]
        pub const fn set_ctsif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CTS interrupt flag"]
        pub const fn ctsif(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CTS flag"]
        pub const fn set_cts(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= if val { 1 << 0xa } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CTS flag"]
        pub const fn cts(self) -> bool {
            ((self.bits >> 0xa) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Receiver timeout"]
        pub const fn set_rtof(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Receiver timeout"]
        pub const fn rtof(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "End of block flag"]
        pub const fn set_eobf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= if val { 1 << 0xc } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "End of block flag"]
        pub const fn eobf(self) -> bool {
            ((self.bits >> 0xc) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Auto baud rate error"]
        pub const fn set_abre(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Auto baud rate error"]
        pub const fn abre(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Auto baud rate flag"]
        pub const fn set_abrf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xf);
            self.bits |= if val { 1 << 0xf } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Auto baud rate flag"]
        pub const fn abrf(self) -> bool {
            ((self.bits >> 0xf) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Busy flag"]
        pub const fn set_busy(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x10);
            self.bits |= if val { 1 << 0x10 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Busy flag"]
        pub const fn busy(self) -> bool {
            ((self.bits >> 0x10) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "character match flag"]
        pub const fn set_cmf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x11);
            self.bits |= if val { 1 << 0x11 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "character match flag"]
        pub const fn cmf(self) -> bool {
            ((self.bits >> 0x11) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Send break flag"]
        pub const fn set_sbkf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x12);
            self.bits |= if val { 1 << 0x12 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Send break flag"]
        pub const fn sbkf(self) -> bool {
            ((self.bits >> 0x12) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Receiver wakeup from Mute mode"]
        pub const fn set_rwu(mut self, val: vals::Rwu) -> Self {
            self.bits &= !(0x1 << 0x13);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x13;
            self
        }
        #[inline(always)]
        #[doc = "Receiver wakeup from Mute mode"]
        pub const fn rwu(self) -> vals::Rwu {
            let val = ((self.bits >> 0x13) & 0x1) as _;
            unsafe { vals::Rwu::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Wakeup from Stop mode flag"]
        pub const fn set_wuf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x14);
            self.bits |= if val { 1 << 0x14 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Wakeup from Stop mode flag"]
        pub const fn wuf(self) -> bool {
            ((self.bits >> 0x14) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transmit enable acknowledge flag"]
        pub const fn set_teack(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x15);
            self.bits |= if val { 1 << 0x15 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transmit enable acknowledge flag"]
        pub const fn teack(self) -> bool {
            ((self.bits >> 0x15) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Receive enable acknowledge flag"]
        pub const fn set_reack(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x16);
            self.bits |= if val { 1 << 0x16 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Receive enable acknowledge flag"]
        pub const fn reack(self) -> bool {
            ((self.bits >> 0x16) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Request register"]
    pub struct Rqr {
        bits: u32,
    }
    impl Default for Rqr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Rqr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Auto baud rate request. Resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame."]
        pub const fn set_abrrq(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Auto baud rate request. Resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame."]
        pub const fn abrrq(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Send break request. Sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
        pub const fn set_sbkrq(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Send break request. Sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
        pub const fn sbkrq(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Mute mode request. Puts the USART in mute mode and sets the RWU flag."]
        pub const fn set_mmrq(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Mute mode request. Puts the USART in mute mode and sets the RWU flag."]
        pub const fn mmrq(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Receive data flush request. Clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
        pub const fn set_rxfrq(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Receive data flush request. Clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
        pub const fn rxfrq(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transmit data flush request. Sets the TXE flags. This allows to discard the transmit data."]
        pub const fn set_txfrq(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transmit data flush request. Sets the TXE flags. This allows to discard the transmit data."]
        pub const fn txfrq(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Receiver timeout register"]
    pub struct Rtor {
        bits: u32,
    }
    impl Default for Rtor {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Rtor {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Receiver timeout value"]
        pub const fn set_rto(mut self, val: u32) -> Self {
            self.bits &= !(0xffffff << 0x0);
            self.bits |= (val as u32 & 0xffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Receiver timeout value"]
        pub const fn rto(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffff) as _
        }
        #[inline(always)]
        #[doc = "Block Length"]
        pub const fn set_blen(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x18);
            self.bits |= (val as u32 & 0xff) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "Block Length"]
        pub const fn blen(self) -> u8 {
            ((self.bits >> 0x18) & 0xff) as _
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum Abrmod {
        #[doc = "Measurement of the start bit is used to detect the baud rate"]
        Start = 0x0,
        #[doc = "Falling edge to falling edge measurement"]
        Edge = 0x1,
        #[doc = "0x7F frame detection"]
        Frame7F = 0x2,
        #[doc = "0x55 frame detection"]
        Frame55 = 0x3,
    }
    impl Abrmod {
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
    pub enum Addm {
        #[doc = "4-bit address detection"]
        Bit4 = 0x0,
        #[doc = "7-bit address detection"]
        Bit7 = 0x1,
    }
    impl Addm {
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
    pub enum Cpha {
        #[doc = "The first clock transition is the first data capture edge"]
        First = 0x0,
        #[doc = "The second clock transition is the first data capture edge"]
        Second = 0x1,
    }
    impl Cpha {
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
    pub enum Cpol {
        #[doc = "Steady low value on CK pin outside transmission window"]
        Low = 0x0,
        #[doc = "Steady high value on CK pin outside transmission window"]
        High = 0x1,
    }
    impl Cpol {
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
    pub enum Dep {
        #[doc = "DE signal is active high"]
        High = 0x0,
        #[doc = "DE signal is active low"]
        Low = 0x1,
    }
    impl Dep {
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
    pub enum Irlp {
        #[doc = "Normal mode"]
        Normal = 0x0,
        #[doc = "Low-power mode"]
        LowPower = 0x1,
    }
    impl Irlp {
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
    pub enum Lbdl {
        #[doc = "10-bit break detection"]
        Bit10 = 0x0,
        #[doc = "11-bit break detection"]
        Bit11 = 0x1,
    }
    impl Lbdl {
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
    pub enum M0 {
        #[doc = "1 start bit, 8 data bits, n stop bits"]
        Bit8 = 0x0,
        #[doc = "1 start bit, 9 data bits, n stop bits"]
        Bit9 = 0x1,
    }
    impl M0 {
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
    pub enum M1 {
        #[doc = "Use M0 to set the data bits"]
        M0 = 0x0,
        #[doc = "1 start bit, 7 data bits, n stop bits"]
        Bit7 = 0x1,
    }
    impl M1 {
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
    pub enum Msbfirst {
        #[doc = "data is transmitted/received with data bit 0 first, following the start bit"]
        Lsb = 0x0,
        #[doc = "data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
        Msb = 0x1,
    }
    impl Msbfirst {
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
    pub enum Over8 {
        #[doc = "Oversampling by 16"]
        Oversampling16 = 0x0,
        #[doc = "Oversampling by 8"]
        Oversampling8 = 0x1,
    }
    impl Over8 {
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
    pub enum Ps {
        #[doc = "Even parity"]
        Even = 0x0,
        #[doc = "Odd parity"]
        Odd = 0x1,
    }
    impl Ps {
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
    pub enum Rwu {
        #[doc = "Receiver in active mode"]
        Active = 0x0,
        #[doc = "Receiver in mute mode"]
        Mute = 0x1,
    }
    impl Rwu {
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
    pub enum Stop {
        #[doc = "1 stop bit"]
        Stop1 = 0x0,
        #[doc = "0.5 stop bits"]
        Stop0p5 = 0x1,
        #[doc = "2 stop bits"]
        Stop2 = 0x2,
        #[doc = "1.5 stop bits"]
        Stop1p5 = 0x3,
    }
    impl Stop {
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
    pub enum Wake {
        #[doc = "USART wakeup on idle line"]
        IdleLine = 0x0,
        #[doc = "USART wakeup on address mark"]
        AddressMark = 0x1,
    }
    impl Wake {
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
    pub enum Wus {
        #[doc = "WUF active on address match"]
        Address = 0x0,
        #[doc = "WuF active on Start bit detection"]
        Start = 0x2,
        #[doc = "WUF active on RXNE"]
        Rxne = 0x3,
    }
    impl Wus {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
            unsafe { ::core::mem::transmute(bits) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            self as u8
        }
    }
}
