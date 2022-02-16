#[doc = "Register `TWIHS_SMR` reader"]
pub struct R(crate::R<TWIHS_SMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_SMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_SMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_SMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWIHS_SMR` writer"]
pub struct W(crate::W<TWIHS_SMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWIHS_SMR_SPEC>;
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
impl From<crate::W<TWIHS_SMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWIHS_SMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NACKEN` reader - Slave Receiver Data Phase NACK enable"]
pub struct NACKEN_R(crate::FieldReader<bool, bool>);
impl NACKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACKEN` writer - Slave Receiver Data Phase NACK enable"]
pub struct NACKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKEN_W<'a> {
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
#[doc = "Field `SMDA` reader - SMBus Default Address"]
pub struct SMDA_R(crate::FieldReader<bool, bool>);
impl SMDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMDA` writer - SMBus Default Address"]
pub struct SMDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SMDA_W<'a> {
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
#[doc = "Field `SMHH` reader - SMBus Host Header"]
pub struct SMHH_R(crate::FieldReader<bool, bool>);
impl SMHH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMHH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMHH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMHH` writer - SMBus Host Header"]
pub struct SMHH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMHH_W<'a> {
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
#[doc = "Field `SCLWSDIS` reader - Clock Wait State Disable"]
pub struct SCLWSDIS_R(crate::FieldReader<bool, bool>);
impl SCLWSDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCLWSDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLWSDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLWSDIS` writer - Clock Wait State Disable"]
pub struct SCLWSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLWSDIS_W<'a> {
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
#[doc = "Field `MASK` reader - Slave Address Mask"]
pub struct MASK_R(crate::FieldReader<u8, u8>);
impl MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - Slave Address Mask"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `SADR` reader - Slave Address"]
pub struct SADR_R(crate::FieldReader<u8, u8>);
impl SADR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADR` writer - Slave Address"]
pub struct SADR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `SADR1EN` reader - Slave Address 1 Enable"]
pub struct SADR1EN_R(crate::FieldReader<bool, bool>);
impl SADR1EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SADR1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADR1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADR1EN` writer - Slave Address 1 Enable"]
pub struct SADR1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR1EN_W<'a> {
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
#[doc = "Field `SADR2EN` reader - Slave Address 2 Enable"]
pub struct SADR2EN_R(crate::FieldReader<bool, bool>);
impl SADR2EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SADR2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADR2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADR2EN` writer - Slave Address 2 Enable"]
pub struct SADR2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `SADR3EN` reader - Slave Address 3 Enable"]
pub struct SADR3EN_R(crate::FieldReader<bool, bool>);
impl SADR3EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SADR3EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADR3EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADR3EN` writer - Slave Address 3 Enable"]
pub struct SADR3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR3EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DATAMEN` reader - Data Matching Enable"]
pub struct DATAMEN_R(crate::FieldReader<bool, bool>);
impl DATAMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAMEN` writer - Data Matching Enable"]
pub struct DATAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK enable"]
    #[inline(always)]
    pub fn nacken(&self) -> NACKEN_R {
        NACKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&self) -> SMDA_R {
        SMDA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&self) -> SMHH_R {
        SMHH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    pub fn sclwsdis(&self) -> SCLWSDIS_R {
        SCLWSDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - Slave Address 1 Enable"]
    #[inline(always)]
    pub fn sadr1en(&self) -> SADR1EN_R {
        SADR1EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Slave Address 2 Enable"]
    #[inline(always)]
    pub fn sadr2en(&self) -> SADR2EN_R {
        SADR2EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Slave Address 3 Enable"]
    #[inline(always)]
    pub fn sadr3en(&self) -> SADR3EN_R {
        SADR3EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Data Matching Enable"]
    #[inline(always)]
    pub fn datamen(&self) -> DATAMEN_R {
        DATAMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK enable"]
    #[inline(always)]
    pub fn nacken(&mut self) -> NACKEN_W {
        NACKEN_W { w: self }
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&mut self) -> SMDA_W {
        SMDA_W { w: self }
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&mut self) -> SMHH_W {
        SMHH_W { w: self }
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    pub fn sclwsdis(&mut self) -> SCLWSDIS_W {
        SCLWSDIS_W { w: self }
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&mut self) -> SADR_W {
        SADR_W { w: self }
    }
    #[doc = "Bit 28 - Slave Address 1 Enable"]
    #[inline(always)]
    pub fn sadr1en(&mut self) -> SADR1EN_W {
        SADR1EN_W { w: self }
    }
    #[doc = "Bit 29 - Slave Address 2 Enable"]
    #[inline(always)]
    pub fn sadr2en(&mut self) -> SADR2EN_W {
        SADR2EN_W { w: self }
    }
    #[doc = "Bit 30 - Slave Address 3 Enable"]
    #[inline(always)]
    pub fn sadr3en(&mut self) -> SADR3EN_W {
        SADR3EN_W { w: self }
    }
    #[doc = "Bit 31 - Data Matching Enable"]
    #[inline(always)]
    pub fn datamen(&mut self) -> DATAMEN_W {
        DATAMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_smr](index.html) module"]
pub struct TWIHS_SMR_SPEC;
impl crate::RegisterSpec for TWIHS_SMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_smr::R](R) reader structure"]
impl crate::Readable for TWIHS_SMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twihs_smr::W](W) writer structure"]
impl crate::Writable for TWIHS_SMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWIHS_SMR to value 0"]
impl crate::Resettable for TWIHS_SMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
