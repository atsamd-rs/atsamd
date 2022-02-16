#[doc = "Register `GMAC_IPGS` reader"]
pub struct R(crate::R<GMAC_IPGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_IPGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_IPGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_IPGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_IPGS` writer"]
pub struct W(crate::W<GMAC_IPGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_IPGS_SPEC>;
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
impl From<crate::W<GMAC_IPGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_IPGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FL` reader - Frame Length"]
pub struct FL_R(crate::FieldReader<u16, u16>);
impl FL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FL` writer - Frame Length"]
pub struct FL_W<'a> {
    w: &'a mut W,
}
impl<'a> FL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Length"]
    #[inline(always)]
    pub fn fl(&self) -> FL_R {
        FL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Length"]
    #[inline(always)]
    pub fn fl(&mut self) -> FL_W {
        FL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPG Stretch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ipgs](index.html) module"]
pub struct GMAC_IPGS_SPEC;
impl crate::RegisterSpec for GMAC_IPGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_ipgs::R](R) reader structure"]
impl crate::Readable for GMAC_IPGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_ipgs::W](W) writer structure"]
impl crate::Writable for GMAC_IPGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_IPGS to value 0"]
impl crate::Resettable for GMAC_IPGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
