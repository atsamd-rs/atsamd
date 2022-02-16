#[doc = "Register `GMAC_TPSF` reader"]
pub struct R(crate::R<GMAC_TPSF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_TPSF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_TPSF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_TPSF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_TPSF` writer"]
pub struct W(crate::W<GMAC_TPSF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_TPSF_SPEC>;
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
impl From<crate::W<GMAC_TPSF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_TPSF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPB1ADR` reader - Transmit Partial Store and Forward Address"]
pub struct TPB1ADR_R(crate::FieldReader<u16, u16>);
impl TPB1ADR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TPB1ADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPB1ADR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPB1ADR` writer - Transmit Partial Store and Forward Address"]
pub struct TPB1ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPB1ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `ENTXP` reader - Enable TX Partial Store and Forward Operation"]
pub struct ENTXP_R(crate::FieldReader<bool, bool>);
impl ENTXP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENTXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENTXP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENTXP` writer - Enable TX Partial Store and Forward Operation"]
pub struct ENTXP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTXP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Transmit Partial Store and Forward Address"]
    #[inline(always)]
    pub fn tpb1adr(&self) -> TPB1ADR_R {
        TPB1ADR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Enable TX Partial Store and Forward Operation"]
    #[inline(always)]
    pub fn entxp(&self) -> ENTXP_R {
        ENTXP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transmit Partial Store and Forward Address"]
    #[inline(always)]
    pub fn tpb1adr(&mut self) -> TPB1ADR_W {
        TPB1ADR_W { w: self }
    }
    #[doc = "Bit 31 - Enable TX Partial Store and Forward Operation"]
    #[inline(always)]
    pub fn entxp(&mut self) -> ENTXP_W {
        ENTXP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Partial Store and Forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tpsf](index.html) module"]
pub struct GMAC_TPSF_SPEC;
impl crate::RegisterSpec for GMAC_TPSF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_tpsf::R](R) reader structure"]
impl crate::Readable for GMAC_TPSF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_tpsf::W](W) writer structure"]
impl crate::Writable for GMAC_TPSF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_TPSF to value 0"]
impl crate::Resettable for GMAC_TPSF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
