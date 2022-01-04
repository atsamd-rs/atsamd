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
#[doc = "Field `PER0` reader - Periodic Interval 0"]
pub struct PER0_R(crate::FieldReader<bool, bool>);
impl PER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER0` writer - Periodic Interval 0"]
pub struct PER0_W<'a> {
    w: &'a mut W,
}
impl<'a> PER0_W<'a> {
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
#[doc = "Field `PER1` reader - Periodic Interval 1"]
pub struct PER1_R(crate::FieldReader<bool, bool>);
impl PER1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER1` writer - Periodic Interval 1"]
pub struct PER1_W<'a> {
    w: &'a mut W,
}
impl<'a> PER1_W<'a> {
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
#[doc = "Field `PER2` reader - Periodic Interval 2"]
pub struct PER2_R(crate::FieldReader<bool, bool>);
impl PER2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER2` writer - Periodic Interval 2"]
pub struct PER2_W<'a> {
    w: &'a mut W,
}
impl<'a> PER2_W<'a> {
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
#[doc = "Field `PER3` reader - Periodic Interval 3"]
pub struct PER3_R(crate::FieldReader<bool, bool>);
impl PER3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER3` writer - Periodic Interval 3"]
pub struct PER3_W<'a> {
    w: &'a mut W,
}
impl<'a> PER3_W<'a> {
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
#[doc = "Field `PER4` reader - Periodic Interval 4"]
pub struct PER4_R(crate::FieldReader<bool, bool>);
impl PER4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER4` writer - Periodic Interval 4"]
pub struct PER4_W<'a> {
    w: &'a mut W,
}
impl<'a> PER4_W<'a> {
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
#[doc = "Field `PER5` reader - Periodic Interval 5"]
pub struct PER5_R(crate::FieldReader<bool, bool>);
impl PER5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER5` writer - Periodic Interval 5"]
pub struct PER5_W<'a> {
    w: &'a mut W,
}
impl<'a> PER5_W<'a> {
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
#[doc = "Field `PER6` reader - Periodic Interval 6"]
pub struct PER6_R(crate::FieldReader<bool, bool>);
impl PER6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER6` writer - Periodic Interval 6"]
pub struct PER6_W<'a> {
    w: &'a mut W,
}
impl<'a> PER6_W<'a> {
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
#[doc = "Field `PER7` reader - Periodic Interval 7"]
pub struct PER7_R(crate::FieldReader<bool, bool>);
impl PER7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER7` writer - Periodic Interval 7"]
pub struct PER7_W<'a> {
    w: &'a mut W,
}
impl<'a> PER7_W<'a> {
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
#[doc = "Field `ALARM0` reader - Alarm 0"]
pub struct ALARM0_R(crate::FieldReader<bool, bool>);
impl ALARM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM0` writer - Alarm 0"]
pub struct ALARM0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM0_W<'a> {
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
#[doc = "Field `ALARM1` reader - Alarm 1"]
pub struct ALARM1_R(crate::FieldReader<bool, bool>);
impl ALARM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM1` writer - Alarm 1"]
pub struct ALARM1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TAMPER` reader - Tamper"]
pub struct TAMPER_R(crate::FieldReader<bool, bool>);
impl TAMPER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAMPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPER` writer - Tamper"]
pub struct TAMPER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u16 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `OVF` reader - Overflow"]
pub struct OVF_R(crate::FieldReader<bool, bool>);
impl OVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF` writer - Overflow"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
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
    #[doc = "Bit 0 - Periodic Interval 0"]
    #[inline(always)]
    pub fn per0(&self) -> PER0_R {
        PER0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1"]
    #[inline(always)]
    pub fn per1(&self) -> PER1_R {
        PER1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Periodic Interval 2"]
    #[inline(always)]
    pub fn per2(&self) -> PER2_R {
        PER2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Periodic Interval 3"]
    #[inline(always)]
    pub fn per3(&self) -> PER3_R {
        PER3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Periodic Interval 4"]
    #[inline(always)]
    pub fn per4(&self) -> PER4_R {
        PER4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Periodic Interval 5"]
    #[inline(always)]
    pub fn per5(&self) -> PER5_R {
        PER5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Periodic Interval 6"]
    #[inline(always)]
    pub fn per6(&self) -> PER6_R {
        PER6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Periodic Interval 7"]
    #[inline(always)]
    pub fn per7(&self) -> PER7_R {
        PER7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tamper"]
    #[inline(always)]
    pub fn tamper(&self) -> TAMPER_R {
        TAMPER_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interval 0"]
    #[inline(always)]
    pub fn per0(&mut self) -> PER0_W {
        PER0_W { w: self }
    }
    #[doc = "Bit 1 - Periodic Interval 1"]
    #[inline(always)]
    pub fn per1(&mut self) -> PER1_W {
        PER1_W { w: self }
    }
    #[doc = "Bit 2 - Periodic Interval 2"]
    #[inline(always)]
    pub fn per2(&mut self) -> PER2_W {
        PER2_W { w: self }
    }
    #[doc = "Bit 3 - Periodic Interval 3"]
    #[inline(always)]
    pub fn per3(&mut self) -> PER3_W {
        PER3_W { w: self }
    }
    #[doc = "Bit 4 - Periodic Interval 4"]
    #[inline(always)]
    pub fn per4(&mut self) -> PER4_W {
        PER4_W { w: self }
    }
    #[doc = "Bit 5 - Periodic Interval 5"]
    #[inline(always)]
    pub fn per5(&mut self) -> PER5_W {
        PER5_W { w: self }
    }
    #[doc = "Bit 6 - Periodic Interval 6"]
    #[inline(always)]
    pub fn per6(&mut self) -> PER6_W {
        PER6_W { w: self }
    }
    #[doc = "Bit 7 - Periodic Interval 7"]
    #[inline(always)]
    pub fn per7(&mut self) -> PER7_W {
        PER7_W { w: self }
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&mut self) -> ALARM0_W {
        ALARM0_W { w: self }
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&mut self) -> ALARM1_W {
        ALARM1_W { w: self }
    }
    #[doc = "Bit 14 - Tamper"]
    #[inline(always)]
    pub fn tamper(&mut self) -> TAMPER_W {
        TAMPER_W { w: self }
    }
    #[doc = "Bit 15 - Overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE2 Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u16;
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
