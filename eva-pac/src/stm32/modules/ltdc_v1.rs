
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
pub struct Layer {
    ptr: *mut u8,
}
impl Layer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "Layerx Control Register"]
    pub const fn cr(&self) -> utils::Reg<fields::Cr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::Cr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Window Horizontal Position Configuration Register"]
    pub const fn whpcr(&self) -> utils::Reg<fields::Whpcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::Whpcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Window Vertical Position Configuration Register"]
    pub const fn wvpcr(&self) -> utils::Reg<fields::Wvpcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Wvpcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Color Keying Configuration Register"]
    pub const fn ckcr(&self) -> utils::Reg<fields::Ckcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Ckcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Pixel Format Configuration Register"]
    pub const fn pfcr(&self) -> utils::Reg<fields::Pfcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Pfcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Constant Alpha Configuration Register"]
    pub const fn cacr(&self) -> utils::Reg<fields::Cacr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Cacr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Default Color Configuration Register"]
    pub const fn dccr(&self) -> utils::Reg<fields::Dccr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Dccr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Blending Factors Configuration Register"]
    pub const fn bfcr(&self) -> utils::Reg<fields::Bfcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<fields::Bfcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Color Frame Buffer Address Register"]
    pub const fn cfbar(&self) -> utils::Reg<fields::Cfbar, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28);
            <utils::Reg<fields::Cfbar, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx Color Frame Buffer Length Register"]
    pub const fn cfblr(&self) -> utils::Reg<fields::Cfblr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::Cfblr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx ColorFrame Buffer Line Number Register"]
    pub const fn cfblnr(&self) -> utils::Reg<fields::Cfblnr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<fields::Cfblnr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Layerx CLUT Write Register"]
    pub const fn clutwr(&self) -> utils::Reg<fields::Clutwr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<fields::Clutwr, utils::WO>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "LCD-TFT Controller"]
pub struct Ltdc {
    ptr: *mut u8,
}
impl Ltdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "Synchronization Size Configuration Register"]
    pub const fn sscr(&self) -> utils::Reg<fields::Sscr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::Sscr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Back Porch Configuration Register"]
    pub const fn bpcr(&self) -> utils::Reg<fields::Bpcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::Bpcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Active Width Configuration Register"]
    pub const fn awcr(&self) -> utils::Reg<fields::Awcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::Awcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Total Width Configuration Register"]
    pub const fn twcr(&self) -> utils::Reg<fields::Twcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::Twcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Global Control Register"]
    pub const fn gcr(&self) -> utils::Reg<fields::Gcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::Gcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Shadow Reload Configuration Register"]
    pub const fn srcr(&self) -> utils::Reg<fields::Srcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24);
            <utils::Reg<fields::Srcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Background Color Configuration Register"]
    pub const fn bccr(&self) -> utils::Reg<fields::Bccr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c);
            <utils::Reg<fields::Bccr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt Enable Register"]
    pub const fn ier(&self) -> utils::Reg<fields::Ier, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<fields::Ier, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt Status Register"]
    pub const fn isr(&self) -> utils::Reg<fields::Isr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<fields::Isr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Interrupt Clear Register"]
    pub const fn icr(&self) -> utils::Reg<fields::Icr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x3c);
            <utils::Reg<fields::Icr, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Line Interrupt Position Configuration Register"]
    pub const fn lipcr(&self) -> utils::Reg<fields::Lipcr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<fields::Lipcr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Current Position Status Register"]
    pub const fn cpsr(&self) -> utils::Reg<fields::Cpsr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<fields::Cpsr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Current Display Status Register"]
    pub const fn cdsr(&self) -> utils::Reg<fields::Cdsr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x48);
            <utils::Reg<fields::Cdsr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    pub const fn layer(&self, idx: usize) -> Layer {
        assert!(idx < 2);
        unsafe {
            let ptr = self.ptr.add(0x84 + idx * 0x80);
            <Layer>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Active Width Configuration Register"]
    pub struct Awcr {
        bits: u32,
    }
    impl Default for Awcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Awcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Accumulated Active Height (in units of horizontal scan line)"]
        pub const fn set_aah(mut self, val: u16) -> Self {
            self.bits &= !(0x7ff << 0x0);
            self.bits |= (val as u32 & 0x7ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Accumulated Active Height (in units of horizontal scan line)"]
        pub const fn aah(self) -> u16 {
            ((self.bits >> 0x0) & 0x7ff) as _
        }
        #[inline(always)]
        #[doc = "Accumulated Active Width (in units of pixel clock period)"]
        pub const fn set_aaw(mut self, val: u16) -> Self {
            self.bits &= !(0xfff << 0x10);
            self.bits |= (val as u32 & 0xfff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Accumulated Active Width (in units of pixel clock period)"]
        pub const fn aaw(self) -> u16 {
            ((self.bits >> 0x10) & 0xfff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Background Color Configuration Register"]
    pub struct Bccr {
        bits: u32,
    }
    impl Default for Bccr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bccr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Background color blue value"]
        pub const fn set_bcblue(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Background color blue value"]
        pub const fn bcblue(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Background color green value"]
        pub const fn set_bcgreen(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Background color green value"]
        pub const fn bcgreen(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Background color red value"]
        pub const fn set_bcred(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x10);
            self.bits |= (val as u32 & 0xff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Background color red value"]
        pub const fn bcred(self) -> u8 {
            ((self.bits >> 0x10) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx Blending Factors Configuration Register"]
    pub struct Bfcr {
        bits: u32,
    }
    impl Default for Bfcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bfcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Blending Factor 2"]
        pub const fn set_bf2(mut self, val: vals::Bf2) -> Self {
            self.bits &= !(0x7 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Blending Factor 2"]
        pub const fn bf2(self) -> vals::Bf2 {
            let val = ((self.bits >> 0x0) & 0x7) as _;
            unsafe { vals::Bf2::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Blending Factor 1"]
        pub const fn set_bf1(mut self, val: vals::Bf1) -> Self {
            self.bits &= !(0x7 << 0x8);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Blending Factor 1"]
        pub const fn bf1(self) -> vals::Bf1 {
            let val = ((self.bits >> 0x8) & 0x7) as _;
            unsafe { vals::Bf1::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Back Porch Configuration Register"]
    pub struct Bpcr {
        bits: u32,
    }
    impl Default for Bpcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Bpcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Accumulated Vertical back porch (in units of horizontal scan line)"]
        pub const fn set_avbp(mut self, val: u16) -> Self {
            self.bits &= !(0x7ff << 0x0);
            self.bits |= (val as u32 & 0x7ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Accumulated Vertical back porch (in units of horizontal scan line)"]
        pub const fn avbp(self) -> u16 {
            ((self.bits >> 0x0) & 0x7ff) as _
        }
        #[inline(always)]
        #[doc = "Accumulated Horizontal back porch (in units of pixel clock period)"]
        pub const fn set_ahbp(mut self, val: u16) -> Self {
            self.bits &= !(0xfff << 0x10);
            self.bits |= (val as u32 & 0xfff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Accumulated Horizontal back porch (in units of pixel clock period)"]
        pub const fn ahbp(self) -> u16 {
            ((self.bits >> 0x10) & 0xfff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx Constant Alpha Configuration Register"]
    pub struct Cacr {
        bits: u32,
    }
    impl Default for Cacr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cacr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Constant Alpha"]
        pub const fn set_consta(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Constant Alpha"]
        pub const fn consta(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Current Display Status Register"]
    pub struct Cdsr {
        bits: u32,
    }
    impl Default for Cdsr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cdsr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Vertical Data Enable display Status"]
        pub const fn set_vdes(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Vertical Data Enable display Status"]
        pub const fn vdes(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Horizontal Data Enable display Status"]
        pub const fn set_hdes(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Horizontal Data Enable display Status"]
        pub const fn hdes(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Vertical Synchronization display Status"]
        pub const fn set_vsyncs(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Vertical Synchronization display Status"]
        pub const fn vsyncs(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Horizontal Synchronization display Status"]
        pub const fn set_hsyncs(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Horizontal Synchronization display Status"]
        pub const fn hsyncs(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx Color Frame Buffer Address Register"]
    pub struct Cfbar {
        bits: u32,
    }
    impl Default for Cfbar {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cfbar {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Color Frame Buffer Start Address"]
        pub const fn set_cfbadd(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Color Frame Buffer Start Address"]
        pub const fn cfbadd(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx ColorFrame Buffer Line Number Register"]
    pub struct Cfblnr {
        bits: u32,
    }
    impl Default for Cfblnr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cfblnr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Frame Buffer Line Number"]
        pub const fn set_cfblnbr(mut self, val: u16) -> Self {
            self.bits &= !(0x7ff << 0x0);
            self.bits |= (val as u32 & 0x7ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Frame Buffer Line Number"]
        pub const fn cfblnbr(self) -> u16 {
            ((self.bits >> 0x0) & 0x7ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx Color Frame Buffer Length Register"]
    pub struct Cfblr {
        bits: u32,
    }
    impl Default for Cfblr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cfblr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Color Frame Buffer Line Length"]
        pub const fn set_cfbll(mut self, val: u16) -> Self {
            self.bits &= !(0x1fff << 0x0);
            self.bits |= (val as u32 & 0x1fff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Color Frame Buffer Line Length"]
        pub const fn cfbll(self) -> u16 {
            ((self.bits >> 0x0) & 0x1fff) as _
        }
        #[inline(always)]
        #[doc = "Color Frame Buffer Pitch in bytes"]
        pub const fn set_cfbp(mut self, val: u16) -> Self {
            self.bits &= !(0x1fff << 0x10);
            self.bits |= (val as u32 & 0x1fff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Color Frame Buffer Pitch in bytes"]
        pub const fn cfbp(self) -> u16 {
            ((self.bits >> 0x10) & 0x1fff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx Color Keying Configuration Register"]
    pub struct Ckcr {
        bits: u32,
    }
    impl Default for Ckcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ckcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Color Key Blue value"]
        pub const fn set_ckblue(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Color Key Blue value"]
        pub const fn ckblue(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Color Key Green value"]
        pub const fn set_ckgreen(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Color Key Green value"]
        pub const fn ckgreen(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Color Key Red value"]
        pub const fn set_ckred(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x10);
            self.bits |= (val as u32 & 0xff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Color Key Red value"]
        pub const fn ckred(self) -> u8 {
            ((self.bits >> 0x10) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx CLUT Write Register"]
    pub struct Clutwr {
        bits: u32,
    }
    impl Default for Clutwr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Clutwr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Blue value"]
        pub const fn set_blue(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Blue value"]
        pub const fn blue(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Green value"]
        pub const fn set_green(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Green value"]
        pub const fn green(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Red value"]
        pub const fn set_red(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x10);
            self.bits |= (val as u32 & 0xff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Red value"]
        pub const fn red(self) -> u8 {
            ((self.bits >> 0x10) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "CLUT Address"]
        pub const fn set_clutadd(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x18);
            self.bits |= (val as u32 & 0xff) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "CLUT Address"]
        pub const fn clutadd(self) -> u8 {
            ((self.bits >> 0x18) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Current Position Status Register"]
    pub struct Cpsr {
        bits: u32,
    }
    impl Default for Cpsr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Cpsr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Current Y Position"]
        pub const fn set_cypos(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x0);
            self.bits |= (val as u32 & 0xffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Current Y Position"]
        pub const fn cypos(self) -> u16 {
            ((self.bits >> 0x0) & 0xffff) as _
        }
        #[inline(always)]
        #[doc = "Current X Position"]
        pub const fn set_cxpos(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x10);
            self.bits |= (val as u32 & 0xffff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Current X Position"]
        pub const fn cxpos(self) -> u16 {
            ((self.bits >> 0x10) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx Control Register"]
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
        #[doc = "Layer Enable"]
        pub const fn set_len(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Layer Enable"]
        pub const fn len(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Color Keying Enable"]
        pub const fn set_colken(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Color Keying Enable"]
        pub const fn colken(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Color Look-Up Table Enable"]
        pub const fn set_cluten(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Color Look-Up Table Enable"]
        pub const fn cluten(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx Default Color Configuration Register"]
    pub struct Dccr {
        bits: u32,
    }
    impl Default for Dccr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dccr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Default Color Blue"]
        pub const fn set_dcblue(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x0);
            self.bits |= (val as u32 & 0xff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Default Color Blue"]
        pub const fn dcblue(self) -> u8 {
            ((self.bits >> 0x0) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Default Color Green"]
        pub const fn set_dcgreen(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x8);
            self.bits |= (val as u32 & 0xff) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Default Color Green"]
        pub const fn dcgreen(self) -> u8 {
            ((self.bits >> 0x8) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Default Color Red"]
        pub const fn set_dcred(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x10);
            self.bits |= (val as u32 & 0xff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Default Color Red"]
        pub const fn dcred(self) -> u8 {
            ((self.bits >> 0x10) & 0xff) as _
        }
        #[inline(always)]
        #[doc = "Default Color Alpha"]
        pub const fn set_dcalpha(mut self, val: u8) -> Self {
            self.bits &= !(0xff << 0x18);
            self.bits |= (val as u32 & 0xff) << 0x18;
            self
        }
        #[inline(always)]
        #[doc = "Default Color Alpha"]
        pub const fn dcalpha(self) -> u8 {
            ((self.bits >> 0x18) & 0xff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Global Control Register"]
    pub struct Gcr {
        bits: u32,
    }
    impl Default for Gcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Gcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "LCD-TFT controller enable bit"]
        pub const fn set_ltdcen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "LCD-TFT controller enable bit"]
        pub const fn ltdcen(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Dither Blue Width"]
        pub const fn set_dbw(mut self, val: u8) -> Self {
            self.bits &= !(0x7 << 0x4);
            self.bits |= (val as u32 & 0x7) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Dither Blue Width"]
        pub const fn dbw(self) -> u8 {
            ((self.bits >> 0x4) & 0x7) as _
        }
        #[inline(always)]
        #[doc = "Dither Green Width"]
        pub const fn set_dgw(mut self, val: u8) -> Self {
            self.bits &= !(0x7 << 0x8);
            self.bits |= (val as u32 & 0x7) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Dither Green Width"]
        pub const fn dgw(self) -> u8 {
            ((self.bits >> 0x8) & 0x7) as _
        }
        #[inline(always)]
        #[doc = "Dither Red Width"]
        pub const fn set_drw(mut self, val: u8) -> Self {
            self.bits &= !(0x7 << 0xc);
            self.bits |= (val as u32 & 0x7) << 0xc;
            self
        }
        #[inline(always)]
        #[doc = "Dither Red Width"]
        pub const fn drw(self) -> u8 {
            ((self.bits >> 0xc) & 0x7) as _
        }
        #[inline(always)]
        #[doc = "Dither Enable"]
        pub const fn set_den(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x10);
            self.bits |= if val { 1 << 0x10 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Dither Enable"]
        pub const fn den(self) -> bool {
            ((self.bits >> 0x10) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Pixel Clock Polarity"]
        pub const fn set_pcpol(mut self, val: vals::Pcpol) -> Self {
            self.bits &= !(0x1 << 0x1c);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1c;
            self
        }
        #[inline(always)]
        #[doc = "Pixel Clock Polarity"]
        pub const fn pcpol(self) -> vals::Pcpol {
            let val = ((self.bits >> 0x1c) & 0x1) as _;
            unsafe { vals::Pcpol::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Data Enable Polarity"]
        pub const fn set_depol(mut self, val: vals::Depol) -> Self {
            self.bits &= !(0x1 << 0x1d);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1d;
            self
        }
        #[inline(always)]
        #[doc = "Data Enable Polarity"]
        pub const fn depol(self) -> vals::Depol {
            let val = ((self.bits >> 0x1d) & 0x1) as _;
            unsafe { vals::Depol::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Vertical Synchronization Polarity"]
        pub const fn set_vspol(mut self, val: vals::Vspol) -> Self {
            self.bits &= !(0x1 << 0x1e);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1e;
            self
        }
        #[inline(always)]
        #[doc = "Vertical Synchronization Polarity"]
        pub const fn vspol(self) -> vals::Vspol {
            let val = ((self.bits >> 0x1e) & 0x1) as _;
            unsafe { vals::Vspol::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Horizontal Synchronization Polarity"]
        pub const fn set_hspol(mut self, val: vals::Hspol) -> Self {
            self.bits &= !(0x1 << 0x1f);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1f;
            self
        }
        #[inline(always)]
        #[doc = "Horizontal Synchronization Polarity"]
        pub const fn hspol(self) -> vals::Hspol {
            let val = ((self.bits >> 0x1f) & 0x1) as _;
            unsafe { vals::Hspol::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Interrupt Clear Register"]
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
        #[doc = "Clears the Line Interrupt Flag"]
        pub const fn set_clif(mut self, val: vals::Clif) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Clears the Line Interrupt Flag"]
        pub const fn clif(self) -> vals::Clif {
            let val = ((self.bits >> 0x0) & 0x1) as _;
            unsafe { vals::Clif::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clears the FIFO Underrun Interrupt flag"]
        pub const fn set_cfuif(mut self, val: vals::Cfuif) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "Clears the FIFO Underrun Interrupt flag"]
        pub const fn cfuif(self) -> vals::Cfuif {
            let val = ((self.bits >> 0x1) & 0x1) as _;
            unsafe { vals::Cfuif::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clears the Transfer Error Interrupt Flag"]
        pub const fn set_cterrif(mut self, val: vals::Cterrif) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Clears the Transfer Error Interrupt Flag"]
        pub const fn cterrif(self) -> vals::Cterrif {
            let val = ((self.bits >> 0x2) & 0x1) as _;
            unsafe { vals::Cterrif::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Clears Register Reload Interrupt Flag"]
        pub const fn set_crrif(mut self, val: vals::Crrif) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x3;
            self
        }
        #[inline(always)]
        #[doc = "Clears Register Reload Interrupt Flag"]
        pub const fn crrif(self) -> vals::Crrif {
            let val = ((self.bits >> 0x3) & 0x1) as _;
            unsafe { vals::Crrif::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Interrupt Enable Register"]
    pub struct Ier {
        bits: u32,
    }
    impl Default for Ier {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Ier {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Line Interrupt Enable"]
        pub const fn set_lie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Line Interrupt Enable"]
        pub const fn lie(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "FIFO Underrun Interrupt Enable"]
        pub const fn set_fuie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "FIFO Underrun Interrupt Enable"]
        pub const fn fuie(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transfer Error Interrupt Enable"]
        pub const fn set_terrie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transfer Error Interrupt Enable"]
        pub const fn terrie(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Register Reload interrupt enable"]
        pub const fn set_rrie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Register Reload interrupt enable"]
        pub const fn rrie(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
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
        #[doc = "Line Interrupt flag"]
        pub const fn set_lif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Line Interrupt flag"]
        pub const fn lif(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "FIFO Underrun Interrupt flag"]
        pub const fn set_fuif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "FIFO Underrun Interrupt flag"]
        pub const fn fuif(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Transfer Error interrupt flag"]
        pub const fn set_terrif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Transfer Error interrupt flag"]
        pub const fn terrif(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Register Reload Interrupt Flag"]
        pub const fn set_rrif(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Register Reload Interrupt Flag"]
        pub const fn rrif(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Line Interrupt Position Configuration Register"]
    pub struct Lipcr {
        bits: u32,
    }
    impl Default for Lipcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Lipcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Line Interrupt Position"]
        pub const fn set_lipos(mut self, val: u16) -> Self {
            self.bits &= !(0x7ff << 0x0);
            self.bits |= (val as u32 & 0x7ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Line Interrupt Position"]
        pub const fn lipos(self) -> u16 {
            ((self.bits >> 0x0) & 0x7ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx Pixel Format Configuration Register"]
    pub struct Pfcr {
        bits: u32,
    }
    impl Default for Pfcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Pfcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Pixel Format"]
        pub const fn set_pf(mut self, val: vals::Pf) -> Self {
            self.bits &= !(0x7 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x7) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Pixel Format"]
        pub const fn pf(self) -> vals::Pf {
            let val = ((self.bits >> 0x0) & 0x7) as _;
            unsafe { vals::Pf::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Shadow Reload Configuration Register"]
    pub struct Srcr {
        bits: u32,
    }
    impl Default for Srcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Srcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Immediate Reload"]
        pub const fn set_imr(mut self, val: vals::Imr) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Immediate Reload"]
        pub const fn imr(self) -> vals::Imr {
            let val = ((self.bits >> 0x0) & 0x1) as _;
            unsafe { vals::Imr::from_bits_unchecked(val) }
        }
        #[inline(always)]
        #[doc = "Vertical Blanking Reload"]
        pub const fn set_vbr(mut self, val: vals::Vbr) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= (val.to_bits() as u32 & 0x1) << 0x1;
            self
        }
        #[inline(always)]
        #[doc = "Vertical Blanking Reload"]
        pub const fn vbr(self) -> vals::Vbr {
            let val = ((self.bits >> 0x1) & 0x1) as _;
            unsafe { vals::Vbr::from_bits_unchecked(val) }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Synchronization Size Configuration Register"]
    pub struct Sscr {
        bits: u32,
    }
    impl Default for Sscr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Sscr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Vertical Synchronization Height (in units of horizontal scan line)"]
        pub const fn set_vsh(mut self, val: u16) -> Self {
            self.bits &= !(0x7ff << 0x0);
            self.bits |= (val as u32 & 0x7ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Vertical Synchronization Height (in units of horizontal scan line)"]
        pub const fn vsh(self) -> u16 {
            ((self.bits >> 0x0) & 0x7ff) as _
        }
        #[inline(always)]
        #[doc = "Horizontal Synchronization Width (in units of pixel clock period)"]
        pub const fn set_hsw(mut self, val: u16) -> Self {
            self.bits &= !(0xfff << 0x10);
            self.bits |= (val as u32 & 0xfff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Horizontal Synchronization Width (in units of pixel clock period)"]
        pub const fn hsw(self) -> u16 {
            ((self.bits >> 0x10) & 0xfff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Total Width Configuration Register"]
    pub struct Twcr {
        bits: u32,
    }
    impl Default for Twcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Twcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Total Height (in units of horizontal scan line)"]
        pub const fn set_totalh(mut self, val: u16) -> Self {
            self.bits &= !(0x7ff << 0x0);
            self.bits |= (val as u32 & 0x7ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Total Height (in units of horizontal scan line)"]
        pub const fn totalh(self) -> u16 {
            ((self.bits >> 0x0) & 0x7ff) as _
        }
        #[inline(always)]
        #[doc = "Total Width (in units of pixel clock period)"]
        pub const fn set_totalw(mut self, val: u16) -> Self {
            self.bits &= !(0xfff << 0x10);
            self.bits |= (val as u32 & 0xfff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Total Width (in units of pixel clock period)"]
        pub const fn totalw(self) -> u16 {
            ((self.bits >> 0x10) & 0xfff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx Window Horizontal Position Configuration Register"]
    pub struct Whpcr {
        bits: u32,
    }
    impl Default for Whpcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Whpcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Window Horizontal Start Position"]
        pub const fn set_whstpos(mut self, val: u16) -> Self {
            self.bits &= !(0xfff << 0x0);
            self.bits |= (val as u32 & 0xfff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Window Horizontal Start Position"]
        pub const fn whstpos(self) -> u16 {
            ((self.bits >> 0x0) & 0xfff) as _
        }
        #[inline(always)]
        #[doc = "Window Horizontal Stop Position"]
        pub const fn set_whsppos(mut self, val: u16) -> Self {
            self.bits &= !(0xfff << 0x10);
            self.bits |= (val as u32 & 0xfff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Window Horizontal Stop Position"]
        pub const fn whsppos(self) -> u16 {
            ((self.bits >> 0x10) & 0xfff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "Layerx Window Vertical Position Configuration Register"]
    pub struct Wvpcr {
        bits: u32,
    }
    impl Default for Wvpcr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Wvpcr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Window Vertical Start Position"]
        pub const fn set_wvstpos(mut self, val: u16) -> Self {
            self.bits &= !(0x7ff << 0x0);
            self.bits |= (val as u32 & 0x7ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Window Vertical Start Position"]
        pub const fn wvstpos(self) -> u16 {
            ((self.bits >> 0x0) & 0x7ff) as _
        }
        #[inline(always)]
        #[doc = "Window Vertical Stop Position"]
        pub const fn set_wvsppos(mut self, val: u16) -> Self {
            self.bits &= !(0x7ff << 0x10);
            self.bits |= (val as u32 & 0x7ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Window Vertical Stop Position"]
        pub const fn wvsppos(self) -> u16 {
            ((self.bits >> 0x10) & 0x7ff) as _
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum Bf1 {
        #[doc = "BF1 = constant alpha"]
        Constant = 0x4,
        #[doc = "BF1 = pixel alpha * constant alpha"]
        Pixel = 0x6,
    }
    impl Bf1 {
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
    pub enum Bf2 {
        #[doc = "BF2 = 1 - constant alpha"]
        Constant = 0x5,
        #[doc = "BF2 = 1 - pixel alpha * constant alpha"]
        Pixel = 0x7,
    }
    impl Bf2 {
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
    pub enum Cfuif {
        #[doc = "Clears the FUIF flag in the ISR register"]
        Clear = 0x1,
    }
    impl Cfuif {
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
    pub enum Clif {
        #[doc = "Clears the LIF flag in the ISR register"]
        Clear = 0x1,
    }
    impl Clif {
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
    pub enum Crrif {
        #[doc = "Clears the RRIF flag in the ISR register"]
        Clear = 0x1,
    }
    impl Crrif {
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
    pub enum Cterrif {
        #[doc = "Clears the TERRIF flag in the ISR register"]
        Clear = 0x1,
    }
    impl Cterrif {
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
    pub enum Depol {
        #[doc = "Data enable polarity is active low"]
        ActiveLow = 0x0,
        #[doc = "Data enable polarity is active high"]
        ActiveHigh = 0x1,
    }
    impl Depol {
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
    pub enum Hspol {
        #[doc = "Horizontal synchronization polarity is active low"]
        ActiveLow = 0x0,
        #[doc = "Horizontal synchronization polarity is active high"]
        ActiveHigh = 0x1,
    }
    impl Hspol {
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
    pub enum Imr {
        #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
        NoEffect = 0x0,
        #[doc = "The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
        Reload = 0x1,
    }
    impl Imr {
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
    pub enum Pcpol {
        #[doc = "Pixel clock on rising edge"]
        RisingEdge = 0x0,
        #[doc = "Pixel clock on falling edge"]
        FallingEdge = 0x1,
    }
    impl Pcpol {
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
    pub enum Pf {
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
        #[doc = "L8 (8-bit luminance)"]
        L8 = 0x5,
        #[doc = "AL44 (4-bit alpha, 4-bit luminance)"]
        Al44 = 0x6,
        #[doc = "AL88 (8-bit alpha, 8-bit luminance)"]
        Al88 = 0x7,
    }
    impl Pf {
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
    pub enum Vbr {
        #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
        NoEffect = 0x0,
        #[doc = "The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
        Reload = 0x1,
    }
    impl Vbr {
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
    pub enum Vspol {
        #[doc = "Vertical synchronization polarity is active low"]
        ActiveLow = 0x0,
        #[doc = "Vertical synchronization polarity is active high"]
        ActiveHigh = 0x1,
    }
    impl Vspol {
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
