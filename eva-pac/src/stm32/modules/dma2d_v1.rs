
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "DMA2D controller"]
pub struct Dma2d {
    ptr: *mut u8,
}
impl Dma2d {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "control register"]
    pub const fn cr(&self) -> utils::Reg<fields::Cr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt Status Register"]
    pub const fn isr(&self) -> utils::Reg<fields::Isr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Isr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "interrupt flag clear register"]
    pub const fn ifcr(&self) -> utils::Reg<fields::Ifcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Ifcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "foreground memory address register"]
    pub const fn fgmar(&self) -> utils::Reg<fields::Fgmar, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Fgmar, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "foreground offset register"]
    pub const fn fgor(&self) -> utils::Reg<fields::Fgor, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Fgor, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "background memory address register"]
    pub const fn bgmar(&self) -> utils::Reg<fields::Bgmar, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Bgmar, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "background offset register"]
    pub const fn bgor(&self) -> utils::Reg<fields::Bgor, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Bgor, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "foreground PFC control register"]
    pub const fn fgpfccr(&self) -> utils::Reg<fields::Fgpfccr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<fields::Fgpfccr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "foreground color register"]
    pub const fn fgcolr(&self) -> utils::Reg<fields::Fgcolr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20);
            <utils::Reg<fields::Fgcolr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "background PFC control register"]
    pub const fn bgpfccr(&self) -> utils::Reg<fields::Bgpfccr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::Bgpfccr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "background color register"]
    pub const fn bgcolr(&self) -> utils::Reg<fields::Bgcolr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<fields::Bgcolr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "foreground CLUT memory address register"]
    pub const fn fgcmar(&self) -> utils::Reg<fields::Fgcmar, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::Fgcmar, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "background CLUT memory address register"]
    pub const fn bgcmar(&self) -> utils::Reg<fields::Bgcmar, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<fields::Bgcmar, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "output PFC control register"]
    pub const fn opfccr(&self) -> utils::Reg<fields::Opfccr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<fields::Opfccr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "output color register"]
    pub const fn ocolr(&self) -> utils::Reg<fields::Ocolr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<fields::Ocolr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "output memory address register"]
    pub const fn omar(&self) -> utils::Reg<fields::Omar, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c);
            <utils::Reg<fields::Omar, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "output offset register"]
    pub const fn oor(&self) -> utils::Reg<fields::Oor, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<fields::Oor, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "number of line register"]
    pub const fn nlr(&self) -> utils::Reg<fields::Nlr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<fields::Nlr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "line watermark register"]
    pub const fn lwr(&self) -> utils::Reg<fields::Lwr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<fields::Lwr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "AHB master timer configuration register"]
    pub const fn amtcr(&self) -> utils::Reg<fields::Amtcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c);
            <utils::Reg<fields::Amtcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "FGCLUT"]
    pub const fn fgclut(&self) -> utils::Reg<fields::Fgclut, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x400);
            <utils::Reg<fields::Fgclut, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "BGCLUT"]
    pub const fn bgclut(&self) -> utils::Reg<fields::Bgclut, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x800);
            <utils::Reg<fields::Bgclut, utils::RW>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "AHB master timer configuration register"]
    pub struct Amtcr {
        bits: u32,
    }
    impl Default for Amtcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Amtcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Enable"]
        pub const fn set_en(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Enable"]
        pub const fn en(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Dead Time"]
        pub const fn set_dt(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Dead Time"]
        pub const fn dt(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "BGCLUT"]
    pub struct Bgclut {
        bits: u32,
    }
    impl Default for Bgclut {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bgclut {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "BLUE"]
        pub const fn set_blue(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "BLUE"]
        pub const fn blue(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "GREEN"]
        pub const fn set_green(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "GREEN"]
        pub const fn green(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "RED"]
        pub const fn set_red(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x10);
            self.bits |= (val as u32 & 0xff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "RED"]
        pub const fn red(self) -> u8 {
            ((self.bits >> 0x10) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "APLHA"]
        pub const fn set_aplha(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x18);
            self.bits |= (val as u32 & 0xff) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "APLHA"]
        pub const fn aplha(self) -> u8 {
            ((self.bits >> 0x18) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "background CLUT memory address register"]
    pub struct Bgcmar {
        bits: u32,
    }
    impl Default for Bgcmar {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bgcmar {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Memory address"]
        pub const fn set_ma(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Memory address"]
        pub const fn ma(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "background color register"]
    pub struct Bgcolr {
        bits: u32,
    }
    impl Default for Bgcolr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bgcolr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Blue Value"]
        pub const fn set_blue(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Blue Value"]
        pub const fn blue(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Green Value"]
        pub const fn set_green(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Green Value"]
        pub const fn green(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Red Value"]
        pub const fn set_red(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x10);
            self.bits |= (val as u32 & 0xff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Red Value"]
        pub const fn red(self) -> u8 {
            ((self.bits >> 0x10) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "background memory address register"]
    pub struct Bgmar {
        bits: u32,
    }
    impl Default for Bgmar {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bgmar {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Memory address"]
        pub const fn set_ma(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Memory address"]
        pub const fn ma(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "background offset register"]
    pub struct Bgor {
        bits: u32,
    }
    impl Default for Bgor {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bgor {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Line offset"]
        pub const fn set_lo(mut self, val: u16) -> Self {
            self.bits &= !(0x3fff << 0x0);
            self.bits |= (val as u32 & 0x3fff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Line offset"]
        pub const fn lo(self) -> u16 {
            ((self.bits >> 0x0) & 0x3fff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "background PFC control register"]
    pub struct Bgpfccr {
        bits: u32,
    }
    impl Default for Bgpfccr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bgpfccr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Color mode"]
        pub const fn set_cm(mut self, val: vals::BgpfccrCm) -> Self {
            self.bits &= !(0xf << 0x0);
            self.bits |= (val.to_bits() as u32 & 0xf) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Color mode"]
        pub const fn cm(self) -> vals::BgpfccrCm {
            let val = ((self.bits >> 0x0) & 0xf) as _;
            unsafe { vals::BgpfccrCm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "CLUT Color mode"]
        pub const fn set_ccm(mut self, val: vals::BgpfccrCcm) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "CLUT Color mode"]
        pub const fn ccm(self) -> vals::BgpfccrCcm {
            let val = ((self.bits >> 0x4) & 0x1) as _;
            unsafe { vals::BgpfccrCcm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Start"]
        pub const fn set_start(mut self, val: vals::BgpfccrStart) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
            self
        }
        #[inline(always)]
        #[doc = "Start"]
        pub const fn start(self) -> vals::BgpfccrStart {
            let val = ((self.bits >> 0x5) & 0x1) as _;
            unsafe { vals::BgpfccrStart::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "CLUT size"]
        pub const fn set_cs(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "CLUT size"]
        pub const fn cs(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Alpha mode"]
        pub const fn set_am(mut self, val: vals::BgpfccrAm) -> Self {
            self.bits &= !(0x3 << 0x10);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Alpha mode"]
        pub const fn am(self) -> vals::BgpfccrAm {
            let val = ((self.bits >> 0x10) & 0x3) as _;
            unsafe { vals::BgpfccrAm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Alpha value"]
        pub const fn set_alpha(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x18);
            self.bits |= (val as u32 & 0xff) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "Alpha value"]
        pub const fn alpha(self) -> u8 {
            ((self.bits >> 0x18) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "control register"]
    pub struct Cr {
        bits: u32,
    }
    impl Default for Cr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Start"]
        pub const fn set_start(mut self, val: vals::CrStart) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Start"]
        pub const fn start(self) -> vals::CrStart {
            let val = ((self.bits >> 0x0) & 0x1) as _;
            unsafe { vals::CrStart::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Suspend"]
        pub const fn set_susp(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Suspend"]
        pub const fn susp(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Abort"]
        pub const fn set_abort(mut self, val: vals::Abort) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Abort"]
        pub const fn abort(self) -> vals::Abort {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Abort::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Transfer error interrupt enable"]
        pub const fn set_teie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transfer error interrupt enable"]
        pub const fn teie(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transfer complete interrupt enable"]
        pub const fn set_tcie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x9);
            self.bits |= if val { 1 << 0x9 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transfer complete interrupt enable"]
        pub const fn tcie(self) -> bool {
            ((self.bits >> 0x9) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transfer watermark interrupt enable"]
        pub const fn set_twie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xa);
            self.bits |= if val { 1 << 0xa } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transfer watermark interrupt enable"]
        pub const fn twie(self) -> bool {
            ((self.bits >> 0xa) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CLUT access error interrupt enable"]
        pub const fn set_caeie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CLUT access error interrupt enable"]
        pub const fn caeie(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CLUT transfer complete interrupt enable"]
        pub const fn set_ctcie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= if val { 1 << 0xc } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CLUT transfer complete interrupt enable"]
        pub const fn ctcie(self) -> bool {
            ((self.bits >> 0xc) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Configuration Error Interrupt Enable"]
        pub const fn set_ceie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= if val { 1 << 0xd } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Configuration Error Interrupt Enable"]
        pub const fn ceie(self) -> bool {
            ((self.bits >> 0xd) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "DMA2D mode"]
        pub const fn set_mode(mut self, val: vals::Mode) -> Self {
            self.bits &= !(0x3 << 0x10);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "DMA2D mode"]
        pub const fn mode(self) -> vals::Mode {
            let val = ((self.bits >> 0x10) & 0x3) as _;
            unsafe { vals::Mode::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "FGCLUT"]
    pub struct Fgclut {
        bits: u32,
    }
    impl Default for Fgclut {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fgclut {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "BLUE"]
        pub const fn set_blue(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "BLUE"]
        pub const fn blue(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "GREEN"]
        pub const fn set_green(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "GREEN"]
        pub const fn green(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "RED"]
        pub const fn set_red(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x10);
            self.bits |= (val as u32 & 0xff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "RED"]
        pub const fn red(self) -> u8 {
            ((self.bits >> 0x10) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "APLHA"]
        pub const fn set_aplha(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x18);
            self.bits |= (val as u32 & 0xff) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "APLHA"]
        pub const fn aplha(self) -> u8 {
            ((self.bits >> 0x18) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "foreground CLUT memory address register"]
    pub struct Fgcmar {
        bits: u32,
    }
    impl Default for Fgcmar {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fgcmar {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Memory Address"]
        pub const fn set_ma(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Memory Address"]
        pub const fn ma(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "foreground color register"]
    pub struct Fgcolr {
        bits: u32,
    }
    impl Default for Fgcolr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fgcolr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Blue Value"]
        pub const fn set_blue(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Blue Value"]
        pub const fn blue(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Green Value"]
        pub const fn set_green(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Green Value"]
        pub const fn green(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Red Value"]
        pub const fn set_red(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x10);
            self.bits |= (val as u32 & 0xff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Red Value"]
        pub const fn red(self) -> u8 {
            ((self.bits >> 0x10) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "foreground memory address register"]
    pub struct Fgmar {
        bits: u32,
    }
    impl Default for Fgmar {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fgmar {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Memory address"]
        pub const fn set_ma(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Memory address"]
        pub const fn ma(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "foreground offset register"]
    pub struct Fgor {
        bits: u32,
    }
    impl Default for Fgor {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fgor {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Line offset"]
        pub const fn set_lo(mut self, val: u16) -> Self {
            self.bits &= !(0x3fff << 0x0);
            self.bits |= (val as u32 & 0x3fff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Line offset"]
        pub const fn lo(self) -> u16 {
            ((self.bits >> 0x0) & 0x3fff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "foreground PFC control register"]
    pub struct Fgpfccr {
        bits: u32,
    }
    impl Default for Fgpfccr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Fgpfccr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Color mode"]
        pub const fn set_cm(mut self, val: vals::FgpfccrCm) -> Self {
            self.bits &= !(0xf << 0x0);
            self.bits |= (val.to_bits() as u32 & 0xf) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Color mode"]
        pub const fn cm(self) -> vals::FgpfccrCm {
            let val = ((self.bits >> 0x0) & 0xf) as _;
            unsafe { vals::FgpfccrCm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "CLUT color mode"]
        pub const fn set_ccm(mut self, val: vals::FgpfccrCcm) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "CLUT color mode"]
        pub const fn ccm(self) -> vals::FgpfccrCcm {
            let val = ((self.bits >> 0x4) & 0x1) as _;
            unsafe { vals::FgpfccrCcm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Start"]
        pub const fn set_start(mut self, val: vals::FgpfccrStart) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
            self
        }
        #[inline(always)]
        #[doc = "Start"]
        pub const fn start(self) -> vals::FgpfccrStart {
            let val = ((self.bits >> 0x5) & 0x1) as _;
            unsafe { vals::FgpfccrStart::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "CLUT size"]
        pub const fn set_cs(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "CLUT size"]
        pub const fn cs(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Alpha mode"]
        pub const fn set_am(mut self, val: vals::FgpfccrAm) -> Self {
            self.bits &= !(0x3 << 0x10);
            self.bits |= (val.to_bits() as u32 & 0x3) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Alpha mode"]
        pub const fn am(self) -> vals::FgpfccrAm {
            let val = ((self.bits >> 0x10) & 0x3) as _;
            unsafe { vals::FgpfccrAm::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Alpha value"]
        pub const fn set_alpha(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x18);
            self.bits |= (val as u32 & 0xff) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "Alpha value"]
        pub const fn alpha(self) -> u8 {
            ((self.bits >> 0x18) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "interrupt flag clear register"]
    pub struct Ifcr {
        bits: u32,
    }
    impl Default for Ifcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ifcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Clear Transfer error interrupt flag"]
        pub const fn set_cteif(mut self, val: vals::Cteif) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Clear Transfer error interrupt flag"]
        pub const fn cteif(self) -> vals::Cteif {
            let val = ((self.bits >> 0x0) & 0x1) as _;
            unsafe { vals::Cteif::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clear transfer complete interrupt flag"]
        pub const fn set_ctcif(mut self, val: vals::Ctcif) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "Clear transfer complete interrupt flag"]
        pub const fn ctcif(self) -> vals::Ctcif {
            let val = ((self.bits >> 0x1) & 0x1) as _;
            unsafe { vals::Ctcif::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clear transfer watermark interrupt flag"]
        pub const fn set_ctwif(mut self, val: vals::Ctwif) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Clear transfer watermark interrupt flag"]
        pub const fn ctwif(self) -> vals::Ctwif {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Ctwif::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clear CLUT access error interrupt flag"]
        pub const fn set_caecif(mut self, val: vals::Caecif) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Clear CLUT access error interrupt flag"]
        pub const fn caecif(self) -> vals::Caecif {
            let val = ((self.bits >> 0x3) & 0x1) as _;
            unsafe { vals::Caecif::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clear CLUT transfer complete interrupt flag"]
        pub const fn set_cctcif(mut self, val: vals::Cctcif) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Clear CLUT transfer complete interrupt flag"]
        pub const fn cctcif(self) -> vals::Cctcif {
            let val = ((self.bits >> 0x4) & 0x1) as _;
            unsafe { vals::Cctcif::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clear configuration error interrupt flag"]
        pub const fn set_cceif(mut self, val: vals::Cceif) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x5;
            self
        }
        #[inline(always)]
        #[doc = "Clear configuration error interrupt flag"]
        pub const fn cceif(self) -> vals::Cceif {
            let val = ((self.bits >> 0x5) & 0x1) as _;
            unsafe { vals::Cceif::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Interrupt Status Register"]
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
        #[doc = "Transfer error interrupt flag"]
        pub const fn set_teif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transfer error interrupt flag"]
        pub const fn teif(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transfer complete interrupt flag"]
        pub const fn set_tcif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transfer complete interrupt flag"]
        pub const fn tcif(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transfer watermark interrupt flag"]
        pub const fn set_twif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transfer watermark interrupt flag"]
        pub const fn twif(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CLUT access error interrupt flag"]
        pub const fn set_caeif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CLUT access error interrupt flag"]
        pub const fn caeif(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "CLUT transfer complete interrupt flag"]
        pub const fn set_ctcif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "CLUT transfer complete interrupt flag"]
        pub const fn ctcif(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Configuration error interrupt flag"]
        pub const fn set_ceif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Configuration error interrupt flag"]
        pub const fn ceif(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "line watermark register"]
    pub struct Lwr {
        bits: u32,
    }
    impl Default for Lwr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Lwr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Line watermark"]
        pub const fn set_lw(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Line watermark"]
        pub const fn lw(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "number of line register"]
    pub struct Nlr {
        bits: u32,
    }
    impl Default for Nlr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Nlr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Number of lines"]
        pub const fn set_nl(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Number of lines"]
        pub const fn nl(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
        #[inline(always)]
        #[doc = "Pixel per lines"]
        pub const fn set_pl(mut self, val: u16) -> Self {
            self.bits &= !(0x3fff << 0x10);
            self.bits |= (val as u32 & 0x3fff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Pixel per lines"]
        pub const fn pl(self) -> u16 {
            ((self.bits >> 0x10) & 0x3fff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "output color register"]
    pub struct Ocolr {
        bits: u32,
    }
    impl Default for Ocolr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ocolr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Blue Value"]
        pub const fn set_blue(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Blue Value"]
        pub const fn blue(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Green Value"]
        pub const fn set_green(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Green Value"]
        pub const fn green(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Red Value"]
        pub const fn set_red(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x10);
            self.bits |= (val as u32 & 0xff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Red Value"]
        pub const fn red(self) -> u8 {
            ((self.bits >> 0x10) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Alpha Channel Value"]
        pub const fn set_aplha(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x18);
            self.bits |= (val as u32 & 0xff) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "Alpha Channel Value"]
        pub const fn aplha(self) -> u8 {
            ((self.bits >> 0x18) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "output memory address register"]
    pub struct Omar {
        bits: u32,
    }
    impl Default for Omar {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Omar {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Memory Address"]
        pub const fn set_ma(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Memory Address"]
        pub const fn ma(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "output offset register"]
    pub struct Oor {
        bits: u32,
    }
    impl Default for Oor {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Oor {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Line Offset"]
        pub const fn set_lo(mut self, val: u16) -> Self {
            self.bits &= !(0x3fff << 0x0);
            self.bits |= (val as u32 & 0x3fff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Line Offset"]
        pub const fn lo(self) -> u16 {
            ((self.bits >> 0x0) & 0x3fff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "output PFC control register"]
    pub struct Opfccr {
        bits: u32,
    }
    impl Default for Opfccr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Opfccr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Color mode"]
        pub const fn set_cm(mut self, val: vals::OpfccrCm) -> Self {
            self.bits &= !(0x7 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Color mode"]
        pub const fn cm(self) -> vals::OpfccrCm {
            let val = ((self.bits >> 0x0) & 0x7) as _;
            unsafe { vals::OpfccrCm::from_bits_unchecked(val) }
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum Abort {
        #[doc = "Transfer abort requested"]
        AbortRequest = 0x1,
    }
    impl Abort {
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
    pub enum BgpfccrAm {
        #[doc = "No modification of alpha channel"]
        NoModify = 0x0,
        #[doc = "Replace with value in ALPHA[7:0]"]
        Replace = 0x1,
        #[doc = "Multiply with value in ALPHA[7:0]"]
        Multiply = 0x2,
    }
    impl BgpfccrAm {
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
    pub enum BgpfccrCcm {
        #[doc = "CLUT color format ARGB8888"]
        Argb8888 = 0x0,
        #[doc = "CLUT color format RGB888"]
        Rgb888 = 0x1,
    }
    impl BgpfccrCcm {
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
    pub enum BgpfccrCm {
        #[doc = "Color mode ARGB8888"]
        Argb8888 = 0x0,
        #[doc = "Color mode RGB888"]
        Rgb888 = 0x1,
        #[doc = "Color mode RGB565"]
        Rgb565 = 0x2,
        #[doc = "Color mode ARGB1555"]
        Argb1555 = 0x3,
        #[doc = "Color mode ARGB4444"]
        Argb4444 = 0x4,
        #[doc = "Color mode L8"]
        L8 = 0x5,
        #[doc = "Color mode AL44"]
        Al44 = 0x6,
        #[doc = "Color mode AL88"]
        Al88 = 0x7,
        #[doc = "Color mode L4"]
        L4 = 0x8,
        #[doc = "Color mode A8"]
        A8 = 0x9,
        #[doc = "Color mode A4"]
        A4 = 0xa,
    }
    impl BgpfccrCm {
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
    pub enum BgpfccrStart {
        #[doc = "Start the automatic loading of the CLUT"]
        Start = 0x1,
    }
    impl BgpfccrStart {
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
    pub enum Caecif {
        #[doc = "Clear the CAEIF flag in the ISR register"]
        Clear = 0x1,
    }
    impl Caecif {
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
    pub enum Cceif {
        #[doc = "Clear the CEIF flag in the ISR register"]
        Clear = 0x1,
    }
    impl Cceif {
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
    pub enum Cctcif {
        #[doc = "Clear the CTCIF flag in the ISR register"]
        Clear = 0x1,
    }
    impl Cctcif {
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
    pub enum CrStart {
        #[doc = "Launch the DMA2D"]
        Start = 0x1,
    }
    impl CrStart {
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
    pub enum Ctcif {
        #[doc = "Clear the TCIF flag in the ISR register"]
        Clear = 0x1,
    }
    impl Ctcif {
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
    pub enum Cteif {
        #[doc = "Clear the TEIF flag in the ISR register"]
        Clear = 0x1,
    }
    impl Cteif {
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
    pub enum Ctwif {
        #[doc = "Clear the TWIF flag in the ISR register"]
        Clear = 0x1,
    }
    impl Ctwif {
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
    pub enum FgpfccrAm {
        #[doc = "No modification of alpha channel"]
        NoModify = 0x0,
        #[doc = "Replace with value in ALPHA[7:0]"]
        Replace = 0x1,
        #[doc = "Multiply with value in ALPHA[7:0]"]
        Multiply = 0x2,
    }
    impl FgpfccrAm {
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
    pub enum FgpfccrCcm {
        #[doc = "CLUT color format ARGB8888"]
        Argb8888 = 0x0,
        #[doc = "CLUT color format RGB888"]
        Rgb888 = 0x1,
    }
    impl FgpfccrCcm {
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
    pub enum FgpfccrCm {
        #[doc = "Color mode ARGB8888"]
        Argb8888 = 0x0,
        #[doc = "Color mode RGB888"]
        Rgb888 = 0x1,
        #[doc = "Color mode RGB565"]
        Rgb565 = 0x2,
        #[doc = "Color mode ARGB1555"]
        Argb1555 = 0x3,
        #[doc = "Color mode ARGB4444"]
        Argb4444 = 0x4,
        #[doc = "Color mode L8"]
        L8 = 0x5,
        #[doc = "Color mode AL44"]
        Al44 = 0x6,
        #[doc = "Color mode AL88"]
        Al88 = 0x7,
        #[doc = "Color mode L4"]
        L4 = 0x8,
        #[doc = "Color mode A8"]
        A8 = 0x9,
        #[doc = "Color mode A4"]
        A4 = 0xa,
    }
    impl FgpfccrCm {
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
    pub enum FgpfccrStart {
        #[doc = "Start the automatic loading of the CLUT"]
        Start = 0x1,
    }
    impl FgpfccrStart {
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
    pub enum Mode {
        #[doc = "Memory-to-memory (FG fetch only)"]
        MemoryToMemory = 0x0,
        #[doc = "Memory-to-memory with PFC (FG fetch only with FG PFC active)"]
        MemoryToMemoryPfc = 0x1,
        #[doc = "Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
        MemoryToMemoryPfcBlending = 0x2,
        #[doc = "Register-to-memory"]
        RegisterToMemory = 0x3,
    }
    impl Mode {
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
    pub enum OpfccrCm {
        #[doc = "ARGB8888"]
        Argb8888 = 0x0,
        #[doc = "RGB888"]
        Rgb888 = 0x1,
        #[doc = "RGB565"]
        Rgb565 = 0x2,
        #[doc = "ARGB1555"]
        Argb1555 = 0x3,
        #[doc = "ARGB4444"]
        Argb4444 = 0x4,
    }
    impl OpfccrCm {
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
