#[doc = "Register `GMAC_SCL` reader"]
pub struct R(crate::R<GMAC_SCL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_SCL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_SCL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_SCL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_SCL` writer"]
pub struct W(crate::W<GMAC_SCL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_SCL_SPEC>;
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
impl From<crate::W<GMAC_SCL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_SCL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - 1588 Timer Second Comparison Value"]
pub struct SEC_R(crate::FieldReader<u32, u32>);
impl SEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC` writer - 1588 Timer Second Comparison Value"]
pub struct SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 1588 Timer Second Comparison Value"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1588 Timer Second Comparison Value"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Second Comparison Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_scl](index.html) module"]
pub struct GMAC_SCL_SPEC;
impl crate::RegisterSpec for GMAC_SCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_scl::R](R) reader structure"]
impl crate::Readable for GMAC_SCL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_scl::W](W) writer structure"]
impl crate::Writable for GMAC_SCL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_SCL to value 0"]
impl crate::Resettable for GMAC_SCL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
