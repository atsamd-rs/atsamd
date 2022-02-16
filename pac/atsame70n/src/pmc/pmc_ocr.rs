#[doc = "Register `PMC_OCR` reader"]
pub struct R(crate::R<PMC_OCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_OCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_OCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_OCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_OCR` writer"]
pub struct W(crate::W<PMC_OCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_OCR_SPEC>;
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
impl From<crate::W<PMC_OCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_OCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL4` reader - Main RC Oscillator Calibration Bits for 4 MHz"]
pub struct CAL4_R(crate::FieldReader<u8, u8>);
impl CAL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL4` writer - Main RC Oscillator Calibration Bits for 4 MHz"]
pub struct CAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `SEL4` reader - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
pub struct SEL4_R(crate::FieldReader<bool, bool>);
impl SEL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL4` writer - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
pub struct SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CAL8` reader - Main RC Oscillator Calibration Bits for 8 MHz"]
pub struct CAL8_R(crate::FieldReader<u8, u8>);
impl CAL8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL8` writer - Main RC Oscillator Calibration Bits for 8 MHz"]
pub struct CAL8_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `SEL8` reader - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
pub struct SEL8_R(crate::FieldReader<bool, bool>);
impl SEL8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL8` writer - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
pub struct SEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL8_W<'a> {
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
#[doc = "Field `CAL12` reader - Main RC Oscillator Calibration Bits for 12 MHz"]
pub struct CAL12_R(crate::FieldReader<u8, u8>);
impl CAL12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAL12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL12` writer - Main RC Oscillator Calibration Bits for 12 MHz"]
pub struct CAL12_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `SEL12` reader - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
pub struct SEL12_R(crate::FieldReader<bool, bool>);
impl SEL12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL12` writer - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
pub struct SEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&self) -> CAL4_R {
        CAL4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&self) -> CAL8_R {
        CAL8_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&self) -> CAL12_R {
        CAL12_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&mut self) -> CAL4_W {
        CAL4_W { w: self }
    }
    #[doc = "Bit 7 - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&mut self) -> SEL4_W {
        SEL4_W { w: self }
    }
    #[doc = "Bits 8:14 - Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&mut self) -> CAL8_W {
        CAL8_W { w: self }
    }
    #[doc = "Bit 15 - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&mut self) -> SEL8_W {
        SEL8_W { w: self }
    }
    #[doc = "Bits 16:22 - Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&mut self) -> CAL12_W {
        CAL12_W { w: self }
    }
    #[doc = "Bit 23 - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&mut self) -> SEL12_W {
        SEL12_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_ocr](index.html) module"]
pub struct PMC_OCR_SPEC;
impl crate::RegisterSpec for PMC_OCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_ocr::R](R) reader structure"]
impl crate::Readable for PMC_OCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_ocr::W](W) writer structure"]
impl crate::Writable for PMC_OCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_OCR to value 0"]
impl crate::Resettable for PMC_OCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
