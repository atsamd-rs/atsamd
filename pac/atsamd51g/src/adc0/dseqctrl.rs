#[doc = "Register `DSEQCTRL` reader"]
pub struct R(crate::R<DSEQCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSEQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSEQCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSEQCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSEQCTRL` writer"]
pub struct W(crate::W<DSEQCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSEQCTRL_SPEC>;
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
impl From<crate::W<DSEQCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSEQCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTCTRL` reader - Input Control"]
pub struct INPUTCTRL_R(crate::FieldReader<bool, bool>);
impl INPUTCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUTCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUTCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUTCTRL` writer - Input Control"]
pub struct INPUTCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTCTRL_W<'a> {
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
#[doc = "Field `CTRLB` reader - Control B"]
pub struct CTRLB_R(crate::FieldReader<bool, bool>);
impl CTRLB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLB` writer - Control B"]
pub struct CTRLB_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLB_W<'a> {
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
#[doc = "Field `REFCTRL` reader - Reference Control"]
pub struct REFCTRL_R(crate::FieldReader<bool, bool>);
impl REFCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCTRL` writer - Reference Control"]
pub struct REFCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCTRL_W<'a> {
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
#[doc = "Field `AVGCTRL` reader - Average Control"]
pub struct AVGCTRL_R(crate::FieldReader<bool, bool>);
impl AVGCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AVGCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVGCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVGCTRL` writer - Average Control"]
pub struct AVGCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> AVGCTRL_W<'a> {
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
#[doc = "Field `SAMPCTRL` reader - Sampling Time Control"]
pub struct SAMPCTRL_R(crate::FieldReader<bool, bool>);
impl SAMPCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMPCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMPCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPCTRL` writer - Sampling Time Control"]
pub struct SAMPCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPCTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `WINLT` reader - Window Monitor Lower Threshold"]
pub struct WINLT_R(crate::FieldReader<bool, bool>);
impl WINLT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINLT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINLT` writer - Window Monitor Lower Threshold"]
pub struct WINLT_W<'a> {
    w: &'a mut W,
}
impl<'a> WINLT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `WINUT` reader - Window Monitor Upper Threshold"]
pub struct WINUT_R(crate::FieldReader<bool, bool>);
impl WINUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINUT` writer - Window Monitor Upper Threshold"]
pub struct WINUT_W<'a> {
    w: &'a mut W,
}
impl<'a> WINUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `GAINCORR` reader - Gain Correction"]
pub struct GAINCORR_R(crate::FieldReader<bool, bool>);
impl GAINCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GAINCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAINCORR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAINCORR` writer - Gain Correction"]
pub struct GAINCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINCORR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `OFFSETCORR` reader - Offset Correction"]
pub struct OFFSETCORR_R(crate::FieldReader<bool, bool>);
impl OFFSETCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFFSETCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSETCORR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSETCORR` writer - Offset Correction"]
pub struct OFFSETCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETCORR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `AUTOSTART` reader - ADC Auto-Start Conversion"]
pub struct AUTOSTART_R(crate::FieldReader<bool, bool>);
impl AUTOSTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOSTART` writer - ADC Auto-Start Conversion"]
pub struct AUTOSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTART_W<'a> {
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
    #[doc = "Bit 0 - Input Control"]
    #[inline(always)]
    pub fn inputctrl(&self) -> INPUTCTRL_R {
        INPUTCTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control B"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reference Control"]
    #[inline(always)]
    pub fn refctrl(&self) -> REFCTRL_R {
        REFCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Average Control"]
    #[inline(always)]
    pub fn avgctrl(&self) -> AVGCTRL_R {
        AVGCTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sampling Time Control"]
    #[inline(always)]
    pub fn sampctrl(&self) -> SAMPCTRL_R {
        SAMPCTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Window Monitor Upper Threshold"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ADC Auto-Start Conversion"]
    #[inline(always)]
    pub fn autostart(&self) -> AUTOSTART_R {
        AUTOSTART_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Control"]
    #[inline(always)]
    pub fn inputctrl(&mut self) -> INPUTCTRL_W {
        INPUTCTRL_W { w: self }
    }
    #[doc = "Bit 1 - Control B"]
    #[inline(always)]
    pub fn ctrlb(&mut self) -> CTRLB_W {
        CTRLB_W { w: self }
    }
    #[doc = "Bit 2 - Reference Control"]
    #[inline(always)]
    pub fn refctrl(&mut self) -> REFCTRL_W {
        REFCTRL_W { w: self }
    }
    #[doc = "Bit 3 - Average Control"]
    #[inline(always)]
    pub fn avgctrl(&mut self) -> AVGCTRL_W {
        AVGCTRL_W { w: self }
    }
    #[doc = "Bit 4 - Sampling Time Control"]
    #[inline(always)]
    pub fn sampctrl(&mut self) -> SAMPCTRL_W {
        SAMPCTRL_W { w: self }
    }
    #[doc = "Bit 5 - Window Monitor Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&mut self) -> WINLT_W {
        WINLT_W { w: self }
    }
    #[doc = "Bit 6 - Window Monitor Upper Threshold"]
    #[inline(always)]
    pub fn winut(&mut self) -> WINUT_W {
        WINUT_W { w: self }
    }
    #[doc = "Bit 7 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&mut self) -> GAINCORR_W {
        GAINCORR_W { w: self }
    }
    #[doc = "Bit 8 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&mut self) -> OFFSETCORR_W {
        OFFSETCORR_W { w: self }
    }
    #[doc = "Bit 31 - ADC Auto-Start Conversion"]
    #[inline(always)]
    pub fn autostart(&mut self) -> AUTOSTART_W {
        AUTOSTART_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Sequential Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dseqctrl](index.html) module"]
pub struct DSEQCTRL_SPEC;
impl crate::RegisterSpec for DSEQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dseqctrl::R](R) reader structure"]
impl crate::Readable for DSEQCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dseqctrl::W](W) writer structure"]
impl crate::Writable for DSEQCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSEQCTRL to value 0"]
impl crate::Resettable for DSEQCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
