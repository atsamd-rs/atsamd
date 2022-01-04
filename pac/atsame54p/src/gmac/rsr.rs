#[doc = "Register `RSR` reader"]
pub struct R(crate::R<RSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSR` writer"]
pub struct W(crate::W<RSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSR_SPEC>;
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
impl From<crate::W<RSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNA` reader - Buffer Not Available"]
pub struct BNA_R(crate::FieldReader<bool, bool>);
impl BNA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BNA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNA` writer - Buffer Not Available"]
pub struct BNA_W<'a> {
    w: &'a mut W,
}
impl<'a> BNA_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `REC` reader - Frame Received"]
pub struct REC_R(crate::FieldReader<bool, bool>);
impl REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REC` writer - Frame Received"]
pub struct REC_W<'a> {
    w: &'a mut W,
}
impl<'a> REC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RXOVR` reader - Receive Overrun"]
pub struct RXOVR_R(crate::FieldReader<bool, bool>);
impl RXOVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVR` writer - Receive Overrun"]
pub struct RXOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVR_W<'a> {
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
#[doc = "Field `HNO` reader - HRESP Not OK"]
pub struct HNO_R(crate::FieldReader<bool, bool>);
impl HNO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HNO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNO` writer - HRESP Not OK"]
pub struct HNO_W<'a> {
    w: &'a mut W,
}
impl<'a> HNO_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Buffer Not Available"]
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frame Received"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HRESP Not OK"]
    #[inline(always)]
    pub fn hno(&self) -> HNO_R {
        HNO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer Not Available"]
    #[inline(always)]
    pub fn bna(&mut self) -> BNA_W {
        BNA_W { w: self }
    }
    #[doc = "Bit 1 - Frame Received"]
    #[inline(always)]
    pub fn rec(&mut self) -> REC_W {
        REC_W { w: self }
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxovr(&mut self) -> RXOVR_W {
        RXOVR_W { w: self }
    }
    #[doc = "Bit 3 - HRESP Not OK"]
    #[inline(always)]
    pub fn hno(&mut self) -> HNO_W {
        HNO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](index.html) module"]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsr::R](R) reader structure"]
impl crate::Readable for RSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsr::W](W) writer structure"]
impl crate::Writable for RSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
