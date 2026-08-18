#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIB` writer"]
pub struct W(crate::W<CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIB_SPEC>;
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
impl From<crate::W<CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIASCOMP` reader - Bias Comparator Scaling"]
pub type BIASCOMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASCOMP` writer - Bias Comparator Scaling"]
pub type BIASCOMP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CALIB_SPEC, u8, u8, 3, O>;
#[doc = "Field `BIASREFBUF` reader - Bias Reference Buffer Scaling"]
pub type BIASREFBUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASREFBUF` writer - Bias Reference Buffer Scaling"]
pub type BIASREFBUF_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CALIB_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Bias Comparator Scaling"]
    #[inline(always)]
    pub fn biascomp(&self) -> BIASCOMP_R {
        BIASCOMP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Bias Reference Buffer Scaling"]
    #[inline(always)]
    pub fn biasrefbuf(&self) -> BIASREFBUF_R {
        BIASREFBUF_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bias Comparator Scaling"]
    #[inline(always)]
    #[must_use]
    pub fn biascomp(&mut self) -> BIASCOMP_W<0> {
        BIASCOMP_W::new(self)
    }
    #[doc = "Bits 8:10 - Bias Reference Buffer Scaling"]
    #[inline(always)]
    #[must_use]
    pub fn biasrefbuf(&mut self) -> BIASREFBUF_W<8> {
        BIASREFBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calib::W](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
