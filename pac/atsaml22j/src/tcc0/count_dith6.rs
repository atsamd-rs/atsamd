#[doc = "Register `COUNT_DITH6` reader"]
pub struct R(crate::R<COUNT_DITH6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_DITH6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_DITH6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_DITH6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT_DITH6` writer"]
pub struct W(crate::W<COUNT_DITH6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT_DITH6_SPEC>;
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
impl From<crate::W<COUNT_DITH6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT_DITH6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Counter Value"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Counter Value"]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COUNT_DITH6_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 6:23 - Counter Value"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 6:23 - Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<6> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_dith6](index.html) module"]
pub struct COUNT_DITH6_SPEC;
impl crate::RegisterSpec for COUNT_DITH6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count_dith6::R](R) reader structure"]
impl crate::Readable for COUNT_DITH6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count_dith6::W](W) writer structure"]
impl crate::Writable for COUNT_DITH6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNT_DITH6 to value 0"]
impl crate::Resettable for COUNT_DITH6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
