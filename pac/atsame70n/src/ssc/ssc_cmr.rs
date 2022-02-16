#[doc = "Register `SSC_CMR` reader"]
pub struct R(crate::R<SSC_CMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSC_CMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSC_CMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSC_CMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSC_CMR` writer"]
pub struct W(crate::W<SSC_CMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSC_CMR_SPEC>;
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
impl From<crate::W<SSC_CMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSC_CMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Clock Divider"]
pub struct DIV_R(crate::FieldReader<u16, u16>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Clock Divider"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock Divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_cmr](index.html) module"]
pub struct SSC_CMR_SPEC;
impl crate::RegisterSpec for SSC_CMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssc_cmr::R](R) reader structure"]
impl crate::Readable for SSC_CMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssc_cmr::W](W) writer structure"]
impl crate::Writable for SSC_CMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSC_CMR to value 0"]
impl crate::Resettable for SSC_CMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
