#[doc = "Register `BCR` reader"]
pub struct R(crate::R<BCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCR` writer"]
pub struct W(crate::W<BCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCR_SPEC>;
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
impl From<crate::W<BCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCNT` reader - Blocks Count for Current Transfer"]
pub type BCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BCNT` writer - Blocks Count for Current Transfer"]
pub type BCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, BCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn bcnt(&mut self) -> BCNT_W<0> {
        BCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr](index.html) module"]
pub struct BCR_SPEC;
impl crate::RegisterSpec for BCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bcr::R](R) reader structure"]
impl crate::Readable for BCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcr::W](W) writer structure"]
impl crate::Writable for BCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
