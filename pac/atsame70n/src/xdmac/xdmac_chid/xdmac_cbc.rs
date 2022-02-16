#[doc = "Register `XDMAC_CBC` reader"]
pub struct R(crate::R<XDMAC_CBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_CBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_CBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_CBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XDMAC_CBC` writer"]
pub struct W(crate::W<XDMAC_CBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_CBC_SPEC>;
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
impl From<crate::W<XDMAC_CBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_CBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLEN` reader - Channel x Block Length"]
pub struct BLEN_R(crate::FieldReader<u16, u16>);
impl BLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEN` writer - Channel x Block Length"]
pub struct BLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Channel x Block Length"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel x Block Length"]
    #[inline(always)]
    pub fn blen(&mut self) -> BLEN_W {
        BLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Block Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cbc](index.html) module"]
pub struct XDMAC_CBC_SPEC;
impl crate::RegisterSpec for XDMAC_CBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_cbc::R](R) reader structure"]
impl crate::Readable for XDMAC_CBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xdmac_cbc::W](W) writer structure"]
impl crate::Writable for XDMAC_CBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_CBC to value 0"]
impl crate::Resettable for XDMAC_CBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
