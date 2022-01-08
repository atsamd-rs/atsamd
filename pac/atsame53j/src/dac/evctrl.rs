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
#[doc = "Field `STARTEI0` reader - Start Conversion Event Input DAC 0"]
pub struct STARTEI0_R(crate::FieldReader<bool, bool>);
impl STARTEI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STARTEI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTEI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTEI0` writer - Start Conversion Event Input DAC 0"]
pub struct STARTEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTEI0_W<'a> {
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
#[doc = "Field `STARTEI1` reader - Start Conversion Event Input DAC 1"]
pub struct STARTEI1_R(crate::FieldReader<bool, bool>);
impl STARTEI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STARTEI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTEI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTEI1` writer - Start Conversion Event Input DAC 1"]
pub struct STARTEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTEI1_W<'a> {
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
#[doc = "Field `EMPTYEO0` reader - Data Buffer Empty Event Output DAC 0"]
pub struct EMPTYEO0_R(crate::FieldReader<bool, bool>);
impl EMPTYEO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPTYEO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPTYEO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPTYEO0` writer - Data Buffer Empty Event Output DAC 0"]
pub struct EMPTYEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTYEO0_W<'a> {
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
#[doc = "Field `EMPTYEO1` reader - Data Buffer Empty Event Output DAC 1"]
pub struct EMPTYEO1_R(crate::FieldReader<bool, bool>);
impl EMPTYEO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPTYEO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPTYEO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPTYEO1` writer - Data Buffer Empty Event Output DAC 1"]
pub struct EMPTYEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTYEO1_W<'a> {
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
#[doc = "Field `INVEI0` reader - Enable Invertion of DAC 0 input event"]
pub struct INVEI0_R(crate::FieldReader<bool, bool>);
impl INVEI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEI0` writer - Enable Invertion of DAC 0 input event"]
pub struct INVEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI0_W<'a> {
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
#[doc = "Field `INVEI1` reader - Enable Invertion of DAC 1 input event"]
pub struct INVEI1_R(crate::FieldReader<bool, bool>);
impl INVEI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEI1` writer - Enable Invertion of DAC 1 input event"]
pub struct INVEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI1_W<'a> {
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
#[doc = "Field `RESRDYEO0` reader - Result Ready Event Output 0"]
pub struct RESRDYEO0_R(crate::FieldReader<bool, bool>);
impl RESRDYEO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESRDYEO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESRDYEO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESRDYEO0` writer - Result Ready Event Output 0"]
pub struct RESRDYEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRDYEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RESRDYEO1` reader - Result Ready Event Output 1"]
pub struct RESRDYEO1_R(crate::FieldReader<bool, bool>);
impl RESRDYEO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESRDYEO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESRDYEO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESRDYEO1` writer - Result Ready Event Output 1"]
pub struct RESRDYEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRDYEO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start Conversion Event Input DAC 0"]
    #[inline(always)]
    pub fn startei0(&self) -> STARTEI0_R {
        STARTEI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Input DAC 1"]
    #[inline(always)]
    pub fn startei1(&self) -> STARTEI1_R {
        STARTEI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Buffer Empty Event Output DAC 0"]
    #[inline(always)]
    pub fn emptyeo0(&self) -> EMPTYEO0_R {
        EMPTYEO0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Buffer Empty Event Output DAC 1"]
    #[inline(always)]
    pub fn emptyeo1(&self) -> EMPTYEO1_R {
        EMPTYEO1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Invertion of DAC 0 input event"]
    #[inline(always)]
    pub fn invei0(&self) -> INVEI0_R {
        INVEI0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Invertion of DAC 1 input event"]
    #[inline(always)]
    pub fn invei1(&self) -> INVEI1_R {
        INVEI1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Result Ready Event Output 0"]
    #[inline(always)]
    pub fn resrdyeo0(&self) -> RESRDYEO0_R {
        RESRDYEO0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Result Ready Event Output 1"]
    #[inline(always)]
    pub fn resrdyeo1(&self) -> RESRDYEO1_R {
        RESRDYEO1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event Input DAC 0"]
    #[inline(always)]
    pub fn startei0(&mut self) -> STARTEI0_W {
        STARTEI0_W { w: self }
    }
    #[doc = "Bit 1 - Start Conversion Event Input DAC 1"]
    #[inline(always)]
    pub fn startei1(&mut self) -> STARTEI1_W {
        STARTEI1_W { w: self }
    }
    #[doc = "Bit 2 - Data Buffer Empty Event Output DAC 0"]
    #[inline(always)]
    pub fn emptyeo0(&mut self) -> EMPTYEO0_W {
        EMPTYEO0_W { w: self }
    }
    #[doc = "Bit 3 - Data Buffer Empty Event Output DAC 1"]
    #[inline(always)]
    pub fn emptyeo1(&mut self) -> EMPTYEO1_W {
        EMPTYEO1_W { w: self }
    }
    #[doc = "Bit 4 - Enable Invertion of DAC 0 input event"]
    #[inline(always)]
    pub fn invei0(&mut self) -> INVEI0_W {
        INVEI0_W { w: self }
    }
    #[doc = "Bit 5 - Enable Invertion of DAC 1 input event"]
    #[inline(always)]
    pub fn invei1(&mut self) -> INVEI1_W {
        INVEI1_W { w: self }
    }
    #[doc = "Bit 6 - Result Ready Event Output 0"]
    #[inline(always)]
    pub fn resrdyeo0(&mut self) -> RESRDYEO0_W {
        RESRDYEO0_W { w: self }
    }
    #[doc = "Bit 7 - Result Ready Event Output 1"]
    #[inline(always)]
    pub fn resrdyeo1(&mut self) -> RESRDYEO1_W {
        RESRDYEO1_W { w: self }
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
