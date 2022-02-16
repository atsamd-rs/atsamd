#[doc = "Register `ISI_CFG2` reader"]
pub struct R(crate::R<ISI_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISI_CFG2` writer"]
pub struct W(crate::W<ISI_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ISI_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM_VSIZE` reader - Vertical Size of the Image Sensor \\[0..2047\\]"]
pub struct IM_VSIZE_R(crate::FieldReader<u16, u16>);
impl IM_VSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IM_VSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM_VSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM_VSIZE` writer - Vertical Size of the Image Sensor \\[0..2047\\]"]
pub struct IM_VSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> IM_VSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `GS_MODE` reader - Grayscale Pixel Format Mode"]
pub struct GS_MODE_R(crate::FieldReader<bool, bool>);
impl GS_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GS_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GS_MODE` writer - Grayscale Pixel Format Mode"]
pub struct GS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GS_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `RGB_MODE` reader - RGB Input Mode"]
pub struct RGB_MODE_R(crate::FieldReader<bool, bool>);
impl RGB_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RGB_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RGB_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGB_MODE` writer - RGB Input Mode"]
pub struct RGB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `GRAYSCALE` reader - Grayscale Mode Format Enable"]
pub struct GRAYSCALE_R(crate::FieldReader<bool, bool>);
impl GRAYSCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GRAYSCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GRAYSCALE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GRAYSCALE` writer - Grayscale Mode Format Enable"]
pub struct GRAYSCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> GRAYSCALE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RGB_SWAP` reader - RGB Format Swap Mode"]
pub struct RGB_SWAP_R(crate::FieldReader<bool, bool>);
impl RGB_SWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RGB_SWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RGB_SWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGB_SWAP` writer - RGB Format Swap Mode"]
pub struct RGB_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB_SWAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `COL_SPACE` reader - Color Space for the Image Data"]
pub struct COL_SPACE_R(crate::FieldReader<bool, bool>);
impl COL_SPACE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COL_SPACE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COL_SPACE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COL_SPACE` writer - Color Space for the Image Data"]
pub struct COL_SPACE_W<'a> {
    w: &'a mut W,
}
impl<'a> COL_SPACE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `IM_HSIZE` reader - Horizontal Size of the Image Sensor \\[0..2047\\]"]
pub struct IM_HSIZE_R(crate::FieldReader<u16, u16>);
impl IM_HSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IM_HSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM_HSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM_HSIZE` writer - Horizontal Size of the Image Sensor \\[0..2047\\]"]
pub struct IM_HSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> IM_HSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
#[doc = "YCrCb Format Swap Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum YCC_SWAP_A {
    #[doc = "0: Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    DEFAULT = 0,
    #[doc = "1: Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    MODE1 = 1,
    #[doc = "2: Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    MODE2 = 2,
    #[doc = "3: Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    MODE3 = 3,
}
impl From<YCC_SWAP_A> for u8 {
    #[inline(always)]
    fn from(variant: YCC_SWAP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `YCC_SWAP` reader - YCrCb Format Swap Mode"]
pub struct YCC_SWAP_R(crate::FieldReader<u8, YCC_SWAP_A>);
impl YCC_SWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        YCC_SWAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YCC_SWAP_A {
        match self.bits {
            0 => YCC_SWAP_A::DEFAULT,
            1 => YCC_SWAP_A::MODE1,
            2 => YCC_SWAP_A::MODE2,
            3 => YCC_SWAP_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == YCC_SWAP_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        **self == YCC_SWAP_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        **self == YCC_SWAP_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        **self == YCC_SWAP_A::MODE3
    }
}
impl core::ops::Deref for YCC_SWAP_R {
    type Target = crate::FieldReader<u8, YCC_SWAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YCC_SWAP` writer - YCrCb Format Swap Mode"]
pub struct YCC_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> YCC_SWAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: YCC_SWAP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(YCC_SWAP_A::DEFAULT)
    }
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(YCC_SWAP_A::MODE1)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(YCC_SWAP_A::MODE2)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(YCC_SWAP_A::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "RGB Pixel Mapping Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RGB_CFG_A {
    #[doc = "0: Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    DEFAULT = 0,
    #[doc = "1: Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    MODE1 = 1,
    #[doc = "2: Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    MODE2 = 2,
    #[doc = "3: Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    MODE3 = 3,
}
impl From<RGB_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RGB_CFG` reader - RGB Pixel Mapping Configuration"]
pub struct RGB_CFG_R(crate::FieldReader<u8, RGB_CFG_A>);
impl RGB_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RGB_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGB_CFG_A {
        match self.bits {
            0 => RGB_CFG_A::DEFAULT,
            1 => RGB_CFG_A::MODE1,
            2 => RGB_CFG_A::MODE2,
            3 => RGB_CFG_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == RGB_CFG_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        **self == RGB_CFG_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        **self == RGB_CFG_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        **self == RGB_CFG_A::MODE3
    }
}
impl core::ops::Deref for RGB_CFG_R {
    type Target = crate::FieldReader<u8, RGB_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGB_CFG` writer - RGB Pixel Mapping Configuration"]
pub struct RGB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGB_CFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(RGB_CFG_A::DEFAULT)
    }
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(RGB_CFG_A::MODE1)
    }
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(RGB_CFG_A::MODE2)
    }
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(RGB_CFG_A::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_vsize(&self) -> IM_VSIZE_R {
        IM_VSIZE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    pub fn gs_mode(&self) -> GS_MODE_R {
        GS_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&self) -> RGB_MODE_R {
        RGB_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&self) -> GRAYSCALE_R {
        GRAYSCALE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&self) -> RGB_SWAP_R {
        RGB_SWAP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&self) -> COL_SPACE_R {
        COL_SPACE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&self) -> IM_HSIZE_R {
        IM_HSIZE_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&self) -> YCC_SWAP_R {
        YCC_SWAP_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&self) -> RGB_CFG_R {
        RGB_CFG_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_vsize(&mut self) -> IM_VSIZE_W {
        IM_VSIZE_W { w: self }
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    pub fn gs_mode(&mut self) -> GS_MODE_W {
        GS_MODE_W { w: self }
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&mut self) -> RGB_MODE_W {
        RGB_MODE_W { w: self }
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&mut self) -> GRAYSCALE_W {
        GRAYSCALE_W { w: self }
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&mut self) -> RGB_SWAP_W {
        RGB_SWAP_W { w: self }
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&mut self) -> COL_SPACE_W {
        COL_SPACE_W { w: self }
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&mut self) -> IM_HSIZE_W {
        IM_HSIZE_W { w: self }
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&mut self) -> YCC_SWAP_W {
        YCC_SWAP_W { w: self }
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&mut self) -> RGB_CFG_W {
        RGB_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_cfg2](index.html) module"]
pub struct ISI_CFG2_SPEC;
impl crate::RegisterSpec for ISI_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_cfg2::R](R) reader structure"]
impl crate::Readable for ISI_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isi_cfg2::W](W) writer structure"]
impl crate::Writable for ISI_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_CFG2 to value 0"]
impl crate::Resettable for ISI_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
