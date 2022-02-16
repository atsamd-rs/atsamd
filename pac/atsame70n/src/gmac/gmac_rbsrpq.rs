#[doc = "Register `GMAC_RBSRPQ[%s]` reader"]
pub struct R(crate::R<GMAC_RBSRPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_RBSRPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_RBSRPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_RBSRPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_RBSRPQ[%s]` writer"]
pub struct W(crate::W<GMAC_RBSRPQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_RBSRPQ_SPEC>;
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
impl From<crate::W<GMAC_RBSRPQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_RBSRPQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBS` reader - Receive Buffer Size"]
pub struct RBS_R(crate::FieldReader<u16, u16>);
impl RBS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBS` writer - Receive Buffer Size"]
pub struct RBS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    pub fn rbs(&mut self) -> RBS_W {
        RBS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Buffer Size Register Priority Queue (1..5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rbsrpq](index.html) module"]
pub struct GMAC_RBSRPQ_SPEC;
impl crate::RegisterSpec for GMAC_RBSRPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_rbsrpq::R](R) reader structure"]
impl crate::Readable for GMAC_RBSRPQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_rbsrpq::W](W) writer structure"]
impl crate::Writable for GMAC_RBSRPQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_RBSRPQ[%s]
to value 0"]
impl crate::Resettable for GMAC_RBSRPQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
