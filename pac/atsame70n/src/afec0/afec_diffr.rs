#[doc = "Register `AFEC_DIFFR` reader"]
pub struct R(crate::R<AFEC_DIFFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_DIFFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_DIFFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_DIFFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_DIFFR` writer"]
pub struct W(crate::W<AFEC_DIFFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_DIFFR_SPEC>;
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
impl From<crate::W<AFEC_DIFFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_DIFFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIFF0` reader - Differential inputs for channel 0"]
pub struct DIFF0_R(crate::FieldReader<bool, bool>);
impl DIFF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF0` writer - Differential inputs for channel 0"]
pub struct DIFF0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF0_W<'a> {
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
#[doc = "Field `DIFF1` reader - Differential inputs for channel 1"]
pub struct DIFF1_R(crate::FieldReader<bool, bool>);
impl DIFF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF1` writer - Differential inputs for channel 1"]
pub struct DIFF1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF1_W<'a> {
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
#[doc = "Field `DIFF2` reader - Differential inputs for channel 2"]
pub struct DIFF2_R(crate::FieldReader<bool, bool>);
impl DIFF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF2` writer - Differential inputs for channel 2"]
pub struct DIFF2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF2_W<'a> {
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
#[doc = "Field `DIFF3` reader - Differential inputs for channel 3"]
pub struct DIFF3_R(crate::FieldReader<bool, bool>);
impl DIFF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF3` writer - Differential inputs for channel 3"]
pub struct DIFF3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF3_W<'a> {
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
#[doc = "Field `DIFF4` reader - Differential inputs for channel 4"]
pub struct DIFF4_R(crate::FieldReader<bool, bool>);
impl DIFF4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF4` writer - Differential inputs for channel 4"]
pub struct DIFF4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF4_W<'a> {
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
#[doc = "Field `DIFF5` reader - Differential inputs for channel 5"]
pub struct DIFF5_R(crate::FieldReader<bool, bool>);
impl DIFF5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF5` writer - Differential inputs for channel 5"]
pub struct DIFF5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF5_W<'a> {
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
#[doc = "Field `DIFF6` reader - Differential inputs for channel 6"]
pub struct DIFF6_R(crate::FieldReader<bool, bool>);
impl DIFF6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF6` writer - Differential inputs for channel 6"]
pub struct DIFF6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF6_W<'a> {
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
#[doc = "Field `DIFF7` reader - Differential inputs for channel 7"]
pub struct DIFF7_R(crate::FieldReader<bool, bool>);
impl DIFF7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF7` writer - Differential inputs for channel 7"]
pub struct DIFF7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF7_W<'a> {
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
#[doc = "Field `DIFF8` reader - Differential inputs for channel 8"]
pub struct DIFF8_R(crate::FieldReader<bool, bool>);
impl DIFF8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF8` writer - Differential inputs for channel 8"]
pub struct DIFF8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF8_W<'a> {
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
#[doc = "Field `DIFF9` reader - Differential inputs for channel 9"]
pub struct DIFF9_R(crate::FieldReader<bool, bool>);
impl DIFF9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF9` writer - Differential inputs for channel 9"]
pub struct DIFF9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF9_W<'a> {
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
#[doc = "Field `DIFF10` reader - Differential inputs for channel 10"]
pub struct DIFF10_R(crate::FieldReader<bool, bool>);
impl DIFF10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF10` writer - Differential inputs for channel 10"]
pub struct DIFF10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF10_W<'a> {
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
#[doc = "Field `DIFF11` reader - Differential inputs for channel 11"]
pub struct DIFF11_R(crate::FieldReader<bool, bool>);
impl DIFF11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF11` writer - Differential inputs for channel 11"]
pub struct DIFF11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF11_W<'a> {
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
    #[doc = "Bit 0 - Differential inputs for channel 0"]
    #[inline(always)]
    pub fn diff0(&self) -> DIFF0_R {
        DIFF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Differential inputs for channel 1"]
    #[inline(always)]
    pub fn diff1(&self) -> DIFF1_R {
        DIFF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Differential inputs for channel 2"]
    #[inline(always)]
    pub fn diff2(&self) -> DIFF2_R {
        DIFF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Differential inputs for channel 3"]
    #[inline(always)]
    pub fn diff3(&self) -> DIFF3_R {
        DIFF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Differential inputs for channel 4"]
    #[inline(always)]
    pub fn diff4(&self) -> DIFF4_R {
        DIFF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Differential inputs for channel 5"]
    #[inline(always)]
    pub fn diff5(&self) -> DIFF5_R {
        DIFF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Differential inputs for channel 6"]
    #[inline(always)]
    pub fn diff6(&self) -> DIFF6_R {
        DIFF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Differential inputs for channel 7"]
    #[inline(always)]
    pub fn diff7(&self) -> DIFF7_R {
        DIFF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Differential inputs for channel 8"]
    #[inline(always)]
    pub fn diff8(&self) -> DIFF8_R {
        DIFF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Differential inputs for channel 9"]
    #[inline(always)]
    pub fn diff9(&self) -> DIFF9_R {
        DIFF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Differential inputs for channel 10"]
    #[inline(always)]
    pub fn diff10(&self) -> DIFF10_R {
        DIFF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Differential inputs for channel 11"]
    #[inline(always)]
    pub fn diff11(&self) -> DIFF11_R {
        DIFF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Differential inputs for channel 0"]
    #[inline(always)]
    pub fn diff0(&mut self) -> DIFF0_W {
        DIFF0_W { w: self }
    }
    #[doc = "Bit 1 - Differential inputs for channel 1"]
    #[inline(always)]
    pub fn diff1(&mut self) -> DIFF1_W {
        DIFF1_W { w: self }
    }
    #[doc = "Bit 2 - Differential inputs for channel 2"]
    #[inline(always)]
    pub fn diff2(&mut self) -> DIFF2_W {
        DIFF2_W { w: self }
    }
    #[doc = "Bit 3 - Differential inputs for channel 3"]
    #[inline(always)]
    pub fn diff3(&mut self) -> DIFF3_W {
        DIFF3_W { w: self }
    }
    #[doc = "Bit 4 - Differential inputs for channel 4"]
    #[inline(always)]
    pub fn diff4(&mut self) -> DIFF4_W {
        DIFF4_W { w: self }
    }
    #[doc = "Bit 5 - Differential inputs for channel 5"]
    #[inline(always)]
    pub fn diff5(&mut self) -> DIFF5_W {
        DIFF5_W { w: self }
    }
    #[doc = "Bit 6 - Differential inputs for channel 6"]
    #[inline(always)]
    pub fn diff6(&mut self) -> DIFF6_W {
        DIFF6_W { w: self }
    }
    #[doc = "Bit 7 - Differential inputs for channel 7"]
    #[inline(always)]
    pub fn diff7(&mut self) -> DIFF7_W {
        DIFF7_W { w: self }
    }
    #[doc = "Bit 8 - Differential inputs for channel 8"]
    #[inline(always)]
    pub fn diff8(&mut self) -> DIFF8_W {
        DIFF8_W { w: self }
    }
    #[doc = "Bit 9 - Differential inputs for channel 9"]
    #[inline(always)]
    pub fn diff9(&mut self) -> DIFF9_W {
        DIFF9_W { w: self }
    }
    #[doc = "Bit 10 - Differential inputs for channel 10"]
    #[inline(always)]
    pub fn diff10(&mut self) -> DIFF10_W {
        DIFF10_W { w: self }
    }
    #[doc = "Bit 11 - Differential inputs for channel 11"]
    #[inline(always)]
    pub fn diff11(&mut self) -> DIFF11_W {
        DIFF11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Differential Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_diffr](index.html) module"]
pub struct AFEC_DIFFR_SPEC;
impl crate::RegisterSpec for AFEC_DIFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_diffr::R](R) reader structure"]
impl crate::Readable for AFEC_DIFFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_diffr::W](W) writer structure"]
impl crate::Writable for AFEC_DIFFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_DIFFR to value 0"]
impl crate::Resettable for AFEC_DIFFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
