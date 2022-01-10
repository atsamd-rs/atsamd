#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMEN` reader - Smart Mode Enable"]
pub struct SMEN_R(crate::FieldReader<bool, bool>);
impl SMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMEN` writer - Smart Mode Enable"]
pub struct SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMEN_W<'a> {
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
#[doc = "Field `GCMD` reader - PMBus Group Command"]
pub struct GCMD_R(crate::FieldReader<bool, bool>);
impl GCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCMD` writer - PMBus Group Command"]
pub struct GCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> GCMD_W<'a> {
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
#[doc = "Field `AACKEN` reader - Automatic Address Acknowledge"]
pub struct AACKEN_R(crate::FieldReader<bool, bool>);
impl AACKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AACKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AACKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AACKEN` writer - Automatic Address Acknowledge"]
pub struct AACKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AACKEN_W<'a> {
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
#[doc = "Field `AMODE` reader - Address Mode"]
pub struct AMODE_R(crate::FieldReader<u8, u8>);
impl AMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMODE` writer - Address Mode"]
pub struct AMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `CMD` writer - Command"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `ACKACT` reader - Acknowledge Action"]
pub struct ACKACT_R(crate::FieldReader<bool, bool>);
impl ACKACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACKACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKACT` writer - Acknowledge Action"]
pub struct ACKACT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKACT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    pub fn gcmd(&self) -> GCMD_R {
        GCMD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    pub fn aacken(&self) -> AACKEN_R {
        AACKEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AMODE_R {
        AMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&self) -> ACKACT_R {
        ACKACT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SMEN_W {
        SMEN_W { w: self }
    }
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    pub fn gcmd(&mut self) -> GCMD_W {
        GCMD_W { w: self }
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    pub fn aacken(&mut self) -> AACKEN_W {
        AACKEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&mut self) -> AMODE_W {
        AMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&mut self) -> ACKACT_W {
        ACKACT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
