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
pub type TRANSP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRANSP` writer - USB Pad Transp calibration"]
pub type TRANSP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PADCAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRANSN` reader - USB Pad Transn calibration"]
pub type TRANSN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRANSN` writer - USB Pad Transn calibration"]
pub type TRANSN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PADCAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIM` reader - USB Pad Trim calibration"]
pub type TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM` writer - USB Pad Trim calibration"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PADCAL_SPEC, u8, u8, 3, O>;
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
        TRIM_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline(always)]
    #[must_use]
    pub fn transp(&mut self) -> TRANSP_W<0> {
        TRANSP_W::new(self)
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline(always)]
    #[must_use]
    pub fn transn(&mut self) -> TRANSN_W<6> {
        TRANSN_W::new(self)
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<12> {
        TRIM_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADCAL to value 0"]
impl crate::Resettable for PADCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
