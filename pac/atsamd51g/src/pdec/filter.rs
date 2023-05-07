#[doc = "Register `FILTER` reader"]
pub struct R(crate::R<FILTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER` writer"]
pub struct W(crate::W<FILTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_SPEC>;
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
impl From<crate::W<FILTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER` reader - Filter Value"]
pub type FILTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTER` writer - Filter Value"]
pub type FILTER_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FILTER_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Filter Value"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter Value"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<0> {
        FILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter](index.html) module"]
pub struct FILTER_SPEC;
impl crate::RegisterSpec for FILTER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [filter::R](R) reader structure"]
impl crate::Readable for FILTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter::W](W) writer structure"]
impl crate::Writable for FILTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FILTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
