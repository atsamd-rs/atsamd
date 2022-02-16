#[doc = "Register `GMAC_RPSF` reader"]
pub struct R(crate::R<GMAC_RPSF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_RPSF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_RPSF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_RPSF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_RPSF` writer"]
pub struct W(crate::W<GMAC_RPSF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_RPSF_SPEC>;
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
impl From<crate::W<GMAC_RPSF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_RPSF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPB1ADR` reader - Receive Partial Store and Forward Address"]
pub struct RPB1ADR_R(crate::FieldReader<u16, u16>);
impl RPB1ADR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RPB1ADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPB1ADR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPB1ADR` writer - Receive Partial Store and Forward Address"]
pub struct RPB1ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> RPB1ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `ENRXP` reader - Enable RX Partial Store and Forward Operation"]
pub struct ENRXP_R(crate::FieldReader<bool, bool>);
impl ENRXP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENRXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENRXP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENRXP` writer - Enable RX Partial Store and Forward Operation"]
pub struct ENRXP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRXP_W<'a> {
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
    #[doc = "Bits 0:11 - Receive Partial Store and Forward Address"]
    #[inline(always)]
    pub fn rpb1adr(&self) -> RPB1ADR_R {
        RPB1ADR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Enable RX Partial Store and Forward Operation"]
    #[inline(always)]
    pub fn enrxp(&self) -> ENRXP_R {
        ENRXP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Receive Partial Store and Forward Address"]
    #[inline(always)]
    pub fn rpb1adr(&mut self) -> RPB1ADR_W {
        RPB1ADR_W { w: self }
    }
    #[doc = "Bit 31 - Enable RX Partial Store and Forward Operation"]
    #[inline(always)]
    pub fn enrxp(&mut self) -> ENRXP_W {
        ENRXP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Partial Store and Forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rpsf](index.html) module"]
pub struct GMAC_RPSF_SPEC;
impl crate::RegisterSpec for GMAC_RPSF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_rpsf::R](R) reader structure"]
impl crate::Readable for GMAC_RPSF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_rpsf::W](W) writer structure"]
impl crate::Writable for GMAC_RPSF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_RPSF to value 0"]
impl crate::Resettable for GMAC_RPSF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
