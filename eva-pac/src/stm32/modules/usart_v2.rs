
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Universal asynchronous receiver transmitter"]
pub struct Uart {
    ptr: *mut u8,
}
impl Uart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "Status register"]
    pub const fn sr(&self) -> utils::Reg<fields::Sr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Sr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Data register"]
    pub const fn dr(&self) -> utils::Reg<fields::Dr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Dr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Baud rate register"]
    pub const fn brr(&self) -> utils::Reg<fields::Brr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Brr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 1"]
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Cr1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Cr2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 3"]
    pub const fn cr3(&self) -> utils::Reg<fields::Cr3, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Cr3, utils::RW>>::from_ptr(ptr)
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
    #[doc = "Status register"]
    pub const fn sr(&self) -> utils::Reg<fields::Sr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Sr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Data register"]
    pub const fn dr(&self) -> utils::Reg<fields::Dr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Dr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Baud rate register"]
    pub const fn brr(&self) -> utils::Reg<fields::Brr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Brr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 1"]
    pub const fn cr1(&self) -> utils::Reg<fields::Cr1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Cr1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 2"]
    pub const fn cr2(&self) -> utils::Reg<fields::Cr2Usart, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Cr2Usart, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Control register 3"]
    pub const fn cr3(&self) -> utils::Reg<fields::Cr3Usart, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Cr3Usart, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Guard time and prescaler register"]
    pub const fn gtpr(&self) -> utils::Reg<fields::Gtpr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Gtpr, utils::RW>>::from_ptr(ptr)
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
        #[doc = "Send break"]
        pub const fn set_sbk(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Send break"]
        pub const fn sbk(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Receiver wakeup"]
        pub const fn set_rwu(mut self, val: vals::Rwu) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "Receiver wakeup"]
        pub const fn rwu(self) -> vals::Rwu {
            let val = ((self.bits >> 0x1) & 0x1) as _;
            unsafe { vals::Rwu::from_bits_unchecked(val) }
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
        #[doc = "USART enable"]
        pub const fn set_ue(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= if val { 1 << 0xd } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "USART enable"]
        pub const fn ue(self) -> bool {
            ((self.bits >> 0xd) & 0x1) != 0
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
        #[doc = "Address of the USART node"]
        pub const fn set_add(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x0);
            self.bits |= (val as u32 & 0xf) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Address of the USART node"]
        pub const fn add(self) -> u8 {
            ((self.bits >> 0x0) & 0xf) as _
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
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Control register 2"]
    pub struct Cr2Usart {
        bits: u32,
    }
    impl Default for Cr2Usart {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr2Usart {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Address of the USART node"]
        pub const fn set_add(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x0);
            self.bits |= (val as u32 & 0xf) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Address of the USART node"]
        pub const fn add(self) -> u8 {
            ((self.bits >> 0x0) & 0xf) as _
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
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Control register 3"]
    pub struct Cr3Usart {
        bits: u32,
    }
    impl Default for Cr3Usart {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr3Usart {
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
    #[doc = "Status register"]
    pub struct Sr {
        bits: u32,
    }
    impl Default for Sr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sr {
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
        #[doc = "CTS flag"]
        pub const fn set_cts(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CTS flag"]
        pub const fn cts(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
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
}
