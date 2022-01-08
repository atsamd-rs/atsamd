#[doc = "Register `WAKEUP` reader"]
pub struct R(crate::R<WAKEUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP` writer"]
pub struct W(crate::W<WAKEUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_SPEC>;
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
impl From<crate::W<WAKEUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUPEN0` reader - External Interrupt 0 Wake-up Enable"]
pub struct WAKEUPEN0_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN0` writer - External Interrupt 0 Wake-up Enable"]
pub struct WAKEUPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN0_W<'a> {
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
#[doc = "Field `WAKEUPEN1` reader - External Interrupt 1 Wake-up Enable"]
pub struct WAKEUPEN1_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN1` writer - External Interrupt 1 Wake-up Enable"]
pub struct WAKEUPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN1_W<'a> {
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
#[doc = "Field `WAKEUPEN2` reader - External Interrupt 2 Wake-up Enable"]
pub struct WAKEUPEN2_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN2` writer - External Interrupt 2 Wake-up Enable"]
pub struct WAKEUPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN2_W<'a> {
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
#[doc = "Field `WAKEUPEN3` reader - External Interrupt 3 Wake-up Enable"]
pub struct WAKEUPEN3_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN3` writer - External Interrupt 3 Wake-up Enable"]
pub struct WAKEUPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN3_W<'a> {
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
#[doc = "Field `WAKEUPEN4` reader - External Interrupt 4 Wake-up Enable"]
pub struct WAKEUPEN4_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN4` writer - External Interrupt 4 Wake-up Enable"]
pub struct WAKEUPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN4_W<'a> {
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
#[doc = "Field `WAKEUPEN5` reader - External Interrupt 5 Wake-up Enable"]
pub struct WAKEUPEN5_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN5` writer - External Interrupt 5 Wake-up Enable"]
pub struct WAKEUPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN5_W<'a> {
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
#[doc = "Field `WAKEUPEN6` reader - External Interrupt 6 Wake-up Enable"]
pub struct WAKEUPEN6_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN6` writer - External Interrupt 6 Wake-up Enable"]
pub struct WAKEUPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN6_W<'a> {
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
#[doc = "Field `WAKEUPEN7` reader - External Interrupt 7 Wake-up Enable"]
pub struct WAKEUPEN7_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN7` writer - External Interrupt 7 Wake-up Enable"]
pub struct WAKEUPEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN7_W<'a> {
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
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen0(&self) -> WAKEUPEN0_R {
        WAKEUPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen1(&self) -> WAKEUPEN1_R {
        WAKEUPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen2(&self) -> WAKEUPEN2_R {
        WAKEUPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen3(&self) -> WAKEUPEN3_R {
        WAKEUPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen4(&self) -> WAKEUPEN4_R {
        WAKEUPEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen5(&self) -> WAKEUPEN5_R {
        WAKEUPEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen6(&self) -> WAKEUPEN6_R {
        WAKEUPEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen7(&self) -> WAKEUPEN7_R {
        WAKEUPEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen0(&mut self) -> WAKEUPEN0_W {
        WAKEUPEN0_W { w: self }
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen1(&mut self) -> WAKEUPEN1_W {
        WAKEUPEN1_W { w: self }
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen2(&mut self) -> WAKEUPEN2_W {
        WAKEUPEN2_W { w: self }
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen3(&mut self) -> WAKEUPEN3_W {
        WAKEUPEN3_W { w: self }
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen4(&mut self) -> WAKEUPEN4_W {
        WAKEUPEN4_W { w: self }
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen5(&mut self) -> WAKEUPEN5_W {
        WAKEUPEN5_W { w: self }
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen6(&mut self) -> WAKEUPEN6_W {
        WAKEUPEN6_W { w: self }
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen7(&mut self) -> WAKEUPEN7_W {
        WAKEUPEN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-Up Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup](index.html) module"]
pub struct WAKEUP_SPEC;
impl crate::RegisterSpec for WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup::R](R) reader structure"]
impl crate::Readable for WAKEUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup::W](W) writer structure"]
impl crate::Writable for WAKEUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEUP to value 0"]
impl crate::Resettable for WAKEUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
