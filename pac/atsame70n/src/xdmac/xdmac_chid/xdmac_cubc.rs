#[doc = "Register `XDMAC_CUBC` reader"]
pub struct R(crate::R<XDMAC_CUBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_CUBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_CUBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_CUBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XDMAC_CUBC` writer"]
pub struct W(crate::W<XDMAC_CUBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_CUBC_SPEC>;
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
impl From<crate::W<XDMAC_CUBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_CUBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UBLEN` reader - Channel x Microblock Length"]
pub struct UBLEN_R(crate::FieldReader<u32, u32>);
impl UBLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        UBLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UBLEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UBLEN` writer - Channel x Microblock Length"]
pub struct UBLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UBLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    pub fn ublen(&self) -> UBLEN_R {
        UBLEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    pub fn ublen(&mut self) -> UBLEN_W {
        UBLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Microblock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cubc](index.html) module"]
pub struct XDMAC_CUBC_SPEC;
impl crate::RegisterSpec for XDMAC_CUBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_cubc::R](R) reader structure"]
impl crate::Readable for XDMAC_CUBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xdmac_cubc::W](W) writer structure"]
impl crate::Writable for XDMAC_CUBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_CUBC to value 0"]
impl crate::Resettable for XDMAC_CUBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
