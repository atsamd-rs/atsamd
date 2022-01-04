#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETMPD` reader - ETM Power Down"]
pub struct ETMPD_R(crate::FieldReader<bool, bool>);
impl ETMPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETMPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETMPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETMPD` writer - ETM Power Down"]
pub struct ETMPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMPD_W<'a> {
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
#[doc = "Field `PORTSIZE` reader - Port Size bits 2:0"]
pub struct PORTSIZE_R(crate::FieldReader<u8, u8>);
impl PORTSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PORTSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTSIZE` writer - Port Size bits 2:0"]
pub struct PORTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `STALL` reader - Stall Processor"]
pub struct STALL_R(crate::FieldReader<bool, bool>);
impl STALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL` writer - Stall Processor"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
#[doc = "Field `BROUT` reader - Branch Output"]
pub struct BROUT_R(crate::FieldReader<bool, bool>);
impl BROUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BROUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROUT` writer - Branch Output"]
pub struct BROUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BROUT_W<'a> {
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
#[doc = "Field `DBGRQ` reader - Debug Request Control"]
pub struct DBGRQ_R(crate::FieldReader<bool, bool>);
impl DBGRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBGRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGRQ` writer - Debug Request Control"]
pub struct DBGRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PROG` reader - ETM Programming"]
pub struct PROG_R(crate::FieldReader<bool, bool>);
impl PROG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROG` writer - ETM Programming"]
pub struct PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PORTSEL` reader - ETM Port Select"]
pub struct PORTSEL_R(crate::FieldReader<bool, bool>);
impl PORTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTSEL` writer - ETM Port Select"]
pub struct PORTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PORTMODE2` reader - Port Mode bit 2"]
pub struct PORTMODE2_R(crate::FieldReader<bool, bool>);
impl PORTMODE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORTMODE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTMODE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTMODE2` writer - Port Mode bit 2"]
pub struct PORTMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTMODE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PORTMODE` reader - Port Mode bits 1:0"]
pub struct PORTMODE_R(crate::FieldReader<u8, u8>);
impl PORTMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PORTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTMODE` writer - Port Mode bits 1:0"]
pub struct PORTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `PORTSIZE3` reader - Port Size bit 3"]
pub struct PORTSIZE3_R(crate::FieldReader<bool, bool>);
impl PORTSIZE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORTSIZE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTSIZE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTSIZE3` writer - Port Size bit 3"]
pub struct PORTSIZE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTSIZE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TSEN` reader - TimeStamp Enable"]
pub struct TSEN_R(crate::FieldReader<bool, bool>);
impl TSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEN` writer - TimeStamp Enable"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ETM Power Down"]
    #[inline(always)]
    pub fn etmpd(&self) -> ETMPD_R {
        ETMPD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Port Size bits 2:0"]
    #[inline(always)]
    pub fn portsize(&self) -> PORTSIZE_R {
        PORTSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn brout(&self) -> BROUT_R {
        BROUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgrq(&self) -> DBGRQ_R {
        DBGRQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ETM Port Select"]
    #[inline(always)]
    pub fn portsel(&self) -> PORTSEL_R {
        PORTSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Mode bit 2"]
    #[inline(always)]
    pub fn portmode2(&self) -> PORTMODE2_R {
        PORTMODE2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Port Mode bits 1:0"]
    #[inline(always)]
    pub fn portmode(&self) -> PORTMODE_R {
        PORTMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Port Size bit 3"]
    #[inline(always)]
    pub fn portsize3(&self) -> PORTSIZE3_R {
        PORTSIZE3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TimeStamp Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETM Power Down"]
    #[inline(always)]
    pub fn etmpd(&mut self) -> ETMPD_W {
        ETMPD_W { w: self }
    }
    #[doc = "Bits 4:6 - Port Size bits 2:0"]
    #[inline(always)]
    pub fn portsize(&mut self) -> PORTSIZE_W {
        PORTSIZE_W { w: self }
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn brout(&mut self) -> BROUT_W {
        BROUT_W { w: self }
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgrq(&mut self) -> DBGRQ_W {
        DBGRQ_W { w: self }
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W {
        PROG_W { w: self }
    }
    #[doc = "Bit 11 - ETM Port Select"]
    #[inline(always)]
    pub fn portsel(&mut self) -> PORTSEL_W {
        PORTSEL_W { w: self }
    }
    #[doc = "Bit 13 - Port Mode bit 2"]
    #[inline(always)]
    pub fn portmode2(&mut self) -> PORTMODE2_W {
        PORTMODE2_W { w: self }
    }
    #[doc = "Bits 16:17 - Port Mode bits 1:0"]
    #[inline(always)]
    pub fn portmode(&mut self) -> PORTMODE_W {
        PORTMODE_W { w: self }
    }
    #[doc = "Bit 21 - Port Size bit 3"]
    #[inline(always)]
    pub fn portsize3(&mut self) -> PORTSIZE3_W {
        PORTSIZE3_W { w: self }
    }
    #[doc = "Bit 28 - TimeStamp Enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x0411"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0411
    }
}
