#[doc = "Register `AFEC_SHMR` reader"]
pub struct R(crate::R<AFEC_SHMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_SHMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_SHMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_SHMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_SHMR` writer"]
pub struct W(crate::W<AFEC_SHMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_SHMR_SPEC>;
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
impl From<crate::W<AFEC_SHMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_SHMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUAL0` reader - Dual Sample & Hold for channel 0"]
pub struct DUAL0_R(crate::FieldReader<bool, bool>);
impl DUAL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL0` writer - Dual Sample & Hold for channel 0"]
pub struct DUAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL0_W<'a> {
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
#[doc = "Field `DUAL1` reader - Dual Sample & Hold for channel 1"]
pub struct DUAL1_R(crate::FieldReader<bool, bool>);
impl DUAL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL1` writer - Dual Sample & Hold for channel 1"]
pub struct DUAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL1_W<'a> {
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
#[doc = "Field `DUAL2` reader - Dual Sample & Hold for channel 2"]
pub struct DUAL2_R(crate::FieldReader<bool, bool>);
impl DUAL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL2` writer - Dual Sample & Hold for channel 2"]
pub struct DUAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL2_W<'a> {
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
#[doc = "Field `DUAL3` reader - Dual Sample & Hold for channel 3"]
pub struct DUAL3_R(crate::FieldReader<bool, bool>);
impl DUAL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL3` writer - Dual Sample & Hold for channel 3"]
pub struct DUAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL3_W<'a> {
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
#[doc = "Field `DUAL4` reader - Dual Sample & Hold for channel 4"]
pub struct DUAL4_R(crate::FieldReader<bool, bool>);
impl DUAL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL4` writer - Dual Sample & Hold for channel 4"]
pub struct DUAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL4_W<'a> {
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
#[doc = "Field `DUAL5` reader - Dual Sample & Hold for channel 5"]
pub struct DUAL5_R(crate::FieldReader<bool, bool>);
impl DUAL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL5` writer - Dual Sample & Hold for channel 5"]
pub struct DUAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL5_W<'a> {
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
#[doc = "Field `DUAL6` reader - Dual Sample & Hold for channel 6"]
pub struct DUAL6_R(crate::FieldReader<bool, bool>);
impl DUAL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL6` writer - Dual Sample & Hold for channel 6"]
pub struct DUAL6_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL6_W<'a> {
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
#[doc = "Field `DUAL7` reader - Dual Sample & Hold for channel 7"]
pub struct DUAL7_R(crate::FieldReader<bool, bool>);
impl DUAL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL7` writer - Dual Sample & Hold for channel 7"]
pub struct DUAL7_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL7_W<'a> {
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
#[doc = "Field `DUAL8` reader - Dual Sample & Hold for channel 8"]
pub struct DUAL8_R(crate::FieldReader<bool, bool>);
impl DUAL8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL8` writer - Dual Sample & Hold for channel 8"]
pub struct DUAL8_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL8_W<'a> {
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
#[doc = "Field `DUAL9` reader - Dual Sample & Hold for channel 9"]
pub struct DUAL9_R(crate::FieldReader<bool, bool>);
impl DUAL9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL9` writer - Dual Sample & Hold for channel 9"]
pub struct DUAL9_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL9_W<'a> {
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
#[doc = "Field `DUAL10` reader - Dual Sample & Hold for channel 10"]
pub struct DUAL10_R(crate::FieldReader<bool, bool>);
impl DUAL10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL10` writer - Dual Sample & Hold for channel 10"]
pub struct DUAL10_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL10_W<'a> {
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
#[doc = "Field `DUAL11` reader - Dual Sample & Hold for channel 11"]
pub struct DUAL11_R(crate::FieldReader<bool, bool>);
impl DUAL11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL11` writer - Dual Sample & Hold for channel 11"]
pub struct DUAL11_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL11_W<'a> {
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
    #[doc = "Bit 0 - Dual Sample & Hold for channel 0"]
    #[inline(always)]
    pub fn dual0(&self) -> DUAL0_R {
        DUAL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Dual Sample & Hold for channel 1"]
    #[inline(always)]
    pub fn dual1(&self) -> DUAL1_R {
        DUAL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Dual Sample & Hold for channel 2"]
    #[inline(always)]
    pub fn dual2(&self) -> DUAL2_R {
        DUAL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dual Sample & Hold for channel 3"]
    #[inline(always)]
    pub fn dual3(&self) -> DUAL3_R {
        DUAL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Dual Sample & Hold for channel 4"]
    #[inline(always)]
    pub fn dual4(&self) -> DUAL4_R {
        DUAL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Dual Sample & Hold for channel 5"]
    #[inline(always)]
    pub fn dual5(&self) -> DUAL5_R {
        DUAL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Dual Sample & Hold for channel 6"]
    #[inline(always)]
    pub fn dual6(&self) -> DUAL6_R {
        DUAL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Dual Sample & Hold for channel 7"]
    #[inline(always)]
    pub fn dual7(&self) -> DUAL7_R {
        DUAL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Dual Sample & Hold for channel 8"]
    #[inline(always)]
    pub fn dual8(&self) -> DUAL8_R {
        DUAL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Dual Sample & Hold for channel 9"]
    #[inline(always)]
    pub fn dual9(&self) -> DUAL9_R {
        DUAL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Dual Sample & Hold for channel 10"]
    #[inline(always)]
    pub fn dual10(&self) -> DUAL10_R {
        DUAL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Dual Sample & Hold for channel 11"]
    #[inline(always)]
    pub fn dual11(&self) -> DUAL11_R {
        DUAL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dual Sample & Hold for channel 0"]
    #[inline(always)]
    pub fn dual0(&mut self) -> DUAL0_W {
        DUAL0_W { w: self }
    }
    #[doc = "Bit 1 - Dual Sample & Hold for channel 1"]
    #[inline(always)]
    pub fn dual1(&mut self) -> DUAL1_W {
        DUAL1_W { w: self }
    }
    #[doc = "Bit 2 - Dual Sample & Hold for channel 2"]
    #[inline(always)]
    pub fn dual2(&mut self) -> DUAL2_W {
        DUAL2_W { w: self }
    }
    #[doc = "Bit 3 - Dual Sample & Hold for channel 3"]
    #[inline(always)]
    pub fn dual3(&mut self) -> DUAL3_W {
        DUAL3_W { w: self }
    }
    #[doc = "Bit 4 - Dual Sample & Hold for channel 4"]
    #[inline(always)]
    pub fn dual4(&mut self) -> DUAL4_W {
        DUAL4_W { w: self }
    }
    #[doc = "Bit 5 - Dual Sample & Hold for channel 5"]
    #[inline(always)]
    pub fn dual5(&mut self) -> DUAL5_W {
        DUAL5_W { w: self }
    }
    #[doc = "Bit 6 - Dual Sample & Hold for channel 6"]
    #[inline(always)]
    pub fn dual6(&mut self) -> DUAL6_W {
        DUAL6_W { w: self }
    }
    #[doc = "Bit 7 - Dual Sample & Hold for channel 7"]
    #[inline(always)]
    pub fn dual7(&mut self) -> DUAL7_W {
        DUAL7_W { w: self }
    }
    #[doc = "Bit 8 - Dual Sample & Hold for channel 8"]
    #[inline(always)]
    pub fn dual8(&mut self) -> DUAL8_W {
        DUAL8_W { w: self }
    }
    #[doc = "Bit 9 - Dual Sample & Hold for channel 9"]
    #[inline(always)]
    pub fn dual9(&mut self) -> DUAL9_W {
        DUAL9_W { w: self }
    }
    #[doc = "Bit 10 - Dual Sample & Hold for channel 10"]
    #[inline(always)]
    pub fn dual10(&mut self) -> DUAL10_W {
        DUAL10_W { w: self }
    }
    #[doc = "Bit 11 - Dual Sample & Hold for channel 11"]
    #[inline(always)]
    pub fn dual11(&mut self) -> DUAL11_W {
        DUAL11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Sample & Hold Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_shmr](index.html) module"]
pub struct AFEC_SHMR_SPEC;
impl crate::RegisterSpec for AFEC_SHMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_shmr::R](R) reader structure"]
impl crate::Readable for AFEC_SHMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_shmr::W](W) writer structure"]
impl crate::Writable for AFEC_SHMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_SHMR to value 0"]
impl crate::Resettable for AFEC_SHMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
