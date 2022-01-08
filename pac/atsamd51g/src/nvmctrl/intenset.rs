#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - Command Done Interrupt Enable"]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE` writer - Command Done Interrupt Enable"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
#[doc = "Field `ADDRE` reader - Address Error Interrupt Enable"]
pub struct ADDRE_R(crate::FieldReader<bool, bool>);
impl ADDRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRE` writer - Address Error Interrupt Enable"]
pub struct ADDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRE_W<'a> {
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
#[doc = "Field `PROGE` reader - Programming Error Interrupt Enable"]
pub struct PROGE_R(crate::FieldReader<bool, bool>);
impl PROGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROGE` writer - Programming Error Interrupt Enable"]
pub struct PROGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGE_W<'a> {
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
#[doc = "Field `LOCKE` reader - Lock Error Interrupt Enable"]
pub struct LOCKE_R(crate::FieldReader<bool, bool>);
impl LOCKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKE` writer - Lock Error Interrupt Enable"]
pub struct LOCKE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKE_W<'a> {
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
#[doc = "Field `ECCSE` reader - ECC Single Error Interrupt Enable"]
pub struct ECCSE_R(crate::FieldReader<bool, bool>);
impl ECCSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCSE` writer - ECC Single Error Interrupt Enable"]
pub struct ECCSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCSE_W<'a> {
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
#[doc = "Field `ECCDE` reader - ECC Dual Error Interrupt Enable"]
pub struct ECCDE_R(crate::FieldReader<bool, bool>);
impl ECCDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCDE` writer - ECC Dual Error Interrupt Enable"]
pub struct ECCDE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCDE_W<'a> {
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
#[doc = "Field `NVME` reader - NVM Error Interrupt Enable"]
pub struct NVME_R(crate::FieldReader<bool, bool>);
impl NVME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NVME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVME` writer - NVM Error Interrupt Enable"]
pub struct NVME_W<'a> {
    w: &'a mut W,
}
impl<'a> NVME_W<'a> {
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
#[doc = "Field `SUSP` reader - Suspended Write Or Erase Interrupt Enable"]
pub struct SUSP_R(crate::FieldReader<bool, bool>);
impl SUSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP` writer - Suspended Write Or Erase Interrupt Enable"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
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
#[doc = "Field `SEESFULL` reader - Active SEES Full Interrupt Enable"]
pub struct SEESFULL_R(crate::FieldReader<bool, bool>);
impl SEESFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEESFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEESFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEESFULL` writer - Active SEES Full Interrupt Enable"]
pub struct SEESFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEESFULL_W<'a> {
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
#[doc = "Field `SEESOVF` reader - Active SEES Overflow Interrupt Enable"]
pub struct SEESOVF_R(crate::FieldReader<bool, bool>);
impl SEESOVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEESOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEESOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEESOVF` writer - Active SEES Overflow Interrupt Enable"]
pub struct SEESOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEESOVF_W<'a> {
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
#[doc = "Field `SEEWRC` reader - SEE Write Completed Interrupt Enable"]
pub struct SEEWRC_R(crate::FieldReader<bool, bool>);
impl SEEWRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEEWRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEEWRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEEWRC` writer - SEE Write Completed Interrupt Enable"]
pub struct SEEWRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEEWRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Command Done Interrupt Enable"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Address Error Interrupt Enable"]
    #[inline(always)]
    pub fn addre(&self) -> ADDRE_R {
        ADDRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Programming Error Interrupt Enable"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Error Interrupt Enable"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ECC Single Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ECC Dual Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccde(&self) -> ECCDE_R {
        ECCDE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NVM Error Interrupt Enable"]
    #[inline(always)]
    pub fn nvme(&self) -> NVME_R {
        NVME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Suspended Write Or Erase Interrupt Enable"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Active SEES Full Interrupt Enable"]
    #[inline(always)]
    pub fn seesfull(&self) -> SEESFULL_R {
        SEESFULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Active SEES Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn seesovf(&self) -> SEESOVF_R {
        SEESOVF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SEE Write Completed Interrupt Enable"]
    #[inline(always)]
    pub fn seewrc(&self) -> SEEWRC_R {
        SEEWRC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Done Interrupt Enable"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bit 1 - Address Error Interrupt Enable"]
    #[inline(always)]
    pub fn addre(&mut self) -> ADDRE_W {
        ADDRE_W { w: self }
    }
    #[doc = "Bit 2 - Programming Error Interrupt Enable"]
    #[inline(always)]
    pub fn proge(&mut self) -> PROGE_W {
        PROGE_W { w: self }
    }
    #[doc = "Bit 3 - Lock Error Interrupt Enable"]
    #[inline(always)]
    pub fn locke(&mut self) -> LOCKE_W {
        LOCKE_W { w: self }
    }
    #[doc = "Bit 4 - ECC Single Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccse(&mut self) -> ECCSE_W {
        ECCSE_W { w: self }
    }
    #[doc = "Bit 5 - ECC Dual Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccde(&mut self) -> ECCDE_W {
        ECCDE_W { w: self }
    }
    #[doc = "Bit 6 - NVM Error Interrupt Enable"]
    #[inline(always)]
    pub fn nvme(&mut self) -> NVME_W {
        NVME_W { w: self }
    }
    #[doc = "Bit 7 - Suspended Write Or Erase Interrupt Enable"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Bit 8 - Active SEES Full Interrupt Enable"]
    #[inline(always)]
    pub fn seesfull(&mut self) -> SEESFULL_W {
        SEESFULL_W { w: self }
    }
    #[doc = "Bit 9 - Active SEES Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn seesovf(&mut self) -> SEESOVF_W {
        SEESOVF_W { w: self }
    }
    #[doc = "Bit 10 - SEE Write Completed Interrupt Enable"]
    #[inline(always)]
    pub fn seewrc(&mut self) -> SEEWRC_W {
        SEEWRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
