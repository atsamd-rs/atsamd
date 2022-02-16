#[doc = "Register `AFEC_CECR` reader"]
pub struct R(crate::R<AFEC_CECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_CECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_CECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_CECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_CECR` writer"]
pub struct W(crate::W<AFEC_CECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_CECR_SPEC>;
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
impl From<crate::W<AFEC_CECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_CECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECORR0` reader - Error Correction Enable for channel 0"]
pub struct ECORR0_R(crate::FieldReader<bool, bool>);
impl ECORR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR0` writer - Error Correction Enable for channel 0"]
pub struct ECORR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR0_W<'a> {
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
#[doc = "Field `ECORR1` reader - Error Correction Enable for channel 1"]
pub struct ECORR1_R(crate::FieldReader<bool, bool>);
impl ECORR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR1` writer - Error Correction Enable for channel 1"]
pub struct ECORR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR1_W<'a> {
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
#[doc = "Field `ECORR2` reader - Error Correction Enable for channel 2"]
pub struct ECORR2_R(crate::FieldReader<bool, bool>);
impl ECORR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR2` writer - Error Correction Enable for channel 2"]
pub struct ECORR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR2_W<'a> {
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
#[doc = "Field `ECORR3` reader - Error Correction Enable for channel 3"]
pub struct ECORR3_R(crate::FieldReader<bool, bool>);
impl ECORR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR3` writer - Error Correction Enable for channel 3"]
pub struct ECORR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR3_W<'a> {
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
#[doc = "Field `ECORR4` reader - Error Correction Enable for channel 4"]
pub struct ECORR4_R(crate::FieldReader<bool, bool>);
impl ECORR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR4` writer - Error Correction Enable for channel 4"]
pub struct ECORR4_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR4_W<'a> {
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
#[doc = "Field `ECORR5` reader - Error Correction Enable for channel 5"]
pub struct ECORR5_R(crate::FieldReader<bool, bool>);
impl ECORR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR5` writer - Error Correction Enable for channel 5"]
pub struct ECORR5_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR5_W<'a> {
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
#[doc = "Field `ECORR6` reader - Error Correction Enable for channel 6"]
pub struct ECORR6_R(crate::FieldReader<bool, bool>);
impl ECORR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR6` writer - Error Correction Enable for channel 6"]
pub struct ECORR6_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR6_W<'a> {
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
#[doc = "Field `ECORR7` reader - Error Correction Enable for channel 7"]
pub struct ECORR7_R(crate::FieldReader<bool, bool>);
impl ECORR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR7` writer - Error Correction Enable for channel 7"]
pub struct ECORR7_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR7_W<'a> {
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
#[doc = "Field `ECORR8` reader - Error Correction Enable for channel 8"]
pub struct ECORR8_R(crate::FieldReader<bool, bool>);
impl ECORR8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR8` writer - Error Correction Enable for channel 8"]
pub struct ECORR8_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR8_W<'a> {
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
#[doc = "Field `ECORR9` reader - Error Correction Enable for channel 9"]
pub struct ECORR9_R(crate::FieldReader<bool, bool>);
impl ECORR9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR9` writer - Error Correction Enable for channel 9"]
pub struct ECORR9_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR9_W<'a> {
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
#[doc = "Field `ECORR10` reader - Error Correction Enable for channel 10"]
pub struct ECORR10_R(crate::FieldReader<bool, bool>);
impl ECORR10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR10` writer - Error Correction Enable for channel 10"]
pub struct ECORR10_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR10_W<'a> {
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
#[doc = "Field `ECORR11` reader - Error Correction Enable for channel 11"]
pub struct ECORR11_R(crate::FieldReader<bool, bool>);
impl ECORR11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECORR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECORR11` writer - Error Correction Enable for channel 11"]
pub struct ECORR11_W<'a> {
    w: &'a mut W,
}
impl<'a> ECORR11_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Error Correction Enable for channel 0"]
    #[inline(always)]
    pub fn ecorr0(&self) -> ECORR0_R {
        ECORR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error Correction Enable for channel 1"]
    #[inline(always)]
    pub fn ecorr1(&self) -> ECORR1_R {
        ECORR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error Correction Enable for channel 2"]
    #[inline(always)]
    pub fn ecorr2(&self) -> ECORR2_R {
        ECORR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error Correction Enable for channel 3"]
    #[inline(always)]
    pub fn ecorr3(&self) -> ECORR3_R {
        ECORR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Error Correction Enable for channel 4"]
    #[inline(always)]
    pub fn ecorr4(&self) -> ECORR4_R {
        ECORR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error Correction Enable for channel 5"]
    #[inline(always)]
    pub fn ecorr5(&self) -> ECORR5_R {
        ECORR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error Correction Enable for channel 6"]
    #[inline(always)]
    pub fn ecorr6(&self) -> ECORR6_R {
        ECORR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error Correction Enable for channel 7"]
    #[inline(always)]
    pub fn ecorr7(&self) -> ECORR7_R {
        ECORR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Error Correction Enable for channel 8"]
    #[inline(always)]
    pub fn ecorr8(&self) -> ECORR8_R {
        ECORR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Error Correction Enable for channel 9"]
    #[inline(always)]
    pub fn ecorr9(&self) -> ECORR9_R {
        ECORR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Error Correction Enable for channel 10"]
    #[inline(always)]
    pub fn ecorr10(&self) -> ECORR10_R {
        ECORR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Error Correction Enable for channel 11"]
    #[inline(always)]
    pub fn ecorr11(&self) -> ECORR11_R {
        ECORR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Correction Enable for channel 0"]
    #[inline(always)]
    pub fn ecorr0(&mut self) -> ECORR0_W {
        ECORR0_W { w: self }
    }
    #[doc = "Bit 1 - Error Correction Enable for channel 1"]
    #[inline(always)]
    pub fn ecorr1(&mut self) -> ECORR1_W {
        ECORR1_W { w: self }
    }
    #[doc = "Bit 2 - Error Correction Enable for channel 2"]
    #[inline(always)]
    pub fn ecorr2(&mut self) -> ECORR2_W {
        ECORR2_W { w: self }
    }
    #[doc = "Bit 3 - Error Correction Enable for channel 3"]
    #[inline(always)]
    pub fn ecorr3(&mut self) -> ECORR3_W {
        ECORR3_W { w: self }
    }
    #[doc = "Bit 4 - Error Correction Enable for channel 4"]
    #[inline(always)]
    pub fn ecorr4(&mut self) -> ECORR4_W {
        ECORR4_W { w: self }
    }
    #[doc = "Bit 5 - Error Correction Enable for channel 5"]
    #[inline(always)]
    pub fn ecorr5(&mut self) -> ECORR5_W {
        ECORR5_W { w: self }
    }
    #[doc = "Bit 6 - Error Correction Enable for channel 6"]
    #[inline(always)]
    pub fn ecorr6(&mut self) -> ECORR6_W {
        ECORR6_W { w: self }
    }
    #[doc = "Bit 7 - Error Correction Enable for channel 7"]
    #[inline(always)]
    pub fn ecorr7(&mut self) -> ECORR7_W {
        ECORR7_W { w: self }
    }
    #[doc = "Bit 8 - Error Correction Enable for channel 8"]
    #[inline(always)]
    pub fn ecorr8(&mut self) -> ECORR8_W {
        ECORR8_W { w: self }
    }
    #[doc = "Bit 9 - Error Correction Enable for channel 9"]
    #[inline(always)]
    pub fn ecorr9(&mut self) -> ECORR9_W {
        ECORR9_W { w: self }
    }
    #[doc = "Bit 10 - Error Correction Enable for channel 10"]
    #[inline(always)]
    pub fn ecorr10(&mut self) -> ECORR10_W {
        ECORR10_W { w: self }
    }
    #[doc = "Bit 11 - Error Correction Enable for channel 11"]
    #[inline(always)]
    pub fn ecorr11(&mut self) -> ECORR11_W {
        ECORR11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Error Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cecr](index.html) module"]
pub struct AFEC_CECR_SPEC;
impl crate::RegisterSpec for AFEC_CECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_cecr::R](R) reader structure"]
impl crate::Readable for AFEC_CECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_cecr::W](W) writer structure"]
impl crate::Writable for AFEC_CECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_CECR to value 0"]
impl crate::Resettable for AFEC_CECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
