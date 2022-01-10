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
pub struct BCNT_R(crate::FieldReader<u16, u16>);
impl BCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCNT` writer - Blocks Count for Current Transfer"]
pub struct BCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value as u16;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(self.bits as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn bcnt(&mut self) -> BCNT_W {
        BCNT_W { w: self }
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
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
