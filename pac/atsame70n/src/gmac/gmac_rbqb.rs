#[doc = "Register `GMAC_RBQB` reader"]
pub struct R(crate::R<GMAC_RBQB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_RBQB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_RBQB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_RBQB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_RBQB` writer"]
pub struct W(crate::W<GMAC_RBQB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_RBQB_SPEC>;
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
impl From<crate::W<GMAC_RBQB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_RBQB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Receive Buffer Queue Base Address"]
pub struct ADDR_R(crate::FieldReader<u32, u32>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Receive Buffer Queue Base Address"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Buffer Queue Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rbqb](index.html) module"]
pub struct GMAC_RBQB_SPEC;
impl crate::RegisterSpec for GMAC_RBQB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_rbqb::R](R) reader structure"]
impl crate::Readable for GMAC_RBQB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_rbqb::W](W) writer structure"]
impl crate::Writable for GMAC_RBQB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_RBQB to value 0"]
impl crate::Resettable for GMAC_RBQB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
