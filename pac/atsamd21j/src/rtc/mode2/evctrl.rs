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
#[doc = "Field `PEREO0` reader - Periodic Interval 0 Event Output Enable"]
pub struct PEREO0_R(crate::FieldReader<bool, bool>);
impl PEREO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO0` writer - Periodic Interval 0 Event Output Enable"]
pub struct PEREO0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `PEREO1` reader - Periodic Interval 1 Event Output Enable"]
pub struct PEREO1_R(crate::FieldReader<bool, bool>);
impl PEREO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO1` writer - Periodic Interval 1 Event Output Enable"]
pub struct PEREO1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PEREO2` reader - Periodic Interval 2 Event Output Enable"]
pub struct PEREO2_R(crate::FieldReader<bool, bool>);
impl PEREO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO2` writer - Periodic Interval 2 Event Output Enable"]
pub struct PEREO2_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PEREO3` reader - Periodic Interval 3 Event Output Enable"]
pub struct PEREO3_R(crate::FieldReader<bool, bool>);
impl PEREO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO3` writer - Periodic Interval 3 Event Output Enable"]
pub struct PEREO3_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PEREO4` reader - Periodic Interval 4 Event Output Enable"]
pub struct PEREO4_R(crate::FieldReader<bool, bool>);
impl PEREO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO4` writer - Periodic Interval 4 Event Output Enable"]
pub struct PEREO4_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PEREO5` reader - Periodic Interval 5 Event Output Enable"]
pub struct PEREO5_R(crate::FieldReader<bool, bool>);
impl PEREO5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO5` writer - Periodic Interval 5 Event Output Enable"]
pub struct PEREO5_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PEREO6` reader - Periodic Interval 6 Event Output Enable"]
pub struct PEREO6_R(crate::FieldReader<bool, bool>);
impl PEREO6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO6` writer - Periodic Interval 6 Event Output Enable"]
pub struct PEREO6_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PEREO7` reader - Periodic Interval 7 Event Output Enable"]
pub struct PEREO7_R(crate::FieldReader<bool, bool>);
impl PEREO7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO7` writer - Periodic Interval 7 Event Output Enable"]
pub struct PEREO7_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ALARMEO0` reader - Alarm 0 Event Output Enable"]
pub struct ALARMEO0_R(crate::FieldReader<bool, bool>);
impl ALARMEO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARMEO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARMEO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARMEO0` writer - Alarm 0 Event Output Enable"]
pub struct ALARMEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARMEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `OVFEO` reader - Overflow Event Output Enable"]
pub struct OVFEO_R(crate::FieldReader<bool, bool>);
impl OVFEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVFEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFEO` writer - Overflow Event Output Enable"]
pub struct OVFEO_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline(always)]
    pub fn pereo0(&self) -> PEREO0_R {
        PEREO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline(always)]
    pub fn pereo1(&self) -> PEREO1_R {
        PEREO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline(always)]
    pub fn pereo2(&self) -> PEREO2_R {
        PEREO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline(always)]
    pub fn pereo3(&self) -> PEREO3_R {
        PEREO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline(always)]
    pub fn pereo4(&self) -> PEREO4_R {
        PEREO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline(always)]
    pub fn pereo5(&self) -> PEREO5_R {
        PEREO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline(always)]
    pub fn pereo6(&self) -> PEREO6_R {
        PEREO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline(always)]
    pub fn pereo7(&self) -> PEREO7_R {
        PEREO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm 0 Event Output Enable"]
    #[inline(always)]
    pub fn alarmeo0(&self) -> ALARMEO0_R {
        ALARMEO0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline(always)]
    pub fn pereo0(&mut self) -> PEREO0_W {
        PEREO0_W { w: self }
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline(always)]
    pub fn pereo1(&mut self) -> PEREO1_W {
        PEREO1_W { w: self }
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline(always)]
    pub fn pereo2(&mut self) -> PEREO2_W {
        PEREO2_W { w: self }
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline(always)]
    pub fn pereo3(&mut self) -> PEREO3_W {
        PEREO3_W { w: self }
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline(always)]
    pub fn pereo4(&mut self) -> PEREO4_W {
        PEREO4_W { w: self }
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline(always)]
    pub fn pereo5(&mut self) -> PEREO5_W {
        PEREO5_W { w: self }
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline(always)]
    pub fn pereo6(&mut self) -> PEREO6_W {
        PEREO6_W { w: self }
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline(always)]
    pub fn pereo7(&mut self) -> PEREO7_W {
        PEREO7_W { w: self }
    }
    #[doc = "Bit 8 - Alarm 0 Event Output Enable"]
    #[inline(always)]
    pub fn alarmeo0(&mut self) -> ALARMEO0_W {
        ALARMEO0_W { w: self }
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&mut self) -> OVFEO_W {
        OVFEO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE2 Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u16;
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
