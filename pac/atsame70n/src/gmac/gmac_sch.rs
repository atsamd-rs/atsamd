#[doc = "Register `GMAC_SCH` reader"]
pub struct R(crate::R<GMAC_SCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_SCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_SCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_SCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_SCH` writer"]
pub struct W(crate::W<GMAC_SCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_SCH_SPEC>;
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
impl From<crate::W<GMAC_SCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_SCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - 1588 Timer Second Comparison Value"]
pub struct SEC_R(crate::FieldReader<u16, u16>);
impl SEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u16, u16>;
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
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 1588 Timer Second Comparison Value"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 1588 Timer Second Comparison Value"]
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
#[doc = "1588 Timer Second Comparison High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_sch](index.html) module"]
pub struct GMAC_SCH_SPEC;
impl crate::RegisterSpec for GMAC_SCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_sch::R](R) reader structure"]
impl crate::Readable for GMAC_SCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_sch::W](W) writer structure"]
impl crate::Writable for GMAC_SCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_SCH to value 0"]
impl crate::Resettable for GMAC_SCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
