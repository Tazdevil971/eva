
#[allow(unused_imports)]
use super::utils;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "JPEG codec"]
pub struct Jpeg {
    ptr: *mut u8,
}
impl Jpeg {
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
    #[doc = "JPEG codec configuration register 0"]
    pub const fn jpeg_confr0(&self) -> utils::Reg<JpegConfr0Bits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x0);
            <utils::Reg<JpegConfr0Bits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 1"]
    pub const fn jpeg_confr1(&self) -> utils::Reg<JpegConfr1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4);
            <utils::Reg<JpegConfr1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 2"]
    pub const fn jpeg_confr2(&self) -> utils::Reg<JpegConfr2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8);
            <utils::Reg<JpegConfr2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 3"]
    pub const fn jpeg_confr3(&self) -> utils::Reg<JpegConfr3Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc);
            <utils::Reg<JpegConfr3Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 4"]
    pub const fn jpeg_confr4(&self) -> utils::Reg<JpegConfr4Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10);
            <utils::Reg<JpegConfr4Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 5"]
    pub const fn jpeg_confr5(&self) -> utils::Reg<JpegConfr5Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14);
            <utils::Reg<JpegConfr5Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 6"]
    pub const fn jpeg_confr6(&self) -> utils::Reg<JpegConfr6Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18);
            <utils::Reg<JpegConfr6Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG codec configuration register 7"]
    pub const fn jpeg_confr7(&self) -> utils::Reg<JpegConfr7Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c);
            <utils::Reg<JpegConfr7Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG control register"]
    pub const fn jpeg_cr(&self) -> utils::Reg<JpegCrBits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30);
            <utils::Reg<JpegCrBits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG status register"]
    pub const fn jpeg_sr(&self) -> utils::Reg<JpegSrBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x34);
            <utils::Reg<JpegSrBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG clear flag register"]
    pub const fn jpeg_cfr(&self) -> utils::Reg<JpegCfrBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x38);
            <utils::Reg<JpegCfrBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG data input register"]
    pub const fn jpeg_dir(&self) -> utils::Reg<JpegDirBits, utils::WO> {
        unsafe {
            let ptr = self.ptr.add(0x40);
            <utils::Reg<JpegDirBits, utils::WO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG data output register"]
    pub const fn jpeg_dor(&self) -> utils::Reg<JpegDorBits, utils::RO> {
        unsafe {
            let ptr = self.ptr.add(0x44);
            <utils::Reg<JpegDorBits, utils::RO>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_0(&self) -> utils::Reg<Qmem00bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50);
            <utils::Reg<Qmem00bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_1(&self) -> utils::Reg<Qmem01bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x54);
            <utils::Reg<Qmem01bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_2(&self) -> utils::Reg<Qmem02bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x58);
            <utils::Reg<Qmem02bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_3(&self) -> utils::Reg<Qmem03bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c);
            <utils::Reg<Qmem03bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_4(&self) -> utils::Reg<Qmem04bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60);
            <utils::Reg<Qmem04bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_5(&self) -> utils::Reg<Qmem05bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x64);
            <utils::Reg<Qmem05bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_6(&self) -> utils::Reg<Qmem06bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68);
            <utils::Reg<Qmem06bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_7(&self) -> utils::Reg<Qmem07bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6c);
            <utils::Reg<Qmem07bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_8(&self) -> utils::Reg<Qmem08bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x70);
            <utils::Reg<Qmem08bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_9(&self) -> utils::Reg<Qmem09bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x74);
            <utils::Reg<Qmem09bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_10(&self) -> utils::Reg<Qmem010bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x78);
            <utils::Reg<Qmem010bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_11(&self) -> utils::Reg<Qmem011bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7c);
            <utils::Reg<Qmem011bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_12(&self) -> utils::Reg<Qmem012bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x80);
            <utils::Reg<Qmem012bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_13(&self) -> utils::Reg<Qmem013bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x84);
            <utils::Reg<Qmem013bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_14(&self) -> utils::Reg<Qmem014bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x88);
            <utils::Reg<Qmem014bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem0_15(&self) -> utils::Reg<Qmem015bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x8c);
            <utils::Reg<Qmem015bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_0(&self) -> utils::Reg<Qmem10bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x90);
            <utils::Reg<Qmem10bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_1(&self) -> utils::Reg<Qmem11bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x94);
            <utils::Reg<Qmem11bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_2(&self) -> utils::Reg<Qmem12bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x98);
            <utils::Reg<Qmem12bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_3(&self) -> utils::Reg<Qmem13bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x9c);
            <utils::Reg<Qmem13bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_4(&self) -> utils::Reg<Qmem14bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xa0);
            <utils::Reg<Qmem14bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_5(&self) -> utils::Reg<Qmem15bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xa4);
            <utils::Reg<Qmem15bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_6(&self) -> utils::Reg<Qmem16bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xa8);
            <utils::Reg<Qmem16bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_7(&self) -> utils::Reg<Qmem17bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xac);
            <utils::Reg<Qmem17bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_8(&self) -> utils::Reg<Qmem18bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xb0);
            <utils::Reg<Qmem18bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_9(&self) -> utils::Reg<Qmem19bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xb4);
            <utils::Reg<Qmem19bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_10(&self) -> utils::Reg<Qmem110bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xb8);
            <utils::Reg<Qmem110bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_11(&self) -> utils::Reg<Qmem111bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xbc);
            <utils::Reg<Qmem111bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_12(&self) -> utils::Reg<Qmem112bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc0);
            <utils::Reg<Qmem112bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_13(&self) -> utils::Reg<Qmem113bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc4);
            <utils::Reg<Qmem113bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_14(&self) -> utils::Reg<Qmem114bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xc8);
            <utils::Reg<Qmem114bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem1_15(&self) -> utils::Reg<Qmem115bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xcc);
            <utils::Reg<Qmem115bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_0(&self) -> utils::Reg<Qmem20bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xd0);
            <utils::Reg<Qmem20bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_1(&self) -> utils::Reg<Qmem21bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xd4);
            <utils::Reg<Qmem21bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_2(&self) -> utils::Reg<Qmem22bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xd8);
            <utils::Reg<Qmem22bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_3(&self) -> utils::Reg<Qmem23bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xdc);
            <utils::Reg<Qmem23bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_4(&self) -> utils::Reg<Qmem24bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xe0);
            <utils::Reg<Qmem24bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_5(&self) -> utils::Reg<Qmem25bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xe4);
            <utils::Reg<Qmem25bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_6(&self) -> utils::Reg<Qmem26bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xe8);
            <utils::Reg<Qmem26bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_7(&self) -> utils::Reg<Qmem27bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xec);
            <utils::Reg<Qmem27bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_8(&self) -> utils::Reg<Qmem28bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xf0);
            <utils::Reg<Qmem28bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_9(&self) -> utils::Reg<Qmem29bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xf4);
            <utils::Reg<Qmem29bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_10(&self) -> utils::Reg<Qmem210bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xf8);
            <utils::Reg<Qmem210bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_11(&self) -> utils::Reg<Qmem211bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0xfc);
            <utils::Reg<Qmem211bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_12(&self) -> utils::Reg<Qmem212bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x100);
            <utils::Reg<Qmem212bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_13(&self) -> utils::Reg<Qmem213bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x104);
            <utils::Reg<Qmem213bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_14(&self) -> utils::Reg<Qmem214bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x108);
            <utils::Reg<Qmem214bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem2_15(&self) -> utils::Reg<Qmem215bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x10c);
            <utils::Reg<Qmem215bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_0(&self) -> utils::Reg<Qmem30bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x110);
            <utils::Reg<Qmem30bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_1(&self) -> utils::Reg<Qmem31bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x114);
            <utils::Reg<Qmem31bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_2(&self) -> utils::Reg<Qmem32bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x118);
            <utils::Reg<Qmem32bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_3(&self) -> utils::Reg<Qmem33bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x11c);
            <utils::Reg<Qmem33bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_4(&self) -> utils::Reg<Qmem34bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x120);
            <utils::Reg<Qmem34bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_5(&self) -> utils::Reg<Qmem35bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x124);
            <utils::Reg<Qmem35bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_6(&self) -> utils::Reg<Qmem36bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x128);
            <utils::Reg<Qmem36bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_7(&self) -> utils::Reg<Qmem37bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x12c);
            <utils::Reg<Qmem37bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_8(&self) -> utils::Reg<Qmem38bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x130);
            <utils::Reg<Qmem38bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_9(&self) -> utils::Reg<Qmem39bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x134);
            <utils::Reg<Qmem39bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_10(&self) -> utils::Reg<Qmem310bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x138);
            <utils::Reg<Qmem310bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_11(&self) -> utils::Reg<Qmem311bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x13c);
            <utils::Reg<Qmem311bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_12(&self) -> utils::Reg<Qmem312bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x140);
            <utils::Reg<Qmem312bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_13(&self) -> utils::Reg<Qmem313bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x144);
            <utils::Reg<Qmem313bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_14(&self) -> utils::Reg<Qmem314bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x148);
            <utils::Reg<Qmem314bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG quantization tables"]
    pub const fn qmem3_15(&self) -> utils::Reg<Qmem315bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x14c);
            <utils::Reg<Qmem315bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_0(&self) -> utils::Reg<Huffmin0bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x150);
            <utils::Reg<Huffmin0bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_1(&self) -> utils::Reg<Huffmin1bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x154);
            <utils::Reg<Huffmin1bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_2(&self) -> utils::Reg<Huffmin2bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x158);
            <utils::Reg<Huffmin2bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_3(&self) -> utils::Reg<Huffmin3bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x15c);
            <utils::Reg<Huffmin3bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_4(&self) -> utils::Reg<Huffmin4bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x160);
            <utils::Reg<Huffmin4bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_5(&self) -> utils::Reg<Huffmin5bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x164);
            <utils::Reg<Huffmin5bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_6(&self) -> utils::Reg<Huffmin6bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x168);
            <utils::Reg<Huffmin6bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_7(&self) -> utils::Reg<Huffmin7bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x16c);
            <utils::Reg<Huffmin7bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_8(&self) -> utils::Reg<Huffmin8bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x170);
            <utils::Reg<Huffmin8bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_9(&self) -> utils::Reg<Huffmin9bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x174);
            <utils::Reg<Huffmin9bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_10(&self) -> utils::Reg<Huffmin10bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x178);
            <utils::Reg<Huffmin10bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_11(&self) -> utils::Reg<Huffmin11bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x17c);
            <utils::Reg<Huffmin11bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_12(&self) -> utils::Reg<Huffmin12bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x180);
            <utils::Reg<Huffmin12bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_13(&self) -> utils::Reg<Huffmin13bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x184);
            <utils::Reg<Huffmin13bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_14(&self) -> utils::Reg<Huffmin14bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x188);
            <utils::Reg<Huffmin14bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffMin tables"]
    pub const fn huffmin_15(&self) -> utils::Reg<Huffmin15bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x18c);
            <utils::Reg<Huffmin15bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase0(&self) -> utils::Reg<Huffbase0Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x190);
            <utils::Reg<Huffbase0Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase1(&self) -> utils::Reg<Huffbase1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x194);
            <utils::Reg<Huffbase1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase2(&self) -> utils::Reg<Huffbase2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x198);
            <utils::Reg<Huffbase2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase3(&self) -> utils::Reg<Huffbase3Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x19c);
            <utils::Reg<Huffbase3Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase4(&self) -> utils::Reg<Huffbase4Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1a0);
            <utils::Reg<Huffbase4Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase5(&self) -> utils::Reg<Huffbase5Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1a4);
            <utils::Reg<Huffbase5Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase6(&self) -> utils::Reg<Huffbase6Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1a8);
            <utils::Reg<Huffbase6Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase7(&self) -> utils::Reg<Huffbase7Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1ac);
            <utils::Reg<Huffbase7Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase8(&self) -> utils::Reg<Huffbase8Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1b0);
            <utils::Reg<Huffbase8Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase9(&self) -> utils::Reg<Huffbase9Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1b4);
            <utils::Reg<Huffbase9Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase10(&self) -> utils::Reg<Huffbase10Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1b8);
            <utils::Reg<Huffbase10Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase11(&self) -> utils::Reg<Huffbase11Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1bc);
            <utils::Reg<Huffbase11Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase12(&self) -> utils::Reg<Huffbase12Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c0);
            <utils::Reg<Huffbase12Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase13(&self) -> utils::Reg<Huffbase13Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c4);
            <utils::Reg<Huffbase13Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase14(&self) -> utils::Reg<Huffbase14Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1c8);
            <utils::Reg<Huffbase14Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase15(&self) -> utils::Reg<Huffbase15Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1cc);
            <utils::Reg<Huffbase15Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase16(&self) -> utils::Reg<Huffbase16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1d0);
            <utils::Reg<Huffbase16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase17(&self) -> utils::Reg<Huffbase17Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1d4);
            <utils::Reg<Huffbase17Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase18(&self) -> utils::Reg<Huffbase18Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1d8);
            <utils::Reg<Huffbase18Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase19(&self) -> utils::Reg<Huffbase19Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1dc);
            <utils::Reg<Huffbase19Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase20(&self) -> utils::Reg<Huffbase20Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1e0);
            <utils::Reg<Huffbase20Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase21(&self) -> utils::Reg<Huffbase21Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1e4);
            <utils::Reg<Huffbase21Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase22(&self) -> utils::Reg<Huffbase22Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1e8);
            <utils::Reg<Huffbase22Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase23(&self) -> utils::Reg<Huffbase23Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1ec);
            <utils::Reg<Huffbase23Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase24(&self) -> utils::Reg<Huffbase24Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1f0);
            <utils::Reg<Huffbase24Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase25(&self) -> utils::Reg<Huffbase25Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1f4);
            <utils::Reg<Huffbase25Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase26(&self) -> utils::Reg<Huffbase26Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1f8);
            <utils::Reg<Huffbase26Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase27(&self) -> utils::Reg<Huffbase27Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x1fc);
            <utils::Reg<Huffbase27Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase28(&self) -> utils::Reg<Huffbase28Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x200);
            <utils::Reg<Huffbase28Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase29(&self) -> utils::Reg<Huffbase29Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x204);
            <utils::Reg<Huffbase29Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase30(&self) -> utils::Reg<Huffbase30Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x208);
            <utils::Reg<Huffbase30Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HuffSymb tables"]
    pub const fn huffbase31(&self) -> utils::Reg<Huffbase31Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x20c);
            <utils::Reg<Huffbase31Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb0(&self) -> utils::Reg<Huffsymb0Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x210);
            <utils::Reg<Huffsymb0Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb1(&self) -> utils::Reg<Huffsymb1Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x214);
            <utils::Reg<Huffsymb1Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb2(&self) -> utils::Reg<Huffsymb2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x218);
            <utils::Reg<Huffsymb2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb3(&self) -> utils::Reg<Huffsymb3Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x21c);
            <utils::Reg<Huffsymb3Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb4(&self) -> utils::Reg<Huffsymb4Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x220);
            <utils::Reg<Huffsymb4Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb5(&self) -> utils::Reg<Huffsymb5Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x224);
            <utils::Reg<Huffsymb5Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb6(&self) -> utils::Reg<Huffsymb6Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x228);
            <utils::Reg<Huffsymb6Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb7(&self) -> utils::Reg<Huffsymb7Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x22c);
            <utils::Reg<Huffsymb7Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb8(&self) -> utils::Reg<Huffsymb8Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x230);
            <utils::Reg<Huffsymb8Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb9(&self) -> utils::Reg<Huffsymb9Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x234);
            <utils::Reg<Huffsymb9Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb10(&self) -> utils::Reg<Huffsymb10Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x238);
            <utils::Reg<Huffsymb10Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb11(&self) -> utils::Reg<Huffsymb11Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x23c);
            <utils::Reg<Huffsymb11Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb12(&self) -> utils::Reg<Huffsymb12Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x240);
            <utils::Reg<Huffsymb12Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb13(&self) -> utils::Reg<Huffsymb13Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x244);
            <utils::Reg<Huffsymb13Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb14(&self) -> utils::Reg<Huffsymb14Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x248);
            <utils::Reg<Huffsymb14Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb15(&self) -> utils::Reg<Huffsymb15Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x24c);
            <utils::Reg<Huffsymb15Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb16(&self) -> utils::Reg<Huffsymb16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x250);
            <utils::Reg<Huffsymb16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb17(&self) -> utils::Reg<Huffsymb17Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x254);
            <utils::Reg<Huffsymb17Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb18(&self) -> utils::Reg<Huffsymb18Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x258);
            <utils::Reg<Huffsymb18Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb19(&self) -> utils::Reg<Huffsymb19Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x25c);
            <utils::Reg<Huffsymb19Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb20(&self) -> utils::Reg<Huffsymb20Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x260);
            <utils::Reg<Huffsymb20Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb21(&self) -> utils::Reg<Huffsymb21Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x264);
            <utils::Reg<Huffsymb21Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb22(&self) -> utils::Reg<Huffsymb22Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x268);
            <utils::Reg<Huffsymb22Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb23(&self) -> utils::Reg<Huffsymb23Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x26c);
            <utils::Reg<Huffsymb23Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb24(&self) -> utils::Reg<Huffsymb24Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x270);
            <utils::Reg<Huffsymb24Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb25(&self) -> utils::Reg<Huffsymb25Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x274);
            <utils::Reg<Huffsymb25Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb26(&self) -> utils::Reg<Huffsymb26Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x278);
            <utils::Reg<Huffsymb26Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb27(&self) -> utils::Reg<Huffsymb27Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x27c);
            <utils::Reg<Huffsymb27Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb28(&self) -> utils::Reg<Huffsymb28Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x280);
            <utils::Reg<Huffsymb28Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb29(&self) -> utils::Reg<Huffsymb29Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x284);
            <utils::Reg<Huffsymb29Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb30(&self) -> utils::Reg<Huffsymb30Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x288);
            <utils::Reg<Huffsymb30Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb31(&self) -> utils::Reg<Huffsymb31Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x28c);
            <utils::Reg<Huffsymb31Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb32(&self) -> utils::Reg<Huffsymb32Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x290);
            <utils::Reg<Huffsymb32Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb33(&self) -> utils::Reg<Huffsymb33Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x294);
            <utils::Reg<Huffsymb33Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb34(&self) -> utils::Reg<Huffsymb34Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x298);
            <utils::Reg<Huffsymb34Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb35(&self) -> utils::Reg<Huffsymb35Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x29c);
            <utils::Reg<Huffsymb35Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb36(&self) -> utils::Reg<Huffsymb36Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2a0);
            <utils::Reg<Huffsymb36Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb37(&self) -> utils::Reg<Huffsymb37Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2a4);
            <utils::Reg<Huffsymb37Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb38(&self) -> utils::Reg<Huffsymb38Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2a8);
            <utils::Reg<Huffsymb38Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb39(&self) -> utils::Reg<Huffsymb39Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2ac);
            <utils::Reg<Huffsymb39Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb40(&self) -> utils::Reg<Huffsymb40Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2b0);
            <utils::Reg<Huffsymb40Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb41(&self) -> utils::Reg<Huffsymb41Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2b4);
            <utils::Reg<Huffsymb41Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb42(&self) -> utils::Reg<Huffsymb42Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2b8);
            <utils::Reg<Huffsymb42Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb43(&self) -> utils::Reg<Huffsymb43Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2bc);
            <utils::Reg<Huffsymb43Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb44(&self) -> utils::Reg<Huffsymb44Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c0);
            <utils::Reg<Huffsymb44Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb45(&self) -> utils::Reg<Huffsymb45Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c4);
            <utils::Reg<Huffsymb45Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb46(&self) -> utils::Reg<Huffsymb46Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2c8);
            <utils::Reg<Huffsymb46Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb47(&self) -> utils::Reg<Huffsymb47Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2cc);
            <utils::Reg<Huffsymb47Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb48(&self) -> utils::Reg<Huffsymb48Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2d0);
            <utils::Reg<Huffsymb48Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb49(&self) -> utils::Reg<Huffsymb49Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2d4);
            <utils::Reg<Huffsymb49Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb50(&self) -> utils::Reg<Huffsymb50Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2d8);
            <utils::Reg<Huffsymb50Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb51(&self) -> utils::Reg<Huffsymb51Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2dc);
            <utils::Reg<Huffsymb51Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb52(&self) -> utils::Reg<Huffsymb52Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2e0);
            <utils::Reg<Huffsymb52Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb53(&self) -> utils::Reg<Huffsymb53Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2e4);
            <utils::Reg<Huffsymb53Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb54(&self) -> utils::Reg<Huffsymb54Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2e8);
            <utils::Reg<Huffsymb54Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb55(&self) -> utils::Reg<Huffsymb55Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2ec);
            <utils::Reg<Huffsymb55Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb56(&self) -> utils::Reg<Huffsymb56Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2f0);
            <utils::Reg<Huffsymb56Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb57(&self) -> utils::Reg<Huffsymb57Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2f4);
            <utils::Reg<Huffsymb57Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb58(&self) -> utils::Reg<Huffsymb58Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2f8);
            <utils::Reg<Huffsymb58Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb59(&self) -> utils::Reg<Huffsymb59Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x2fc);
            <utils::Reg<Huffsymb59Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb60(&self) -> utils::Reg<Huffsymb60Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x300);
            <utils::Reg<Huffsymb60Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb61(&self) -> utils::Reg<Huffsymb61Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x304);
            <utils::Reg<Huffsymb61Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb62(&self) -> utils::Reg<Huffsymb62Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x308);
            <utils::Reg<Huffsymb62Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb63(&self) -> utils::Reg<Huffsymb63Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x30c);
            <utils::Reg<Huffsymb63Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb64(&self) -> utils::Reg<Huffsymb64Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x310);
            <utils::Reg<Huffsymb64Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb65(&self) -> utils::Reg<Huffsymb65Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x314);
            <utils::Reg<Huffsymb65Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb66(&self) -> utils::Reg<Huffsymb66Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x318);
            <utils::Reg<Huffsymb66Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb67(&self) -> utils::Reg<Huffsymb67Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x31c);
            <utils::Reg<Huffsymb67Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb68(&self) -> utils::Reg<Huffsymb68Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x320);
            <utils::Reg<Huffsymb68Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb69(&self) -> utils::Reg<Huffsymb69Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x324);
            <utils::Reg<Huffsymb69Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb70(&self) -> utils::Reg<Huffsymb70Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x328);
            <utils::Reg<Huffsymb70Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb71(&self) -> utils::Reg<Huffsymb71Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x32c);
            <utils::Reg<Huffsymb71Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb72(&self) -> utils::Reg<Huffsymb72Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x330);
            <utils::Reg<Huffsymb72Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb73(&self) -> utils::Reg<Huffsymb73Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x334);
            <utils::Reg<Huffsymb73Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb74(&self) -> utils::Reg<Huffsymb74Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x338);
            <utils::Reg<Huffsymb74Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb75(&self) -> utils::Reg<Huffsymb75Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x33c);
            <utils::Reg<Huffsymb75Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb76(&self) -> utils::Reg<Huffsymb76Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x340);
            <utils::Reg<Huffsymb76Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb77(&self) -> utils::Reg<Huffsymb77Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x344);
            <utils::Reg<Huffsymb77Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb78(&self) -> utils::Reg<Huffsymb78Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x348);
            <utils::Reg<Huffsymb78Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb79(&self) -> utils::Reg<Huffsymb79Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x34c);
            <utils::Reg<Huffsymb79Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb80(&self) -> utils::Reg<Huffsymb80Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x350);
            <utils::Reg<Huffsymb80Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb81(&self) -> utils::Reg<Huffsymb81Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x354);
            <utils::Reg<Huffsymb81Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb82(&self) -> utils::Reg<Huffsymb82Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x358);
            <utils::Reg<Huffsymb82Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG HUFFSYMB tables"]
    pub const fn huffsymb83(&self) -> utils::Reg<Huffsymb83Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x35c);
            <utils::Reg<Huffsymb83Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem0(&self) -> utils::Reg<Dhtmem0Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x360);
            <utils::Reg<Dhtmem0Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem2(&self) -> utils::Reg<Dhtmem2Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x364);
            <utils::Reg<Dhtmem2Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem3(&self) -> utils::Reg<Dhtmem3Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x368);
            <utils::Reg<Dhtmem3Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem4(&self) -> utils::Reg<Dhtmem4Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x36c);
            <utils::Reg<Dhtmem4Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem5(&self) -> utils::Reg<Dhtmem5Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x370);
            <utils::Reg<Dhtmem5Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem6(&self) -> utils::Reg<Dhtmem6Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x374);
            <utils::Reg<Dhtmem6Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem7(&self) -> utils::Reg<Dhtmem7Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x378);
            <utils::Reg<Dhtmem7Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem8(&self) -> utils::Reg<Dhtmem8Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x37c);
            <utils::Reg<Dhtmem8Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem9(&self) -> utils::Reg<Dhtmem9Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x380);
            <utils::Reg<Dhtmem9Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem10(&self) -> utils::Reg<Dhtmem10Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x384);
            <utils::Reg<Dhtmem10Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem11(&self) -> utils::Reg<Dhtmem11Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x388);
            <utils::Reg<Dhtmem11Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem12(&self) -> utils::Reg<Dhtmem12Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x38c);
            <utils::Reg<Dhtmem12Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem13(&self) -> utils::Reg<Dhtmem13Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x390);
            <utils::Reg<Dhtmem13Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem14(&self) -> utils::Reg<Dhtmem14Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x394);
            <utils::Reg<Dhtmem14Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem15(&self) -> utils::Reg<Dhtmem15Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x398);
            <utils::Reg<Dhtmem15Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem16(&self) -> utils::Reg<Dhtmem16Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x39c);
            <utils::Reg<Dhtmem16Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem17(&self) -> utils::Reg<Dhtmem17Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3a0);
            <utils::Reg<Dhtmem17Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem18(&self) -> utils::Reg<Dhtmem18Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3a4);
            <utils::Reg<Dhtmem18Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem19(&self) -> utils::Reg<Dhtmem19Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3a8);
            <utils::Reg<Dhtmem19Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem20(&self) -> utils::Reg<Dhtmem20Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3ac);
            <utils::Reg<Dhtmem20Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem21(&self) -> utils::Reg<Dhtmem21Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3b0);
            <utils::Reg<Dhtmem21Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem22(&self) -> utils::Reg<Dhtmem22Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3b4);
            <utils::Reg<Dhtmem22Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem23(&self) -> utils::Reg<Dhtmem23Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3b8);
            <utils::Reg<Dhtmem23Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem24(&self) -> utils::Reg<Dhtmem24Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3bc);
            <utils::Reg<Dhtmem24Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem25(&self) -> utils::Reg<Dhtmem25Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c0);
            <utils::Reg<Dhtmem25Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem26(&self) -> utils::Reg<Dhtmem26Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c4);
            <utils::Reg<Dhtmem26Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem27(&self) -> utils::Reg<Dhtmem27Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3c8);
            <utils::Reg<Dhtmem27Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem28(&self) -> utils::Reg<Dhtmem28Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3cc);
            <utils::Reg<Dhtmem28Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem29(&self) -> utils::Reg<Dhtmem29Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3d0);
            <utils::Reg<Dhtmem29Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem30(&self) -> utils::Reg<Dhtmem30Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3d4);
            <utils::Reg<Dhtmem30Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem31(&self) -> utils::Reg<Dhtmem31Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3d8);
            <utils::Reg<Dhtmem31Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem32(&self) -> utils::Reg<Dhtmem32Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3dc);
            <utils::Reg<Dhtmem32Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem33(&self) -> utils::Reg<Dhtmem33Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3e0);
            <utils::Reg<Dhtmem33Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem34(&self) -> utils::Reg<Dhtmem34Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3e4);
            <utils::Reg<Dhtmem34Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem35(&self) -> utils::Reg<Dhtmem35Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3e8);
            <utils::Reg<Dhtmem35Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem36(&self) -> utils::Reg<Dhtmem36Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3ec);
            <utils::Reg<Dhtmem36Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem37(&self) -> utils::Reg<Dhtmem37Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3f0);
            <utils::Reg<Dhtmem37Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem38(&self) -> utils::Reg<Dhtmem38Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3f4);
            <utils::Reg<Dhtmem38Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem39(&self) -> utils::Reg<Dhtmem39Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3f8);
            <utils::Reg<Dhtmem39Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem40(&self) -> utils::Reg<Dhtmem40Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x3fc);
            <utils::Reg<Dhtmem40Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem41(&self) -> utils::Reg<Dhtmem41Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x400);
            <utils::Reg<Dhtmem41Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem42(&self) -> utils::Reg<Dhtmem42Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x404);
            <utils::Reg<Dhtmem42Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem43(&self) -> utils::Reg<Dhtmem43Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x408);
            <utils::Reg<Dhtmem43Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem44(&self) -> utils::Reg<Dhtmem44Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x40c);
            <utils::Reg<Dhtmem44Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem45(&self) -> utils::Reg<Dhtmem45Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x410);
            <utils::Reg<Dhtmem45Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem46(&self) -> utils::Reg<Dhtmem46Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x414);
            <utils::Reg<Dhtmem46Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem47(&self) -> utils::Reg<Dhtmem47Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x418);
            <utils::Reg<Dhtmem47Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem48(&self) -> utils::Reg<Dhtmem48Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x41c);
            <utils::Reg<Dhtmem48Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem49(&self) -> utils::Reg<Dhtmem49Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x420);
            <utils::Reg<Dhtmem49Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem50(&self) -> utils::Reg<Dhtmem50Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x424);
            <utils::Reg<Dhtmem50Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem51(&self) -> utils::Reg<Dhtmem51Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x428);
            <utils::Reg<Dhtmem51Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem52(&self) -> utils::Reg<Dhtmem52Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x42c);
            <utils::Reg<Dhtmem52Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem53(&self) -> utils::Reg<Dhtmem53Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x430);
            <utils::Reg<Dhtmem53Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem54(&self) -> utils::Reg<Dhtmem54Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x434);
            <utils::Reg<Dhtmem54Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem55(&self) -> utils::Reg<Dhtmem55Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x438);
            <utils::Reg<Dhtmem55Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem56(&self) -> utils::Reg<Dhtmem56Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x43c);
            <utils::Reg<Dhtmem56Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem57(&self) -> utils::Reg<Dhtmem57Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x440);
            <utils::Reg<Dhtmem57Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem58(&self) -> utils::Reg<Dhtmem58Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x444);
            <utils::Reg<Dhtmem58Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem59(&self) -> utils::Reg<Dhtmem59Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x448);
            <utils::Reg<Dhtmem59Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem60(&self) -> utils::Reg<Dhtmem60Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x44c);
            <utils::Reg<Dhtmem60Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem61(&self) -> utils::Reg<Dhtmem61Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x450);
            <utils::Reg<Dhtmem61Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem62(&self) -> utils::Reg<Dhtmem62Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x454);
            <utils::Reg<Dhtmem62Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem63(&self) -> utils::Reg<Dhtmem63Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x458);
            <utils::Reg<Dhtmem63Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem64(&self) -> utils::Reg<Dhtmem64Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x45c);
            <utils::Reg<Dhtmem64Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem65(&self) -> utils::Reg<Dhtmem65Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x460);
            <utils::Reg<Dhtmem65Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem66(&self) -> utils::Reg<Dhtmem66Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x464);
            <utils::Reg<Dhtmem66Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem67(&self) -> utils::Reg<Dhtmem67Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x468);
            <utils::Reg<Dhtmem67Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem68(&self) -> utils::Reg<Dhtmem68Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x46c);
            <utils::Reg<Dhtmem68Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem69(&self) -> utils::Reg<Dhtmem69Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x470);
            <utils::Reg<Dhtmem69Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem70(&self) -> utils::Reg<Dhtmem70Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x474);
            <utils::Reg<Dhtmem70Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem71(&self) -> utils::Reg<Dhtmem71Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x478);
            <utils::Reg<Dhtmem71Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem72(&self) -> utils::Reg<Dhtmem72Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x47c);
            <utils::Reg<Dhtmem72Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem73(&self) -> utils::Reg<Dhtmem73Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x480);
            <utils::Reg<Dhtmem73Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem74(&self) -> utils::Reg<Dhtmem74Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x484);
            <utils::Reg<Dhtmem74Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem75(&self) -> utils::Reg<Dhtmem75Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x488);
            <utils::Reg<Dhtmem75Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem76(&self) -> utils::Reg<Dhtmem76Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x48c);
            <utils::Reg<Dhtmem76Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem77(&self) -> utils::Reg<Dhtmem77Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x490);
            <utils::Reg<Dhtmem77Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem78(&self) -> utils::Reg<Dhtmem78Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x494);
            <utils::Reg<Dhtmem78Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem79(&self) -> utils::Reg<Dhtmem79Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x498);
            <utils::Reg<Dhtmem79Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem80(&self) -> utils::Reg<Dhtmem80Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x49c);
            <utils::Reg<Dhtmem80Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem81(&self) -> utils::Reg<Dhtmem81Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4a0);
            <utils::Reg<Dhtmem81Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem82(&self) -> utils::Reg<Dhtmem82Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4a4);
            <utils::Reg<Dhtmem82Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem83(&self) -> utils::Reg<Dhtmem83Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4a8);
            <utils::Reg<Dhtmem83Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem84(&self) -> utils::Reg<Dhtmem84Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4ac);
            <utils::Reg<Dhtmem84Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem85(&self) -> utils::Reg<Dhtmem85Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4b0);
            <utils::Reg<Dhtmem85Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem86(&self) -> utils::Reg<Dhtmem86Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4b4);
            <utils::Reg<Dhtmem86Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem87(&self) -> utils::Reg<Dhtmem87Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4b8);
            <utils::Reg<Dhtmem87Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem88(&self) -> utils::Reg<Dhtmem88Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4bc);
            <utils::Reg<Dhtmem88Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem89(&self) -> utils::Reg<Dhtmem89Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c0);
            <utils::Reg<Dhtmem89Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem90(&self) -> utils::Reg<Dhtmem90Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c4);
            <utils::Reg<Dhtmem90Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem91(&self) -> utils::Reg<Dhtmem91Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4c8);
            <utils::Reg<Dhtmem91Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem92(&self) -> utils::Reg<Dhtmem92Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4cc);
            <utils::Reg<Dhtmem92Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem93(&self) -> utils::Reg<Dhtmem93Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4d0);
            <utils::Reg<Dhtmem93Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem94(&self) -> utils::Reg<Dhtmem94Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4d4);
            <utils::Reg<Dhtmem94Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem95(&self) -> utils::Reg<Dhtmem95Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4d8);
            <utils::Reg<Dhtmem95Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem96(&self) -> utils::Reg<Dhtmem96Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4dc);
            <utils::Reg<Dhtmem96Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem97(&self) -> utils::Reg<Dhtmem97Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4e0);
            <utils::Reg<Dhtmem97Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem98(&self) -> utils::Reg<Dhtmem98Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4e4);
            <utils::Reg<Dhtmem98Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem99(&self) -> utils::Reg<Dhtmem99Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4e8);
            <utils::Reg<Dhtmem99Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem100(&self) -> utils::Reg<Dhtmem100Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4ec);
            <utils::Reg<Dhtmem100Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem101(&self) -> utils::Reg<Dhtmem101Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4f0);
            <utils::Reg<Dhtmem101Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem102(&self) -> utils::Reg<Dhtmem102Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4f4);
            <utils::Reg<Dhtmem102Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG DHTMem tables"]
    pub const fn dhtmem103(&self) -> utils::Reg<Dhtmem103Bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x4f8);
            <utils::Reg<Dhtmem103Bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_0(&self) -> utils::Reg<HuffencAc00bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x500);
            <utils::Reg<HuffencAc00bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_1(&self) -> utils::Reg<HuffencAc01bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x504);
            <utils::Reg<HuffencAc01bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_2(&self) -> utils::Reg<HuffencAc02bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x508);
            <utils::Reg<HuffencAc02bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_3(&self) -> utils::Reg<HuffencAc03bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x50c);
            <utils::Reg<HuffencAc03bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_4(&self) -> utils::Reg<HuffencAc04bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x510);
            <utils::Reg<HuffencAc04bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_5(&self) -> utils::Reg<HuffencAc05bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x514);
            <utils::Reg<HuffencAc05bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_6(&self) -> utils::Reg<HuffencAc06bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x518);
            <utils::Reg<HuffencAc06bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_7(&self) -> utils::Reg<HuffencAc07bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x51c);
            <utils::Reg<HuffencAc07bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_8(&self) -> utils::Reg<HuffencAc08bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x520);
            <utils::Reg<HuffencAc08bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_9(&self) -> utils::Reg<HuffencAc09bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x524);
            <utils::Reg<HuffencAc09bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_10(&self) -> utils::Reg<HuffencAc010bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x528);
            <utils::Reg<HuffencAc010bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_11(&self) -> utils::Reg<HuffencAc011bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x52c);
            <utils::Reg<HuffencAc011bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_12(&self) -> utils::Reg<HuffencAc012bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x530);
            <utils::Reg<HuffencAc012bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_13(&self) -> utils::Reg<HuffencAc013bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x534);
            <utils::Reg<HuffencAc013bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_14(&self) -> utils::Reg<HuffencAc014bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x538);
            <utils::Reg<HuffencAc014bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_15(&self) -> utils::Reg<HuffencAc015bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x53c);
            <utils::Reg<HuffencAc015bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_16(&self) -> utils::Reg<HuffencAc016bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x540);
            <utils::Reg<HuffencAc016bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_17(&self) -> utils::Reg<HuffencAc017bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x544);
            <utils::Reg<HuffencAc017bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_18(&self) -> utils::Reg<HuffencAc018bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x548);
            <utils::Reg<HuffencAc018bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_19(&self) -> utils::Reg<HuffencAc019bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x54c);
            <utils::Reg<HuffencAc019bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_20(&self) -> utils::Reg<HuffencAc020bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x550);
            <utils::Reg<HuffencAc020bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_21(&self) -> utils::Reg<HuffencAc021bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x554);
            <utils::Reg<HuffencAc021bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_22(&self) -> utils::Reg<HuffencAc022bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x558);
            <utils::Reg<HuffencAc022bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_23(&self) -> utils::Reg<HuffencAc023bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x55c);
            <utils::Reg<HuffencAc023bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_24(&self) -> utils::Reg<HuffencAc024bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x560);
            <utils::Reg<HuffencAc024bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_25(&self) -> utils::Reg<HuffencAc025bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x564);
            <utils::Reg<HuffencAc025bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_26(&self) -> utils::Reg<HuffencAc026bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x568);
            <utils::Reg<HuffencAc026bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_27(&self) -> utils::Reg<HuffencAc027bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x56c);
            <utils::Reg<HuffencAc027bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_28(&self) -> utils::Reg<HuffencAc028bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x570);
            <utils::Reg<HuffencAc028bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_29(&self) -> utils::Reg<HuffencAc029bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x574);
            <utils::Reg<HuffencAc029bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_30(&self) -> utils::Reg<HuffencAc030bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x578);
            <utils::Reg<HuffencAc030bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_31(&self) -> utils::Reg<HuffencAc031bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x57c);
            <utils::Reg<HuffencAc031bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_32(&self) -> utils::Reg<HuffencAc032bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x580);
            <utils::Reg<HuffencAc032bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_33(&self) -> utils::Reg<HuffencAc033bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x584);
            <utils::Reg<HuffencAc033bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_34(&self) -> utils::Reg<HuffencAc034bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x588);
            <utils::Reg<HuffencAc034bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_35(&self) -> utils::Reg<HuffencAc035bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x58c);
            <utils::Reg<HuffencAc035bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_36(&self) -> utils::Reg<HuffencAc036bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x590);
            <utils::Reg<HuffencAc036bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_37(&self) -> utils::Reg<HuffencAc037bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x594);
            <utils::Reg<HuffencAc037bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_38(&self) -> utils::Reg<HuffencAc038bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x598);
            <utils::Reg<HuffencAc038bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_39(&self) -> utils::Reg<HuffencAc039bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x59c);
            <utils::Reg<HuffencAc039bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_40(&self) -> utils::Reg<HuffencAc040bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5a0);
            <utils::Reg<HuffencAc040bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_41(&self) -> utils::Reg<HuffencAc041bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5a4);
            <utils::Reg<HuffencAc041bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_42(&self) -> utils::Reg<HuffencAc042bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5a8);
            <utils::Reg<HuffencAc042bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_43(&self) -> utils::Reg<HuffencAc043bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5ac);
            <utils::Reg<HuffencAc043bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_44(&self) -> utils::Reg<HuffencAc044bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5b0);
            <utils::Reg<HuffencAc044bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_45(&self) -> utils::Reg<HuffencAc045bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5b4);
            <utils::Reg<HuffencAc045bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_46(&self) -> utils::Reg<HuffencAc046bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5b8);
            <utils::Reg<HuffencAc046bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_47(&self) -> utils::Reg<HuffencAc047bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5bc);
            <utils::Reg<HuffencAc047bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_48(&self) -> utils::Reg<HuffencAc048bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c0);
            <utils::Reg<HuffencAc048bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_49(&self) -> utils::Reg<HuffencAc049bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c4);
            <utils::Reg<HuffencAc049bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_50(&self) -> utils::Reg<HuffencAc050bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5c8);
            <utils::Reg<HuffencAc050bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_51(&self) -> utils::Reg<HuffencAc051bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5cc);
            <utils::Reg<HuffencAc051bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_52(&self) -> utils::Reg<HuffencAc052bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5d0);
            <utils::Reg<HuffencAc052bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_53(&self) -> utils::Reg<HuffencAc053bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5d4);
            <utils::Reg<HuffencAc053bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_54(&self) -> utils::Reg<HuffencAc054bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5d8);
            <utils::Reg<HuffencAc054bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_55(&self) -> utils::Reg<HuffencAc055bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5dc);
            <utils::Reg<HuffencAc055bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_56(&self) -> utils::Reg<HuffencAc056bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5e0);
            <utils::Reg<HuffencAc056bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_57(&self) -> utils::Reg<HuffencAc057bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5e4);
            <utils::Reg<HuffencAc057bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_58(&self) -> utils::Reg<HuffencAc058bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5e8);
            <utils::Reg<HuffencAc058bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_59(&self) -> utils::Reg<HuffencAc059bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5ec);
            <utils::Reg<HuffencAc059bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_60(&self) -> utils::Reg<HuffencAc060bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5f0);
            <utils::Reg<HuffencAc060bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_61(&self) -> utils::Reg<HuffencAc061bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5f4);
            <utils::Reg<HuffencAc061bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_62(&self) -> utils::Reg<HuffencAc062bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5f8);
            <utils::Reg<HuffencAc062bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_63(&self) -> utils::Reg<HuffencAc063bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x5fc);
            <utils::Reg<HuffencAc063bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_64(&self) -> utils::Reg<HuffencAc064bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x600);
            <utils::Reg<HuffencAc064bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_65(&self) -> utils::Reg<HuffencAc065bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x604);
            <utils::Reg<HuffencAc065bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_66(&self) -> utils::Reg<HuffencAc066bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x608);
            <utils::Reg<HuffencAc066bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_67(&self) -> utils::Reg<HuffencAc067bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x60c);
            <utils::Reg<HuffencAc067bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_68(&self) -> utils::Reg<HuffencAc068bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x610);
            <utils::Reg<HuffencAc068bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_69(&self) -> utils::Reg<HuffencAc069bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x614);
            <utils::Reg<HuffencAc069bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_70(&self) -> utils::Reg<HuffencAc070bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x618);
            <utils::Reg<HuffencAc070bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_71(&self) -> utils::Reg<HuffencAc071bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x61c);
            <utils::Reg<HuffencAc071bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_72(&self) -> utils::Reg<HuffencAc072bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x620);
            <utils::Reg<HuffencAc072bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_73(&self) -> utils::Reg<HuffencAc073bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x624);
            <utils::Reg<HuffencAc073bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_74(&self) -> utils::Reg<HuffencAc074bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x628);
            <utils::Reg<HuffencAc074bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_75(&self) -> utils::Reg<HuffencAc075bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x62c);
            <utils::Reg<HuffencAc075bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_76(&self) -> utils::Reg<HuffencAc076bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x630);
            <utils::Reg<HuffencAc076bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_77(&self) -> utils::Reg<HuffencAc077bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x634);
            <utils::Reg<HuffencAc077bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_78(&self) -> utils::Reg<HuffencAc078bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x638);
            <utils::Reg<HuffencAc078bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_79(&self) -> utils::Reg<HuffencAc079bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x63c);
            <utils::Reg<HuffencAc079bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_80(&self) -> utils::Reg<HuffencAc080bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x640);
            <utils::Reg<HuffencAc080bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_81(&self) -> utils::Reg<HuffencAc081bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x644);
            <utils::Reg<HuffencAc081bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_82(&self) -> utils::Reg<HuffencAc082bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x648);
            <utils::Reg<HuffencAc082bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_83(&self) -> utils::Reg<HuffencAc083bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x64c);
            <utils::Reg<HuffencAc083bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_84(&self) -> utils::Reg<HuffencAc084bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x650);
            <utils::Reg<HuffencAc084bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_85(&self) -> utils::Reg<HuffencAc085bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x654);
            <utils::Reg<HuffencAc085bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_86(&self) -> utils::Reg<HuffencAc086bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x658);
            <utils::Reg<HuffencAc086bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 0"]
    pub const fn huffenc_ac0_87(&self) -> utils::Reg<HuffencAc087bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x65c);
            <utils::Reg<HuffencAc087bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_0(&self) -> utils::Reg<HuffencAc10bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x660);
            <utils::Reg<HuffencAc10bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_1(&self) -> utils::Reg<HuffencAc11bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x664);
            <utils::Reg<HuffencAc11bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_2(&self) -> utils::Reg<HuffencAc12bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x668);
            <utils::Reg<HuffencAc12bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_3(&self) -> utils::Reg<HuffencAc13bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x66c);
            <utils::Reg<HuffencAc13bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_4(&self) -> utils::Reg<HuffencAc14bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x670);
            <utils::Reg<HuffencAc14bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_5(&self) -> utils::Reg<HuffencAc15bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x674);
            <utils::Reg<HuffencAc15bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_6(&self) -> utils::Reg<HuffencAc16bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x678);
            <utils::Reg<HuffencAc16bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_7(&self) -> utils::Reg<HuffencAc17bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x67c);
            <utils::Reg<HuffencAc17bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_8(&self) -> utils::Reg<HuffencAc18bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x680);
            <utils::Reg<HuffencAc18bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_9(&self) -> utils::Reg<HuffencAc19bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x684);
            <utils::Reg<HuffencAc19bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_10(&self) -> utils::Reg<HuffencAc110bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x688);
            <utils::Reg<HuffencAc110bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_11(&self) -> utils::Reg<HuffencAc111bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x68c);
            <utils::Reg<HuffencAc111bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_12(&self) -> utils::Reg<HuffencAc112bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x690);
            <utils::Reg<HuffencAc112bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_13(&self) -> utils::Reg<HuffencAc113bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x694);
            <utils::Reg<HuffencAc113bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_14(&self) -> utils::Reg<HuffencAc114bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x698);
            <utils::Reg<HuffencAc114bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_15(&self) -> utils::Reg<HuffencAc115bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x69c);
            <utils::Reg<HuffencAc115bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_16(&self) -> utils::Reg<HuffencAc116bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6a0);
            <utils::Reg<HuffencAc116bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_17(&self) -> utils::Reg<HuffencAc117bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6a4);
            <utils::Reg<HuffencAc117bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_18(&self) -> utils::Reg<HuffencAc118bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6a8);
            <utils::Reg<HuffencAc118bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_19(&self) -> utils::Reg<HuffencAc119bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6ac);
            <utils::Reg<HuffencAc119bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_20(&self) -> utils::Reg<HuffencAc120bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6b0);
            <utils::Reg<HuffencAc120bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_21(&self) -> utils::Reg<HuffencAc121bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6b4);
            <utils::Reg<HuffencAc121bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_22(&self) -> utils::Reg<HuffencAc122bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6b8);
            <utils::Reg<HuffencAc122bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_23(&self) -> utils::Reg<HuffencAc123bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6bc);
            <utils::Reg<HuffencAc123bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_24(&self) -> utils::Reg<HuffencAc124bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6c0);
            <utils::Reg<HuffencAc124bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_25(&self) -> utils::Reg<HuffencAc125bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6c4);
            <utils::Reg<HuffencAc125bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_26(&self) -> utils::Reg<HuffencAc126bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6c8);
            <utils::Reg<HuffencAc126bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_27(&self) -> utils::Reg<HuffencAc127bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6cc);
            <utils::Reg<HuffencAc127bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_28(&self) -> utils::Reg<HuffencAc128bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6d0);
            <utils::Reg<HuffencAc128bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_29(&self) -> utils::Reg<HuffencAc129bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6d4);
            <utils::Reg<HuffencAc129bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_30(&self) -> utils::Reg<HuffencAc130bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6d8);
            <utils::Reg<HuffencAc130bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_31(&self) -> utils::Reg<HuffencAc131bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6dc);
            <utils::Reg<HuffencAc131bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_32(&self) -> utils::Reg<HuffencAc132bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6e0);
            <utils::Reg<HuffencAc132bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_33(&self) -> utils::Reg<HuffencAc133bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6e4);
            <utils::Reg<HuffencAc133bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_34(&self) -> utils::Reg<HuffencAc134bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6e8);
            <utils::Reg<HuffencAc134bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_35(&self) -> utils::Reg<HuffencAc135bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6ec);
            <utils::Reg<HuffencAc135bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_36(&self) -> utils::Reg<HuffencAc136bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6f0);
            <utils::Reg<HuffencAc136bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_37(&self) -> utils::Reg<HuffencAc137bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6f4);
            <utils::Reg<HuffencAc137bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_38(&self) -> utils::Reg<HuffencAc138bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6f8);
            <utils::Reg<HuffencAc138bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_39(&self) -> utils::Reg<HuffencAc139bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x6fc);
            <utils::Reg<HuffencAc139bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_40(&self) -> utils::Reg<HuffencAc140bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x700);
            <utils::Reg<HuffencAc140bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_41(&self) -> utils::Reg<HuffencAc141bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x704);
            <utils::Reg<HuffencAc141bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_42(&self) -> utils::Reg<HuffencAc142bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x708);
            <utils::Reg<HuffencAc142bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_43(&self) -> utils::Reg<HuffencAc143bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x70c);
            <utils::Reg<HuffencAc143bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_44(&self) -> utils::Reg<HuffencAc144bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x710);
            <utils::Reg<HuffencAc144bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_45(&self) -> utils::Reg<HuffencAc145bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x714);
            <utils::Reg<HuffencAc145bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_46(&self) -> utils::Reg<HuffencAc146bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x718);
            <utils::Reg<HuffencAc146bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_47(&self) -> utils::Reg<HuffencAc147bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x71c);
            <utils::Reg<HuffencAc147bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_48(&self) -> utils::Reg<HuffencAc148bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x720);
            <utils::Reg<HuffencAc148bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_49(&self) -> utils::Reg<HuffencAc149bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x724);
            <utils::Reg<HuffencAc149bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_50(&self) -> utils::Reg<HuffencAc150bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x728);
            <utils::Reg<HuffencAc150bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_51(&self) -> utils::Reg<HuffencAc151bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x72c);
            <utils::Reg<HuffencAc151bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_52(&self) -> utils::Reg<HuffencAc152bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x730);
            <utils::Reg<HuffencAc152bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_53(&self) -> utils::Reg<HuffencAc153bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x734);
            <utils::Reg<HuffencAc153bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_54(&self) -> utils::Reg<HuffencAc154bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x738);
            <utils::Reg<HuffencAc154bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_55(&self) -> utils::Reg<HuffencAc155bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x73c);
            <utils::Reg<HuffencAc155bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_56(&self) -> utils::Reg<HuffencAc156bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x740);
            <utils::Reg<HuffencAc156bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_57(&self) -> utils::Reg<HuffencAc157bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x744);
            <utils::Reg<HuffencAc157bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_58(&self) -> utils::Reg<HuffencAc158bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x748);
            <utils::Reg<HuffencAc158bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_59(&self) -> utils::Reg<HuffencAc159bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x74c);
            <utils::Reg<HuffencAc159bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_60(&self) -> utils::Reg<HuffencAc160bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x750);
            <utils::Reg<HuffencAc160bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_61(&self) -> utils::Reg<HuffencAc161bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x754);
            <utils::Reg<HuffencAc161bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_62(&self) -> utils::Reg<HuffencAc162bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x758);
            <utils::Reg<HuffencAc162bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_63(&self) -> utils::Reg<HuffencAc163bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x75c);
            <utils::Reg<HuffencAc163bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_64(&self) -> utils::Reg<HuffencAc164bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x760);
            <utils::Reg<HuffencAc164bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_65(&self) -> utils::Reg<HuffencAc165bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x764);
            <utils::Reg<HuffencAc165bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_66(&self) -> utils::Reg<HuffencAc166bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x768);
            <utils::Reg<HuffencAc166bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_67(&self) -> utils::Reg<HuffencAc167bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x76c);
            <utils::Reg<HuffencAc167bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_68(&self) -> utils::Reg<HuffencAc168bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x770);
            <utils::Reg<HuffencAc168bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_69(&self) -> utils::Reg<HuffencAc169bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x774);
            <utils::Reg<HuffencAc169bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_70(&self) -> utils::Reg<HuffencAc170bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x778);
            <utils::Reg<HuffencAc170bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_71(&self) -> utils::Reg<HuffencAc171bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x77c);
            <utils::Reg<HuffencAc171bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_72(&self) -> utils::Reg<HuffencAc172bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x780);
            <utils::Reg<HuffencAc172bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_73(&self) -> utils::Reg<HuffencAc173bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x784);
            <utils::Reg<HuffencAc173bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_74(&self) -> utils::Reg<HuffencAc174bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x788);
            <utils::Reg<HuffencAc174bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_75(&self) -> utils::Reg<HuffencAc175bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x78c);
            <utils::Reg<HuffencAc175bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_76(&self) -> utils::Reg<HuffencAc176bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x790);
            <utils::Reg<HuffencAc176bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_77(&self) -> utils::Reg<HuffencAc177bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x794);
            <utils::Reg<HuffencAc177bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_78(&self) -> utils::Reg<HuffencAc178bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x798);
            <utils::Reg<HuffencAc178bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_79(&self) -> utils::Reg<HuffencAc179bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x79c);
            <utils::Reg<HuffencAc179bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_80(&self) -> utils::Reg<HuffencAc180bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7a0);
            <utils::Reg<HuffencAc180bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_81(&self) -> utils::Reg<HuffencAc181bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7a4);
            <utils::Reg<HuffencAc181bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_82(&self) -> utils::Reg<HuffencAc182bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7a8);
            <utils::Reg<HuffencAc182bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_83(&self) -> utils::Reg<HuffencAc183bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7ac);
            <utils::Reg<HuffencAc183bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_84(&self) -> utils::Reg<HuffencAc184bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7b0);
            <utils::Reg<HuffencAc184bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_85(&self) -> utils::Reg<HuffencAc185bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7b4);
            <utils::Reg<HuffencAc185bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_86(&self) -> utils::Reg<HuffencAc186bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7b8);
            <utils::Reg<HuffencAc186bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, AC Huffman table 1"]
    pub const fn huffenc_ac1_87(&self) -> utils::Reg<HuffencAc187bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7bc);
            <utils::Reg<HuffencAc187bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_0(&self) -> utils::Reg<HuffencDc00bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7c0);
            <utils::Reg<HuffencDc00bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_1(&self) -> utils::Reg<HuffencDc01bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7c4);
            <utils::Reg<HuffencDc01bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_2(&self) -> utils::Reg<HuffencDc02bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7c8);
            <utils::Reg<HuffencDc02bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_3(&self) -> utils::Reg<HuffencDc03bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7cc);
            <utils::Reg<HuffencDc03bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_4(&self) -> utils::Reg<HuffencDc04bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7d0);
            <utils::Reg<HuffencDc04bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_5(&self) -> utils::Reg<HuffencDc05bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7d4);
            <utils::Reg<HuffencDc05bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_6(&self) -> utils::Reg<HuffencDc06bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7d8);
            <utils::Reg<HuffencDc06bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 0"]
    pub const fn huffenc_dc0_7(&self) -> utils::Reg<HuffencDc07bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7dc);
            <utils::Reg<HuffencDc07bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_0(&self) -> utils::Reg<HuffencDc10bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7e0);
            <utils::Reg<HuffencDc10bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_1(&self) -> utils::Reg<HuffencDc11bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7e4);
            <utils::Reg<HuffencDc11bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_2(&self) -> utils::Reg<HuffencDc12bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7e8);
            <utils::Reg<HuffencDc12bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_3(&self) -> utils::Reg<HuffencDc13bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7ec);
            <utils::Reg<HuffencDc13bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_4(&self) -> utils::Reg<HuffencDc14bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7f0);
            <utils::Reg<HuffencDc14bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_5(&self) -> utils::Reg<HuffencDc15bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7f4);
            <utils::Reg<HuffencDc15bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_6(&self) -> utils::Reg<HuffencDc16bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7f8);
            <utils::Reg<HuffencDc16bits, utils::RW>>::from_ptr(ptr)
        }
    }
    #[inline(always)]
    #[doc = "JPEG encoder, DC Huffman table 1"]
    pub const fn huffenc_dc1_7(&self) -> utils::Reg<HuffencDc17bits, utils::RW> {
        unsafe {
            let ptr = self.ptr.add(0x7fc);
            <utils::Reg<HuffencDc17bits, utils::RW>>::from_ptr(ptr)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[doc = "JPEG DHTMem tables"]
pub struct Dhtmem0Bits {
    bits: u32,
}
impl Default for Dhtmem0Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem0Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem10Bits {
    bits: u32,
}
impl Default for Dhtmem10Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem10Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem100Bits {
    bits: u32,
}
impl Default for Dhtmem100Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem100Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem101Bits {
    bits: u32,
}
impl Default for Dhtmem101Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem101Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem102Bits {
    bits: u32,
}
impl Default for Dhtmem102Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem102Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem103Bits {
    bits: u32,
}
impl Default for Dhtmem103Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem103Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem11Bits {
    bits: u32,
}
impl Default for Dhtmem11Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem11Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem12Bits {
    bits: u32,
}
impl Default for Dhtmem12Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem12Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem13Bits {
    bits: u32,
}
impl Default for Dhtmem13Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem13Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem14Bits {
    bits: u32,
}
impl Default for Dhtmem14Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem14Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem15Bits {
    bits: u32,
}
impl Default for Dhtmem15Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem15Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem16Bits {
    bits: u32,
}
impl Default for Dhtmem16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem16Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem17Bits {
    bits: u32,
}
impl Default for Dhtmem17Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem17Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem18Bits {
    bits: u32,
}
impl Default for Dhtmem18Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem18Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem19Bits {
    bits: u32,
}
impl Default for Dhtmem19Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem19Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem2Bits {
    bits: u32,
}
impl Default for Dhtmem2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem2Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem20Bits {
    bits: u32,
}
impl Default for Dhtmem20Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem20Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem21Bits {
    bits: u32,
}
impl Default for Dhtmem21Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem21Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem22Bits {
    bits: u32,
}
impl Default for Dhtmem22Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem22Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem23Bits {
    bits: u32,
}
impl Default for Dhtmem23Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem23Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem24Bits {
    bits: u32,
}
impl Default for Dhtmem24Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem24Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem25Bits {
    bits: u32,
}
impl Default for Dhtmem25Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem25Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem26Bits {
    bits: u32,
}
impl Default for Dhtmem26Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem26Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem27Bits {
    bits: u32,
}
impl Default for Dhtmem27Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem27Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem28Bits {
    bits: u32,
}
impl Default for Dhtmem28Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem28Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem29Bits {
    bits: u32,
}
impl Default for Dhtmem29Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem29Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem3Bits {
    bits: u32,
}
impl Default for Dhtmem3Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem3Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem30Bits {
    bits: u32,
}
impl Default for Dhtmem30Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem30Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem31Bits {
    bits: u32,
}
impl Default for Dhtmem31Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem31Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem32Bits {
    bits: u32,
}
impl Default for Dhtmem32Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem32Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem33Bits {
    bits: u32,
}
impl Default for Dhtmem33Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem33Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem34Bits {
    bits: u32,
}
impl Default for Dhtmem34Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem34Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem35Bits {
    bits: u32,
}
impl Default for Dhtmem35Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem35Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem36Bits {
    bits: u32,
}
impl Default for Dhtmem36Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem36Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem37Bits {
    bits: u32,
}
impl Default for Dhtmem37Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem37Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem38Bits {
    bits: u32,
}
impl Default for Dhtmem38Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem38Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem39Bits {
    bits: u32,
}
impl Default for Dhtmem39Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem39Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem4Bits {
    bits: u32,
}
impl Default for Dhtmem4Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem4Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem40Bits {
    bits: u32,
}
impl Default for Dhtmem40Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem40Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem41Bits {
    bits: u32,
}
impl Default for Dhtmem41Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem41Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem42Bits {
    bits: u32,
}
impl Default for Dhtmem42Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem42Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem43Bits {
    bits: u32,
}
impl Default for Dhtmem43Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem43Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem44Bits {
    bits: u32,
}
impl Default for Dhtmem44Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem44Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem45Bits {
    bits: u32,
}
impl Default for Dhtmem45Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem45Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem46Bits {
    bits: u32,
}
impl Default for Dhtmem46Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem46Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem47Bits {
    bits: u32,
}
impl Default for Dhtmem47Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem47Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem48Bits {
    bits: u32,
}
impl Default for Dhtmem48Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem48Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem49Bits {
    bits: u32,
}
impl Default for Dhtmem49Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem49Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem5Bits {
    bits: u32,
}
impl Default for Dhtmem5Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem5Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem50Bits {
    bits: u32,
}
impl Default for Dhtmem50Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem50Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem51Bits {
    bits: u32,
}
impl Default for Dhtmem51Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem51Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem52Bits {
    bits: u32,
}
impl Default for Dhtmem52Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem52Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem53Bits {
    bits: u32,
}
impl Default for Dhtmem53Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem53Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem54Bits {
    bits: u32,
}
impl Default for Dhtmem54Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem54Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem55Bits {
    bits: u32,
}
impl Default for Dhtmem55Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem55Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem56Bits {
    bits: u32,
}
impl Default for Dhtmem56Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem56Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem57Bits {
    bits: u32,
}
impl Default for Dhtmem57Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem57Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem58Bits {
    bits: u32,
}
impl Default for Dhtmem58Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem58Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem59Bits {
    bits: u32,
}
impl Default for Dhtmem59Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem59Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem6Bits {
    bits: u32,
}
impl Default for Dhtmem6Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem6Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem60Bits {
    bits: u32,
}
impl Default for Dhtmem60Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem60Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem61Bits {
    bits: u32,
}
impl Default for Dhtmem61Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem61Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem62Bits {
    bits: u32,
}
impl Default for Dhtmem62Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem62Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem63Bits {
    bits: u32,
}
impl Default for Dhtmem63Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem63Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem64Bits {
    bits: u32,
}
impl Default for Dhtmem64Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem64Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem65Bits {
    bits: u32,
}
impl Default for Dhtmem65Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem65Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem66Bits {
    bits: u32,
}
impl Default for Dhtmem66Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem66Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem67Bits {
    bits: u32,
}
impl Default for Dhtmem67Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem67Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem68Bits {
    bits: u32,
}
impl Default for Dhtmem68Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem68Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem69Bits {
    bits: u32,
}
impl Default for Dhtmem69Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem69Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem7Bits {
    bits: u32,
}
impl Default for Dhtmem7Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem7Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem70Bits {
    bits: u32,
}
impl Default for Dhtmem70Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem70Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem71Bits {
    bits: u32,
}
impl Default for Dhtmem71Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem71Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem72Bits {
    bits: u32,
}
impl Default for Dhtmem72Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem72Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem73Bits {
    bits: u32,
}
impl Default for Dhtmem73Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem73Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem74Bits {
    bits: u32,
}
impl Default for Dhtmem74Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem74Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem75Bits {
    bits: u32,
}
impl Default for Dhtmem75Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem75Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem76Bits {
    bits: u32,
}
impl Default for Dhtmem76Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem76Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem77Bits {
    bits: u32,
}
impl Default for Dhtmem77Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem77Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem78Bits {
    bits: u32,
}
impl Default for Dhtmem78Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem78Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem79Bits {
    bits: u32,
}
impl Default for Dhtmem79Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem79Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem8Bits {
    bits: u32,
}
impl Default for Dhtmem8Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem8Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem80Bits {
    bits: u32,
}
impl Default for Dhtmem80Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem80Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem81Bits {
    bits: u32,
}
impl Default for Dhtmem81Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem81Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem82Bits {
    bits: u32,
}
impl Default for Dhtmem82Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem82Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem83Bits {
    bits: u32,
}
impl Default for Dhtmem83Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem83Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem84Bits {
    bits: u32,
}
impl Default for Dhtmem84Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem84Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem85Bits {
    bits: u32,
}
impl Default for Dhtmem85Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem85Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem86Bits {
    bits: u32,
}
impl Default for Dhtmem86Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem86Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem87Bits {
    bits: u32,
}
impl Default for Dhtmem87Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem87Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem88Bits {
    bits: u32,
}
impl Default for Dhtmem88Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem88Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem89Bits {
    bits: u32,
}
impl Default for Dhtmem89Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem89Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem9Bits {
    bits: u32,
}
impl Default for Dhtmem9Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem9Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem90Bits {
    bits: u32,
}
impl Default for Dhtmem90Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem90Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem91Bits {
    bits: u32,
}
impl Default for Dhtmem91Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem91Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem92Bits {
    bits: u32,
}
impl Default for Dhtmem92Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem92Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem93Bits {
    bits: u32,
}
impl Default for Dhtmem93Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem93Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem94Bits {
    bits: u32,
}
impl Default for Dhtmem94Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem94Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem95Bits {
    bits: u32,
}
impl Default for Dhtmem95Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem95Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem96Bits {
    bits: u32,
}
impl Default for Dhtmem96Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem96Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem97Bits {
    bits: u32,
}
impl Default for Dhtmem97Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem97Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem98Bits {
    bits: u32,
}
impl Default for Dhtmem98Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem98Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Dhtmem99Bits {
    bits: u32,
}
impl Default for Dhtmem99Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Dhtmem99Bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Huffbase0Bits {
    bits: u32,
}
impl Default for Huffbase0Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase0Bits {
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
pub struct Huffbase1Bits {
    bits: u32,
}
impl Default for Huffbase1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase1Bits {
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
pub struct Huffbase10Bits {
    bits: u32,
}
impl Default for Huffbase10Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase10Bits {
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
pub struct Huffbase11Bits {
    bits: u32,
}
impl Default for Huffbase11Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase11Bits {
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
pub struct Huffbase12Bits {
    bits: u32,
}
impl Default for Huffbase12Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase12Bits {
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
pub struct Huffbase13Bits {
    bits: u32,
}
impl Default for Huffbase13Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase13Bits {
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
pub struct Huffbase14Bits {
    bits: u32,
}
impl Default for Huffbase14Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase14Bits {
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
pub struct Huffbase15Bits {
    bits: u32,
}
impl Default for Huffbase15Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase15Bits {
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
pub struct Huffbase16Bits {
    bits: u32,
}
impl Default for Huffbase16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase16Bits {
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
pub struct Huffbase17Bits {
    bits: u32,
}
impl Default for Huffbase17Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase17Bits {
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
pub struct Huffbase18Bits {
    bits: u32,
}
impl Default for Huffbase18Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase18Bits {
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
pub struct Huffbase19Bits {
    bits: u32,
}
impl Default for Huffbase19Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase19Bits {
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
pub struct Huffbase2Bits {
    bits: u32,
}
impl Default for Huffbase2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase2Bits {
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
pub struct Huffbase20Bits {
    bits: u32,
}
impl Default for Huffbase20Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase20Bits {
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
pub struct Huffbase21Bits {
    bits: u32,
}
impl Default for Huffbase21Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase21Bits {
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
pub struct Huffbase22Bits {
    bits: u32,
}
impl Default for Huffbase22Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase22Bits {
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
pub struct Huffbase23Bits {
    bits: u32,
}
impl Default for Huffbase23Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase23Bits {
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
pub struct Huffbase24Bits {
    bits: u32,
}
impl Default for Huffbase24Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase24Bits {
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
pub struct Huffbase25Bits {
    bits: u32,
}
impl Default for Huffbase25Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase25Bits {
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
pub struct Huffbase26Bits {
    bits: u32,
}
impl Default for Huffbase26Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase26Bits {
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
pub struct Huffbase27Bits {
    bits: u32,
}
impl Default for Huffbase27Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase27Bits {
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
pub struct Huffbase28Bits {
    bits: u32,
}
impl Default for Huffbase28Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase28Bits {
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
pub struct Huffbase29Bits {
    bits: u32,
}
impl Default for Huffbase29Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase29Bits {
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
pub struct Huffbase3Bits {
    bits: u32,
}
impl Default for Huffbase3Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase3Bits {
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
pub struct Huffbase30Bits {
    bits: u32,
}
impl Default for Huffbase30Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase30Bits {
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
pub struct Huffbase31Bits {
    bits: u32,
}
impl Default for Huffbase31Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase31Bits {
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
pub struct Huffbase4Bits {
    bits: u32,
}
impl Default for Huffbase4Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase4Bits {
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
pub struct Huffbase5Bits {
    bits: u32,
}
impl Default for Huffbase5Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase5Bits {
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
pub struct Huffbase6Bits {
    bits: u32,
}
impl Default for Huffbase6Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase6Bits {
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
pub struct Huffbase7Bits {
    bits: u32,
}
impl Default for Huffbase7Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase7Bits {
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
pub struct Huffbase8Bits {
    bits: u32,
}
impl Default for Huffbase8Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase8Bits {
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
pub struct Huffbase9Bits {
    bits: u32,
}
impl Default for Huffbase9Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffbase9Bits {
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
pub struct HuffencAc00bits {
    bits: u32,
}
impl Default for HuffencAc00bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc00bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc01bits {
    bits: u32,
}
impl Default for HuffencAc01bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc01bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc010bits {
    bits: u32,
}
impl Default for HuffencAc010bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc010bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc011bits {
    bits: u32,
}
impl Default for HuffencAc011bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc011bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc012bits {
    bits: u32,
}
impl Default for HuffencAc012bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc012bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc013bits {
    bits: u32,
}
impl Default for HuffencAc013bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc013bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc014bits {
    bits: u32,
}
impl Default for HuffencAc014bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc014bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc015bits {
    bits: u32,
}
impl Default for HuffencAc015bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc015bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc016bits {
    bits: u32,
}
impl Default for HuffencAc016bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc016bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc017bits {
    bits: u32,
}
impl Default for HuffencAc017bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc017bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc018bits {
    bits: u32,
}
impl Default for HuffencAc018bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc018bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc019bits {
    bits: u32,
}
impl Default for HuffencAc019bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc019bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc02bits {
    bits: u32,
}
impl Default for HuffencAc02bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc02bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc020bits {
    bits: u32,
}
impl Default for HuffencAc020bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc020bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc021bits {
    bits: u32,
}
impl Default for HuffencAc021bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc021bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc022bits {
    bits: u32,
}
impl Default for HuffencAc022bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc022bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc023bits {
    bits: u32,
}
impl Default for HuffencAc023bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc023bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc024bits {
    bits: u32,
}
impl Default for HuffencAc024bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc024bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc025bits {
    bits: u32,
}
impl Default for HuffencAc025bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc025bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc026bits {
    bits: u32,
}
impl Default for HuffencAc026bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc026bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc027bits {
    bits: u32,
}
impl Default for HuffencAc027bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc027bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc028bits {
    bits: u32,
}
impl Default for HuffencAc028bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc028bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc029bits {
    bits: u32,
}
impl Default for HuffencAc029bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc029bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc03bits {
    bits: u32,
}
impl Default for HuffencAc03bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc03bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc030bits {
    bits: u32,
}
impl Default for HuffencAc030bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc030bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc031bits {
    bits: u32,
}
impl Default for HuffencAc031bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc031bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc032bits {
    bits: u32,
}
impl Default for HuffencAc032bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc032bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc033bits {
    bits: u32,
}
impl Default for HuffencAc033bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc033bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc034bits {
    bits: u32,
}
impl Default for HuffencAc034bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc034bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc035bits {
    bits: u32,
}
impl Default for HuffencAc035bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc035bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc036bits {
    bits: u32,
}
impl Default for HuffencAc036bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc036bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc037bits {
    bits: u32,
}
impl Default for HuffencAc037bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc037bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc038bits {
    bits: u32,
}
impl Default for HuffencAc038bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc038bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc039bits {
    bits: u32,
}
impl Default for HuffencAc039bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc039bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc04bits {
    bits: u32,
}
impl Default for HuffencAc04bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc04bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc040bits {
    bits: u32,
}
impl Default for HuffencAc040bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc040bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc041bits {
    bits: u32,
}
impl Default for HuffencAc041bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc041bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc042bits {
    bits: u32,
}
impl Default for HuffencAc042bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc042bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc043bits {
    bits: u32,
}
impl Default for HuffencAc043bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc043bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc044bits {
    bits: u32,
}
impl Default for HuffencAc044bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc044bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc045bits {
    bits: u32,
}
impl Default for HuffencAc045bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc045bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc046bits {
    bits: u32,
}
impl Default for HuffencAc046bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc046bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc047bits {
    bits: u32,
}
impl Default for HuffencAc047bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc047bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc048bits {
    bits: u32,
}
impl Default for HuffencAc048bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc048bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc049bits {
    bits: u32,
}
impl Default for HuffencAc049bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc049bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc05bits {
    bits: u32,
}
impl Default for HuffencAc05bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc05bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc050bits {
    bits: u32,
}
impl Default for HuffencAc050bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc050bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc051bits {
    bits: u32,
}
impl Default for HuffencAc051bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc051bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc052bits {
    bits: u32,
}
impl Default for HuffencAc052bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc052bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc053bits {
    bits: u32,
}
impl Default for HuffencAc053bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc053bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc054bits {
    bits: u32,
}
impl Default for HuffencAc054bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc054bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc055bits {
    bits: u32,
}
impl Default for HuffencAc055bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc055bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc056bits {
    bits: u32,
}
impl Default for HuffencAc056bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc056bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc057bits {
    bits: u32,
}
impl Default for HuffencAc057bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc057bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc058bits {
    bits: u32,
}
impl Default for HuffencAc058bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc058bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc059bits {
    bits: u32,
}
impl Default for HuffencAc059bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc059bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc06bits {
    bits: u32,
}
impl Default for HuffencAc06bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc06bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc060bits {
    bits: u32,
}
impl Default for HuffencAc060bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc060bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc061bits {
    bits: u32,
}
impl Default for HuffencAc061bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc061bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc062bits {
    bits: u32,
}
impl Default for HuffencAc062bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc062bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc063bits {
    bits: u32,
}
impl Default for HuffencAc063bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc063bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc064bits {
    bits: u32,
}
impl Default for HuffencAc064bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc064bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc065bits {
    bits: u32,
}
impl Default for HuffencAc065bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc065bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc066bits {
    bits: u32,
}
impl Default for HuffencAc066bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc066bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc067bits {
    bits: u32,
}
impl Default for HuffencAc067bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc067bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc068bits {
    bits: u32,
}
impl Default for HuffencAc068bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc068bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc069bits {
    bits: u32,
}
impl Default for HuffencAc069bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc069bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc07bits {
    bits: u32,
}
impl Default for HuffencAc07bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc07bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc070bits {
    bits: u32,
}
impl Default for HuffencAc070bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc070bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc071bits {
    bits: u32,
}
impl Default for HuffencAc071bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc071bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc072bits {
    bits: u32,
}
impl Default for HuffencAc072bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc072bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc073bits {
    bits: u32,
}
impl Default for HuffencAc073bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc073bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc074bits {
    bits: u32,
}
impl Default for HuffencAc074bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc074bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc075bits {
    bits: u32,
}
impl Default for HuffencAc075bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc075bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc076bits {
    bits: u32,
}
impl Default for HuffencAc076bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc076bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc077bits {
    bits: u32,
}
impl Default for HuffencAc077bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc077bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc078bits {
    bits: u32,
}
impl Default for HuffencAc078bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc078bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc079bits {
    bits: u32,
}
impl Default for HuffencAc079bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc079bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc08bits {
    bits: u32,
}
impl Default for HuffencAc08bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc08bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc080bits {
    bits: u32,
}
impl Default for HuffencAc080bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc080bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc081bits {
    bits: u32,
}
impl Default for HuffencAc081bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc081bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc082bits {
    bits: u32,
}
impl Default for HuffencAc082bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc082bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc083bits {
    bits: u32,
}
impl Default for HuffencAc083bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc083bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc084bits {
    bits: u32,
}
impl Default for HuffencAc084bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc084bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc085bits {
    bits: u32,
}
impl Default for HuffencAc085bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc085bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc086bits {
    bits: u32,
}
impl Default for HuffencAc086bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc086bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc087bits {
    bits: u32,
}
impl Default for HuffencAc087bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc087bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc09bits {
    bits: u32,
}
impl Default for HuffencAc09bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc09bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc10bits {
    bits: u32,
}
impl Default for HuffencAc10bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc10bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc11bits {
    bits: u32,
}
impl Default for HuffencAc11bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc11bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc110bits {
    bits: u32,
}
impl Default for HuffencAc110bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc110bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc111bits {
    bits: u32,
}
impl Default for HuffencAc111bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc111bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc112bits {
    bits: u32,
}
impl Default for HuffencAc112bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc112bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc113bits {
    bits: u32,
}
impl Default for HuffencAc113bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc113bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc114bits {
    bits: u32,
}
impl Default for HuffencAc114bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc114bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc115bits {
    bits: u32,
}
impl Default for HuffencAc115bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc115bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc116bits {
    bits: u32,
}
impl Default for HuffencAc116bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc116bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc117bits {
    bits: u32,
}
impl Default for HuffencAc117bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc117bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc118bits {
    bits: u32,
}
impl Default for HuffencAc118bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc118bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc119bits {
    bits: u32,
}
impl Default for HuffencAc119bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc119bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc12bits {
    bits: u32,
}
impl Default for HuffencAc12bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc12bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc120bits {
    bits: u32,
}
impl Default for HuffencAc120bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc120bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc121bits {
    bits: u32,
}
impl Default for HuffencAc121bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc121bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc122bits {
    bits: u32,
}
impl Default for HuffencAc122bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc122bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc123bits {
    bits: u32,
}
impl Default for HuffencAc123bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc123bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc124bits {
    bits: u32,
}
impl Default for HuffencAc124bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc124bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc125bits {
    bits: u32,
}
impl Default for HuffencAc125bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc125bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc126bits {
    bits: u32,
}
impl Default for HuffencAc126bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc126bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc127bits {
    bits: u32,
}
impl Default for HuffencAc127bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc127bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc128bits {
    bits: u32,
}
impl Default for HuffencAc128bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc128bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc129bits {
    bits: u32,
}
impl Default for HuffencAc129bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc129bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc13bits {
    bits: u32,
}
impl Default for HuffencAc13bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc13bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc130bits {
    bits: u32,
}
impl Default for HuffencAc130bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc130bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc131bits {
    bits: u32,
}
impl Default for HuffencAc131bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc131bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc132bits {
    bits: u32,
}
impl Default for HuffencAc132bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc132bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc133bits {
    bits: u32,
}
impl Default for HuffencAc133bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc133bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc134bits {
    bits: u32,
}
impl Default for HuffencAc134bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc134bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc135bits {
    bits: u32,
}
impl Default for HuffencAc135bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc135bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc136bits {
    bits: u32,
}
impl Default for HuffencAc136bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc136bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc137bits {
    bits: u32,
}
impl Default for HuffencAc137bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc137bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc138bits {
    bits: u32,
}
impl Default for HuffencAc138bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc138bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc139bits {
    bits: u32,
}
impl Default for HuffencAc139bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc139bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc14bits {
    bits: u32,
}
impl Default for HuffencAc14bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc14bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc140bits {
    bits: u32,
}
impl Default for HuffencAc140bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc140bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc141bits {
    bits: u32,
}
impl Default for HuffencAc141bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc141bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc142bits {
    bits: u32,
}
impl Default for HuffencAc142bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc142bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc143bits {
    bits: u32,
}
impl Default for HuffencAc143bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc143bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc144bits {
    bits: u32,
}
impl Default for HuffencAc144bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc144bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc145bits {
    bits: u32,
}
impl Default for HuffencAc145bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc145bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc146bits {
    bits: u32,
}
impl Default for HuffencAc146bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc146bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc147bits {
    bits: u32,
}
impl Default for HuffencAc147bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc147bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc148bits {
    bits: u32,
}
impl Default for HuffencAc148bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc148bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc149bits {
    bits: u32,
}
impl Default for HuffencAc149bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc149bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc15bits {
    bits: u32,
}
impl Default for HuffencAc15bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc15bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc150bits {
    bits: u32,
}
impl Default for HuffencAc150bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc150bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc151bits {
    bits: u32,
}
impl Default for HuffencAc151bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc151bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc152bits {
    bits: u32,
}
impl Default for HuffencAc152bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc152bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc153bits {
    bits: u32,
}
impl Default for HuffencAc153bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc153bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc154bits {
    bits: u32,
}
impl Default for HuffencAc154bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc154bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc155bits {
    bits: u32,
}
impl Default for HuffencAc155bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc155bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc156bits {
    bits: u32,
}
impl Default for HuffencAc156bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc156bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc157bits {
    bits: u32,
}
impl Default for HuffencAc157bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc157bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc158bits {
    bits: u32,
}
impl Default for HuffencAc158bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc158bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc159bits {
    bits: u32,
}
impl Default for HuffencAc159bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc159bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc16bits {
    bits: u32,
}
impl Default for HuffencAc16bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc16bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc160bits {
    bits: u32,
}
impl Default for HuffencAc160bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc160bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc161bits {
    bits: u32,
}
impl Default for HuffencAc161bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc161bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc162bits {
    bits: u32,
}
impl Default for HuffencAc162bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc162bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc163bits {
    bits: u32,
}
impl Default for HuffencAc163bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc163bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc164bits {
    bits: u32,
}
impl Default for HuffencAc164bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc164bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc165bits {
    bits: u32,
}
impl Default for HuffencAc165bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc165bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc166bits {
    bits: u32,
}
impl Default for HuffencAc166bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc166bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc167bits {
    bits: u32,
}
impl Default for HuffencAc167bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc167bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc168bits {
    bits: u32,
}
impl Default for HuffencAc168bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc168bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc169bits {
    bits: u32,
}
impl Default for HuffencAc169bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc169bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc17bits {
    bits: u32,
}
impl Default for HuffencAc17bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc17bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc170bits {
    bits: u32,
}
impl Default for HuffencAc170bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc170bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc171bits {
    bits: u32,
}
impl Default for HuffencAc171bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc171bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc172bits {
    bits: u32,
}
impl Default for HuffencAc172bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc172bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc173bits {
    bits: u32,
}
impl Default for HuffencAc173bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc173bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc174bits {
    bits: u32,
}
impl Default for HuffencAc174bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc174bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc175bits {
    bits: u32,
}
impl Default for HuffencAc175bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc175bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc176bits {
    bits: u32,
}
impl Default for HuffencAc176bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc176bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc177bits {
    bits: u32,
}
impl Default for HuffencAc177bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc177bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc178bits {
    bits: u32,
}
impl Default for HuffencAc178bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc178bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc179bits {
    bits: u32,
}
impl Default for HuffencAc179bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc179bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc18bits {
    bits: u32,
}
impl Default for HuffencAc18bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc18bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc180bits {
    bits: u32,
}
impl Default for HuffencAc180bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc180bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc181bits {
    bits: u32,
}
impl Default for HuffencAc181bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc181bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc182bits {
    bits: u32,
}
impl Default for HuffencAc182bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc182bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc183bits {
    bits: u32,
}
impl Default for HuffencAc183bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc183bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc184bits {
    bits: u32,
}
impl Default for HuffencAc184bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc184bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc185bits {
    bits: u32,
}
impl Default for HuffencAc185bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc185bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc186bits {
    bits: u32,
}
impl Default for HuffencAc186bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc186bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc187bits {
    bits: u32,
}
impl Default for HuffencAc187bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc187bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencAc19bits {
    bits: u32,
}
impl Default for HuffencAc19bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencAc19bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc00bits {
    bits: u32,
}
impl Default for HuffencDc00bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc00bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc01bits {
    bits: u32,
}
impl Default for HuffencDc01bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc01bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc02bits {
    bits: u32,
}
impl Default for HuffencDc02bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc02bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc03bits {
    bits: u32,
}
impl Default for HuffencDc03bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc03bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc04bits {
    bits: u32,
}
impl Default for HuffencDc04bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc04bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc05bits {
    bits: u32,
}
impl Default for HuffencDc05bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc05bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc06bits {
    bits: u32,
}
impl Default for HuffencDc06bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc06bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc07bits {
    bits: u32,
}
impl Default for HuffencDc07bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc07bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc10bits {
    bits: u32,
}
impl Default for HuffencDc10bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc10bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc11bits {
    bits: u32,
}
impl Default for HuffencDc11bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc11bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc12bits {
    bits: u32,
}
impl Default for HuffencDc12bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc12bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc13bits {
    bits: u32,
}
impl Default for HuffencDc13bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc13bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc14bits {
    bits: u32,
}
impl Default for HuffencDc14bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc14bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc15bits {
    bits: u32,
}
impl Default for HuffencDc15bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc15bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc16bits {
    bits: u32,
}
impl Default for HuffencDc16bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc16bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct HuffencDc17bits {
    bits: u32,
}
impl Default for HuffencDc17bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl HuffencDc17bits {
    #[inline(always)]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self { bits }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.bits
    }
    #[inline(always)]
    #[doc = "DHTMem RAM"]
    pub const fn set_dht_mem_ram(mut self, val: u32) -> Self {
        self.bits &= !(0xffffffff << 0x0);
        self.bits |= (val as u32 & 0xffffffff) << 0x0;
        self
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
pub struct Huffmin0bits {
    bits: u32,
}
impl Default for Huffmin0bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin0bits {
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
pub struct Huffmin1bits {
    bits: u32,
}
impl Default for Huffmin1bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin1bits {
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
pub struct Huffmin10bits {
    bits: u32,
}
impl Default for Huffmin10bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin10bits {
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
pub struct Huffmin11bits {
    bits: u32,
}
impl Default for Huffmin11bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin11bits {
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
pub struct Huffmin12bits {
    bits: u32,
}
impl Default for Huffmin12bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin12bits {
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
pub struct Huffmin13bits {
    bits: u32,
}
impl Default for Huffmin13bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin13bits {
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
pub struct Huffmin14bits {
    bits: u32,
}
impl Default for Huffmin14bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin14bits {
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
pub struct Huffmin15bits {
    bits: u32,
}
impl Default for Huffmin15bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin15bits {
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
pub struct Huffmin2bits {
    bits: u32,
}
impl Default for Huffmin2bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin2bits {
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
pub struct Huffmin3bits {
    bits: u32,
}
impl Default for Huffmin3bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin3bits {
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
pub struct Huffmin4bits {
    bits: u32,
}
impl Default for Huffmin4bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin4bits {
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
pub struct Huffmin5bits {
    bits: u32,
}
impl Default for Huffmin5bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin5bits {
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
pub struct Huffmin6bits {
    bits: u32,
}
impl Default for Huffmin6bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin6bits {
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
pub struct Huffmin7bits {
    bits: u32,
}
impl Default for Huffmin7bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin7bits {
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
pub struct Huffmin8bits {
    bits: u32,
}
impl Default for Huffmin8bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin8bits {
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
pub struct Huffmin9bits {
    bits: u32,
}
impl Default for Huffmin9bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffmin9bits {
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
pub struct Huffsymb0Bits {
    bits: u32,
}
impl Default for Huffsymb0Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb0Bits {
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
pub struct Huffsymb1Bits {
    bits: u32,
}
impl Default for Huffsymb1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb1Bits {
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
pub struct Huffsymb10Bits {
    bits: u32,
}
impl Default for Huffsymb10Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb10Bits {
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
pub struct Huffsymb11Bits {
    bits: u32,
}
impl Default for Huffsymb11Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb11Bits {
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
pub struct Huffsymb12Bits {
    bits: u32,
}
impl Default for Huffsymb12Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb12Bits {
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
pub struct Huffsymb13Bits {
    bits: u32,
}
impl Default for Huffsymb13Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb13Bits {
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
pub struct Huffsymb14Bits {
    bits: u32,
}
impl Default for Huffsymb14Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb14Bits {
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
pub struct Huffsymb15Bits {
    bits: u32,
}
impl Default for Huffsymb15Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb15Bits {
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
pub struct Huffsymb16Bits {
    bits: u32,
}
impl Default for Huffsymb16Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb16Bits {
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
pub struct Huffsymb17Bits {
    bits: u32,
}
impl Default for Huffsymb17Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb17Bits {
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
pub struct Huffsymb18Bits {
    bits: u32,
}
impl Default for Huffsymb18Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb18Bits {
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
pub struct Huffsymb19Bits {
    bits: u32,
}
impl Default for Huffsymb19Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb19Bits {
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
pub struct Huffsymb2Bits {
    bits: u32,
}
impl Default for Huffsymb2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb2Bits {
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
pub struct Huffsymb20Bits {
    bits: u32,
}
impl Default for Huffsymb20Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb20Bits {
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
pub struct Huffsymb21Bits {
    bits: u32,
}
impl Default for Huffsymb21Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb21Bits {
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
pub struct Huffsymb22Bits {
    bits: u32,
}
impl Default for Huffsymb22Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb22Bits {
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
pub struct Huffsymb23Bits {
    bits: u32,
}
impl Default for Huffsymb23Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb23Bits {
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
pub struct Huffsymb24Bits {
    bits: u32,
}
impl Default for Huffsymb24Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb24Bits {
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
pub struct Huffsymb25Bits {
    bits: u32,
}
impl Default for Huffsymb25Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb25Bits {
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
pub struct Huffsymb26Bits {
    bits: u32,
}
impl Default for Huffsymb26Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb26Bits {
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
pub struct Huffsymb27Bits {
    bits: u32,
}
impl Default for Huffsymb27Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb27Bits {
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
pub struct Huffsymb28Bits {
    bits: u32,
}
impl Default for Huffsymb28Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb28Bits {
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
pub struct Huffsymb29Bits {
    bits: u32,
}
impl Default for Huffsymb29Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb29Bits {
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
pub struct Huffsymb3Bits {
    bits: u32,
}
impl Default for Huffsymb3Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb3Bits {
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
pub struct Huffsymb30Bits {
    bits: u32,
}
impl Default for Huffsymb30Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb30Bits {
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
pub struct Huffsymb31Bits {
    bits: u32,
}
impl Default for Huffsymb31Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb31Bits {
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
pub struct Huffsymb32Bits {
    bits: u32,
}
impl Default for Huffsymb32Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb32Bits {
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
pub struct Huffsymb33Bits {
    bits: u32,
}
impl Default for Huffsymb33Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb33Bits {
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
pub struct Huffsymb34Bits {
    bits: u32,
}
impl Default for Huffsymb34Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb34Bits {
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
pub struct Huffsymb35Bits {
    bits: u32,
}
impl Default for Huffsymb35Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb35Bits {
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
pub struct Huffsymb36Bits {
    bits: u32,
}
impl Default for Huffsymb36Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb36Bits {
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
pub struct Huffsymb37Bits {
    bits: u32,
}
impl Default for Huffsymb37Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb37Bits {
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
pub struct Huffsymb38Bits {
    bits: u32,
}
impl Default for Huffsymb38Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb38Bits {
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
pub struct Huffsymb39Bits {
    bits: u32,
}
impl Default for Huffsymb39Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb39Bits {
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
pub struct Huffsymb4Bits {
    bits: u32,
}
impl Default for Huffsymb4Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb4Bits {
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
pub struct Huffsymb40Bits {
    bits: u32,
}
impl Default for Huffsymb40Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb40Bits {
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
pub struct Huffsymb41Bits {
    bits: u32,
}
impl Default for Huffsymb41Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb41Bits {
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
pub struct Huffsymb42Bits {
    bits: u32,
}
impl Default for Huffsymb42Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb42Bits {
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
pub struct Huffsymb43Bits {
    bits: u32,
}
impl Default for Huffsymb43Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb43Bits {
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
pub struct Huffsymb44Bits {
    bits: u32,
}
impl Default for Huffsymb44Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb44Bits {
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
pub struct Huffsymb45Bits {
    bits: u32,
}
impl Default for Huffsymb45Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb45Bits {
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
pub struct Huffsymb46Bits {
    bits: u32,
}
impl Default for Huffsymb46Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb46Bits {
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
pub struct Huffsymb47Bits {
    bits: u32,
}
impl Default for Huffsymb47Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb47Bits {
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
pub struct Huffsymb48Bits {
    bits: u32,
}
impl Default for Huffsymb48Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb48Bits {
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
pub struct Huffsymb49Bits {
    bits: u32,
}
impl Default for Huffsymb49Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb49Bits {
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
pub struct Huffsymb5Bits {
    bits: u32,
}
impl Default for Huffsymb5Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb5Bits {
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
pub struct Huffsymb50Bits {
    bits: u32,
}
impl Default for Huffsymb50Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb50Bits {
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
pub struct Huffsymb51Bits {
    bits: u32,
}
impl Default for Huffsymb51Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb51Bits {
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
pub struct Huffsymb52Bits {
    bits: u32,
}
impl Default for Huffsymb52Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb52Bits {
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
pub struct Huffsymb53Bits {
    bits: u32,
}
impl Default for Huffsymb53Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb53Bits {
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
pub struct Huffsymb54Bits {
    bits: u32,
}
impl Default for Huffsymb54Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb54Bits {
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
pub struct Huffsymb55Bits {
    bits: u32,
}
impl Default for Huffsymb55Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb55Bits {
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
pub struct Huffsymb56Bits {
    bits: u32,
}
impl Default for Huffsymb56Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb56Bits {
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
pub struct Huffsymb57Bits {
    bits: u32,
}
impl Default for Huffsymb57Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb57Bits {
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
pub struct Huffsymb58Bits {
    bits: u32,
}
impl Default for Huffsymb58Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb58Bits {
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
pub struct Huffsymb59Bits {
    bits: u32,
}
impl Default for Huffsymb59Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb59Bits {
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
pub struct Huffsymb6Bits {
    bits: u32,
}
impl Default for Huffsymb6Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb6Bits {
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
pub struct Huffsymb60Bits {
    bits: u32,
}
impl Default for Huffsymb60Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb60Bits {
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
pub struct Huffsymb61Bits {
    bits: u32,
}
impl Default for Huffsymb61Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb61Bits {
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
pub struct Huffsymb62Bits {
    bits: u32,
}
impl Default for Huffsymb62Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb62Bits {
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
pub struct Huffsymb63Bits {
    bits: u32,
}
impl Default for Huffsymb63Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb63Bits {
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
pub struct Huffsymb64Bits {
    bits: u32,
}
impl Default for Huffsymb64Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb64Bits {
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
pub struct Huffsymb65Bits {
    bits: u32,
}
impl Default for Huffsymb65Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb65Bits {
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
pub struct Huffsymb66Bits {
    bits: u32,
}
impl Default for Huffsymb66Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb66Bits {
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
pub struct Huffsymb67Bits {
    bits: u32,
}
impl Default for Huffsymb67Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb67Bits {
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
pub struct Huffsymb68Bits {
    bits: u32,
}
impl Default for Huffsymb68Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb68Bits {
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
pub struct Huffsymb69Bits {
    bits: u32,
}
impl Default for Huffsymb69Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb69Bits {
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
pub struct Huffsymb7Bits {
    bits: u32,
}
impl Default for Huffsymb7Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb7Bits {
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
pub struct Huffsymb70Bits {
    bits: u32,
}
impl Default for Huffsymb70Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb70Bits {
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
pub struct Huffsymb71Bits {
    bits: u32,
}
impl Default for Huffsymb71Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb71Bits {
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
pub struct Huffsymb72Bits {
    bits: u32,
}
impl Default for Huffsymb72Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb72Bits {
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
pub struct Huffsymb73Bits {
    bits: u32,
}
impl Default for Huffsymb73Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb73Bits {
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
pub struct Huffsymb74Bits {
    bits: u32,
}
impl Default for Huffsymb74Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb74Bits {
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
pub struct Huffsymb75Bits {
    bits: u32,
}
impl Default for Huffsymb75Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb75Bits {
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
pub struct Huffsymb76Bits {
    bits: u32,
}
impl Default for Huffsymb76Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb76Bits {
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
pub struct Huffsymb77Bits {
    bits: u32,
}
impl Default for Huffsymb77Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb77Bits {
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
pub struct Huffsymb78Bits {
    bits: u32,
}
impl Default for Huffsymb78Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb78Bits {
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
pub struct Huffsymb79Bits {
    bits: u32,
}
impl Default for Huffsymb79Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb79Bits {
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
pub struct Huffsymb8Bits {
    bits: u32,
}
impl Default for Huffsymb8Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb8Bits {
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
pub struct Huffsymb80Bits {
    bits: u32,
}
impl Default for Huffsymb80Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb80Bits {
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
pub struct Huffsymb81Bits {
    bits: u32,
}
impl Default for Huffsymb81Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb81Bits {
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
pub struct Huffsymb82Bits {
    bits: u32,
}
impl Default for Huffsymb82Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb82Bits {
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
pub struct Huffsymb83Bits {
    bits: u32,
}
impl Default for Huffsymb83Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb83Bits {
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
pub struct Huffsymb9Bits {
    bits: u32,
}
impl Default for Huffsymb9Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Huffsymb9Bits {
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
pub struct JpegCfrBits {
    bits: u32,
}
impl Default for JpegCfrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegCfrBits {
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
pub struct JpegConfr0Bits {
    bits: u32,
}
impl Default for JpegConfr0Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegConfr0Bits {
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
pub struct JpegConfr1Bits {
    bits: u32,
}
impl Default for JpegConfr1Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegConfr1Bits {
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
pub struct JpegConfr2Bits {
    bits: u32,
}
impl Default for JpegConfr2Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegConfr2Bits {
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
pub struct JpegConfr3Bits {
    bits: u32,
}
impl Default for JpegConfr3Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegConfr3Bits {
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
pub struct JpegConfr4Bits {
    bits: u32,
}
impl Default for JpegConfr4Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegConfr4Bits {
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
pub struct JpegConfr5Bits {
    bits: u32,
}
impl Default for JpegConfr5Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegConfr5Bits {
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
pub struct JpegConfr6Bits {
    bits: u32,
}
impl Default for JpegConfr6Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegConfr6Bits {
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
pub struct JpegConfr7Bits {
    bits: u32,
}
impl Default for JpegConfr7Bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegConfr7Bits {
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
pub struct JpegCrBits {
    bits: u32,
}
impl Default for JpegCrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegCrBits {
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
pub struct JpegDirBits {
    bits: u32,
}
impl Default for JpegDirBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegDirBits {
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
pub struct JpegDorBits {
    bits: u32,
}
impl Default for JpegDorBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegDorBits {
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
pub struct JpegSrBits {
    bits: u32,
}
impl Default for JpegSrBits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl JpegSrBits {
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
pub struct Qmem00bits {
    bits: u32,
}
impl Default for Qmem00bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem00bits {
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
pub struct Qmem01bits {
    bits: u32,
}
impl Default for Qmem01bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem01bits {
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
pub struct Qmem010bits {
    bits: u32,
}
impl Default for Qmem010bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem010bits {
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
pub struct Qmem011bits {
    bits: u32,
}
impl Default for Qmem011bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem011bits {
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
pub struct Qmem012bits {
    bits: u32,
}
impl Default for Qmem012bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem012bits {
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
pub struct Qmem013bits {
    bits: u32,
}
impl Default for Qmem013bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem013bits {
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
pub struct Qmem014bits {
    bits: u32,
}
impl Default for Qmem014bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem014bits {
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
pub struct Qmem015bits {
    bits: u32,
}
impl Default for Qmem015bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem015bits {
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
pub struct Qmem02bits {
    bits: u32,
}
impl Default for Qmem02bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem02bits {
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
pub struct Qmem03bits {
    bits: u32,
}
impl Default for Qmem03bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem03bits {
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
pub struct Qmem04bits {
    bits: u32,
}
impl Default for Qmem04bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem04bits {
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
pub struct Qmem05bits {
    bits: u32,
}
impl Default for Qmem05bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem05bits {
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
pub struct Qmem06bits {
    bits: u32,
}
impl Default for Qmem06bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem06bits {
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
pub struct Qmem07bits {
    bits: u32,
}
impl Default for Qmem07bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem07bits {
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
pub struct Qmem08bits {
    bits: u32,
}
impl Default for Qmem08bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem08bits {
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
pub struct Qmem09bits {
    bits: u32,
}
impl Default for Qmem09bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem09bits {
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
pub struct Qmem10bits {
    bits: u32,
}
impl Default for Qmem10bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem10bits {
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
pub struct Qmem11bits {
    bits: u32,
}
impl Default for Qmem11bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem11bits {
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
pub struct Qmem110bits {
    bits: u32,
}
impl Default for Qmem110bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem110bits {
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
pub struct Qmem111bits {
    bits: u32,
}
impl Default for Qmem111bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem111bits {
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
pub struct Qmem112bits {
    bits: u32,
}
impl Default for Qmem112bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem112bits {
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
pub struct Qmem113bits {
    bits: u32,
}
impl Default for Qmem113bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem113bits {
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
pub struct Qmem114bits {
    bits: u32,
}
impl Default for Qmem114bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem114bits {
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
pub struct Qmem115bits {
    bits: u32,
}
impl Default for Qmem115bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem115bits {
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
pub struct Qmem12bits {
    bits: u32,
}
impl Default for Qmem12bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem12bits {
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
pub struct Qmem13bits {
    bits: u32,
}
impl Default for Qmem13bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem13bits {
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
pub struct Qmem14bits {
    bits: u32,
}
impl Default for Qmem14bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem14bits {
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
pub struct Qmem15bits {
    bits: u32,
}
impl Default for Qmem15bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem15bits {
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
pub struct Qmem16bits {
    bits: u32,
}
impl Default for Qmem16bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem16bits {
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
pub struct Qmem17bits {
    bits: u32,
}
impl Default for Qmem17bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem17bits {
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
pub struct Qmem18bits {
    bits: u32,
}
impl Default for Qmem18bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem18bits {
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
pub struct Qmem19bits {
    bits: u32,
}
impl Default for Qmem19bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem19bits {
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
pub struct Qmem20bits {
    bits: u32,
}
impl Default for Qmem20bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem20bits {
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
pub struct Qmem21bits {
    bits: u32,
}
impl Default for Qmem21bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem21bits {
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
pub struct Qmem210bits {
    bits: u32,
}
impl Default for Qmem210bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem210bits {
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
pub struct Qmem211bits {
    bits: u32,
}
impl Default for Qmem211bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem211bits {
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
pub struct Qmem212bits {
    bits: u32,
}
impl Default for Qmem212bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem212bits {
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
pub struct Qmem213bits {
    bits: u32,
}
impl Default for Qmem213bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem213bits {
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
pub struct Qmem214bits {
    bits: u32,
}
impl Default for Qmem214bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem214bits {
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
pub struct Qmem215bits {
    bits: u32,
}
impl Default for Qmem215bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem215bits {
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
pub struct Qmem22bits {
    bits: u32,
}
impl Default for Qmem22bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem22bits {
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
pub struct Qmem23bits {
    bits: u32,
}
impl Default for Qmem23bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem23bits {
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
pub struct Qmem24bits {
    bits: u32,
}
impl Default for Qmem24bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem24bits {
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
pub struct Qmem25bits {
    bits: u32,
}
impl Default for Qmem25bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem25bits {
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
pub struct Qmem26bits {
    bits: u32,
}
impl Default for Qmem26bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem26bits {
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
pub struct Qmem27bits {
    bits: u32,
}
impl Default for Qmem27bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem27bits {
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
pub struct Qmem28bits {
    bits: u32,
}
impl Default for Qmem28bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem28bits {
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
pub struct Qmem29bits {
    bits: u32,
}
impl Default for Qmem29bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem29bits {
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
pub struct Qmem30bits {
    bits: u32,
}
impl Default for Qmem30bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem30bits {
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
pub struct Qmem31bits {
    bits: u32,
}
impl Default for Qmem31bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem31bits {
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
pub struct Qmem310bits {
    bits: u32,
}
impl Default for Qmem310bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem310bits {
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
pub struct Qmem311bits {
    bits: u32,
}
impl Default for Qmem311bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem311bits {
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
pub struct Qmem312bits {
    bits: u32,
}
impl Default for Qmem312bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem312bits {
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
pub struct Qmem313bits {
    bits: u32,
}
impl Default for Qmem313bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem313bits {
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
pub struct Qmem314bits {
    bits: u32,
}
impl Default for Qmem314bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem314bits {
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
pub struct Qmem315bits {
    bits: u32,
}
impl Default for Qmem315bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem315bits {
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
pub struct Qmem32bits {
    bits: u32,
}
impl Default for Qmem32bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem32bits {
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
pub struct Qmem33bits {
    bits: u32,
}
impl Default for Qmem33bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem33bits {
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
pub struct Qmem34bits {
    bits: u32,
}
impl Default for Qmem34bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem34bits {
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
pub struct Qmem35bits {
    bits: u32,
}
impl Default for Qmem35bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem35bits {
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
pub struct Qmem36bits {
    bits: u32,
}
impl Default for Qmem36bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem36bits {
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
pub struct Qmem37bits {
    bits: u32,
}
impl Default for Qmem37bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem37bits {
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
pub struct Qmem38bits {
    bits: u32,
}
impl Default for Qmem38bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem38bits {
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
pub struct Qmem39bits {
    bits: u32,
}
impl Default for Qmem39bits {
    fn default() -> Self {
        unsafe { Self::from_bits_unchecked(0x0) }
    }
}
impl Qmem39bits {
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
