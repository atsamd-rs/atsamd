#[doc = "Register `BAUD_FRACFP_MODE` reader"]
pub struct R(crate::R<BAUD_FRACFP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD_FRACFP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD_FRACFP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD_FRACFP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD_FRACFP_MODE` writer"]
pub struct W(crate::W<BAUD_FRACFP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD_FRACFP_MODE_SPEC>;
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
impl From<crate::W<BAUD_FRACFP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD_FRACFP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD` reader - Baud Rate Value"]
pub type BAUD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BAUD` writer - Baud Rate Value"]
pub type BAUD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, BAUD_FRACFP_MODE_SPEC, u16, u16, 13, O>;
#[doc = "Field `FP` reader - Fractional Part"]
pub type FP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FP` writer - Fractional Part"]
pub type FP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, BAUD_FRACFP_MODE_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new(self.bits & 0x1fff)
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BAUD_W<0> {
        BAUD_W::new(self)
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    #[must_use]
    pub fn fp(&mut self) -> FP_W<13> {
        FP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud_fracfp_mode](index.html) module"]
pub struct BAUD_FRACFP_MODE_SPEC;
impl crate::RegisterSpec for BAUD_FRACFP_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [baud_fracfp_mode::R](R) reader structure"]
impl crate::Readable for BAUD_FRACFP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud_fracfp_mode::W](W) writer structure"]
impl crate::Writable for BAUD_FRACFP_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUD_FRACFP_MODE to value 0"]
impl crate::Resettable for BAUD_FRACFP_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
