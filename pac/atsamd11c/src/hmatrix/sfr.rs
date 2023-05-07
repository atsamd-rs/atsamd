#[doc = "Register `SFR%s` reader"]
pub struct R(crate::R<SFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFR%s` writer"]
pub struct W(crate::W<SFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFR_SPEC>;
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
impl From<crate::W<SFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFR` reader - Special Function Register"]
pub type SFR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SFR` writer - Special Function Register"]
pub type SFR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Special Function Register"]
    #[inline(always)]
    pub fn sfr(&self) -> SFR_R {
        SFR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Special Function Register"]
    #[inline(always)]
    #[must_use]
    pub fn sfr(&mut self) -> SFR_W<0> {
        SFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Special Function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](index.html) module"]
pub struct SFR_SPEC;
impl crate::RegisterSpec for SFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfr::R](R) reader structure"]
impl crate::Readable for SFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfr::W](W) writer structure"]
impl crate::Writable for SFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFR%s to value 0"]
impl crate::Resettable for SFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
