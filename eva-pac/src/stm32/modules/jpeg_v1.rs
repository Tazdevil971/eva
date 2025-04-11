
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "JPEG codec"]
pub struct Jpeg {
    ptr: *mut u8,
}
impl Jpeg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 0"]
    pub const fn jpeg_confr0(&self) -> utils::Reg<fields::JpegConfr0, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<fields::JpegConfr0, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 1"]
    pub const fn jpeg_confr1(&self) -> utils::Reg<fields::JpegConfr1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<fields::JpegConfr1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 2"]
    pub const fn jpeg_confr2(&self) -> utils::Reg<fields::JpegConfr2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<fields::JpegConfr2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 3"]
    pub const fn jpeg_confr3(&self) -> utils::Reg<fields::JpegConfr3, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<fields::JpegConfr3, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 4"]
    pub const fn jpeg_confr4(&self) -> utils::Reg<fields::JpegConfr4, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<fields::JpegConfr4, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 5"]
    pub const fn jpeg_confr5(&self) -> utils::Reg<fields::JpegConfr5, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<fields::JpegConfr5, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 6"]
    pub const fn jpeg_confr6(&self) -> utils::Reg<fields::JpegConfr6, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<fields::JpegConfr6, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 7"]
    pub const fn jpeg_confr7(&self) -> utils::Reg<fields::JpegConfr7, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<fields::JpegConfr7, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG control register"]
    pub const fn jpeg_cr(&self) -> utils::Reg<fields::JpegCr, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<fields::JpegCr, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG status register"]
    pub const fn jpeg_sr(&self) -> utils::Reg<fields::JpegSr, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<fields::JpegSr, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG clear flag register"]
    pub const fn jpeg_cfr(&self) -> utils::Reg<fields::JpegCfr, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<fields::JpegCfr, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG data input register"]
    pub const fn jpeg_dir(&self) -> utils::Reg<fields::JpegDir, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<fields::JpegDir, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG data output register"]
    pub const fn jpeg_dor(&self) -> utils::Reg<fields::JpegDor, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<fields::JpegDor, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_0(&self) -> utils::Reg<fields::Qmem00, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<fields::Qmem00, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_1(&self) -> utils::Reg<fields::Qmem01, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x54);
            <utils::Reg<fields::Qmem01, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_2(&self) -> utils::Reg<fields::Qmem02, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x58);
            <utils::Reg<fields::Qmem02, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_3(&self) -> utils::Reg<fields::Qmem03, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c);
            <utils::Reg<fields::Qmem03, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_4(&self) -> utils::Reg<fields::Qmem04, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<fields::Qmem04, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_5(&self) -> utils::Reg<fields::Qmem05, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x64);
            <utils::Reg<fields::Qmem05, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_6(&self) -> utils::Reg<fields::Qmem06, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<fields::Qmem06, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_7(&self) -> utils::Reg<fields::Qmem07, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6c);
            <utils::Reg<fields::Qmem07, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_8(&self) -> utils::Reg<fields::Qmem08, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x70);
            <utils::Reg<fields::Qmem08, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_9(&self) -> utils::Reg<fields::Qmem09, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x74);
            <utils::Reg<fields::Qmem09, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_10(&self) -> utils::Reg<fields::Qmem010, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x78);
            <utils::Reg<fields::Qmem010, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_11(&self) -> utils::Reg<fields::Qmem011, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7c);
            <utils::Reg<fields::Qmem011, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_12(&self) -> utils::Reg<fields::Qmem012, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x80);
            <utils::Reg<fields::Qmem012, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_13(&self) -> utils::Reg<fields::Qmem013, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x84);
            <utils::Reg<fields::Qmem013, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_14(&self) -> utils::Reg<fields::Qmem014, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x88);
            <utils::Reg<fields::Qmem014, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_15(&self) -> utils::Reg<fields::Qmem015, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8c);
            <utils::Reg<fields::Qmem015, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_0(&self) -> utils::Reg<fields::Qmem10, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x90);
            <utils::Reg<fields::Qmem10, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_1(&self) -> utils::Reg<fields::Qmem11, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x94);
            <utils::Reg<fields::Qmem11, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_2(&self) -> utils::Reg<fields::Qmem12, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x98);
            <utils::Reg<fields::Qmem12, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_3(&self) -> utils::Reg<fields::Qmem13, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x9c);
            <utils::Reg<fields::Qmem13, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_4(&self) -> utils::Reg<fields::Qmem14, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xa0);
            <utils::Reg<fields::Qmem14, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_5(&self) -> utils::Reg<fields::Qmem15, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xa4);
            <utils::Reg<fields::Qmem15, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_6(&self) -> utils::Reg<fields::Qmem16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xa8);
            <utils::Reg<fields::Qmem16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_7(&self) -> utils::Reg<fields::Qmem17, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xac);
            <utils::Reg<fields::Qmem17, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_8(&self) -> utils::Reg<fields::Qmem18, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xb0);
            <utils::Reg<fields::Qmem18, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_9(&self) -> utils::Reg<fields::Qmem19, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xb4);
            <utils::Reg<fields::Qmem19, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_10(&self) -> utils::Reg<fields::Qmem110, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xb8);
            <utils::Reg<fields::Qmem110, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_11(&self) -> utils::Reg<fields::Qmem111, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xbc);
            <utils::Reg<fields::Qmem111, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_12(&self) -> utils::Reg<fields::Qmem112, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc0);
            <utils::Reg<fields::Qmem112, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_13(&self) -> utils::Reg<fields::Qmem113, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc4);
            <utils::Reg<fields::Qmem113, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_14(&self) -> utils::Reg<fields::Qmem114, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc8);
            <utils::Reg<fields::Qmem114, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_15(&self) -> utils::Reg<fields::Qmem115, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xcc);
            <utils::Reg<fields::Qmem115, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_0(&self) -> utils::Reg<fields::Qmem20, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xd0);
            <utils::Reg<fields::Qmem20, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_1(&self) -> utils::Reg<fields::Qmem21, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xd4);
            <utils::Reg<fields::Qmem21, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_2(&self) -> utils::Reg<fields::Qmem22, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xd8);
            <utils::Reg<fields::Qmem22, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_3(&self) -> utils::Reg<fields::Qmem23, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xdc);
            <utils::Reg<fields::Qmem23, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_4(&self) -> utils::Reg<fields::Qmem24, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xe0);
            <utils::Reg<fields::Qmem24, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_5(&self) -> utils::Reg<fields::Qmem25, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xe4);
            <utils::Reg<fields::Qmem25, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_6(&self) -> utils::Reg<fields::Qmem26, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xe8);
            <utils::Reg<fields::Qmem26, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_7(&self) -> utils::Reg<fields::Qmem27, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xec);
            <utils::Reg<fields::Qmem27, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_8(&self) -> utils::Reg<fields::Qmem28, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xf0);
            <utils::Reg<fields::Qmem28, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_9(&self) -> utils::Reg<fields::Qmem29, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xf4);
            <utils::Reg<fields::Qmem29, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_10(&self) -> utils::Reg<fields::Qmem210, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xf8);
            <utils::Reg<fields::Qmem210, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_11(&self) -> utils::Reg<fields::Qmem211, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xfc);
            <utils::Reg<fields::Qmem211, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_12(&self) -> utils::Reg<fields::Qmem212, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x100);
            <utils::Reg<fields::Qmem212, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_13(&self) -> utils::Reg<fields::Qmem213, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x104);
            <utils::Reg<fields::Qmem213, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_14(&self) -> utils::Reg<fields::Qmem214, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x108);
            <utils::Reg<fields::Qmem214, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_15(&self) -> utils::Reg<fields::Qmem215, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10c);
            <utils::Reg<fields::Qmem215, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_0(&self) -> utils::Reg<fields::Qmem30, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x110);
            <utils::Reg<fields::Qmem30, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_1(&self) -> utils::Reg<fields::Qmem31, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x114);
            <utils::Reg<fields::Qmem31, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_2(&self) -> utils::Reg<fields::Qmem32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x118);
            <utils::Reg<fields::Qmem32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_3(&self) -> utils::Reg<fields::Qmem33, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x11c);
            <utils::Reg<fields::Qmem33, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_4(&self) -> utils::Reg<fields::Qmem34, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x120);
            <utils::Reg<fields::Qmem34, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_5(&self) -> utils::Reg<fields::Qmem35, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x124);
            <utils::Reg<fields::Qmem35, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_6(&self) -> utils::Reg<fields::Qmem36, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x128);
            <utils::Reg<fields::Qmem36, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_7(&self) -> utils::Reg<fields::Qmem37, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x12c);
            <utils::Reg<fields::Qmem37, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_8(&self) -> utils::Reg<fields::Qmem38, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x130);
            <utils::Reg<fields::Qmem38, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_9(&self) -> utils::Reg<fields::Qmem39, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x134);
            <utils::Reg<fields::Qmem39, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_10(&self) -> utils::Reg<fields::Qmem310, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x138);
            <utils::Reg<fields::Qmem310, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_11(&self) -> utils::Reg<fields::Qmem311, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x13c);
            <utils::Reg<fields::Qmem311, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_12(&self) -> utils::Reg<fields::Qmem312, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x140);
            <utils::Reg<fields::Qmem312, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_13(&self) -> utils::Reg<fields::Qmem313, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x144);
            <utils::Reg<fields::Qmem313, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_14(&self) -> utils::Reg<fields::Qmem314, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x148);
            <utils::Reg<fields::Qmem314, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_15(&self) -> utils::Reg<fields::Qmem315, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14c);
            <utils::Reg<fields::Qmem315, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_0(&self) -> utils::Reg<fields::Huffmin0, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x150);
            <utils::Reg<fields::Huffmin0, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_1(&self) -> utils::Reg<fields::Huffmin1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x154);
            <utils::Reg<fields::Huffmin1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_2(&self) -> utils::Reg<fields::Huffmin2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x158);
            <utils::Reg<fields::Huffmin2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_3(&self) -> utils::Reg<fields::Huffmin3, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x15c);
            <utils::Reg<fields::Huffmin3, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_4(&self) -> utils::Reg<fields::Huffmin4, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x160);
            <utils::Reg<fields::Huffmin4, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_5(&self) -> utils::Reg<fields::Huffmin5, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x164);
            <utils::Reg<fields::Huffmin5, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_6(&self) -> utils::Reg<fields::Huffmin6, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x168);
            <utils::Reg<fields::Huffmin6, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_7(&self) -> utils::Reg<fields::Huffmin7, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x16c);
            <utils::Reg<fields::Huffmin7, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_8(&self) -> utils::Reg<fields::Huffmin8, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x170);
            <utils::Reg<fields::Huffmin8, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_9(&self) -> utils::Reg<fields::Huffmin9, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x174);
            <utils::Reg<fields::Huffmin9, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_10(&self) -> utils::Reg<fields::Huffmin10, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x178);
            <utils::Reg<fields::Huffmin10, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_11(&self) -> utils::Reg<fields::Huffmin11, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x17c);
            <utils::Reg<fields::Huffmin11, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_12(&self) -> utils::Reg<fields::Huffmin12, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x180);
            <utils::Reg<fields::Huffmin12, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_13(&self) -> utils::Reg<fields::Huffmin13, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x184);
            <utils::Reg<fields::Huffmin13, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_14(&self) -> utils::Reg<fields::Huffmin14, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x188);
            <utils::Reg<fields::Huffmin14, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_15(&self) -> utils::Reg<fields::Huffmin15, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18c);
            <utils::Reg<fields::Huffmin15, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase0(&self) -> utils::Reg<fields::Huffbase0, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x190);
            <utils::Reg<fields::Huffbase0, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase1(&self) -> utils::Reg<fields::Huffbase1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x194);
            <utils::Reg<fields::Huffbase1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase2(&self) -> utils::Reg<fields::Huffbase2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x198);
            <utils::Reg<fields::Huffbase2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase3(&self) -> utils::Reg<fields::Huffbase3, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x19c);
            <utils::Reg<fields::Huffbase3, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase4(&self) -> utils::Reg<fields::Huffbase4, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1a0);
            <utils::Reg<fields::Huffbase4, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase5(&self) -> utils::Reg<fields::Huffbase5, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1a4);
            <utils::Reg<fields::Huffbase5, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase6(&self) -> utils::Reg<fields::Huffbase6, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1a8);
            <utils::Reg<fields::Huffbase6, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase7(&self) -> utils::Reg<fields::Huffbase7, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1ac);
            <utils::Reg<fields::Huffbase7, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase8(&self) -> utils::Reg<fields::Huffbase8, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1b0);
            <utils::Reg<fields::Huffbase8, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase9(&self) -> utils::Reg<fields::Huffbase9, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1b4);
            <utils::Reg<fields::Huffbase9, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase10(&self) -> utils::Reg<fields::Huffbase10, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1b8);
            <utils::Reg<fields::Huffbase10, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase11(&self) -> utils::Reg<fields::Huffbase11, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1bc);
            <utils::Reg<fields::Huffbase11, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase12(&self) -> utils::Reg<fields::Huffbase12, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c0);
            <utils::Reg<fields::Huffbase12, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase13(&self) -> utils::Reg<fields::Huffbase13, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c4);
            <utils::Reg<fields::Huffbase13, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase14(&self) -> utils::Reg<fields::Huffbase14, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c8);
            <utils::Reg<fields::Huffbase14, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase15(&self) -> utils::Reg<fields::Huffbase15, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1cc);
            <utils::Reg<fields::Huffbase15, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase16(&self) -> utils::Reg<fields::Huffbase16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1d0);
            <utils::Reg<fields::Huffbase16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase17(&self) -> utils::Reg<fields::Huffbase17, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1d4);
            <utils::Reg<fields::Huffbase17, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase18(&self) -> utils::Reg<fields::Huffbase18, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1d8);
            <utils::Reg<fields::Huffbase18, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase19(&self) -> utils::Reg<fields::Huffbase19, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1dc);
            <utils::Reg<fields::Huffbase19, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase20(&self) -> utils::Reg<fields::Huffbase20, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1e0);
            <utils::Reg<fields::Huffbase20, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase21(&self) -> utils::Reg<fields::Huffbase21, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1e4);
            <utils::Reg<fields::Huffbase21, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase22(&self) -> utils::Reg<fields::Huffbase22, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1e8);
            <utils::Reg<fields::Huffbase22, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase23(&self) -> utils::Reg<fields::Huffbase23, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1ec);
            <utils::Reg<fields::Huffbase23, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase24(&self) -> utils::Reg<fields::Huffbase24, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1f0);
            <utils::Reg<fields::Huffbase24, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase25(&self) -> utils::Reg<fields::Huffbase25, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1f4);
            <utils::Reg<fields::Huffbase25, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase26(&self) -> utils::Reg<fields::Huffbase26, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1f8);
            <utils::Reg<fields::Huffbase26, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase27(&self) -> utils::Reg<fields::Huffbase27, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1fc);
            <utils::Reg<fields::Huffbase27, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase28(&self) -> utils::Reg<fields::Huffbase28, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x200);
            <utils::Reg<fields::Huffbase28, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase29(&self) -> utils::Reg<fields::Huffbase29, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x204);
            <utils::Reg<fields::Huffbase29, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase30(&self) -> utils::Reg<fields::Huffbase30, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x208);
            <utils::Reg<fields::Huffbase30, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase31(&self) -> utils::Reg<fields::Huffbase31, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20c);
            <utils::Reg<fields::Huffbase31, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb0(&self) -> utils::Reg<fields::Huffsymb0, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x210);
            <utils::Reg<fields::Huffsymb0, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb1(&self) -> utils::Reg<fields::Huffsymb1, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x214);
            <utils::Reg<fields::Huffsymb1, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb2(&self) -> utils::Reg<fields::Huffsymb2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x218);
            <utils::Reg<fields::Huffsymb2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb3(&self) -> utils::Reg<fields::Huffsymb3, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x21c);
            <utils::Reg<fields::Huffsymb3, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb4(&self) -> utils::Reg<fields::Huffsymb4, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x220);
            <utils::Reg<fields::Huffsymb4, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb5(&self) -> utils::Reg<fields::Huffsymb5, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x224);
            <utils::Reg<fields::Huffsymb5, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb6(&self) -> utils::Reg<fields::Huffsymb6, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x228);
            <utils::Reg<fields::Huffsymb6, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb7(&self) -> utils::Reg<fields::Huffsymb7, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x22c);
            <utils::Reg<fields::Huffsymb7, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb8(&self) -> utils::Reg<fields::Huffsymb8, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x230);
            <utils::Reg<fields::Huffsymb8, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb9(&self) -> utils::Reg<fields::Huffsymb9, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x234);
            <utils::Reg<fields::Huffsymb9, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb10(&self) -> utils::Reg<fields::Huffsymb10, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x238);
            <utils::Reg<fields::Huffsymb10, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb11(&self) -> utils::Reg<fields::Huffsymb11, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x23c);
            <utils::Reg<fields::Huffsymb11, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb12(&self) -> utils::Reg<fields::Huffsymb12, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x240);
            <utils::Reg<fields::Huffsymb12, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb13(&self) -> utils::Reg<fields::Huffsymb13, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x244);
            <utils::Reg<fields::Huffsymb13, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb14(&self) -> utils::Reg<fields::Huffsymb14, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x248);
            <utils::Reg<fields::Huffsymb14, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb15(&self) -> utils::Reg<fields::Huffsymb15, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24c);
            <utils::Reg<fields::Huffsymb15, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb16(&self) -> utils::Reg<fields::Huffsymb16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x250);
            <utils::Reg<fields::Huffsymb16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb17(&self) -> utils::Reg<fields::Huffsymb17, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x254);
            <utils::Reg<fields::Huffsymb17, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb18(&self) -> utils::Reg<fields::Huffsymb18, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x258);
            <utils::Reg<fields::Huffsymb18, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb19(&self) -> utils::Reg<fields::Huffsymb19, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x25c);
            <utils::Reg<fields::Huffsymb19, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb20(&self) -> utils::Reg<fields::Huffsymb20, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x260);
            <utils::Reg<fields::Huffsymb20, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb21(&self) -> utils::Reg<fields::Huffsymb21, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x264);
            <utils::Reg<fields::Huffsymb21, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb22(&self) -> utils::Reg<fields::Huffsymb22, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x268);
            <utils::Reg<fields::Huffsymb22, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb23(&self) -> utils::Reg<fields::Huffsymb23, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x26c);
            <utils::Reg<fields::Huffsymb23, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb24(&self) -> utils::Reg<fields::Huffsymb24, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x270);
            <utils::Reg<fields::Huffsymb24, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb25(&self) -> utils::Reg<fields::Huffsymb25, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x274);
            <utils::Reg<fields::Huffsymb25, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb26(&self) -> utils::Reg<fields::Huffsymb26, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x278);
            <utils::Reg<fields::Huffsymb26, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb27(&self) -> utils::Reg<fields::Huffsymb27, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x27c);
            <utils::Reg<fields::Huffsymb27, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb28(&self) -> utils::Reg<fields::Huffsymb28, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x280);
            <utils::Reg<fields::Huffsymb28, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb29(&self) -> utils::Reg<fields::Huffsymb29, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x284);
            <utils::Reg<fields::Huffsymb29, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb30(&self) -> utils::Reg<fields::Huffsymb30, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x288);
            <utils::Reg<fields::Huffsymb30, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb31(&self) -> utils::Reg<fields::Huffsymb31, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28c);
            <utils::Reg<fields::Huffsymb31, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb32(&self) -> utils::Reg<fields::Huffsymb32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x290);
            <utils::Reg<fields::Huffsymb32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb33(&self) -> utils::Reg<fields::Huffsymb33, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x294);
            <utils::Reg<fields::Huffsymb33, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb34(&self) -> utils::Reg<fields::Huffsymb34, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x298);
            <utils::Reg<fields::Huffsymb34, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb35(&self) -> utils::Reg<fields::Huffsymb35, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x29c);
            <utils::Reg<fields::Huffsymb35, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb36(&self) -> utils::Reg<fields::Huffsymb36, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2a0);
            <utils::Reg<fields::Huffsymb36, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb37(&self) -> utils::Reg<fields::Huffsymb37, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2a4);
            <utils::Reg<fields::Huffsymb37, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb38(&self) -> utils::Reg<fields::Huffsymb38, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2a8);
            <utils::Reg<fields::Huffsymb38, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb39(&self) -> utils::Reg<fields::Huffsymb39, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2ac);
            <utils::Reg<fields::Huffsymb39, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb40(&self) -> utils::Reg<fields::Huffsymb40, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2b0);
            <utils::Reg<fields::Huffsymb40, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb41(&self) -> utils::Reg<fields::Huffsymb41, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2b4);
            <utils::Reg<fields::Huffsymb41, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb42(&self) -> utils::Reg<fields::Huffsymb42, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2b8);
            <utils::Reg<fields::Huffsymb42, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb43(&self) -> utils::Reg<fields::Huffsymb43, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2bc);
            <utils::Reg<fields::Huffsymb43, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb44(&self) -> utils::Reg<fields::Huffsymb44, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c0);
            <utils::Reg<fields::Huffsymb44, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb45(&self) -> utils::Reg<fields::Huffsymb45, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c4);
            <utils::Reg<fields::Huffsymb45, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb46(&self) -> utils::Reg<fields::Huffsymb46, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c8);
            <utils::Reg<fields::Huffsymb46, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb47(&self) -> utils::Reg<fields::Huffsymb47, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2cc);
            <utils::Reg<fields::Huffsymb47, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb48(&self) -> utils::Reg<fields::Huffsymb48, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2d0);
            <utils::Reg<fields::Huffsymb48, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb49(&self) -> utils::Reg<fields::Huffsymb49, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2d4);
            <utils::Reg<fields::Huffsymb49, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb50(&self) -> utils::Reg<fields::Huffsymb50, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2d8);
            <utils::Reg<fields::Huffsymb50, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb51(&self) -> utils::Reg<fields::Huffsymb51, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2dc);
            <utils::Reg<fields::Huffsymb51, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb52(&self) -> utils::Reg<fields::Huffsymb52, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2e0);
            <utils::Reg<fields::Huffsymb52, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb53(&self) -> utils::Reg<fields::Huffsymb53, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2e4);
            <utils::Reg<fields::Huffsymb53, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb54(&self) -> utils::Reg<fields::Huffsymb54, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2e8);
            <utils::Reg<fields::Huffsymb54, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb55(&self) -> utils::Reg<fields::Huffsymb55, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2ec);
            <utils::Reg<fields::Huffsymb55, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb56(&self) -> utils::Reg<fields::Huffsymb56, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2f0);
            <utils::Reg<fields::Huffsymb56, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb57(&self) -> utils::Reg<fields::Huffsymb57, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2f4);
            <utils::Reg<fields::Huffsymb57, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb58(&self) -> utils::Reg<fields::Huffsymb58, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2f8);
            <utils::Reg<fields::Huffsymb58, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb59(&self) -> utils::Reg<fields::Huffsymb59, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2fc);
            <utils::Reg<fields::Huffsymb59, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb60(&self) -> utils::Reg<fields::Huffsymb60, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x300);
            <utils::Reg<fields::Huffsymb60, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb61(&self) -> utils::Reg<fields::Huffsymb61, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x304);
            <utils::Reg<fields::Huffsymb61, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb62(&self) -> utils::Reg<fields::Huffsymb62, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x308);
            <utils::Reg<fields::Huffsymb62, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb63(&self) -> utils::Reg<fields::Huffsymb63, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30c);
            <utils::Reg<fields::Huffsymb63, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb64(&self) -> utils::Reg<fields::Huffsymb64, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x310);
            <utils::Reg<fields::Huffsymb64, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb65(&self) -> utils::Reg<fields::Huffsymb65, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x314);
            <utils::Reg<fields::Huffsymb65, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb66(&self) -> utils::Reg<fields::Huffsymb66, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x318);
            <utils::Reg<fields::Huffsymb66, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb67(&self) -> utils::Reg<fields::Huffsymb67, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x31c);
            <utils::Reg<fields::Huffsymb67, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb68(&self) -> utils::Reg<fields::Huffsymb68, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x320);
            <utils::Reg<fields::Huffsymb68, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb69(&self) -> utils::Reg<fields::Huffsymb69, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x324);
            <utils::Reg<fields::Huffsymb69, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb70(&self) -> utils::Reg<fields::Huffsymb70, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x328);
            <utils::Reg<fields::Huffsymb70, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb71(&self) -> utils::Reg<fields::Huffsymb71, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x32c);
            <utils::Reg<fields::Huffsymb71, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb72(&self) -> utils::Reg<fields::Huffsymb72, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x330);
            <utils::Reg<fields::Huffsymb72, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb73(&self) -> utils::Reg<fields::Huffsymb73, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x334);
            <utils::Reg<fields::Huffsymb73, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb74(&self) -> utils::Reg<fields::Huffsymb74, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x338);
            <utils::Reg<fields::Huffsymb74, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb75(&self) -> utils::Reg<fields::Huffsymb75, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x33c);
            <utils::Reg<fields::Huffsymb75, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb76(&self) -> utils::Reg<fields::Huffsymb76, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x340);
            <utils::Reg<fields::Huffsymb76, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb77(&self) -> utils::Reg<fields::Huffsymb77, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x344);
            <utils::Reg<fields::Huffsymb77, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb78(&self) -> utils::Reg<fields::Huffsymb78, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x348);
            <utils::Reg<fields::Huffsymb78, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb79(&self) -> utils::Reg<fields::Huffsymb79, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34c);
            <utils::Reg<fields::Huffsymb79, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb80(&self) -> utils::Reg<fields::Huffsymb80, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x350);
            <utils::Reg<fields::Huffsymb80, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb81(&self) -> utils::Reg<fields::Huffsymb81, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x354);
            <utils::Reg<fields::Huffsymb81, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb82(&self) -> utils::Reg<fields::Huffsymb82, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x358);
            <utils::Reg<fields::Huffsymb82, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb83(&self) -> utils::Reg<fields::Huffsymb83, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x35c);
            <utils::Reg<fields::Huffsymb83, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem0(&self) -> utils::Reg<fields::Dhtmem0, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x360);
            <utils::Reg<fields::Dhtmem0, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem2(&self) -> utils::Reg<fields::Dhtmem2, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x364);
            <utils::Reg<fields::Dhtmem2, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem3(&self) -> utils::Reg<fields::Dhtmem3, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x368);
            <utils::Reg<fields::Dhtmem3, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem4(&self) -> utils::Reg<fields::Dhtmem4, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x36c);
            <utils::Reg<fields::Dhtmem4, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem5(&self) -> utils::Reg<fields::Dhtmem5, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x370);
            <utils::Reg<fields::Dhtmem5, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem6(&self) -> utils::Reg<fields::Dhtmem6, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x374);
            <utils::Reg<fields::Dhtmem6, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem7(&self) -> utils::Reg<fields::Dhtmem7, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x378);
            <utils::Reg<fields::Dhtmem7, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem8(&self) -> utils::Reg<fields::Dhtmem8, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x37c);
            <utils::Reg<fields::Dhtmem8, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem9(&self) -> utils::Reg<fields::Dhtmem9, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x380);
            <utils::Reg<fields::Dhtmem9, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem10(&self) -> utils::Reg<fields::Dhtmem10, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x384);
            <utils::Reg<fields::Dhtmem10, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem11(&self) -> utils::Reg<fields::Dhtmem11, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x388);
            <utils::Reg<fields::Dhtmem11, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem12(&self) -> utils::Reg<fields::Dhtmem12, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38c);
            <utils::Reg<fields::Dhtmem12, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem13(&self) -> utils::Reg<fields::Dhtmem13, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x390);
            <utils::Reg<fields::Dhtmem13, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem14(&self) -> utils::Reg<fields::Dhtmem14, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x394);
            <utils::Reg<fields::Dhtmem14, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem15(&self) -> utils::Reg<fields::Dhtmem15, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x398);
            <utils::Reg<fields::Dhtmem15, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem16(&self) -> utils::Reg<fields::Dhtmem16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x39c);
            <utils::Reg<fields::Dhtmem16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem17(&self) -> utils::Reg<fields::Dhtmem17, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3a0);
            <utils::Reg<fields::Dhtmem17, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem18(&self) -> utils::Reg<fields::Dhtmem18, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3a4);
            <utils::Reg<fields::Dhtmem18, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem19(&self) -> utils::Reg<fields::Dhtmem19, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3a8);
            <utils::Reg<fields::Dhtmem19, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem20(&self) -> utils::Reg<fields::Dhtmem20, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3ac);
            <utils::Reg<fields::Dhtmem20, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem21(&self) -> utils::Reg<fields::Dhtmem21, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3b0);
            <utils::Reg<fields::Dhtmem21, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem22(&self) -> utils::Reg<fields::Dhtmem22, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3b4);
            <utils::Reg<fields::Dhtmem22, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem23(&self) -> utils::Reg<fields::Dhtmem23, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3b8);
            <utils::Reg<fields::Dhtmem23, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem24(&self) -> utils::Reg<fields::Dhtmem24, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3bc);
            <utils::Reg<fields::Dhtmem24, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem25(&self) -> utils::Reg<fields::Dhtmem25, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c0);
            <utils::Reg<fields::Dhtmem25, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem26(&self) -> utils::Reg<fields::Dhtmem26, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c4);
            <utils::Reg<fields::Dhtmem26, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem27(&self) -> utils::Reg<fields::Dhtmem27, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c8);
            <utils::Reg<fields::Dhtmem27, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem28(&self) -> utils::Reg<fields::Dhtmem28, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3cc);
            <utils::Reg<fields::Dhtmem28, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem29(&self) -> utils::Reg<fields::Dhtmem29, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3d0);
            <utils::Reg<fields::Dhtmem29, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem30(&self) -> utils::Reg<fields::Dhtmem30, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3d4);
            <utils::Reg<fields::Dhtmem30, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem31(&self) -> utils::Reg<fields::Dhtmem31, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3d8);
            <utils::Reg<fields::Dhtmem31, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem32(&self) -> utils::Reg<fields::Dhtmem32, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3dc);
            <utils::Reg<fields::Dhtmem32, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem33(&self) -> utils::Reg<fields::Dhtmem33, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3e0);
            <utils::Reg<fields::Dhtmem33, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem34(&self) -> utils::Reg<fields::Dhtmem34, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3e4);
            <utils::Reg<fields::Dhtmem34, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem35(&self) -> utils::Reg<fields::Dhtmem35, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3e8);
            <utils::Reg<fields::Dhtmem35, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem36(&self) -> utils::Reg<fields::Dhtmem36, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3ec);
            <utils::Reg<fields::Dhtmem36, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem37(&self) -> utils::Reg<fields::Dhtmem37, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3f0);
            <utils::Reg<fields::Dhtmem37, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem38(&self) -> utils::Reg<fields::Dhtmem38, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3f4);
            <utils::Reg<fields::Dhtmem38, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem39(&self) -> utils::Reg<fields::Dhtmem39, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3f8);
            <utils::Reg<fields::Dhtmem39, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem40(&self) -> utils::Reg<fields::Dhtmem40, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3fc);
            <utils::Reg<fields::Dhtmem40, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem41(&self) -> utils::Reg<fields::Dhtmem41, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x400);
            <utils::Reg<fields::Dhtmem41, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem42(&self) -> utils::Reg<fields::Dhtmem42, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x404);
            <utils::Reg<fields::Dhtmem42, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem43(&self) -> utils::Reg<fields::Dhtmem43, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x408);
            <utils::Reg<fields::Dhtmem43, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem44(&self) -> utils::Reg<fields::Dhtmem44, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40c);
            <utils::Reg<fields::Dhtmem44, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem45(&self) -> utils::Reg<fields::Dhtmem45, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x410);
            <utils::Reg<fields::Dhtmem45, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem46(&self) -> utils::Reg<fields::Dhtmem46, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x414);
            <utils::Reg<fields::Dhtmem46, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem47(&self) -> utils::Reg<fields::Dhtmem47, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x418);
            <utils::Reg<fields::Dhtmem47, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem48(&self) -> utils::Reg<fields::Dhtmem48, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x41c);
            <utils::Reg<fields::Dhtmem48, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem49(&self) -> utils::Reg<fields::Dhtmem49, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x420);
            <utils::Reg<fields::Dhtmem49, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem50(&self) -> utils::Reg<fields::Dhtmem50, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x424);
            <utils::Reg<fields::Dhtmem50, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem51(&self) -> utils::Reg<fields::Dhtmem51, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x428);
            <utils::Reg<fields::Dhtmem51, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem52(&self) -> utils::Reg<fields::Dhtmem52, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x42c);
            <utils::Reg<fields::Dhtmem52, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem53(&self) -> utils::Reg<fields::Dhtmem53, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x430);
            <utils::Reg<fields::Dhtmem53, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem54(&self) -> utils::Reg<fields::Dhtmem54, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x434);
            <utils::Reg<fields::Dhtmem54, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem55(&self) -> utils::Reg<fields::Dhtmem55, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x438);
            <utils::Reg<fields::Dhtmem55, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem56(&self) -> utils::Reg<fields::Dhtmem56, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x43c);
            <utils::Reg<fields::Dhtmem56, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem57(&self) -> utils::Reg<fields::Dhtmem57, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x440);
            <utils::Reg<fields::Dhtmem57, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem58(&self) -> utils::Reg<fields::Dhtmem58, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x444);
            <utils::Reg<fields::Dhtmem58, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem59(&self) -> utils::Reg<fields::Dhtmem59, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x448);
            <utils::Reg<fields::Dhtmem59, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem60(&self) -> utils::Reg<fields::Dhtmem60, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44c);
            <utils::Reg<fields::Dhtmem60, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem61(&self) -> utils::Reg<fields::Dhtmem61, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x450);
            <utils::Reg<fields::Dhtmem61, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem62(&self) -> utils::Reg<fields::Dhtmem62, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x454);
            <utils::Reg<fields::Dhtmem62, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem63(&self) -> utils::Reg<fields::Dhtmem63, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x458);
            <utils::Reg<fields::Dhtmem63, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem64(&self) -> utils::Reg<fields::Dhtmem64, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x45c);
            <utils::Reg<fields::Dhtmem64, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem65(&self) -> utils::Reg<fields::Dhtmem65, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x460);
            <utils::Reg<fields::Dhtmem65, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem66(&self) -> utils::Reg<fields::Dhtmem66, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x464);
            <utils::Reg<fields::Dhtmem66, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem67(&self) -> utils::Reg<fields::Dhtmem67, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x468);
            <utils::Reg<fields::Dhtmem67, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem68(&self) -> utils::Reg<fields::Dhtmem68, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x46c);
            <utils::Reg<fields::Dhtmem68, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem69(&self) -> utils::Reg<fields::Dhtmem69, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x470);
            <utils::Reg<fields::Dhtmem69, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem70(&self) -> utils::Reg<fields::Dhtmem70, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x474);
            <utils::Reg<fields::Dhtmem70, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem71(&self) -> utils::Reg<fields::Dhtmem71, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x478);
            <utils::Reg<fields::Dhtmem71, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem72(&self) -> utils::Reg<fields::Dhtmem72, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x47c);
            <utils::Reg<fields::Dhtmem72, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem73(&self) -> utils::Reg<fields::Dhtmem73, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x480);
            <utils::Reg<fields::Dhtmem73, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem74(&self) -> utils::Reg<fields::Dhtmem74, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x484);
            <utils::Reg<fields::Dhtmem74, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem75(&self) -> utils::Reg<fields::Dhtmem75, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x488);
            <utils::Reg<fields::Dhtmem75, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem76(&self) -> utils::Reg<fields::Dhtmem76, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48c);
            <utils::Reg<fields::Dhtmem76, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem77(&self) -> utils::Reg<fields::Dhtmem77, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x490);
            <utils::Reg<fields::Dhtmem77, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem78(&self) -> utils::Reg<fields::Dhtmem78, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x494);
            <utils::Reg<fields::Dhtmem78, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem79(&self) -> utils::Reg<fields::Dhtmem79, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x498);
            <utils::Reg<fields::Dhtmem79, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem80(&self) -> utils::Reg<fields::Dhtmem80, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x49c);
            <utils::Reg<fields::Dhtmem80, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem81(&self) -> utils::Reg<fields::Dhtmem81, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4a0);
            <utils::Reg<fields::Dhtmem81, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem82(&self) -> utils::Reg<fields::Dhtmem82, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4a4);
            <utils::Reg<fields::Dhtmem82, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem83(&self) -> utils::Reg<fields::Dhtmem83, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4a8);
            <utils::Reg<fields::Dhtmem83, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem84(&self) -> utils::Reg<fields::Dhtmem84, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4ac);
            <utils::Reg<fields::Dhtmem84, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem85(&self) -> utils::Reg<fields::Dhtmem85, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4b0);
            <utils::Reg<fields::Dhtmem85, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem86(&self) -> utils::Reg<fields::Dhtmem86, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4b4);
            <utils::Reg<fields::Dhtmem86, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem87(&self) -> utils::Reg<fields::Dhtmem87, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4b8);
            <utils::Reg<fields::Dhtmem87, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem88(&self) -> utils::Reg<fields::Dhtmem88, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4bc);
            <utils::Reg<fields::Dhtmem88, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem89(&self) -> utils::Reg<fields::Dhtmem89, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c0);
            <utils::Reg<fields::Dhtmem89, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem90(&self) -> utils::Reg<fields::Dhtmem90, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c4);
            <utils::Reg<fields::Dhtmem90, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem91(&self) -> utils::Reg<fields::Dhtmem91, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c8);
            <utils::Reg<fields::Dhtmem91, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem92(&self) -> utils::Reg<fields::Dhtmem92, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4cc);
            <utils::Reg<fields::Dhtmem92, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem93(&self) -> utils::Reg<fields::Dhtmem93, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4d0);
            <utils::Reg<fields::Dhtmem93, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem94(&self) -> utils::Reg<fields::Dhtmem94, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4d4);
            <utils::Reg<fields::Dhtmem94, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem95(&self) -> utils::Reg<fields::Dhtmem95, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4d8);
            <utils::Reg<fields::Dhtmem95, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem96(&self) -> utils::Reg<fields::Dhtmem96, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4dc);
            <utils::Reg<fields::Dhtmem96, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem97(&self) -> utils::Reg<fields::Dhtmem97, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4e0);
            <utils::Reg<fields::Dhtmem97, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem98(&self) -> utils::Reg<fields::Dhtmem98, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4e4);
            <utils::Reg<fields::Dhtmem98, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem99(&self) -> utils::Reg<fields::Dhtmem99, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4e8);
            <utils::Reg<fields::Dhtmem99, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem100(&self) -> utils::Reg<fields::Dhtmem100, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4ec);
            <utils::Reg<fields::Dhtmem100, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem101(&self) -> utils::Reg<fields::Dhtmem101, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4f0);
            <utils::Reg<fields::Dhtmem101, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem102(&self) -> utils::Reg<fields::Dhtmem102, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4f4);
            <utils::Reg<fields::Dhtmem102, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem103(&self) -> utils::Reg<fields::Dhtmem103, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4f8);
            <utils::Reg<fields::Dhtmem103, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_0(&self) -> utils::Reg<fields::HuffencAc00, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x500);
            <utils::Reg<fields::HuffencAc00, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_1(&self) -> utils::Reg<fields::HuffencAc01, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x504);
            <utils::Reg<fields::HuffencAc01, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_2(&self) -> utils::Reg<fields::HuffencAc02, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x508);
            <utils::Reg<fields::HuffencAc02, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_3(&self) -> utils::Reg<fields::HuffencAc03, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50c);
            <utils::Reg<fields::HuffencAc03, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_4(&self) -> utils::Reg<fields::HuffencAc04, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x510);
            <utils::Reg<fields::HuffencAc04, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_5(&self) -> utils::Reg<fields::HuffencAc05, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x514);
            <utils::Reg<fields::HuffencAc05, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_6(&self) -> utils::Reg<fields::HuffencAc06, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x518);
            <utils::Reg<fields::HuffencAc06, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_7(&self) -> utils::Reg<fields::HuffencAc07, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x51c);
            <utils::Reg<fields::HuffencAc07, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_8(&self) -> utils::Reg<fields::HuffencAc08, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x520);
            <utils::Reg<fields::HuffencAc08, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_9(&self) -> utils::Reg<fields::HuffencAc09, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x524);
            <utils::Reg<fields::HuffencAc09, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_10(&self) -> utils::Reg<fields::HuffencAc010, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x528);
            <utils::Reg<fields::HuffencAc010, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_11(&self) -> utils::Reg<fields::HuffencAc011, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x52c);
            <utils::Reg<fields::HuffencAc011, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_12(&self) -> utils::Reg<fields::HuffencAc012, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x530);
            <utils::Reg<fields::HuffencAc012, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_13(&self) -> utils::Reg<fields::HuffencAc013, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x534);
            <utils::Reg<fields::HuffencAc013, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_14(&self) -> utils::Reg<fields::HuffencAc014, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x538);
            <utils::Reg<fields::HuffencAc014, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_15(&self) -> utils::Reg<fields::HuffencAc015, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x53c);
            <utils::Reg<fields::HuffencAc015, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_16(&self) -> utils::Reg<fields::HuffencAc016, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x540);
            <utils::Reg<fields::HuffencAc016, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_17(&self) -> utils::Reg<fields::HuffencAc017, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x544);
            <utils::Reg<fields::HuffencAc017, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_18(&self) -> utils::Reg<fields::HuffencAc018, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x548);
            <utils::Reg<fields::HuffencAc018, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_19(&self) -> utils::Reg<fields::HuffencAc019, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x54c);
            <utils::Reg<fields::HuffencAc019, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_20(&self) -> utils::Reg<fields::HuffencAc020, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x550);
            <utils::Reg<fields::HuffencAc020, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_21(&self) -> utils::Reg<fields::HuffencAc021, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x554);
            <utils::Reg<fields::HuffencAc021, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_22(&self) -> utils::Reg<fields::HuffencAc022, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x558);
            <utils::Reg<fields::HuffencAc022, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_23(&self) -> utils::Reg<fields::HuffencAc023, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x55c);
            <utils::Reg<fields::HuffencAc023, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_24(&self) -> utils::Reg<fields::HuffencAc024, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x560);
            <utils::Reg<fields::HuffencAc024, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_25(&self) -> utils::Reg<fields::HuffencAc025, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x564);
            <utils::Reg<fields::HuffencAc025, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_26(&self) -> utils::Reg<fields::HuffencAc026, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x568);
            <utils::Reg<fields::HuffencAc026, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_27(&self) -> utils::Reg<fields::HuffencAc027, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x56c);
            <utils::Reg<fields::HuffencAc027, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_28(&self) -> utils::Reg<fields::HuffencAc028, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x570);
            <utils::Reg<fields::HuffencAc028, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_29(&self) -> utils::Reg<fields::HuffencAc029, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x574);
            <utils::Reg<fields::HuffencAc029, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_30(&self) -> utils::Reg<fields::HuffencAc030, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x578);
            <utils::Reg<fields::HuffencAc030, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_31(&self) -> utils::Reg<fields::HuffencAc031, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x57c);
            <utils::Reg<fields::HuffencAc031, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_32(&self) -> utils::Reg<fields::HuffencAc032, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x580);
            <utils::Reg<fields::HuffencAc032, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_33(&self) -> utils::Reg<fields::HuffencAc033, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x584);
            <utils::Reg<fields::HuffencAc033, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_34(&self) -> utils::Reg<fields::HuffencAc034, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x588);
            <utils::Reg<fields::HuffencAc034, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_35(&self) -> utils::Reg<fields::HuffencAc035, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x58c);
            <utils::Reg<fields::HuffencAc035, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_36(&self) -> utils::Reg<fields::HuffencAc036, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x590);
            <utils::Reg<fields::HuffencAc036, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_37(&self) -> utils::Reg<fields::HuffencAc037, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x594);
            <utils::Reg<fields::HuffencAc037, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_38(&self) -> utils::Reg<fields::HuffencAc038, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x598);
            <utils::Reg<fields::HuffencAc038, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_39(&self) -> utils::Reg<fields::HuffencAc039, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x59c);
            <utils::Reg<fields::HuffencAc039, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_40(&self) -> utils::Reg<fields::HuffencAc040, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5a0);
            <utils::Reg<fields::HuffencAc040, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_41(&self) -> utils::Reg<fields::HuffencAc041, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5a4);
            <utils::Reg<fields::HuffencAc041, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_42(&self) -> utils::Reg<fields::HuffencAc042, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5a8);
            <utils::Reg<fields::HuffencAc042, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_43(&self) -> utils::Reg<fields::HuffencAc043, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5ac);
            <utils::Reg<fields::HuffencAc043, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_44(&self) -> utils::Reg<fields::HuffencAc044, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5b0);
            <utils::Reg<fields::HuffencAc044, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_45(&self) -> utils::Reg<fields::HuffencAc045, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5b4);
            <utils::Reg<fields::HuffencAc045, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_46(&self) -> utils::Reg<fields::HuffencAc046, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5b8);
            <utils::Reg<fields::HuffencAc046, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_47(&self) -> utils::Reg<fields::HuffencAc047, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5bc);
            <utils::Reg<fields::HuffencAc047, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_48(&self) -> utils::Reg<fields::HuffencAc048, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c0);
            <utils::Reg<fields::HuffencAc048, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_49(&self) -> utils::Reg<fields::HuffencAc049, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c4);
            <utils::Reg<fields::HuffencAc049, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_50(&self) -> utils::Reg<fields::HuffencAc050, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c8);
            <utils::Reg<fields::HuffencAc050, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_51(&self) -> utils::Reg<fields::HuffencAc051, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5cc);
            <utils::Reg<fields::HuffencAc051, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_52(&self) -> utils::Reg<fields::HuffencAc052, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5d0);
            <utils::Reg<fields::HuffencAc052, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_53(&self) -> utils::Reg<fields::HuffencAc053, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5d4);
            <utils::Reg<fields::HuffencAc053, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_54(&self) -> utils::Reg<fields::HuffencAc054, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5d8);
            <utils::Reg<fields::HuffencAc054, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_55(&self) -> utils::Reg<fields::HuffencAc055, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5dc);
            <utils::Reg<fields::HuffencAc055, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_56(&self) -> utils::Reg<fields::HuffencAc056, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5e0);
            <utils::Reg<fields::HuffencAc056, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_57(&self) -> utils::Reg<fields::HuffencAc057, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5e4);
            <utils::Reg<fields::HuffencAc057, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_58(&self) -> utils::Reg<fields::HuffencAc058, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5e8);
            <utils::Reg<fields::HuffencAc058, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_59(&self) -> utils::Reg<fields::HuffencAc059, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5ec);
            <utils::Reg<fields::HuffencAc059, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_60(&self) -> utils::Reg<fields::HuffencAc060, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5f0);
            <utils::Reg<fields::HuffencAc060, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_61(&self) -> utils::Reg<fields::HuffencAc061, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5f4);
            <utils::Reg<fields::HuffencAc061, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_62(&self) -> utils::Reg<fields::HuffencAc062, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5f8);
            <utils::Reg<fields::HuffencAc062, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_63(&self) -> utils::Reg<fields::HuffencAc063, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5fc);
            <utils::Reg<fields::HuffencAc063, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_64(&self) -> utils::Reg<fields::HuffencAc064, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x600);
            <utils::Reg<fields::HuffencAc064, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_65(&self) -> utils::Reg<fields::HuffencAc065, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x604);
            <utils::Reg<fields::HuffencAc065, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_66(&self) -> utils::Reg<fields::HuffencAc066, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x608);
            <utils::Reg<fields::HuffencAc066, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_67(&self) -> utils::Reg<fields::HuffencAc067, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60c);
            <utils::Reg<fields::HuffencAc067, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_68(&self) -> utils::Reg<fields::HuffencAc068, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x610);
            <utils::Reg<fields::HuffencAc068, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_69(&self) -> utils::Reg<fields::HuffencAc069, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x614);
            <utils::Reg<fields::HuffencAc069, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_70(&self) -> utils::Reg<fields::HuffencAc070, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x618);
            <utils::Reg<fields::HuffencAc070, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_71(&self) -> utils::Reg<fields::HuffencAc071, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x61c);
            <utils::Reg<fields::HuffencAc071, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_72(&self) -> utils::Reg<fields::HuffencAc072, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x620);
            <utils::Reg<fields::HuffencAc072, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_73(&self) -> utils::Reg<fields::HuffencAc073, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x624);
            <utils::Reg<fields::HuffencAc073, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_74(&self) -> utils::Reg<fields::HuffencAc074, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x628);
            <utils::Reg<fields::HuffencAc074, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_75(&self) -> utils::Reg<fields::HuffencAc075, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x62c);
            <utils::Reg<fields::HuffencAc075, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_76(&self) -> utils::Reg<fields::HuffencAc076, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x630);
            <utils::Reg<fields::HuffencAc076, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_77(&self) -> utils::Reg<fields::HuffencAc077, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x634);
            <utils::Reg<fields::HuffencAc077, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_78(&self) -> utils::Reg<fields::HuffencAc078, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x638);
            <utils::Reg<fields::HuffencAc078, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_79(&self) -> utils::Reg<fields::HuffencAc079, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x63c);
            <utils::Reg<fields::HuffencAc079, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_80(&self) -> utils::Reg<fields::HuffencAc080, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x640);
            <utils::Reg<fields::HuffencAc080, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_81(&self) -> utils::Reg<fields::HuffencAc081, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x644);
            <utils::Reg<fields::HuffencAc081, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_82(&self) -> utils::Reg<fields::HuffencAc082, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x648);
            <utils::Reg<fields::HuffencAc082, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_83(&self) -> utils::Reg<fields::HuffencAc083, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x64c);
            <utils::Reg<fields::HuffencAc083, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_84(&self) -> utils::Reg<fields::HuffencAc084, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x650);
            <utils::Reg<fields::HuffencAc084, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_85(&self) -> utils::Reg<fields::HuffencAc085, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x654);
            <utils::Reg<fields::HuffencAc085, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_86(&self) -> utils::Reg<fields::HuffencAc086, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x658);
            <utils::Reg<fields::HuffencAc086, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_87(&self) -> utils::Reg<fields::HuffencAc087, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x65c);
            <utils::Reg<fields::HuffencAc087, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_0(&self) -> utils::Reg<fields::HuffencAc10, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x660);
            <utils::Reg<fields::HuffencAc10, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_1(&self) -> utils::Reg<fields::HuffencAc11, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x664);
            <utils::Reg<fields::HuffencAc11, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_2(&self) -> utils::Reg<fields::HuffencAc12, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x668);
            <utils::Reg<fields::HuffencAc12, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_3(&self) -> utils::Reg<fields::HuffencAc13, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x66c);
            <utils::Reg<fields::HuffencAc13, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_4(&self) -> utils::Reg<fields::HuffencAc14, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x670);
            <utils::Reg<fields::HuffencAc14, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_5(&self) -> utils::Reg<fields::HuffencAc15, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x674);
            <utils::Reg<fields::HuffencAc15, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_6(&self) -> utils::Reg<fields::HuffencAc16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x678);
            <utils::Reg<fields::HuffencAc16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_7(&self) -> utils::Reg<fields::HuffencAc17, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x67c);
            <utils::Reg<fields::HuffencAc17, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_8(&self) -> utils::Reg<fields::HuffencAc18, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x680);
            <utils::Reg<fields::HuffencAc18, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_9(&self) -> utils::Reg<fields::HuffencAc19, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x684);
            <utils::Reg<fields::HuffencAc19, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_10(&self) -> utils::Reg<fields::HuffencAc110, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x688);
            <utils::Reg<fields::HuffencAc110, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_11(&self) -> utils::Reg<fields::HuffencAc111, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68c);
            <utils::Reg<fields::HuffencAc111, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_12(&self) -> utils::Reg<fields::HuffencAc112, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x690);
            <utils::Reg<fields::HuffencAc112, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_13(&self) -> utils::Reg<fields::HuffencAc113, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x694);
            <utils::Reg<fields::HuffencAc113, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_14(&self) -> utils::Reg<fields::HuffencAc114, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x698);
            <utils::Reg<fields::HuffencAc114, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_15(&self) -> utils::Reg<fields::HuffencAc115, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x69c);
            <utils::Reg<fields::HuffencAc115, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_16(&self) -> utils::Reg<fields::HuffencAc116, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6a0);
            <utils::Reg<fields::HuffencAc116, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_17(&self) -> utils::Reg<fields::HuffencAc117, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6a4);
            <utils::Reg<fields::HuffencAc117, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_18(&self) -> utils::Reg<fields::HuffencAc118, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6a8);
            <utils::Reg<fields::HuffencAc118, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_19(&self) -> utils::Reg<fields::HuffencAc119, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6ac);
            <utils::Reg<fields::HuffencAc119, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_20(&self) -> utils::Reg<fields::HuffencAc120, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6b0);
            <utils::Reg<fields::HuffencAc120, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_21(&self) -> utils::Reg<fields::HuffencAc121, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6b4);
            <utils::Reg<fields::HuffencAc121, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_22(&self) -> utils::Reg<fields::HuffencAc122, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6b8);
            <utils::Reg<fields::HuffencAc122, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_23(&self) -> utils::Reg<fields::HuffencAc123, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6bc);
            <utils::Reg<fields::HuffencAc123, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_24(&self) -> utils::Reg<fields::HuffencAc124, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6c0);
            <utils::Reg<fields::HuffencAc124, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_25(&self) -> utils::Reg<fields::HuffencAc125, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6c4);
            <utils::Reg<fields::HuffencAc125, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_26(&self) -> utils::Reg<fields::HuffencAc126, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6c8);
            <utils::Reg<fields::HuffencAc126, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_27(&self) -> utils::Reg<fields::HuffencAc127, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6cc);
            <utils::Reg<fields::HuffencAc127, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_28(&self) -> utils::Reg<fields::HuffencAc128, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6d0);
            <utils::Reg<fields::HuffencAc128, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_29(&self) -> utils::Reg<fields::HuffencAc129, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6d4);
            <utils::Reg<fields::HuffencAc129, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_30(&self) -> utils::Reg<fields::HuffencAc130, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6d8);
            <utils::Reg<fields::HuffencAc130, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_31(&self) -> utils::Reg<fields::HuffencAc131, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6dc);
            <utils::Reg<fields::HuffencAc131, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_32(&self) -> utils::Reg<fields::HuffencAc132, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6e0);
            <utils::Reg<fields::HuffencAc132, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_33(&self) -> utils::Reg<fields::HuffencAc133, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6e4);
            <utils::Reg<fields::HuffencAc133, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_34(&self) -> utils::Reg<fields::HuffencAc134, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6e8);
            <utils::Reg<fields::HuffencAc134, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_35(&self) -> utils::Reg<fields::HuffencAc135, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6ec);
            <utils::Reg<fields::HuffencAc135, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_36(&self) -> utils::Reg<fields::HuffencAc136, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6f0);
            <utils::Reg<fields::HuffencAc136, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_37(&self) -> utils::Reg<fields::HuffencAc137, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6f4);
            <utils::Reg<fields::HuffencAc137, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_38(&self) -> utils::Reg<fields::HuffencAc138, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6f8);
            <utils::Reg<fields::HuffencAc138, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_39(&self) -> utils::Reg<fields::HuffencAc139, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6fc);
            <utils::Reg<fields::HuffencAc139, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_40(&self) -> utils::Reg<fields::HuffencAc140, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x700);
            <utils::Reg<fields::HuffencAc140, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_41(&self) -> utils::Reg<fields::HuffencAc141, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x704);
            <utils::Reg<fields::HuffencAc141, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_42(&self) -> utils::Reg<fields::HuffencAc142, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x708);
            <utils::Reg<fields::HuffencAc142, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_43(&self) -> utils::Reg<fields::HuffencAc143, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x70c);
            <utils::Reg<fields::HuffencAc143, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_44(&self) -> utils::Reg<fields::HuffencAc144, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x710);
            <utils::Reg<fields::HuffencAc144, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_45(&self) -> utils::Reg<fields::HuffencAc145, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x714);
            <utils::Reg<fields::HuffencAc145, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_46(&self) -> utils::Reg<fields::HuffencAc146, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x718);
            <utils::Reg<fields::HuffencAc146, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_47(&self) -> utils::Reg<fields::HuffencAc147, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x71c);
            <utils::Reg<fields::HuffencAc147, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_48(&self) -> utils::Reg<fields::HuffencAc148, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x720);
            <utils::Reg<fields::HuffencAc148, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_49(&self) -> utils::Reg<fields::HuffencAc149, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x724);
            <utils::Reg<fields::HuffencAc149, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_50(&self) -> utils::Reg<fields::HuffencAc150, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x728);
            <utils::Reg<fields::HuffencAc150, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_51(&self) -> utils::Reg<fields::HuffencAc151, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x72c);
            <utils::Reg<fields::HuffencAc151, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_52(&self) -> utils::Reg<fields::HuffencAc152, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x730);
            <utils::Reg<fields::HuffencAc152, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_53(&self) -> utils::Reg<fields::HuffencAc153, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x734);
            <utils::Reg<fields::HuffencAc153, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_54(&self) -> utils::Reg<fields::HuffencAc154, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x738);
            <utils::Reg<fields::HuffencAc154, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_55(&self) -> utils::Reg<fields::HuffencAc155, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x73c);
            <utils::Reg<fields::HuffencAc155, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_56(&self) -> utils::Reg<fields::HuffencAc156, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x740);
            <utils::Reg<fields::HuffencAc156, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_57(&self) -> utils::Reg<fields::HuffencAc157, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x744);
            <utils::Reg<fields::HuffencAc157, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_58(&self) -> utils::Reg<fields::HuffencAc158, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x748);
            <utils::Reg<fields::HuffencAc158, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_59(&self) -> utils::Reg<fields::HuffencAc159, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x74c);
            <utils::Reg<fields::HuffencAc159, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_60(&self) -> utils::Reg<fields::HuffencAc160, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x750);
            <utils::Reg<fields::HuffencAc160, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_61(&self) -> utils::Reg<fields::HuffencAc161, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x754);
            <utils::Reg<fields::HuffencAc161, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_62(&self) -> utils::Reg<fields::HuffencAc162, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x758);
            <utils::Reg<fields::HuffencAc162, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_63(&self) -> utils::Reg<fields::HuffencAc163, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x75c);
            <utils::Reg<fields::HuffencAc163, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_64(&self) -> utils::Reg<fields::HuffencAc164, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x760);
            <utils::Reg<fields::HuffencAc164, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_65(&self) -> utils::Reg<fields::HuffencAc165, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x764);
            <utils::Reg<fields::HuffencAc165, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_66(&self) -> utils::Reg<fields::HuffencAc166, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x768);
            <utils::Reg<fields::HuffencAc166, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_67(&self) -> utils::Reg<fields::HuffencAc167, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x76c);
            <utils::Reg<fields::HuffencAc167, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_68(&self) -> utils::Reg<fields::HuffencAc168, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x770);
            <utils::Reg<fields::HuffencAc168, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_69(&self) -> utils::Reg<fields::HuffencAc169, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x774);
            <utils::Reg<fields::HuffencAc169, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_70(&self) -> utils::Reg<fields::HuffencAc170, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x778);
            <utils::Reg<fields::HuffencAc170, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_71(&self) -> utils::Reg<fields::HuffencAc171, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x77c);
            <utils::Reg<fields::HuffencAc171, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_72(&self) -> utils::Reg<fields::HuffencAc172, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x780);
            <utils::Reg<fields::HuffencAc172, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_73(&self) -> utils::Reg<fields::HuffencAc173, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x784);
            <utils::Reg<fields::HuffencAc173, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_74(&self) -> utils::Reg<fields::HuffencAc174, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x788);
            <utils::Reg<fields::HuffencAc174, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_75(&self) -> utils::Reg<fields::HuffencAc175, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x78c);
            <utils::Reg<fields::HuffencAc175, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_76(&self) -> utils::Reg<fields::HuffencAc176, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x790);
            <utils::Reg<fields::HuffencAc176, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_77(&self) -> utils::Reg<fields::HuffencAc177, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x794);
            <utils::Reg<fields::HuffencAc177, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_78(&self) -> utils::Reg<fields::HuffencAc178, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x798);
            <utils::Reg<fields::HuffencAc178, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_79(&self) -> utils::Reg<fields::HuffencAc179, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x79c);
            <utils::Reg<fields::HuffencAc179, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_80(&self) -> utils::Reg<fields::HuffencAc180, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7a0);
            <utils::Reg<fields::HuffencAc180, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_81(&self) -> utils::Reg<fields::HuffencAc181, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7a4);
            <utils::Reg<fields::HuffencAc181, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_82(&self) -> utils::Reg<fields::HuffencAc182, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7a8);
            <utils::Reg<fields::HuffencAc182, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_83(&self) -> utils::Reg<fields::HuffencAc183, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7ac);
            <utils::Reg<fields::HuffencAc183, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_84(&self) -> utils::Reg<fields::HuffencAc184, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7b0);
            <utils::Reg<fields::HuffencAc184, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_85(&self) -> utils::Reg<fields::HuffencAc185, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7b4);
            <utils::Reg<fields::HuffencAc185, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_86(&self) -> utils::Reg<fields::HuffencAc186, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7b8);
            <utils::Reg<fields::HuffencAc186, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_87(&self) -> utils::Reg<fields::HuffencAc187, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7bc);
            <utils::Reg<fields::HuffencAc187, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_0(&self) -> utils::Reg<fields::HuffencDc00, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7c0);
            <utils::Reg<fields::HuffencDc00, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_1(&self) -> utils::Reg<fields::HuffencDc01, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7c4);
            <utils::Reg<fields::HuffencDc01, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_2(&self) -> utils::Reg<fields::HuffencDc02, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7c8);
            <utils::Reg<fields::HuffencDc02, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_3(&self) -> utils::Reg<fields::HuffencDc03, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7cc);
            <utils::Reg<fields::HuffencDc03, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_4(&self) -> utils::Reg<fields::HuffencDc04, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7d0);
            <utils::Reg<fields::HuffencDc04, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_5(&self) -> utils::Reg<fields::HuffencDc05, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7d4);
            <utils::Reg<fields::HuffencDc05, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_6(&self) -> utils::Reg<fields::HuffencDc06, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7d8);
            <utils::Reg<fields::HuffencDc06, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_7(&self) -> utils::Reg<fields::HuffencDc07, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7dc);
            <utils::Reg<fields::HuffencDc07, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_0(&self) -> utils::Reg<fields::HuffencDc10, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7e0);
            <utils::Reg<fields::HuffencDc10, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_1(&self) -> utils::Reg<fields::HuffencDc11, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7e4);
            <utils::Reg<fields::HuffencDc11, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_2(&self) -> utils::Reg<fields::HuffencDc12, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7e8);
            <utils::Reg<fields::HuffencDc12, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_3(&self) -> utils::Reg<fields::HuffencDc13, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7ec);
            <utils::Reg<fields::HuffencDc13, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_4(&self) -> utils::Reg<fields::HuffencDc14, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7f0);
            <utils::Reg<fields::HuffencDc14, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_5(&self) -> utils::Reg<fields::HuffencDc15, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7f4);
            <utils::Reg<fields::HuffencDc15, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_6(&self) -> utils::Reg<fields::HuffencDc16, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7f8);
            <utils::Reg<fields::HuffencDc16, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_7(&self) -> utils::Reg<fields::HuffencDc17, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7fc);
            <utils::Reg<fields::HuffencDc17, utils::RW>>::from_ptr(ptr)
        }
    }
}
pub mod fields {
    #[allow(unused)]
    use super::*;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem0 {
        bits: u32,
    }
    impl Default for Dhtmem0 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem0 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem10 {
        bits: u32,
    }
    impl Default for Dhtmem10 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem10 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem100 {
        bits: u32,
    }
    impl Default for Dhtmem100 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem100 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem101 {
        bits: u32,
    }
    impl Default for Dhtmem101 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem101 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem102 {
        bits: u32,
    }
    impl Default for Dhtmem102 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem102 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem103 {
        bits: u32,
    }
    impl Default for Dhtmem103 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem103 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem11 {
        bits: u32,
    }
    impl Default for Dhtmem11 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem11 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem12 {
        bits: u32,
    }
    impl Default for Dhtmem12 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem12 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem13 {
        bits: u32,
    }
    impl Default for Dhtmem13 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem13 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem14 {
        bits: u32,
    }
    impl Default for Dhtmem14 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem14 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem15 {
        bits: u32,
    }
    impl Default for Dhtmem15 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem15 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem16 {
        bits: u32,
    }
    impl Default for Dhtmem16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem17 {
        bits: u32,
    }
    impl Default for Dhtmem17 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem17 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem18 {
        bits: u32,
    }
    impl Default for Dhtmem18 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem18 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem19 {
        bits: u32,
    }
    impl Default for Dhtmem19 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem19 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem2 {
        bits: u32,
    }
    impl Default for Dhtmem2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem20 {
        bits: u32,
    }
    impl Default for Dhtmem20 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem20 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem21 {
        bits: u32,
    }
    impl Default for Dhtmem21 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem21 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem22 {
        bits: u32,
    }
    impl Default for Dhtmem22 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem22 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem23 {
        bits: u32,
    }
    impl Default for Dhtmem23 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem23 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem24 {
        bits: u32,
    }
    impl Default for Dhtmem24 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem24 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem25 {
        bits: u32,
    }
    impl Default for Dhtmem25 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem25 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem26 {
        bits: u32,
    }
    impl Default for Dhtmem26 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem26 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem27 {
        bits: u32,
    }
    impl Default for Dhtmem27 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem27 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem28 {
        bits: u32,
    }
    impl Default for Dhtmem28 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem28 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem29 {
        bits: u32,
    }
    impl Default for Dhtmem29 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem29 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem3 {
        bits: u32,
    }
    impl Default for Dhtmem3 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem3 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem30 {
        bits: u32,
    }
    impl Default for Dhtmem30 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem30 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem31 {
        bits: u32,
    }
    impl Default for Dhtmem31 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem31 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem32 {
        bits: u32,
    }
    impl Default for Dhtmem32 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem32 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem33 {
        bits: u32,
    }
    impl Default for Dhtmem33 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem33 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem34 {
        bits: u32,
    }
    impl Default for Dhtmem34 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem34 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem35 {
        bits: u32,
    }
    impl Default for Dhtmem35 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem35 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem36 {
        bits: u32,
    }
    impl Default for Dhtmem36 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem36 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem37 {
        bits: u32,
    }
    impl Default for Dhtmem37 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem37 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem38 {
        bits: u32,
    }
    impl Default for Dhtmem38 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem38 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem39 {
        bits: u32,
    }
    impl Default for Dhtmem39 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem39 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem4 {
        bits: u32,
    }
    impl Default for Dhtmem4 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem4 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem40 {
        bits: u32,
    }
    impl Default for Dhtmem40 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem40 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem41 {
        bits: u32,
    }
    impl Default for Dhtmem41 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem41 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem42 {
        bits: u32,
    }
    impl Default for Dhtmem42 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem42 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem43 {
        bits: u32,
    }
    impl Default for Dhtmem43 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem43 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem44 {
        bits: u32,
    }
    impl Default for Dhtmem44 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem44 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem45 {
        bits: u32,
    }
    impl Default for Dhtmem45 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem45 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem46 {
        bits: u32,
    }
    impl Default for Dhtmem46 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem46 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem47 {
        bits: u32,
    }
    impl Default for Dhtmem47 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem47 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem48 {
        bits: u32,
    }
    impl Default for Dhtmem48 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem48 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem49 {
        bits: u32,
    }
    impl Default for Dhtmem49 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem49 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem5 {
        bits: u32,
    }
    impl Default for Dhtmem5 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem5 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem50 {
        bits: u32,
    }
    impl Default for Dhtmem50 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem50 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem51 {
        bits: u32,
    }
    impl Default for Dhtmem51 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem51 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem52 {
        bits: u32,
    }
    impl Default for Dhtmem52 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem52 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem53 {
        bits: u32,
    }
    impl Default for Dhtmem53 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem53 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem54 {
        bits: u32,
    }
    impl Default for Dhtmem54 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem54 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem55 {
        bits: u32,
    }
    impl Default for Dhtmem55 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem55 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem56 {
        bits: u32,
    }
    impl Default for Dhtmem56 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem56 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem57 {
        bits: u32,
    }
    impl Default for Dhtmem57 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem57 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem58 {
        bits: u32,
    }
    impl Default for Dhtmem58 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem58 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem59 {
        bits: u32,
    }
    impl Default for Dhtmem59 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem59 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem6 {
        bits: u32,
    }
    impl Default for Dhtmem6 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem6 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem60 {
        bits: u32,
    }
    impl Default for Dhtmem60 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem60 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem61 {
        bits: u32,
    }
    impl Default for Dhtmem61 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem61 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem62 {
        bits: u32,
    }
    impl Default for Dhtmem62 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem62 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem63 {
        bits: u32,
    }
    impl Default for Dhtmem63 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem63 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem64 {
        bits: u32,
    }
    impl Default for Dhtmem64 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem64 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem65 {
        bits: u32,
    }
    impl Default for Dhtmem65 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem65 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem66 {
        bits: u32,
    }
    impl Default for Dhtmem66 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem66 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem67 {
        bits: u32,
    }
    impl Default for Dhtmem67 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem67 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem68 {
        bits: u32,
    }
    impl Default for Dhtmem68 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem68 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem69 {
        bits: u32,
    }
    impl Default for Dhtmem69 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem69 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem7 {
        bits: u32,
    }
    impl Default for Dhtmem7 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem7 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem70 {
        bits: u32,
    }
    impl Default for Dhtmem70 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem70 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem71 {
        bits: u32,
    }
    impl Default for Dhtmem71 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem71 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem72 {
        bits: u32,
    }
    impl Default for Dhtmem72 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem72 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem73 {
        bits: u32,
    }
    impl Default for Dhtmem73 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem73 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem74 {
        bits: u32,
    }
    impl Default for Dhtmem74 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem74 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem75 {
        bits: u32,
    }
    impl Default for Dhtmem75 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem75 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem76 {
        bits: u32,
    }
    impl Default for Dhtmem76 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem76 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem77 {
        bits: u32,
    }
    impl Default for Dhtmem77 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem77 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem78 {
        bits: u32,
    }
    impl Default for Dhtmem78 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem78 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem79 {
        bits: u32,
    }
    impl Default for Dhtmem79 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem79 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem8 {
        bits: u32,
    }
    impl Default for Dhtmem8 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem8 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem80 {
        bits: u32,
    }
    impl Default for Dhtmem80 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem80 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem81 {
        bits: u32,
    }
    impl Default for Dhtmem81 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem81 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem82 {
        bits: u32,
    }
    impl Default for Dhtmem82 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem82 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem83 {
        bits: u32,
    }
    impl Default for Dhtmem83 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem83 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem84 {
        bits: u32,
    }
    impl Default for Dhtmem84 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem84 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem85 {
        bits: u32,
    }
    impl Default for Dhtmem85 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem85 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem86 {
        bits: u32,
    }
    impl Default for Dhtmem86 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem86 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem87 {
        bits: u32,
    }
    impl Default for Dhtmem87 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem87 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem88 {
        bits: u32,
    }
    impl Default for Dhtmem88 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem88 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem89 {
        bits: u32,
    }
    impl Default for Dhtmem89 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem89 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem9 {
        bits: u32,
    }
    impl Default for Dhtmem9 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem9 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem90 {
        bits: u32,
    }
    impl Default for Dhtmem90 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem90 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem91 {
        bits: u32,
    }
    impl Default for Dhtmem91 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem91 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem92 {
        bits: u32,
    }
    impl Default for Dhtmem92 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem92 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem93 {
        bits: u32,
    }
    impl Default for Dhtmem93 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem93 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem94 {
        bits: u32,
    }
    impl Default for Dhtmem94 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem94 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem95 {
        bits: u32,
    }
    impl Default for Dhtmem95 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem95 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem96 {
        bits: u32,
    }
    impl Default for Dhtmem96 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem96 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem97 {
        bits: u32,
    }
    impl Default for Dhtmem97 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem97 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem98 {
        bits: u32,
    }
    impl Default for Dhtmem98 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem98 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG DHTMem tables"]
    pub struct Dhtmem99 {
        bits: u32,
    }
    impl Default for Dhtmem99 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Dhtmem99 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase0 {
        bits: u32,
    }
    impl Default for Huffbase0 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase0 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase1 {
        bits: u32,
    }
    impl Default for Huffbase1 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase1 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase10 {
        bits: u32,
    }
    impl Default for Huffbase10 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase10 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase11 {
        bits: u32,
    }
    impl Default for Huffbase11 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase11 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase12 {
        bits: u32,
    }
    impl Default for Huffbase12 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase12 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase13 {
        bits: u32,
    }
    impl Default for Huffbase13 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase13 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase14 {
        bits: u32,
    }
    impl Default for Huffbase14 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase14 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase15 {
        bits: u32,
    }
    impl Default for Huffbase15 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase15 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase16 {
        bits: u32,
    }
    impl Default for Huffbase16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase17 {
        bits: u32,
    }
    impl Default for Huffbase17 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase17 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase18 {
        bits: u32,
    }
    impl Default for Huffbase18 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase18 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase19 {
        bits: u32,
    }
    impl Default for Huffbase19 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase19 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase2 {
        bits: u32,
    }
    impl Default for Huffbase2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase20 {
        bits: u32,
    }
    impl Default for Huffbase20 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase20 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase21 {
        bits: u32,
    }
    impl Default for Huffbase21 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase21 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase22 {
        bits: u32,
    }
    impl Default for Huffbase22 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase22 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase23 {
        bits: u32,
    }
    impl Default for Huffbase23 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase23 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase24 {
        bits: u32,
    }
    impl Default for Huffbase24 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase24 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase25 {
        bits: u32,
    }
    impl Default for Huffbase25 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase25 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase26 {
        bits: u32,
    }
    impl Default for Huffbase26 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase26 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase27 {
        bits: u32,
    }
    impl Default for Huffbase27 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase27 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase28 {
        bits: u32,
    }
    impl Default for Huffbase28 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase28 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase29 {
        bits: u32,
    }
    impl Default for Huffbase29 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase29 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase3 {
        bits: u32,
    }
    impl Default for Huffbase3 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase3 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase30 {
        bits: u32,
    }
    impl Default for Huffbase30 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase30 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase31 {
        bits: u32,
    }
    impl Default for Huffbase31 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase31 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase4 {
        bits: u32,
    }
    impl Default for Huffbase4 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase4 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase5 {
        bits: u32,
    }
    impl Default for Huffbase5 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase5 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase6 {
        bits: u32,
    }
    impl Default for Huffbase6 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase6 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase7 {
        bits: u32,
    }
    impl Default for Huffbase7 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase7 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase8 {
        bits: u32,
    }
    impl Default for Huffbase8 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase8 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffSymb tables"]
    pub struct Huffbase9 {
        bits: u32,
    }
    impl Default for Huffbase9 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffbase9 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_0(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x0);
            self.bits |= (val as u32 & 0x1ff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_0(self) -> u16 {
            ((self.bits >> 0x0) & 0x1ff) as _
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn set_huff_base_ram_1(mut self, val: u16) -> Self {
            self.bits &= !(0x1ff << 0x10);
            self.bits |= (val as u32 & 0x1ff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "HuffBase RAM"]
        pub const fn huff_base_ram_1(self) -> u16 {
            ((self.bits >> 0x10) & 0x1ff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc00 {
        bits: u32,
    }
    impl Default for HuffencAc00 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc00 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc01 {
        bits: u32,
    }
    impl Default for HuffencAc01 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc01 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc010 {
        bits: u32,
    }
    impl Default for HuffencAc010 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc010 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc011 {
        bits: u32,
    }
    impl Default for HuffencAc011 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc011 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc012 {
        bits: u32,
    }
    impl Default for HuffencAc012 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc012 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc013 {
        bits: u32,
    }
    impl Default for HuffencAc013 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc013 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc014 {
        bits: u32,
    }
    impl Default for HuffencAc014 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc014 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc015 {
        bits: u32,
    }
    impl Default for HuffencAc015 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc015 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc016 {
        bits: u32,
    }
    impl Default for HuffencAc016 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc016 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc017 {
        bits: u32,
    }
    impl Default for HuffencAc017 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc017 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc018 {
        bits: u32,
    }
    impl Default for HuffencAc018 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc018 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc019 {
        bits: u32,
    }
    impl Default for HuffencAc019 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc019 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc02 {
        bits: u32,
    }
    impl Default for HuffencAc02 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc02 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc020 {
        bits: u32,
    }
    impl Default for HuffencAc020 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc020 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc021 {
        bits: u32,
    }
    impl Default for HuffencAc021 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc021 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc022 {
        bits: u32,
    }
    impl Default for HuffencAc022 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc022 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc023 {
        bits: u32,
    }
    impl Default for HuffencAc023 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc023 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc024 {
        bits: u32,
    }
    impl Default for HuffencAc024 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc024 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc025 {
        bits: u32,
    }
    impl Default for HuffencAc025 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc025 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc026 {
        bits: u32,
    }
    impl Default for HuffencAc026 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc026 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc027 {
        bits: u32,
    }
    impl Default for HuffencAc027 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc027 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc028 {
        bits: u32,
    }
    impl Default for HuffencAc028 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc028 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc029 {
        bits: u32,
    }
    impl Default for HuffencAc029 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc029 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc03 {
        bits: u32,
    }
    impl Default for HuffencAc03 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc03 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc030 {
        bits: u32,
    }
    impl Default for HuffencAc030 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc030 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc031 {
        bits: u32,
    }
    impl Default for HuffencAc031 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc031 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc032 {
        bits: u32,
    }
    impl Default for HuffencAc032 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc032 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc033 {
        bits: u32,
    }
    impl Default for HuffencAc033 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc033 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc034 {
        bits: u32,
    }
    impl Default for HuffencAc034 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc034 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc035 {
        bits: u32,
    }
    impl Default for HuffencAc035 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc035 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc036 {
        bits: u32,
    }
    impl Default for HuffencAc036 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc036 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc037 {
        bits: u32,
    }
    impl Default for HuffencAc037 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc037 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc038 {
        bits: u32,
    }
    impl Default for HuffencAc038 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc038 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc039 {
        bits: u32,
    }
    impl Default for HuffencAc039 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc039 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc04 {
        bits: u32,
    }
    impl Default for HuffencAc04 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc04 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc040 {
        bits: u32,
    }
    impl Default for HuffencAc040 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc040 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc041 {
        bits: u32,
    }
    impl Default for HuffencAc041 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc041 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc042 {
        bits: u32,
    }
    impl Default for HuffencAc042 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc042 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc043 {
        bits: u32,
    }
    impl Default for HuffencAc043 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc043 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc044 {
        bits: u32,
    }
    impl Default for HuffencAc044 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc044 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc045 {
        bits: u32,
    }
    impl Default for HuffencAc045 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc045 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc046 {
        bits: u32,
    }
    impl Default for HuffencAc046 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc046 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc047 {
        bits: u32,
    }
    impl Default for HuffencAc047 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc047 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc048 {
        bits: u32,
    }
    impl Default for HuffencAc048 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc048 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc049 {
        bits: u32,
    }
    impl Default for HuffencAc049 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc049 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc05 {
        bits: u32,
    }
    impl Default for HuffencAc05 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc05 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc050 {
        bits: u32,
    }
    impl Default for HuffencAc050 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc050 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc051 {
        bits: u32,
    }
    impl Default for HuffencAc051 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc051 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc052 {
        bits: u32,
    }
    impl Default for HuffencAc052 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc052 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc053 {
        bits: u32,
    }
    impl Default for HuffencAc053 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc053 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc054 {
        bits: u32,
    }
    impl Default for HuffencAc054 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc054 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc055 {
        bits: u32,
    }
    impl Default for HuffencAc055 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc055 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc056 {
        bits: u32,
    }
    impl Default for HuffencAc056 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc056 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc057 {
        bits: u32,
    }
    impl Default for HuffencAc057 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc057 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc058 {
        bits: u32,
    }
    impl Default for HuffencAc058 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc058 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc059 {
        bits: u32,
    }
    impl Default for HuffencAc059 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc059 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc06 {
        bits: u32,
    }
    impl Default for HuffencAc06 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc06 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc060 {
        bits: u32,
    }
    impl Default for HuffencAc060 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc060 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc061 {
        bits: u32,
    }
    impl Default for HuffencAc061 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc061 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc062 {
        bits: u32,
    }
    impl Default for HuffencAc062 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc062 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc063 {
        bits: u32,
    }
    impl Default for HuffencAc063 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc063 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc064 {
        bits: u32,
    }
    impl Default for HuffencAc064 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc064 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc065 {
        bits: u32,
    }
    impl Default for HuffencAc065 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc065 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc066 {
        bits: u32,
    }
    impl Default for HuffencAc066 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc066 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc067 {
        bits: u32,
    }
    impl Default for HuffencAc067 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc067 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc068 {
        bits: u32,
    }
    impl Default for HuffencAc068 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc068 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc069 {
        bits: u32,
    }
    impl Default for HuffencAc069 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc069 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc07 {
        bits: u32,
    }
    impl Default for HuffencAc07 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc07 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc070 {
        bits: u32,
    }
    impl Default for HuffencAc070 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc070 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc071 {
        bits: u32,
    }
    impl Default for HuffencAc071 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc071 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc072 {
        bits: u32,
    }
    impl Default for HuffencAc072 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc072 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc073 {
        bits: u32,
    }
    impl Default for HuffencAc073 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc073 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc074 {
        bits: u32,
    }
    impl Default for HuffencAc074 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc074 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc075 {
        bits: u32,
    }
    impl Default for HuffencAc075 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc075 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc076 {
        bits: u32,
    }
    impl Default for HuffencAc076 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc076 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc077 {
        bits: u32,
    }
    impl Default for HuffencAc077 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc077 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc078 {
        bits: u32,
    }
    impl Default for HuffencAc078 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc078 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc079 {
        bits: u32,
    }
    impl Default for HuffencAc079 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc079 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc08 {
        bits: u32,
    }
    impl Default for HuffencAc08 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc08 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc080 {
        bits: u32,
    }
    impl Default for HuffencAc080 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc080 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc081 {
        bits: u32,
    }
    impl Default for HuffencAc081 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc081 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc082 {
        bits: u32,
    }
    impl Default for HuffencAc082 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc082 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc083 {
        bits: u32,
    }
    impl Default for HuffencAc083 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc083 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc084 {
        bits: u32,
    }
    impl Default for HuffencAc084 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc084 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc085 {
        bits: u32,
    }
    impl Default for HuffencAc085 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc085 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc086 {
        bits: u32,
    }
    impl Default for HuffencAc086 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc086 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc087 {
        bits: u32,
    }
    impl Default for HuffencAc087 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc087 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub struct HuffencAc09 {
        bits: u32,
    }
    impl Default for HuffencAc09 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc09 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc10 {
        bits: u32,
    }
    impl Default for HuffencAc10 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc10 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc11 {
        bits: u32,
    }
    impl Default for HuffencAc11 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc11 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc110 {
        bits: u32,
    }
    impl Default for HuffencAc110 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc110 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc111 {
        bits: u32,
    }
    impl Default for HuffencAc111 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc111 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc112 {
        bits: u32,
    }
    impl Default for HuffencAc112 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc112 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc113 {
        bits: u32,
    }
    impl Default for HuffencAc113 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc113 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc114 {
        bits: u32,
    }
    impl Default for HuffencAc114 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc114 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc115 {
        bits: u32,
    }
    impl Default for HuffencAc115 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc115 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc116 {
        bits: u32,
    }
    impl Default for HuffencAc116 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc116 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc117 {
        bits: u32,
    }
    impl Default for HuffencAc117 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc117 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc118 {
        bits: u32,
    }
    impl Default for HuffencAc118 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc118 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc119 {
        bits: u32,
    }
    impl Default for HuffencAc119 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc119 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc12 {
        bits: u32,
    }
    impl Default for HuffencAc12 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc12 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc120 {
        bits: u32,
    }
    impl Default for HuffencAc120 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc120 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc121 {
        bits: u32,
    }
    impl Default for HuffencAc121 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc121 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc122 {
        bits: u32,
    }
    impl Default for HuffencAc122 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc122 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc123 {
        bits: u32,
    }
    impl Default for HuffencAc123 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc123 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc124 {
        bits: u32,
    }
    impl Default for HuffencAc124 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc124 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc125 {
        bits: u32,
    }
    impl Default for HuffencAc125 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc125 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc126 {
        bits: u32,
    }
    impl Default for HuffencAc126 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc126 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc127 {
        bits: u32,
    }
    impl Default for HuffencAc127 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc127 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc128 {
        bits: u32,
    }
    impl Default for HuffencAc128 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc128 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc129 {
        bits: u32,
    }
    impl Default for HuffencAc129 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc129 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc13 {
        bits: u32,
    }
    impl Default for HuffencAc13 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc13 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc130 {
        bits: u32,
    }
    impl Default for HuffencAc130 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc130 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc131 {
        bits: u32,
    }
    impl Default for HuffencAc131 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc131 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc132 {
        bits: u32,
    }
    impl Default for HuffencAc132 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc132 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc133 {
        bits: u32,
    }
    impl Default for HuffencAc133 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc133 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc134 {
        bits: u32,
    }
    impl Default for HuffencAc134 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc134 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc135 {
        bits: u32,
    }
    impl Default for HuffencAc135 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc135 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc136 {
        bits: u32,
    }
    impl Default for HuffencAc136 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc136 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc137 {
        bits: u32,
    }
    impl Default for HuffencAc137 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc137 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc138 {
        bits: u32,
    }
    impl Default for HuffencAc138 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc138 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc139 {
        bits: u32,
    }
    impl Default for HuffencAc139 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc139 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc14 {
        bits: u32,
    }
    impl Default for HuffencAc14 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc14 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc140 {
        bits: u32,
    }
    impl Default for HuffencAc140 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc140 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc141 {
        bits: u32,
    }
    impl Default for HuffencAc141 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc141 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc142 {
        bits: u32,
    }
    impl Default for HuffencAc142 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc142 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc143 {
        bits: u32,
    }
    impl Default for HuffencAc143 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc143 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc144 {
        bits: u32,
    }
    impl Default for HuffencAc144 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc144 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc145 {
        bits: u32,
    }
    impl Default for HuffencAc145 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc145 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc146 {
        bits: u32,
    }
    impl Default for HuffencAc146 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc146 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc147 {
        bits: u32,
    }
    impl Default for HuffencAc147 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc147 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc148 {
        bits: u32,
    }
    impl Default for HuffencAc148 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc148 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc149 {
        bits: u32,
    }
    impl Default for HuffencAc149 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc149 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc15 {
        bits: u32,
    }
    impl Default for HuffencAc15 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc15 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc150 {
        bits: u32,
    }
    impl Default for HuffencAc150 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc150 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc151 {
        bits: u32,
    }
    impl Default for HuffencAc151 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc151 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc152 {
        bits: u32,
    }
    impl Default for HuffencAc152 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc152 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc153 {
        bits: u32,
    }
    impl Default for HuffencAc153 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc153 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc154 {
        bits: u32,
    }
    impl Default for HuffencAc154 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc154 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc155 {
        bits: u32,
    }
    impl Default for HuffencAc155 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc155 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc156 {
        bits: u32,
    }
    impl Default for HuffencAc156 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc156 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc157 {
        bits: u32,
    }
    impl Default for HuffencAc157 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc157 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc158 {
        bits: u32,
    }
    impl Default for HuffencAc158 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc158 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc159 {
        bits: u32,
    }
    impl Default for HuffencAc159 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc159 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc16 {
        bits: u32,
    }
    impl Default for HuffencAc16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc160 {
        bits: u32,
    }
    impl Default for HuffencAc160 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc160 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc161 {
        bits: u32,
    }
    impl Default for HuffencAc161 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc161 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc162 {
        bits: u32,
    }
    impl Default for HuffencAc162 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc162 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc163 {
        bits: u32,
    }
    impl Default for HuffencAc163 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc163 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc164 {
        bits: u32,
    }
    impl Default for HuffencAc164 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc164 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc165 {
        bits: u32,
    }
    impl Default for HuffencAc165 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc165 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc166 {
        bits: u32,
    }
    impl Default for HuffencAc166 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc166 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc167 {
        bits: u32,
    }
    impl Default for HuffencAc167 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc167 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc168 {
        bits: u32,
    }
    impl Default for HuffencAc168 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc168 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc169 {
        bits: u32,
    }
    impl Default for HuffencAc169 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc169 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc17 {
        bits: u32,
    }
    impl Default for HuffencAc17 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc17 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc170 {
        bits: u32,
    }
    impl Default for HuffencAc170 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc170 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc171 {
        bits: u32,
    }
    impl Default for HuffencAc171 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc171 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc172 {
        bits: u32,
    }
    impl Default for HuffencAc172 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc172 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc173 {
        bits: u32,
    }
    impl Default for HuffencAc173 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc173 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc174 {
        bits: u32,
    }
    impl Default for HuffencAc174 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc174 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc175 {
        bits: u32,
    }
    impl Default for HuffencAc175 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc175 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc176 {
        bits: u32,
    }
    impl Default for HuffencAc176 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc176 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc177 {
        bits: u32,
    }
    impl Default for HuffencAc177 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc177 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc178 {
        bits: u32,
    }
    impl Default for HuffencAc178 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc178 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc179 {
        bits: u32,
    }
    impl Default for HuffencAc179 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc179 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc18 {
        bits: u32,
    }
    impl Default for HuffencAc18 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc18 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc180 {
        bits: u32,
    }
    impl Default for HuffencAc180 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc180 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc181 {
        bits: u32,
    }
    impl Default for HuffencAc181 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc181 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc182 {
        bits: u32,
    }
    impl Default for HuffencAc182 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc182 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc183 {
        bits: u32,
    }
    impl Default for HuffencAc183 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc183 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc184 {
        bits: u32,
    }
    impl Default for HuffencAc184 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc184 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc185 {
        bits: u32,
    }
    impl Default for HuffencAc185 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc185 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc186 {
        bits: u32,
    }
    impl Default for HuffencAc186 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc186 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc187 {
        bits: u32,
    }
    impl Default for HuffencAc187 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc187 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub struct HuffencAc19 {
        bits: u32,
    }
    impl Default for HuffencAc19 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencAc19 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub struct HuffencDc00 {
        bits: u32,
    }
    impl Default for HuffencDc00 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc00 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub struct HuffencDc01 {
        bits: u32,
    }
    impl Default for HuffencDc01 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc01 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub struct HuffencDc02 {
        bits: u32,
    }
    impl Default for HuffencDc02 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc02 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub struct HuffencDc03 {
        bits: u32,
    }
    impl Default for HuffencDc03 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc03 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub struct HuffencDc04 {
        bits: u32,
    }
    impl Default for HuffencDc04 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc04 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub struct HuffencDc05 {
        bits: u32,
    }
    impl Default for HuffencDc05 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc05 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub struct HuffencDc06 {
        bits: u32,
    }
    impl Default for HuffencDc06 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc06 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub struct HuffencDc07 {
        bits: u32,
    }
    impl Default for HuffencDc07 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc07 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub struct HuffencDc10 {
        bits: u32,
    }
    impl Default for HuffencDc10 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc10 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub struct HuffencDc11 {
        bits: u32,
    }
    impl Default for HuffencDc11 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc11 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub struct HuffencDc12 {
        bits: u32,
    }
    impl Default for HuffencDc12 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc12 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub struct HuffencDc13 {
        bits: u32,
    }
    impl Default for HuffencDc13 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc13 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub struct HuffencDc14 {
        bits: u32,
    }
    impl Default for HuffencDc14 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc14 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub struct HuffencDc15 {
        bits: u32,
    }
    impl Default for HuffencDc15 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc15 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub struct HuffencDc16 {
        bits: u32,
    }
    impl Default for HuffencDc16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub struct HuffencDc17 {
        bits: u32,
    }
    impl Default for HuffencDc17 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl HuffencDc17 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTMem RAM"]
        pub const fn dht_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin0 {
        bits: u32,
    }
    impl Default for Huffmin0 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin0 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin1 {
        bits: u32,
    }
    impl Default for Huffmin1 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin1 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin10 {
        bits: u32,
    }
    impl Default for Huffmin10 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin10 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin11 {
        bits: u32,
    }
    impl Default for Huffmin11 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin11 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin12 {
        bits: u32,
    }
    impl Default for Huffmin12 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin12 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin13 {
        bits: u32,
    }
    impl Default for Huffmin13 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin13 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin14 {
        bits: u32,
    }
    impl Default for Huffmin14 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin14 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin15 {
        bits: u32,
    }
    impl Default for Huffmin15 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin15 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin2 {
        bits: u32,
    }
    impl Default for Huffmin2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin3 {
        bits: u32,
    }
    impl Default for Huffmin3 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin3 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin4 {
        bits: u32,
    }
    impl Default for Huffmin4 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin4 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin5 {
        bits: u32,
    }
    impl Default for Huffmin5 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin5 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin6 {
        bits: u32,
    }
    impl Default for Huffmin6 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin6 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin7 {
        bits: u32,
    }
    impl Default for Huffmin7 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin7 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin8 {
        bits: u32,
    }
    impl Default for Huffmin8 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin8 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HuffMin tables"]
    pub struct Huffmin9 {
        bits: u32,
    }
    impl Default for Huffmin9 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffmin9 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn set_huff_min_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "HuffMin RAM"]
        pub const fn huff_min_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb0 {
        bits: u32,
    }
    impl Default for Huffsymb0 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb0 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb1 {
        bits: u32,
    }
    impl Default for Huffsymb1 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb1 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb10 {
        bits: u32,
    }
    impl Default for Huffsymb10 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb10 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb11 {
        bits: u32,
    }
    impl Default for Huffsymb11 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb11 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb12 {
        bits: u32,
    }
    impl Default for Huffsymb12 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb12 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb13 {
        bits: u32,
    }
    impl Default for Huffsymb13 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb13 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb14 {
        bits: u32,
    }
    impl Default for Huffsymb14 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb14 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb15 {
        bits: u32,
    }
    impl Default for Huffsymb15 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb15 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb16 {
        bits: u32,
    }
    impl Default for Huffsymb16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb17 {
        bits: u32,
    }
    impl Default for Huffsymb17 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb17 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb18 {
        bits: u32,
    }
    impl Default for Huffsymb18 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb18 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb19 {
        bits: u32,
    }
    impl Default for Huffsymb19 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb19 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb2 {
        bits: u32,
    }
    impl Default for Huffsymb2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb20 {
        bits: u32,
    }
    impl Default for Huffsymb20 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb20 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb21 {
        bits: u32,
    }
    impl Default for Huffsymb21 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb21 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb22 {
        bits: u32,
    }
    impl Default for Huffsymb22 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb22 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb23 {
        bits: u32,
    }
    impl Default for Huffsymb23 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb23 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb24 {
        bits: u32,
    }
    impl Default for Huffsymb24 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb24 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb25 {
        bits: u32,
    }
    impl Default for Huffsymb25 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb25 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb26 {
        bits: u32,
    }
    impl Default for Huffsymb26 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb26 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb27 {
        bits: u32,
    }
    impl Default for Huffsymb27 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb27 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb28 {
        bits: u32,
    }
    impl Default for Huffsymb28 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb28 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb29 {
        bits: u32,
    }
    impl Default for Huffsymb29 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb29 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb3 {
        bits: u32,
    }
    impl Default for Huffsymb3 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb3 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb30 {
        bits: u32,
    }
    impl Default for Huffsymb30 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb30 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb31 {
        bits: u32,
    }
    impl Default for Huffsymb31 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb31 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb32 {
        bits: u32,
    }
    impl Default for Huffsymb32 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb32 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb33 {
        bits: u32,
    }
    impl Default for Huffsymb33 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb33 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb34 {
        bits: u32,
    }
    impl Default for Huffsymb34 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb34 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb35 {
        bits: u32,
    }
    impl Default for Huffsymb35 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb35 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb36 {
        bits: u32,
    }
    impl Default for Huffsymb36 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb36 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb37 {
        bits: u32,
    }
    impl Default for Huffsymb37 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb37 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb38 {
        bits: u32,
    }
    impl Default for Huffsymb38 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb38 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb39 {
        bits: u32,
    }
    impl Default for Huffsymb39 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb39 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb4 {
        bits: u32,
    }
    impl Default for Huffsymb4 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb4 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb40 {
        bits: u32,
    }
    impl Default for Huffsymb40 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb40 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb41 {
        bits: u32,
    }
    impl Default for Huffsymb41 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb41 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb42 {
        bits: u32,
    }
    impl Default for Huffsymb42 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb42 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb43 {
        bits: u32,
    }
    impl Default for Huffsymb43 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb43 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb44 {
        bits: u32,
    }
    impl Default for Huffsymb44 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb44 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb45 {
        bits: u32,
    }
    impl Default for Huffsymb45 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb45 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb46 {
        bits: u32,
    }
    impl Default for Huffsymb46 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb46 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb47 {
        bits: u32,
    }
    impl Default for Huffsymb47 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb47 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb48 {
        bits: u32,
    }
    impl Default for Huffsymb48 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb48 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb49 {
        bits: u32,
    }
    impl Default for Huffsymb49 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb49 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb5 {
        bits: u32,
    }
    impl Default for Huffsymb5 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb5 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb50 {
        bits: u32,
    }
    impl Default for Huffsymb50 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb50 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb51 {
        bits: u32,
    }
    impl Default for Huffsymb51 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb51 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb52 {
        bits: u32,
    }
    impl Default for Huffsymb52 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb52 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb53 {
        bits: u32,
    }
    impl Default for Huffsymb53 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb53 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb54 {
        bits: u32,
    }
    impl Default for Huffsymb54 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb54 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb55 {
        bits: u32,
    }
    impl Default for Huffsymb55 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb55 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb56 {
        bits: u32,
    }
    impl Default for Huffsymb56 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb56 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb57 {
        bits: u32,
    }
    impl Default for Huffsymb57 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb57 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb58 {
        bits: u32,
    }
    impl Default for Huffsymb58 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb58 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb59 {
        bits: u32,
    }
    impl Default for Huffsymb59 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb59 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb6 {
        bits: u32,
    }
    impl Default for Huffsymb6 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb6 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb60 {
        bits: u32,
    }
    impl Default for Huffsymb60 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb60 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb61 {
        bits: u32,
    }
    impl Default for Huffsymb61 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb61 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb62 {
        bits: u32,
    }
    impl Default for Huffsymb62 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb62 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb63 {
        bits: u32,
    }
    impl Default for Huffsymb63 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb63 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb64 {
        bits: u32,
    }
    impl Default for Huffsymb64 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb64 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb65 {
        bits: u32,
    }
    impl Default for Huffsymb65 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb65 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb66 {
        bits: u32,
    }
    impl Default for Huffsymb66 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb66 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb67 {
        bits: u32,
    }
    impl Default for Huffsymb67 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb67 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb68 {
        bits: u32,
    }
    impl Default for Huffsymb68 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb68 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb69 {
        bits: u32,
    }
    impl Default for Huffsymb69 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb69 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb7 {
        bits: u32,
    }
    impl Default for Huffsymb7 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb7 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb70 {
        bits: u32,
    }
    impl Default for Huffsymb70 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb70 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb71 {
        bits: u32,
    }
    impl Default for Huffsymb71 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb71 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb72 {
        bits: u32,
    }
    impl Default for Huffsymb72 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb72 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb73 {
        bits: u32,
    }
    impl Default for Huffsymb73 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb73 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb74 {
        bits: u32,
    }
    impl Default for Huffsymb74 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb74 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb75 {
        bits: u32,
    }
    impl Default for Huffsymb75 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb75 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb76 {
        bits: u32,
    }
    impl Default for Huffsymb76 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb76 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb77 {
        bits: u32,
    }
    impl Default for Huffsymb77 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb77 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb78 {
        bits: u32,
    }
    impl Default for Huffsymb78 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb78 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb79 {
        bits: u32,
    }
    impl Default for Huffsymb79 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb79 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb8 {
        bits: u32,
    }
    impl Default for Huffsymb8 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb8 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb80 {
        bits: u32,
    }
    impl Default for Huffsymb80 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb80 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb81 {
        bits: u32,
    }
    impl Default for Huffsymb81 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb81 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb82 {
        bits: u32,
    }
    impl Default for Huffsymb82 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb82 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb83 {
        bits: u32,
    }
    impl Default for Huffsymb83 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb83 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub struct Huffsymb9 {
        bits: u32,
    }
    impl Default for Huffsymb9 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Huffsymb9 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn set_huff_symb_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "DHTSymb RAM"]
        pub const fn huff_symb_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG clear flag register"]
    pub struct JpegCfr {
        bits: u32,
    }
    impl Default for JpegCfr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegCfr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Clear End of Conversion Flag"]
        pub const fn set_ceocf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Clear End of Conversion Flag"]
        pub const fn ceocf(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Clear Header Parsing Done Flag"]
        pub const fn set_chpdf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Clear Header Parsing Done Flag"]
        pub const fn chpdf(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG codec configuration register 0"]
    pub struct JpegConfr0 {
        bits: u32,
    }
    impl Default for JpegConfr0 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegConfr0 {
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
        pub const fn set_start(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Start"]
        pub const fn start(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG codec configuration register 1"]
    pub struct JpegConfr1 {
        bits: u32,
    }
    impl Default for JpegConfr1 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegConfr1 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Number of color components"]
        pub const fn set_nf(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x0);
            self.bits |= (val as u32 & 0x3) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Number of color components"]
        pub const fn nf(self) -> u8 {
            ((self.bits >> 0x0) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Decoding Enable"]
        pub const fn set_de(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Decoding Enable"]
        pub const fn de(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Color Space"]
        pub const fn set_colorspace(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x4);
            self.bits |= (val as u32 & 0x3) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Color Space"]
        pub const fn colorspace(self) -> u8 {
            ((self.bits >> 0x4) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Number of components for Scan"]
        pub const fn set_ns(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x6);
            self.bits |= (val as u32 & 0x3) << 0x6;
            self
        }
        #[inline(always)]
        #[doc = "Number of components for Scan"]
        pub const fn ns(self) -> u8 {
            ((self.bits >> 0x6) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Header Processing"]
        pub const fn set_hdr(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x8);
            self.bits |= if val { 1 << 0x8 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Header Processing"]
        pub const fn hdr(self) -> bool {
            ((self.bits >> 0x8) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Y Size"]
        pub const fn set_ysize(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x10);
            self.bits |= (val as u32 & 0xffff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "Y Size"]
        pub const fn ysize(self) -> u16 {
            ((self.bits >> 0x10) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG codec configuration register 2"]
    pub struct JpegConfr2 {
        bits: u32,
    }
    impl Default for JpegConfr2 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegConfr2 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Number of MCU"]
        pub const fn set_nmcu(mut self, val: u32) -> Self {
            self.bits &= !(0x3ffffff << 0x0);
            self.bits |= (val as u32 & 0x3ffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Number of MCU"]
        pub const fn nmcu(self) -> u32 {
            ((self.bits >> 0x0) & 0x3ffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG codec configuration register 3"]
    pub struct JpegConfr3 {
        bits: u32,
    }
    impl Default for JpegConfr3 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegConfr3 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "X size"]
        pub const fn set_xsize(mut self, val: u16) -> Self {
            self.bits &= !(0xffff << 0x10);
            self.bits |= (val as u32 & 0xffff) << 0x10;
            self
        }
        #[inline(always)]
        #[doc = "X size"]
        pub const fn xsize(self) -> u16 {
            ((self.bits >> 0x10) & 0xffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG codec configuration register 4"]
    pub struct JpegConfr4 {
        bits: u32,
    }
    impl Default for JpegConfr4 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegConfr4 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Huffman DC"]
        pub const fn set_hd(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Huffman DC"]
        pub const fn hd(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Huffman AC"]
        pub const fn set_ha(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Huffman AC"]
        pub const fn ha(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Quantization Table"]
        pub const fn set_qt(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x2);
            self.bits |= (val as u32 & 0x3) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Quantization Table"]
        pub const fn qt(self) -> u8 {
            ((self.bits >> 0x2) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Number of Block"]
        pub const fn set_nb(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x4);
            self.bits |= (val as u32 & 0xf) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Number of Block"]
        pub const fn nb(self) -> u8 {
            ((self.bits >> 0x4) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Vertical Sampling Factor"]
        pub const fn set_vsf(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x8);
            self.bits |= (val as u32 & 0xf) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Vertical Sampling Factor"]
        pub const fn vsf(self) -> u8 {
            ((self.bits >> 0x8) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Horizontal Sampling Factor"]
        pub const fn set_hsf(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0xc);
            self.bits |= (val as u32 & 0xf) << 0xc;
            self
        }
        #[inline(always)]
        #[doc = "Horizontal Sampling Factor"]
        pub const fn hsf(self) -> u8 {
            ((self.bits >> 0xc) & 0xf) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG codec configuration register 5"]
    pub struct JpegConfr5 {
        bits: u32,
    }
    impl Default for JpegConfr5 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegConfr5 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Huffman DC"]
        pub const fn set_hd(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Huffman DC"]
        pub const fn hd(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Huffman AC"]
        pub const fn set_ha(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Huffman AC"]
        pub const fn ha(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Quantization Table"]
        pub const fn set_qt(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x2);
            self.bits |= (val as u32 & 0x3) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Quantization Table"]
        pub const fn qt(self) -> u8 {
            ((self.bits >> 0x2) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Number of Block"]
        pub const fn set_nb(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x4);
            self.bits |= (val as u32 & 0xf) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Number of Block"]
        pub const fn nb(self) -> u8 {
            ((self.bits >> 0x4) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Vertical Sampling Factor"]
        pub const fn set_vsf(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x8);
            self.bits |= (val as u32 & 0xf) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Vertical Sampling Factor"]
        pub const fn vsf(self) -> u8 {
            ((self.bits >> 0x8) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Horizontal Sampling Factor"]
        pub const fn set_hsf(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0xc);
            self.bits |= (val as u32 & 0xf) << 0xc;
            self
        }
        #[inline(always)]
        #[doc = "Horizontal Sampling Factor"]
        pub const fn hsf(self) -> u8 {
            ((self.bits >> 0xc) & 0xf) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG codec configuration register 6"]
    pub struct JpegConfr6 {
        bits: u32,
    }
    impl Default for JpegConfr6 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegConfr6 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Huffman DC"]
        pub const fn set_hd(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Huffman DC"]
        pub const fn hd(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Huffman AC"]
        pub const fn set_ha(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Huffman AC"]
        pub const fn ha(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Quantization Table"]
        pub const fn set_qt(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x2);
            self.bits |= (val as u32 & 0x3) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Quantization Table"]
        pub const fn qt(self) -> u8 {
            ((self.bits >> 0x2) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Number of Block"]
        pub const fn set_nb(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x4);
            self.bits |= (val as u32 & 0xf) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Number of Block"]
        pub const fn nb(self) -> u8 {
            ((self.bits >> 0x4) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Vertical Sampling Factor"]
        pub const fn set_vsf(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x8);
            self.bits |= (val as u32 & 0xf) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Vertical Sampling Factor"]
        pub const fn vsf(self) -> u8 {
            ((self.bits >> 0x8) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Horizontal Sampling Factor"]
        pub const fn set_hsf(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0xc);
            self.bits |= (val as u32 & 0xf) << 0xc;
            self
        }
        #[inline(always)]
        #[doc = "Horizontal Sampling Factor"]
        pub const fn hsf(self) -> u8 {
            ((self.bits >> 0xc) & 0xf) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG codec configuration register 7"]
    pub struct JpegConfr7 {
        bits: u32,
    }
    impl Default for JpegConfr7 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegConfr7 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Huffman DC"]
        pub const fn set_hd(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Huffman DC"]
        pub const fn hd(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Huffman AC"]
        pub const fn set_ha(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Huffman AC"]
        pub const fn ha(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Quantization Table"]
        pub const fn set_qt(mut self, val: u8) -> Self {
            self.bits &= !(0x3 << 0x2);
            self.bits |= (val as u32 & 0x3) << 0x2;
            self
        }
        #[inline(always)]
        #[doc = "Quantization Table"]
        pub const fn qt(self) -> u8 {
            ((self.bits >> 0x2) & 0x3) as _
        }
        #[inline(always)]
        #[doc = "Number of Block"]
        pub const fn set_nb(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x4);
            self.bits |= (val as u32 & 0xf) << 0x4;
            self
        }
        #[inline(always)]
        #[doc = "Number of Block"]
        pub const fn nb(self) -> u8 {
            ((self.bits >> 0x4) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Vertical Sampling Factor"]
        pub const fn set_vsf(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0x8);
            self.bits |= (val as u32 & 0xf) << 0x8;
            self
        }
        #[inline(always)]
        #[doc = "Vertical Sampling Factor"]
        pub const fn vsf(self) -> u8 {
            ((self.bits >> 0x8) & 0xf) as _
        }
        #[inline(always)]
        #[doc = "Horizontal Sampling Factor"]
        pub const fn set_hsf(mut self, val: u8) -> Self {
            self.bits &= !(0xf << 0xc);
            self.bits |= (val as u32 & 0xf) << 0xc;
            self
        }
        #[inline(always)]
        #[doc = "Horizontal Sampling Factor"]
        pub const fn hsf(self) -> u8 {
            ((self.bits >> 0xc) & 0xf) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG control register"]
    pub struct JpegCr {
        bits: u32,
    }
    impl Default for JpegCr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegCr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "JPEG Core Enable"]
        pub const fn set_jcen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x0);
            self.bits |= if val { 1 << 0x0 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "JPEG Core Enable"]
        pub const fn jcen(self) -> bool {
            ((self.bits >> 0x0) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Input FIFO Threshold Interrupt Enable"]
        pub const fn set_iftie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input FIFO Threshold Interrupt Enable"]
        pub const fn iftie(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Input FIFO Not Full Interrupt Enable"]
        pub const fn set_ifnfie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input FIFO Not Full Interrupt Enable"]
        pub const fn ifnfie(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output FIFO Threshold Interrupt Enable"]
        pub const fn set_oftie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output FIFO Threshold Interrupt Enable"]
        pub const fn oftie(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output FIFO Not Empty Interrupt Enable"]
        pub const fn set_ofneie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output FIFO Not Empty Interrupt Enable"]
        pub const fn ofneie(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "End of Conversion Interrupt Enable"]
        pub const fn set_eocie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "End of Conversion Interrupt Enable"]
        pub const fn eocie(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Header Parsing Done Interrupt Enable"]
        pub const fn set_hpdie(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Header Parsing Done Interrupt Enable"]
        pub const fn hpdie(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Input DMA Enable"]
        pub const fn set_idmaen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xb);
            self.bits |= if val { 1 << 0xb } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input DMA Enable"]
        pub const fn idmaen(self) -> bool {
            ((self.bits >> 0xb) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output DMA Enable"]
        pub const fn set_odmaen(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xc);
            self.bits |= if val { 1 << 0xc } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output DMA Enable"]
        pub const fn odmaen(self) -> bool {
            ((self.bits >> 0xc) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Input FIFO Flush"]
        pub const fn set_iff(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xd);
            self.bits |= if val { 1 << 0xd } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input FIFO Flush"]
        pub const fn iff(self) -> bool {
            ((self.bits >> 0xd) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output FIFO Flush"]
        pub const fn set_off(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0xe);
            self.bits |= if val { 1 << 0xe } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output FIFO Flush"]
        pub const fn off(self) -> bool {
            ((self.bits >> 0xe) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG data input register"]
    pub struct JpegDir {
        bits: u32,
    }
    impl Default for JpegDir {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegDir {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Data Input FIFO"]
        pub const fn set_datain(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Data Input FIFO"]
        pub const fn datain(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG data output register"]
    pub struct JpegDor {
        bits: u32,
    }
    impl Default for JpegDor {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegDor {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Data Output FIFO"]
        pub const fn set_dataout(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "Data Output FIFO"]
        pub const fn dataout(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG status register"]
    pub struct JpegSr {
        bits: u32,
    }
    impl Default for JpegSr {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl JpegSr {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "Input FIFO Threshold Flag"]
        pub const fn set_iftf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x1);
            self.bits |= if val { 1 << 0x1 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input FIFO Threshold Flag"]
        pub const fn iftf(self) -> bool {
            ((self.bits >> 0x1) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Input FIFO Not Full Flag"]
        pub const fn set_ifnff(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x2);
            self.bits |= if val { 1 << 0x2 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Input FIFO Not Full Flag"]
        pub const fn ifnff(self) -> bool {
            ((self.bits >> 0x2) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output FIFO Threshold Flag"]
        pub const fn set_oftf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x3);
            self.bits |= if val { 1 << 0x3 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output FIFO Threshold Flag"]
        pub const fn oftf(self) -> bool {
            ((self.bits >> 0x3) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Output FIFO Not Empty Flag"]
        pub const fn set_ofnef(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x4);
            self.bits |= if val { 1 << 0x4 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Output FIFO Not Empty Flag"]
        pub const fn ofnef(self) -> bool {
            ((self.bits >> 0x4) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "End of Conversion Flag"]
        pub const fn set_eocf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x5);
            self.bits |= if val { 1 << 0x5 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "End of Conversion Flag"]
        pub const fn eocf(self) -> bool {
            ((self.bits >> 0x5) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Header Parsing Done Flag"]
        pub const fn set_hpdf(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x6);
            self.bits |= if val { 1 << 0x6 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Header Parsing Done Flag"]
        pub const fn hpdf(self) -> bool {
            ((self.bits >> 0x6) & 0x1) != 0
        }
        #[inline(always)]
        #[doc = "Codec Operation Flag"]
        pub const fn set_cof(mut self, val: bool) -> Self {
            self.bits &= !(0x1 << 0x7);
            self.bits |= if val { 1 << 0x7 } else { 0 };
            self
        }
        #[inline(always)]
        #[doc = "Codec Operation Flag"]
        pub const fn cof(self) -> bool {
            ((self.bits >> 0x7) & 0x1) != 0
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem00 {
        bits: u32,
    }
    impl Default for Qmem00 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem00 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem01 {
        bits: u32,
    }
    impl Default for Qmem01 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem01 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem010 {
        bits: u32,
    }
    impl Default for Qmem010 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem010 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem011 {
        bits: u32,
    }
    impl Default for Qmem011 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem011 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem012 {
        bits: u32,
    }
    impl Default for Qmem012 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem012 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem013 {
        bits: u32,
    }
    impl Default for Qmem013 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem013 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem014 {
        bits: u32,
    }
    impl Default for Qmem014 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem014 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem015 {
        bits: u32,
    }
    impl Default for Qmem015 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem015 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem02 {
        bits: u32,
    }
    impl Default for Qmem02 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem02 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem03 {
        bits: u32,
    }
    impl Default for Qmem03 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem03 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem04 {
        bits: u32,
    }
    impl Default for Qmem04 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem04 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem05 {
        bits: u32,
    }
    impl Default for Qmem05 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem05 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem06 {
        bits: u32,
    }
    impl Default for Qmem06 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem06 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem07 {
        bits: u32,
    }
    impl Default for Qmem07 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem07 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem08 {
        bits: u32,
    }
    impl Default for Qmem08 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem08 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem09 {
        bits: u32,
    }
    impl Default for Qmem09 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem09 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem10 {
        bits: u32,
    }
    impl Default for Qmem10 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem10 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem11 {
        bits: u32,
    }
    impl Default for Qmem11 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem11 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem110 {
        bits: u32,
    }
    impl Default for Qmem110 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem110 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem111 {
        bits: u32,
    }
    impl Default for Qmem111 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem111 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem112 {
        bits: u32,
    }
    impl Default for Qmem112 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem112 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem113 {
        bits: u32,
    }
    impl Default for Qmem113 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem113 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem114 {
        bits: u32,
    }
    impl Default for Qmem114 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem114 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem115 {
        bits: u32,
    }
    impl Default for Qmem115 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem115 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem12 {
        bits: u32,
    }
    impl Default for Qmem12 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem12 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem13 {
        bits: u32,
    }
    impl Default for Qmem13 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem13 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem14 {
        bits: u32,
    }
    impl Default for Qmem14 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem14 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem15 {
        bits: u32,
    }
    impl Default for Qmem15 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem15 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem16 {
        bits: u32,
    }
    impl Default for Qmem16 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem16 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem17 {
        bits: u32,
    }
    impl Default for Qmem17 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem17 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem18 {
        bits: u32,
    }
    impl Default for Qmem18 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem18 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem19 {
        bits: u32,
    }
    impl Default for Qmem19 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem19 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem20 {
        bits: u32,
    }
    impl Default for Qmem20 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem20 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem21 {
        bits: u32,
    }
    impl Default for Qmem21 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem21 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem210 {
        bits: u32,
    }
    impl Default for Qmem210 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem210 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem211 {
        bits: u32,
    }
    impl Default for Qmem211 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem211 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem212 {
        bits: u32,
    }
    impl Default for Qmem212 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem212 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem213 {
        bits: u32,
    }
    impl Default for Qmem213 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem213 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem214 {
        bits: u32,
    }
    impl Default for Qmem214 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem214 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem215 {
        bits: u32,
    }
    impl Default for Qmem215 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem215 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem22 {
        bits: u32,
    }
    impl Default for Qmem22 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem22 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem23 {
        bits: u32,
    }
    impl Default for Qmem23 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem23 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem24 {
        bits: u32,
    }
    impl Default for Qmem24 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem24 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem25 {
        bits: u32,
    }
    impl Default for Qmem25 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem25 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem26 {
        bits: u32,
    }
    impl Default for Qmem26 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem26 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem27 {
        bits: u32,
    }
    impl Default for Qmem27 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem27 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem28 {
        bits: u32,
    }
    impl Default for Qmem28 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem28 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem29 {
        bits: u32,
    }
    impl Default for Qmem29 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem29 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem30 {
        bits: u32,
    }
    impl Default for Qmem30 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem30 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem31 {
        bits: u32,
    }
    impl Default for Qmem31 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem31 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem310 {
        bits: u32,
    }
    impl Default for Qmem310 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem310 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem311 {
        bits: u32,
    }
    impl Default for Qmem311 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem311 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem312 {
        bits: u32,
    }
    impl Default for Qmem312 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem312 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem313 {
        bits: u32,
    }
    impl Default for Qmem313 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem313 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem314 {
        bits: u32,
    }
    impl Default for Qmem314 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem314 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem315 {
        bits: u32,
    }
    impl Default for Qmem315 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem315 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem32 {
        bits: u32,
    }
    impl Default for Qmem32 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem32 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem33 {
        bits: u32,
    }
    impl Default for Qmem33 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem33 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem34 {
        bits: u32,
    }
    impl Default for Qmem34 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem34 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem35 {
        bits: u32,
    }
    impl Default for Qmem35 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem35 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem36 {
        bits: u32,
    }
    impl Default for Qmem36 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem36 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem37 {
        bits: u32,
    }
    impl Default for Qmem37 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem37 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem38 {
        bits: u32,
    }
    impl Default for Qmem38 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem38 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[doc = "JPEG quantization tables"]
    pub struct Qmem39 {
        bits: u32,
    }
    impl Default for Qmem39 {
        fn default() -> Self {
            unsafe { Self::from_bits_unchecked(0x0) }
        }
    }
    impl Qmem39 {
        #[inline(always)]
        pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
            Self { bits }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u32 {
            self.bits
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn set_q_mem_ram(mut self, val: u32) -> Self {
            self.bits &= !(0xffffffff << 0x0);
            self.bits |= (val as u32 & 0xffffffff) << 0x0;
            self
        }
        #[inline(always)]
        #[doc = "QMem RAM"]
        pub const fn q_mem_ram(self) -> u32 {
            ((self.bits >> 0x0) & 0xffffffff) as _
        }
    }
}
pub mod vals {
    #[allow(unused)]
    use super::*;
}
