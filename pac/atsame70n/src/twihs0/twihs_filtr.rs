#[doc = "Register `TWIHS_FILTR` reader"]
pub struct R(crate::R<TWIHS_FILTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_FILTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_FILTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_FILTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWIHS_FILTR` writer"]
pub struct W(crate::W<TWIHS_FILTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWIHS_FILTR_SPEC>;
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
impl From<crate::W<TWIHS_FILTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWIHS_FILTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILT` reader - RX Digital Filter"]
pub struct FILT_R(crate::FieldReader<bool, bool>);
impl FILT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILT` writer - RX Digital Filter"]
pub struct FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_W<'a> {
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
#[doc = "Field `PADFEN` reader - PAD Filter Enable"]
pub struct PADFEN_R(crate::FieldReader<bool, bool>);
impl PADFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PADFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADFEN` writer - PAD Filter Enable"]
pub struct PADFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PADFEN_W<'a> {
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
#[doc = "Field `PADFCFG` reader - PAD Filter Config"]
pub struct PADFCFG_R(crate::FieldReader<bool, bool>);
impl PADFCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PADFCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADFCFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADFCFG` writer - PAD Filter Config"]
pub struct PADFCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PADFCFG_W<'a> {
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
#[doc = "Field `THRES` reader - Digital Filter Threshold"]
pub struct THRES_R(crate::FieldReader<u8, u8>);
impl THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES` writer - Digital Filter Threshold"]
pub struct THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    pub fn padfen(&self) -> PADFEN_R {
        PADFEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PAD Filter Config"]
    #[inline(always)]
    pub fn padfcfg(&self) -> PADFCFG_R {
        PADFCFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    pub fn thres(&self) -> THRES_R {
        THRES_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FILT_W {
        FILT_W { w: self }
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    pub fn padfen(&mut self) -> PADFEN_W {
        PADFEN_W { w: self }
    }
    #[doc = "Bit 2 - PAD Filter Config"]
    #[inline(always)]
    pub fn padfcfg(&mut self) -> PADFCFG_W {
        PADFCFG_W { w: self }
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    pub fn thres(&mut self) -> THRES_W {
        THRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_filtr](index.html) module"]
pub struct TWIHS_FILTR_SPEC;
impl crate::RegisterSpec for TWIHS_FILTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_filtr::R](R) reader structure"]
impl crate::Readable for TWIHS_FILTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twihs_filtr::W](W) writer structure"]
impl crate::Writable for TWIHS_FILTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWIHS_FILTR to value 0"]
impl crate::Resettable for TWIHS_FILTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
