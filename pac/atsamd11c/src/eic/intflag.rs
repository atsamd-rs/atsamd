#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTINT0` reader - External Interrupt 0"]
pub struct EXTINT0_R(crate::FieldReader<bool, bool>);
impl EXTINT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT0` writer - External Interrupt 0"]
pub struct EXTINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT0_W<'a> {
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
#[doc = "Field `EXTINT1` reader - External Interrupt 1"]
pub struct EXTINT1_R(crate::FieldReader<bool, bool>);
impl EXTINT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT1` writer - External Interrupt 1"]
pub struct EXTINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT1_W<'a> {
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
#[doc = "Field `EXTINT2` reader - External Interrupt 2"]
pub struct EXTINT2_R(crate::FieldReader<bool, bool>);
impl EXTINT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT2` writer - External Interrupt 2"]
pub struct EXTINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT2_W<'a> {
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
#[doc = "Field `EXTINT3` reader - External Interrupt 3"]
pub struct EXTINT3_R(crate::FieldReader<bool, bool>);
impl EXTINT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT3` writer - External Interrupt 3"]
pub struct EXTINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT3_W<'a> {
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
#[doc = "Field `EXTINT4` reader - External Interrupt 4"]
pub struct EXTINT4_R(crate::FieldReader<bool, bool>);
impl EXTINT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT4` writer - External Interrupt 4"]
pub struct EXTINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT4_W<'a> {
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
#[doc = "Field `EXTINT5` reader - External Interrupt 5"]
pub struct EXTINT5_R(crate::FieldReader<bool, bool>);
impl EXTINT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT5` writer - External Interrupt 5"]
pub struct EXTINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT5_W<'a> {
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
#[doc = "Field `EXTINT6` reader - External Interrupt 6"]
pub struct EXTINT6_R(crate::FieldReader<bool, bool>);
impl EXTINT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT6` writer - External Interrupt 6"]
pub struct EXTINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT6_W<'a> {
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
#[doc = "Field `EXTINT7` reader - External Interrupt 7"]
pub struct EXTINT7_R(crate::FieldReader<bool, bool>);
impl EXTINT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINT7` writer - External Interrupt 7"]
pub struct EXTINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT7_W<'a> {
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
    #[doc = "Bit 0 - External Interrupt 0"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn extint4(&self) -> EXTINT4_R {
        EXTINT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn extint5(&self) -> EXTINT5_R {
        EXTINT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn extint6(&self) -> EXTINT6_R {
        EXTINT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn extint7(&self) -> EXTINT7_R {
        EXTINT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0"]
    #[inline(always)]
    pub fn extint0(&mut self) -> EXTINT0_W {
        EXTINT0_W { w: self }
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn extint1(&mut self) -> EXTINT1_W {
        EXTINT1_W { w: self }
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn extint2(&mut self) -> EXTINT2_W {
        EXTINT2_W { w: self }
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn extint3(&mut self) -> EXTINT3_W {
        EXTINT3_W { w: self }
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn extint4(&mut self) -> EXTINT4_W {
        EXTINT4_W { w: self }
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn extint5(&mut self) -> EXTINT5_W {
        EXTINT5_W { w: self }
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn extint6(&mut self) -> EXTINT6_W {
        EXTINT6_W { w: self }
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn extint7(&mut self) -> EXTINT7_W {
        EXTINT7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
