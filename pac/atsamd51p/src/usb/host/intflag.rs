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
#[doc = "Field `HSOF` reader - Host Start Of Frame"]
pub struct HSOF_R(crate::FieldReader<bool, bool>);
impl HSOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSOF` writer - Host Start Of Frame"]
pub struct HSOF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSOF_W<'a> {
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
#[doc = "Field `RST` reader - Bus Reset"]
pub struct RST_R(crate::FieldReader<bool, bool>);
impl RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST` writer - Bus Reset"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
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
#[doc = "Field `WAKEUP` reader - Wake Up"]
pub struct WAKEUP_R(crate::FieldReader<bool, bool>);
impl WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` writer - Wake Up"]
pub struct WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
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
#[doc = "Field `DNRSM` reader - Downstream"]
pub struct DNRSM_R(crate::FieldReader<bool, bool>);
impl DNRSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DNRSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNRSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNRSM` writer - Downstream"]
pub struct DNRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> DNRSM_W<'a> {
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
#[doc = "Field `UPRSM` reader - Upstream Resume from the Device"]
pub struct UPRSM_R(crate::FieldReader<bool, bool>);
impl UPRSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPRSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPRSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPRSM` writer - Upstream Resume from the Device"]
pub struct UPRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSM_W<'a> {
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
#[doc = "Field `RAMACER` reader - Ram Access"]
pub struct RAMACER_R(crate::FieldReader<bool, bool>);
impl RAMACER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMACER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMACER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMACER` writer - Ram Access"]
pub struct RAMACER_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACER_W<'a> {
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
#[doc = "Field `DCONN` reader - Device Connection"]
pub struct DCONN_R(crate::FieldReader<bool, bool>);
impl DCONN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCONN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCONN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCONN` writer - Device Connection"]
pub struct DCONN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONN_W<'a> {
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
#[doc = "Field `DDISC` reader - Device Disconnection"]
pub struct DDISC_R(crate::FieldReader<bool, bool>);
impl DDISC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDISC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDISC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDISC` writer - Device Disconnection"]
pub struct DDISC_W<'a> {
    w: &'a mut W,
}
impl<'a> DDISC_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Host Start Of Frame"]
    #[inline(always)]
    pub fn hsof(&self) -> HSOF_R {
        HSOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bus Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake Up"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Downstream"]
    #[inline(always)]
    pub fn dnrsm(&self) -> DNRSM_R {
        DNRSM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume from the Device"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Ram Access"]
    #[inline(always)]
    pub fn ramacer(&self) -> RAMACER_R {
        RAMACER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Device Connection"]
    #[inline(always)]
    pub fn dconn(&self) -> DCONN_R {
        DCONN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Device Disconnection"]
    #[inline(always)]
    pub fn ddisc(&self) -> DDISC_R {
        DDISC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Host Start Of Frame"]
    #[inline(always)]
    pub fn hsof(&mut self) -> HSOF_W {
        HSOF_W { w: self }
    }
    #[doc = "Bit 3 - Bus Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 4 - Wake Up"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
    #[doc = "Bit 5 - Downstream"]
    #[inline(always)]
    pub fn dnrsm(&mut self) -> DNRSM_W {
        DNRSM_W { w: self }
    }
    #[doc = "Bit 6 - Upstream Resume from the Device"]
    #[inline(always)]
    pub fn uprsm(&mut self) -> UPRSM_W {
        UPRSM_W { w: self }
    }
    #[doc = "Bit 7 - Ram Access"]
    #[inline(always)]
    pub fn ramacer(&mut self) -> RAMACER_W {
        RAMACER_W { w: self }
    }
    #[doc = "Bit 8 - Device Connection"]
    #[inline(always)]
    pub fn dconn(&mut self) -> DCONN_W {
        DCONN_W { w: self }
    }
    #[doc = "Bit 9 - Device Disconnection"]
    #[inline(always)]
    pub fn ddisc(&mut self) -> DDISC_W {
        DDISC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST Host Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
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
