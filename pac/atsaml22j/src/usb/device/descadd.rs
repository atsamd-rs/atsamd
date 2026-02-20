#[doc = "Register `DESCADD` reader"]
pub struct R(crate::R<DESCADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESCADD` writer"]
pub struct W(crate::W<DESCADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESCADD_SPEC>;
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
impl From<crate::W<DESCADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESCADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DESCADD` reader - Descriptor Address Value"]
pub type DESCADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DESCADD` writer - Descriptor Address Value"]
pub type DESCADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESCADD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Descriptor Address Value"]
    #[inline(always)]
    pub fn descadd(&self) -> DESCADD_R {
        DESCADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Address Value"]
    #[inline(always)]
    #[must_use]
    pub fn descadd(&mut self) -> DESCADD_W<0> {
        DESCADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descadd](index.html) module"]
pub struct DESCADD_SPEC;
impl crate::RegisterSpec for DESCADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descadd::R](R) reader structure"]
impl crate::Readable for DESCADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [descadd::W](W) writer structure"]
impl crate::Writable for DESCADD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESCADD to value 0"]
impl crate::Resettable for DESCADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
