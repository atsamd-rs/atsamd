#[doc = "Register `MATRIX_MRCR` reader"]
pub struct R(crate::R<MATRIX_MRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_MRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_MRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_MRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_MRCR` writer"]
pub struct W(crate::W<MATRIX_MRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_MRCR_SPEC>;
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
impl From<crate::W<MATRIX_MRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_MRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCB0` reader - Remap Command Bit for Master 0"]
pub struct RCB0_R(crate::FieldReader<bool, bool>);
impl RCB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB0` writer - Remap Command Bit for Master 0"]
pub struct RCB0_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB0_W<'a> {
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
#[doc = "Field `RCB1` reader - Remap Command Bit for Master 1"]
pub struct RCB1_R(crate::FieldReader<bool, bool>);
impl RCB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB1` writer - Remap Command Bit for Master 1"]
pub struct RCB1_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB1_W<'a> {
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
#[doc = "Field `RCB2` reader - Remap Command Bit for Master 2"]
pub struct RCB2_R(crate::FieldReader<bool, bool>);
impl RCB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB2` writer - Remap Command Bit for Master 2"]
pub struct RCB2_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB2_W<'a> {
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
#[doc = "Field `RCB3` reader - Remap Command Bit for Master 3"]
pub struct RCB3_R(crate::FieldReader<bool, bool>);
impl RCB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB3` writer - Remap Command Bit for Master 3"]
pub struct RCB3_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB3_W<'a> {
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
#[doc = "Field `RCB4` reader - Remap Command Bit for Master 4"]
pub struct RCB4_R(crate::FieldReader<bool, bool>);
impl RCB4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB4` writer - Remap Command Bit for Master 4"]
pub struct RCB4_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB4_W<'a> {
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
#[doc = "Field `RCB5` reader - Remap Command Bit for Master 5"]
pub struct RCB5_R(crate::FieldReader<bool, bool>);
impl RCB5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB5` writer - Remap Command Bit for Master 5"]
pub struct RCB5_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB5_W<'a> {
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
#[doc = "Field `RCB6` reader - Remap Command Bit for Master 6"]
pub struct RCB6_R(crate::FieldReader<bool, bool>);
impl RCB6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB6` writer - Remap Command Bit for Master 6"]
pub struct RCB6_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB6_W<'a> {
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
#[doc = "Field `RCB8` reader - Remap Command Bit for Master 8"]
pub struct RCB8_R(crate::FieldReader<bool, bool>);
impl RCB8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB8` writer - Remap Command Bit for Master 8"]
pub struct RCB8_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB8_W<'a> {
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
#[doc = "Field `RCB9` reader - Remap Command Bit for Master 9"]
pub struct RCB9_R(crate::FieldReader<bool, bool>);
impl RCB9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB9` writer - Remap Command Bit for Master 9"]
pub struct RCB9_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB9_W<'a> {
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
#[doc = "Field `RCB10` reader - Remap Command Bit for Master 10"]
pub struct RCB10_R(crate::FieldReader<bool, bool>);
impl RCB10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB10` writer - Remap Command Bit for Master 10"]
pub struct RCB10_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB10_W<'a> {
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
#[doc = "Field `RCB11` reader - Remap Command Bit for Master 11"]
pub struct RCB11_R(crate::FieldReader<bool, bool>);
impl RCB11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB11` writer - Remap Command Bit for Master 11"]
pub struct RCB11_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB11_W<'a> {
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
#[doc = "Field `RCB12` reader - Remap Command Bit for Master 12"]
pub struct RCB12_R(crate::FieldReader<bool, bool>);
impl RCB12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB12` writer - Remap Command Bit for Master 12"]
pub struct RCB12_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&self) -> RCB0_R {
        RCB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&self) -> RCB1_R {
        RCB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&self) -> RCB2_R {
        RCB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&self) -> RCB3_R {
        RCB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&self) -> RCB4_R {
        RCB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&self) -> RCB5_R {
        RCB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Remap Command Bit for Master 6"]
    #[inline(always)]
    pub fn rcb6(&self) -> RCB6_R {
        RCB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Remap Command Bit for Master 8"]
    #[inline(always)]
    pub fn rcb8(&self) -> RCB8_R {
        RCB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Remap Command Bit for Master 9"]
    #[inline(always)]
    pub fn rcb9(&self) -> RCB9_R {
        RCB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Remap Command Bit for Master 10"]
    #[inline(always)]
    pub fn rcb10(&self) -> RCB10_R {
        RCB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Remap Command Bit for Master 11"]
    #[inline(always)]
    pub fn rcb11(&self) -> RCB11_R {
        RCB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Remap Command Bit for Master 12"]
    #[inline(always)]
    pub fn rcb12(&self) -> RCB12_R {
        RCB12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&mut self) -> RCB0_W {
        RCB0_W { w: self }
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&mut self) -> RCB1_W {
        RCB1_W { w: self }
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&mut self) -> RCB2_W {
        RCB2_W { w: self }
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&mut self) -> RCB3_W {
        RCB3_W { w: self }
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&mut self) -> RCB4_W {
        RCB4_W { w: self }
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&mut self) -> RCB5_W {
        RCB5_W { w: self }
    }
    #[doc = "Bit 6 - Remap Command Bit for Master 6"]
    #[inline(always)]
    pub fn rcb6(&mut self) -> RCB6_W {
        RCB6_W { w: self }
    }
    #[doc = "Bit 8 - Remap Command Bit for Master 8"]
    #[inline(always)]
    pub fn rcb8(&mut self) -> RCB8_W {
        RCB8_W { w: self }
    }
    #[doc = "Bit 9 - Remap Command Bit for Master 9"]
    #[inline(always)]
    pub fn rcb9(&mut self) -> RCB9_W {
        RCB9_W { w: self }
    }
    #[doc = "Bit 10 - Remap Command Bit for Master 10"]
    #[inline(always)]
    pub fn rcb10(&mut self) -> RCB10_W {
        RCB10_W { w: self }
    }
    #[doc = "Bit 11 - Remap Command Bit for Master 11"]
    #[inline(always)]
    pub fn rcb11(&mut self) -> RCB11_W {
        RCB11_W { w: self }
    }
    #[doc = "Bit 12 - Remap Command Bit for Master 12"]
    #[inline(always)]
    pub fn rcb12(&mut self) -> RCB12_W {
        RCB12_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Remap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mrcr](index.html) module"]
pub struct MATRIX_MRCR_SPEC;
impl crate::RegisterSpec for MATRIX_MRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_mrcr::R](R) reader structure"]
impl crate::Readable for MATRIX_MRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_mrcr::W](W) writer structure"]
impl crate::Writable for MATRIX_MRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATRIX_MRCR to value 0"]
impl crate::Resettable for MATRIX_MRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
