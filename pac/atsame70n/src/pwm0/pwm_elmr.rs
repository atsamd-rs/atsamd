#[doc = "Register `PWM_ELMR[%s]` reader"]
pub struct R(crate::R<PWM_ELMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_ELMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_ELMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_ELMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_ELMR[%s]` writer"]
pub struct W(crate::W<PWM_ELMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_ELMR_SPEC>;
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
impl From<crate::W<PWM_ELMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_ELMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEL0` reader - Comparison 0 Selection"]
pub struct CSEL0_R(crate::FieldReader<bool, bool>);
impl CSEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEL0` writer - Comparison 0 Selection"]
pub struct CSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL0_W<'a> {
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
#[doc = "Field `CSEL1` reader - Comparison 1 Selection"]
pub struct CSEL1_R(crate::FieldReader<bool, bool>);
impl CSEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEL1` writer - Comparison 1 Selection"]
pub struct CSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL1_W<'a> {
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
#[doc = "Field `CSEL2` reader - Comparison 2 Selection"]
pub struct CSEL2_R(crate::FieldReader<bool, bool>);
impl CSEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEL2` writer - Comparison 2 Selection"]
pub struct CSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL2_W<'a> {
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
#[doc = "Field `CSEL3` reader - Comparison 3 Selection"]
pub struct CSEL3_R(crate::FieldReader<bool, bool>);
impl CSEL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEL3` writer - Comparison 3 Selection"]
pub struct CSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL3_W<'a> {
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
#[doc = "Field `CSEL4` reader - Comparison 4 Selection"]
pub struct CSEL4_R(crate::FieldReader<bool, bool>);
impl CSEL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEL4` writer - Comparison 4 Selection"]
pub struct CSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL4_W<'a> {
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
#[doc = "Field `CSEL5` reader - Comparison 5 Selection"]
pub struct CSEL5_R(crate::FieldReader<bool, bool>);
impl CSEL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSEL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEL5` writer - Comparison 5 Selection"]
pub struct CSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL5_W<'a> {
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
#[doc = "Field `CSEL6` reader - Comparison 6 Selection"]
pub struct CSEL6_R(crate::FieldReader<bool, bool>);
impl CSEL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSEL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEL6` writer - Comparison 6 Selection"]
pub struct CSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL6_W<'a> {
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
#[doc = "Field `CSEL7` reader - Comparison 7 Selection"]
pub struct CSEL7_R(crate::FieldReader<bool, bool>);
impl CSEL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSEL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEL7` writer - Comparison 7 Selection"]
pub struct CSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Comparison 0 Selection"]
    #[inline(always)]
    pub fn csel0(&self) -> CSEL0_R {
        CSEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparison 1 Selection"]
    #[inline(always)]
    pub fn csel1(&self) -> CSEL1_R {
        CSEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparison 2 Selection"]
    #[inline(always)]
    pub fn csel2(&self) -> CSEL2_R {
        CSEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparison 3 Selection"]
    #[inline(always)]
    pub fn csel3(&self) -> CSEL3_R {
        CSEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparison 4 Selection"]
    #[inline(always)]
    pub fn csel4(&self) -> CSEL4_R {
        CSEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparison 5 Selection"]
    #[inline(always)]
    pub fn csel5(&self) -> CSEL5_R {
        CSEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comparison 6 Selection"]
    #[inline(always)]
    pub fn csel6(&self) -> CSEL6_R {
        CSEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comparison 7 Selection"]
    #[inline(always)]
    pub fn csel7(&self) -> CSEL7_R {
        CSEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison 0 Selection"]
    #[inline(always)]
    pub fn csel0(&mut self) -> CSEL0_W {
        CSEL0_W { w: self }
    }
    #[doc = "Bit 1 - Comparison 1 Selection"]
    #[inline(always)]
    pub fn csel1(&mut self) -> CSEL1_W {
        CSEL1_W { w: self }
    }
    #[doc = "Bit 2 - Comparison 2 Selection"]
    #[inline(always)]
    pub fn csel2(&mut self) -> CSEL2_W {
        CSEL2_W { w: self }
    }
    #[doc = "Bit 3 - Comparison 3 Selection"]
    #[inline(always)]
    pub fn csel3(&mut self) -> CSEL3_W {
        CSEL3_W { w: self }
    }
    #[doc = "Bit 4 - Comparison 4 Selection"]
    #[inline(always)]
    pub fn csel4(&mut self) -> CSEL4_W {
        CSEL4_W { w: self }
    }
    #[doc = "Bit 5 - Comparison 5 Selection"]
    #[inline(always)]
    pub fn csel5(&mut self) -> CSEL5_W {
        CSEL5_W { w: self }
    }
    #[doc = "Bit 6 - Comparison 6 Selection"]
    #[inline(always)]
    pub fn csel6(&mut self) -> CSEL6_W {
        CSEL6_W { w: self }
    }
    #[doc = "Bit 7 - Comparison 7 Selection"]
    #[inline(always)]
    pub fn csel7(&mut self) -> CSEL7_W {
        CSEL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Event Line 0 Mode Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_elmr](index.html) module"]
pub struct PWM_ELMR_SPEC;
impl crate::RegisterSpec for PWM_ELMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_elmr::R](R) reader structure"]
impl crate::Readable for PWM_ELMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_elmr::W](W) writer structure"]
impl crate::Writable for PWM_ELMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_ELMR[%s]
to value 0"]
impl crate::Resettable for PWM_ELMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
