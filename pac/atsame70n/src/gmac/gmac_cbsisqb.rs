#[doc = "Register `GMAC_CBSISQB` reader"]
pub struct R(crate::R<GMAC_CBSISQB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_CBSISQB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_CBSISQB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_CBSISQB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_CBSISQB` writer"]
pub struct W(crate::W<GMAC_CBSISQB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_CBSISQB_SPEC>;
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
impl From<crate::W<GMAC_CBSISQB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_CBSISQB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IS` reader - IdleSlope"]
pub struct IS_R(crate::FieldReader<u32, u32>);
impl IS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS` writer - IdleSlope"]
pub struct IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IdleSlope"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IdleSlope"]
    #[inline(always)]
    pub fn is(&mut self) -> IS_W {
        IS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_cbsisqb](index.html) module"]
pub struct GMAC_CBSISQB_SPEC;
impl crate::RegisterSpec for GMAC_CBSISQB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_cbsisqb::R](R) reader structure"]
impl crate::Readable for GMAC_CBSISQB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_cbsisqb::W](W) writer structure"]
impl crate::Writable for GMAC_CBSISQB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_CBSISQB to value 0"]
impl crate::Resettable for GMAC_CBSISQB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
