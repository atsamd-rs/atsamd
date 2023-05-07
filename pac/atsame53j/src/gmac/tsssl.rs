#[doc = "Register `TSSSL` reader"]
pub struct R(crate::R<TSSSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSSSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSSSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSSSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSSSL` writer"]
pub struct W(crate::W<TSSSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSSSL_SPEC>;
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
impl From<crate::W<TSSSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSSSL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTS` reader - Value of Timer Seconds Register Capture"]
pub type VTS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VTS` writer - Value of Timer Seconds Register Capture"]
pub type VTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSSSL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    pub fn vts(&self) -> VTS_R {
        VTS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    #[must_use]
    pub fn vts(&mut self) -> VTS_W<0> {
        VTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Sync Strobe Seconds \\[31:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsssl](index.html) module"]
pub struct TSSSL_SPEC;
impl crate::RegisterSpec for TSSSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsssl::R](R) reader structure"]
impl crate::Readable for TSSSL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsssl::W](W) writer structure"]
impl crate::Writable for TSSSL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSSSL to value 0"]
impl crate::Resettable for TSSSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
