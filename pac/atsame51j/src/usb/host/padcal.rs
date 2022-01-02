#[doc = "Register `PADCAL` reader"]
pub struct R(crate::R<PADCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCAL` writer"]
pub struct W(crate::W<PADCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCAL_SPEC>;
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
impl From<crate::W<PADCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRANSP` reader - USB Pad Transp calibration"]
pub struct TRANSP_R(crate::FieldReader<u8, u8>);
impl TRANSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRANSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANSP` writer - USB Pad Transp calibration"]
pub struct TRANSP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u16 & 0x1f);
        self.w
    }
}
#[doc = "Field `TRANSN` reader - USB Pad Transn calibration"]
pub struct TRANSN_R(crate::FieldReader<u8, u8>);
impl TRANSN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRANSN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANSN` writer - USB Pad Transn calibration"]
pub struct TRANSN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u16 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `TRIM` reader - USB Pad Trim calibration"]
pub struct TRIM_R(crate::FieldReader<u8, u8>);
impl TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM` writer - USB Pad Trim calibration"]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u16 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline(always)]
    pub fn transp(&self) -> TRANSP_R {
        TRANSP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline(always)]
    pub fn transn(&self) -> TRANSN_R {
        TRANSN_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline(always)]
    pub fn transp(&mut self) -> TRANSP_W {
        TRANSP_W { w: self }
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline(always)]
    pub fn transn(&mut self) -> TRANSN_W {
        TRANSN_W { w: self }
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PAD Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcal](index.html) module"]
pub struct PADCAL_SPEC;
impl crate::RegisterSpec for PADCAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [padcal::R](R) reader structure"]
impl crate::Readable for PADCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padcal::W](W) writer structure"]
impl crate::Writable for PADCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADCAL to value 0"]
impl crate::Resettable for PADCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
