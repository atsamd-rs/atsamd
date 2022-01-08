#[doc = "Register `BKOUT` reader"]
pub struct R(crate::R<BKOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKOUT` writer"]
pub struct W(crate::W<BKOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKOUT_SPEC>;
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
impl From<crate::W<BKOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENOUT0` reader - Enable OUT0"]
pub struct ENOUT0_R(crate::FieldReader<bool, bool>);
impl ENOUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENOUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENOUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENOUT0` writer - Enable OUT0"]
pub struct ENOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENOUT0_W<'a> {
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
#[doc = "Field `ENOUT1` reader - Enable OUT1"]
pub struct ENOUT1_R(crate::FieldReader<bool, bool>);
impl ENOUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENOUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENOUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENOUT1` writer - Enable OUT1"]
pub struct ENOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENOUT1_W<'a> {
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
#[doc = "Field `CLROUT0` reader - Clear OUT0"]
pub struct CLROUT0_R(crate::FieldReader<bool, bool>);
impl CLROUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLROUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLROUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLROUT0` writer - Clear OUT0"]
pub struct CLROUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLROUT0_W<'a> {
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
#[doc = "Field `CLROUT1` reader - Clear OUT1"]
pub struct CLROUT1_R(crate::FieldReader<bool, bool>);
impl CLROUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLROUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLROUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLROUT1` writer - Clear OUT1"]
pub struct CLROUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLROUT1_W<'a> {
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
#[doc = "Field `SETOUT0` reader - Set OUT0"]
pub struct SETOUT0_R(crate::FieldReader<bool, bool>);
impl SETOUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETOUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETOUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETOUT0` writer - Set OUT0"]
pub struct SETOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SETOUT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SETOUT1` reader - Set OUT1"]
pub struct SETOUT1_R(crate::FieldReader<bool, bool>);
impl SETOUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETOUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETOUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETOUT1` writer - Set OUT1"]
pub struct SETOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SETOUT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RTCTGLOUT0` reader - RTC Toggle OUT0"]
pub struct RTCTGLOUT0_R(crate::FieldReader<bool, bool>);
impl RTCTGLOUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCTGLOUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTGLOUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTGLOUT0` writer - RTC Toggle OUT0"]
pub struct RTCTGLOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTGLOUT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RTCTGLOUT1` reader - RTC Toggle OUT1"]
pub struct RTCTGLOUT1_R(crate::FieldReader<bool, bool>);
impl RTCTGLOUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCTGLOUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTGLOUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTGLOUT1` writer - RTC Toggle OUT1"]
pub struct RTCTGLOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTGLOUT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable OUT0"]
    #[inline(always)]
    pub fn enout0(&self) -> ENOUT0_R {
        ENOUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable OUT1"]
    #[inline(always)]
    pub fn enout1(&self) -> ENOUT1_R {
        ENOUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clear OUT0"]
    #[inline(always)]
    pub fn clrout0(&self) -> CLROUT0_R {
        CLROUT0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear OUT1"]
    #[inline(always)]
    pub fn clrout1(&self) -> CLROUT1_R {
        CLROUT1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set OUT0"]
    #[inline(always)]
    pub fn setout0(&self) -> SETOUT0_R {
        SETOUT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set OUT1"]
    #[inline(always)]
    pub fn setout1(&self) -> SETOUT1_R {
        SETOUT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RTC Toggle OUT0"]
    #[inline(always)]
    pub fn rtctglout0(&self) -> RTCTGLOUT0_R {
        RTCTGLOUT0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RTC Toggle OUT1"]
    #[inline(always)]
    pub fn rtctglout1(&self) -> RTCTGLOUT1_R {
        RTCTGLOUT1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable OUT0"]
    #[inline(always)]
    pub fn enout0(&mut self) -> ENOUT0_W {
        ENOUT0_W { w: self }
    }
    #[doc = "Bit 1 - Enable OUT1"]
    #[inline(always)]
    pub fn enout1(&mut self) -> ENOUT1_W {
        ENOUT1_W { w: self }
    }
    #[doc = "Bit 8 - Clear OUT0"]
    #[inline(always)]
    pub fn clrout0(&mut self) -> CLROUT0_W {
        CLROUT0_W { w: self }
    }
    #[doc = "Bit 9 - Clear OUT1"]
    #[inline(always)]
    pub fn clrout1(&mut self) -> CLROUT1_W {
        CLROUT1_W { w: self }
    }
    #[doc = "Bit 16 - Set OUT0"]
    #[inline(always)]
    pub fn setout0(&mut self) -> SETOUT0_W {
        SETOUT0_W { w: self }
    }
    #[doc = "Bit 17 - Set OUT1"]
    #[inline(always)]
    pub fn setout1(&mut self) -> SETOUT1_W {
        SETOUT1_W { w: self }
    }
    #[doc = "Bit 24 - RTC Toggle OUT0"]
    #[inline(always)]
    pub fn rtctglout0(&mut self) -> RTCTGLOUT0_W {
        RTCTGLOUT0_W { w: self }
    }
    #[doc = "Bit 25 - RTC Toggle OUT1"]
    #[inline(always)]
    pub fn rtctglout1(&mut self) -> RTCTGLOUT1_W {
        RTCTGLOUT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkout](index.html) module"]
pub struct BKOUT_SPEC;
impl crate::RegisterSpec for BKOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkout::R](R) reader structure"]
impl crate::Readable for BKOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkout::W](W) writer structure"]
impl crate::Writable for BKOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BKOUT to value 0"]
impl crate::Resettable for BKOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
