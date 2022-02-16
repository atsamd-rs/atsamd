#[doc = "Register `EEFC_FMR` reader"]
pub struct R(crate::R<EEFC_FMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFC_FMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFC_FMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFC_FMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEFC_FMR` writer"]
pub struct W(crate::W<EEFC_FMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFC_FMR_SPEC>;
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
impl From<crate::W<EEFC_FMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFC_FMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Interrupt Enable"]
pub struct FRDY_R(crate::FieldReader<bool, bool>);
impl FRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRDY` writer - Flash Ready Interrupt Enable"]
pub struct FRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDY_W<'a> {
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
#[doc = "Field `FWS` reader - Flash Wait State"]
pub struct FWS_R(crate::FieldReader<u8, u8>);
impl FWS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWS` writer - Flash Wait State"]
pub struct FWS_W<'a> {
    w: &'a mut W,
}
impl<'a> FWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SCOD` reader - Sequential Code Optimization Disable"]
pub struct SCOD_R(crate::FieldReader<bool, bool>);
impl SCOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCOD` writer - Sequential Code Optimization Disable"]
pub struct SCOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CLOE` reader - Code Loop Optimization Enable"]
pub struct CLOE_R(crate::FieldReader<bool, bool>);
impl CLOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOE` writer - Code Loop Optimization Enable"]
pub struct CLOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    pub fn scod(&self) -> SCOD_R {
        SCOD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Code Loop Optimization Enable"]
    #[inline(always)]
    pub fn cloe(&self) -> CLOE_R {
        CLOE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&mut self) -> FRDY_W {
        FRDY_W { w: self }
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&mut self) -> FWS_W {
        FWS_W { w: self }
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    pub fn scod(&mut self) -> SCOD_W {
        SCOD_W { w: self }
    }
    #[doc = "Bit 26 - Code Loop Optimization Enable"]
    #[inline(always)]
    pub fn cloe(&mut self) -> CLOE_W {
        CLOE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEFC Flash Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefc_fmr](index.html) module"]
pub struct EEFC_FMR_SPEC;
impl crate::RegisterSpec for EEFC_FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefc_fmr::R](R) reader structure"]
impl crate::Readable for EEFC_FMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eefc_fmr::W](W) writer structure"]
impl crate::Writable for EEFC_FMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEFC_FMR to value 0"]
impl crate::Resettable for EEFC_FMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
