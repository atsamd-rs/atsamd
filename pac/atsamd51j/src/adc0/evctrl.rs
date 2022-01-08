#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLUSHEI` reader - Flush Event Input Enable"]
pub struct FLUSHEI_R(crate::FieldReader<bool, bool>);
impl FLUSHEI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLUSHEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLUSHEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLUSHEI` writer - Flush Event Input Enable"]
pub struct FLUSHEI_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHEI_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `STARTEI` reader - Start Conversion Event Input Enable"]
pub struct STARTEI_R(crate::FieldReader<bool, bool>);
impl STARTEI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STARTEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTEI` writer - Start Conversion Event Input Enable"]
pub struct STARTEI_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTEI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `FLUSHINV` reader - Flush Event Invert Enable"]
pub struct FLUSHINV_R(crate::FieldReader<bool, bool>);
impl FLUSHINV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLUSHINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLUSHINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLUSHINV` writer - Flush Event Invert Enable"]
pub struct FLUSHINV_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `STARTINV` reader - Start Conversion Event Invert Enable"]
pub struct STARTINV_R(crate::FieldReader<bool, bool>);
impl STARTINV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STARTINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTINV` writer - Start Conversion Event Invert Enable"]
pub struct STARTINV_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RESRDYEO` reader - Result Ready Event Out"]
pub struct RESRDYEO_R(crate::FieldReader<bool, bool>);
impl RESRDYEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESRDYEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESRDYEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESRDYEO` writer - Result Ready Event Out"]
pub struct RESRDYEO_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRDYEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `WINMONEO` reader - Window Monitor Event Out"]
pub struct WINMONEO_R(crate::FieldReader<bool, bool>);
impl WINMONEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINMONEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINMONEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINMONEO` writer - Window Monitor Event Out"]
pub struct WINMONEO_W<'a> {
    w: &'a mut W,
}
impl<'a> WINMONEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flush Event Input Enable"]
    #[inline(always)]
    pub fn flushei(&self) -> FLUSHEI_R {
        FLUSHEI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Input Enable"]
    #[inline(always)]
    pub fn startei(&self) -> STARTEI_R {
        STARTEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flush Event Invert Enable"]
    #[inline(always)]
    pub fn flushinv(&self) -> FLUSHINV_R {
        FLUSHINV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start Conversion Event Invert Enable"]
    #[inline(always)]
    pub fn startinv(&self) -> STARTINV_R {
        STARTINV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    pub fn resrdyeo(&self) -> RESRDYEO_R {
        RESRDYEO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn winmoneo(&self) -> WINMONEO_R {
        WINMONEO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Event Input Enable"]
    #[inline(always)]
    pub fn flushei(&mut self) -> FLUSHEI_W {
        FLUSHEI_W { w: self }
    }
    #[doc = "Bit 1 - Start Conversion Event Input Enable"]
    #[inline(always)]
    pub fn startei(&mut self) -> STARTEI_W {
        STARTEI_W { w: self }
    }
    #[doc = "Bit 2 - Flush Event Invert Enable"]
    #[inline(always)]
    pub fn flushinv(&mut self) -> FLUSHINV_W {
        FLUSHINV_W { w: self }
    }
    #[doc = "Bit 3 - Start Conversion Event Invert Enable"]
    #[inline(always)]
    pub fn startinv(&mut self) -> STARTINV_W {
        STARTINV_W { w: self }
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    pub fn resrdyeo(&mut self) -> RESRDYEO_W {
        RESRDYEO_W { w: self }
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn winmoneo(&mut self) -> WINMONEO_W {
        WINMONEO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
