#[doc = "Register `AFEC_ACR` reader"]
pub struct R(crate::R<AFEC_ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_ACR` writer"]
pub struct W(crate::W<AFEC_ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_ACR_SPEC>;
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
impl From<crate::W<AFEC_ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGA0EN` reader - PGA0 Enable"]
pub struct PGA0EN_R(crate::FieldReader<bool, bool>);
impl PGA0EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGA0EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGA0EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGA0EN` writer - PGA0 Enable"]
pub struct PGA0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA0EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PGA1EN` reader - PGA1 Enable"]
pub struct PGA1EN_R(crate::FieldReader<bool, bool>);
impl PGA1EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGA1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGA1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGA1EN` writer - PGA1 Enable"]
pub struct PGA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `IBCTL` reader - AFE Bias Current Control"]
pub struct IBCTL_R(crate::FieldReader<u8, u8>);
impl IBCTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IBCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBCTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBCTL` writer - AFE Bias Current Control"]
pub struct IBCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> IBCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - PGA0 Enable"]
    #[inline(always)]
    pub fn pga0en(&self) -> PGA0EN_R {
        PGA0EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PGA1 Enable"]
    #[inline(always)]
    pub fn pga1en(&self) -> PGA1EN_R {
        PGA1EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - AFE Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&self) -> IBCTL_R {
        IBCTL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - PGA0 Enable"]
    #[inline(always)]
    pub fn pga0en(&mut self) -> PGA0EN_W {
        PGA0EN_W { w: self }
    }
    #[doc = "Bit 3 - PGA1 Enable"]
    #[inline(always)]
    pub fn pga1en(&mut self) -> PGA1EN_W {
        PGA1EN_W { w: self }
    }
    #[doc = "Bits 8:9 - AFE Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&mut self) -> IBCTL_W {
        IBCTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_acr](index.html) module"]
pub struct AFEC_ACR_SPEC;
impl crate::RegisterSpec for AFEC_ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_acr::R](R) reader structure"]
impl crate::Readable for AFEC_ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_acr::W](W) writer structure"]
impl crate::Writable for AFEC_ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_ACR to value 0"]
impl crate::Resettable for AFEC_ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
