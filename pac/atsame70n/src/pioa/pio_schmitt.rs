#[doc = "Register `PIO_SCHMITT` reader"]
pub struct R(crate::R<PIO_SCHMITT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_SCHMITT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_SCHMITT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_SCHMITT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO_SCHMITT` writer"]
pub struct W(crate::W<PIO_SCHMITT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO_SCHMITT_SPEC>;
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
impl From<crate::W<PIO_SCHMITT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO_SCHMITT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCHMITT0` reader - Schmitt Trigger Control"]
pub struct SCHMITT0_R(crate::FieldReader<bool, bool>);
impl SCHMITT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT0` writer - Schmitt Trigger Control"]
pub struct SCHMITT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT0_W<'a> {
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
#[doc = "Field `SCHMITT1` reader - Schmitt Trigger Control"]
pub struct SCHMITT1_R(crate::FieldReader<bool, bool>);
impl SCHMITT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT1` writer - Schmitt Trigger Control"]
pub struct SCHMITT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT1_W<'a> {
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
#[doc = "Field `SCHMITT2` reader - Schmitt Trigger Control"]
pub struct SCHMITT2_R(crate::FieldReader<bool, bool>);
impl SCHMITT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT2` writer - Schmitt Trigger Control"]
pub struct SCHMITT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT2_W<'a> {
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
#[doc = "Field `SCHMITT3` reader - Schmitt Trigger Control"]
pub struct SCHMITT3_R(crate::FieldReader<bool, bool>);
impl SCHMITT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT3` writer - Schmitt Trigger Control"]
pub struct SCHMITT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT3_W<'a> {
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
#[doc = "Field `SCHMITT4` reader - Schmitt Trigger Control"]
pub struct SCHMITT4_R(crate::FieldReader<bool, bool>);
impl SCHMITT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT4` writer - Schmitt Trigger Control"]
pub struct SCHMITT4_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT4_W<'a> {
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
#[doc = "Field `SCHMITT5` reader - Schmitt Trigger Control"]
pub struct SCHMITT5_R(crate::FieldReader<bool, bool>);
impl SCHMITT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT5` writer - Schmitt Trigger Control"]
pub struct SCHMITT5_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT5_W<'a> {
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
#[doc = "Field `SCHMITT6` reader - Schmitt Trigger Control"]
pub struct SCHMITT6_R(crate::FieldReader<bool, bool>);
impl SCHMITT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT6` writer - Schmitt Trigger Control"]
pub struct SCHMITT6_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT6_W<'a> {
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
#[doc = "Field `SCHMITT7` reader - Schmitt Trigger Control"]
pub struct SCHMITT7_R(crate::FieldReader<bool, bool>);
impl SCHMITT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT7` writer - Schmitt Trigger Control"]
pub struct SCHMITT7_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT7_W<'a> {
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
#[doc = "Field `SCHMITT8` reader - Schmitt Trigger Control"]
pub struct SCHMITT8_R(crate::FieldReader<bool, bool>);
impl SCHMITT8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT8` writer - Schmitt Trigger Control"]
pub struct SCHMITT8_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT8_W<'a> {
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
#[doc = "Field `SCHMITT9` reader - Schmitt Trigger Control"]
pub struct SCHMITT9_R(crate::FieldReader<bool, bool>);
impl SCHMITT9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT9` writer - Schmitt Trigger Control"]
pub struct SCHMITT9_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT9_W<'a> {
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
#[doc = "Field `SCHMITT10` reader - Schmitt Trigger Control"]
pub struct SCHMITT10_R(crate::FieldReader<bool, bool>);
impl SCHMITT10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT10` writer - Schmitt Trigger Control"]
pub struct SCHMITT10_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT10_W<'a> {
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
#[doc = "Field `SCHMITT11` reader - Schmitt Trigger Control"]
pub struct SCHMITT11_R(crate::FieldReader<bool, bool>);
impl SCHMITT11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT11` writer - Schmitt Trigger Control"]
pub struct SCHMITT11_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT11_W<'a> {
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
#[doc = "Field `SCHMITT12` reader - Schmitt Trigger Control"]
pub struct SCHMITT12_R(crate::FieldReader<bool, bool>);
impl SCHMITT12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT12` writer - Schmitt Trigger Control"]
pub struct SCHMITT12_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT12_W<'a> {
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
#[doc = "Field `SCHMITT13` reader - Schmitt Trigger Control"]
pub struct SCHMITT13_R(crate::FieldReader<bool, bool>);
impl SCHMITT13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT13` writer - Schmitt Trigger Control"]
pub struct SCHMITT13_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT13_W<'a> {
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
#[doc = "Field `SCHMITT14` reader - Schmitt Trigger Control"]
pub struct SCHMITT14_R(crate::FieldReader<bool, bool>);
impl SCHMITT14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT14` writer - Schmitt Trigger Control"]
pub struct SCHMITT14_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SCHMITT15` reader - Schmitt Trigger Control"]
pub struct SCHMITT15_R(crate::FieldReader<bool, bool>);
impl SCHMITT15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT15` writer - Schmitt Trigger Control"]
pub struct SCHMITT15_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SCHMITT16` reader - Schmitt Trigger Control"]
pub struct SCHMITT16_R(crate::FieldReader<bool, bool>);
impl SCHMITT16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT16` writer - Schmitt Trigger Control"]
pub struct SCHMITT16_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT16_W<'a> {
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
#[doc = "Field `SCHMITT17` reader - Schmitt Trigger Control"]
pub struct SCHMITT17_R(crate::FieldReader<bool, bool>);
impl SCHMITT17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT17` writer - Schmitt Trigger Control"]
pub struct SCHMITT17_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT17_W<'a> {
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
#[doc = "Field `SCHMITT18` reader - Schmitt Trigger Control"]
pub struct SCHMITT18_R(crate::FieldReader<bool, bool>);
impl SCHMITT18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT18` writer - Schmitt Trigger Control"]
pub struct SCHMITT18_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT18_W<'a> {
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
#[doc = "Field `SCHMITT19` reader - Schmitt Trigger Control"]
pub struct SCHMITT19_R(crate::FieldReader<bool, bool>);
impl SCHMITT19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT19` writer - Schmitt Trigger Control"]
pub struct SCHMITT19_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SCHMITT20` reader - Schmitt Trigger Control"]
pub struct SCHMITT20_R(crate::FieldReader<bool, bool>);
impl SCHMITT20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT20` writer - Schmitt Trigger Control"]
pub struct SCHMITT20_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `SCHMITT21` reader - Schmitt Trigger Control"]
pub struct SCHMITT21_R(crate::FieldReader<bool, bool>);
impl SCHMITT21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT21` writer - Schmitt Trigger Control"]
pub struct SCHMITT21_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT21_W<'a> {
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
#[doc = "Field `SCHMITT22` reader - Schmitt Trigger Control"]
pub struct SCHMITT22_R(crate::FieldReader<bool, bool>);
impl SCHMITT22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT22` writer - Schmitt Trigger Control"]
pub struct SCHMITT22_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `SCHMITT23` reader - Schmitt Trigger Control"]
pub struct SCHMITT23_R(crate::FieldReader<bool, bool>);
impl SCHMITT23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT23` writer - Schmitt Trigger Control"]
pub struct SCHMITT23_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `SCHMITT24` reader - Schmitt Trigger Control"]
pub struct SCHMITT24_R(crate::FieldReader<bool, bool>);
impl SCHMITT24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT24` writer - Schmitt Trigger Control"]
pub struct SCHMITT24_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT24_W<'a> {
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
#[doc = "Field `SCHMITT25` reader - Schmitt Trigger Control"]
pub struct SCHMITT25_R(crate::FieldReader<bool, bool>);
impl SCHMITT25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT25` writer - Schmitt Trigger Control"]
pub struct SCHMITT25_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT25_W<'a> {
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
#[doc = "Field `SCHMITT26` reader - Schmitt Trigger Control"]
pub struct SCHMITT26_R(crate::FieldReader<bool, bool>);
impl SCHMITT26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT26` writer - Schmitt Trigger Control"]
pub struct SCHMITT26_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `SCHMITT27` reader - Schmitt Trigger Control"]
pub struct SCHMITT27_R(crate::FieldReader<bool, bool>);
impl SCHMITT27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT27` writer - Schmitt Trigger Control"]
pub struct SCHMITT27_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `SCHMITT28` reader - Schmitt Trigger Control"]
pub struct SCHMITT28_R(crate::FieldReader<bool, bool>);
impl SCHMITT28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT28` writer - Schmitt Trigger Control"]
pub struct SCHMITT28_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT28_W<'a> {
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
#[doc = "Field `SCHMITT29` reader - Schmitt Trigger Control"]
pub struct SCHMITT29_R(crate::FieldReader<bool, bool>);
impl SCHMITT29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT29` writer - Schmitt Trigger Control"]
pub struct SCHMITT29_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT29_W<'a> {
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
#[doc = "Field `SCHMITT30` reader - Schmitt Trigger Control"]
pub struct SCHMITT30_R(crate::FieldReader<bool, bool>);
impl SCHMITT30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT30` writer - Schmitt Trigger Control"]
pub struct SCHMITT30_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT30_W<'a> {
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
#[doc = "Field `SCHMITT31` reader - Schmitt Trigger Control"]
pub struct SCHMITT31_R(crate::FieldReader<bool, bool>);
impl SCHMITT31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCHMITT31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHMITT31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHMITT31` writer - Schmitt Trigger Control"]
pub struct SCHMITT31_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT31_W<'a> {
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
    #[doc = "Bit 0 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt0(&self) -> SCHMITT0_R {
        SCHMITT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt1(&self) -> SCHMITT1_R {
        SCHMITT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt2(&self) -> SCHMITT2_R {
        SCHMITT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt3(&self) -> SCHMITT3_R {
        SCHMITT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt4(&self) -> SCHMITT4_R {
        SCHMITT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt5(&self) -> SCHMITT5_R {
        SCHMITT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt6(&self) -> SCHMITT6_R {
        SCHMITT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt7(&self) -> SCHMITT7_R {
        SCHMITT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt8(&self) -> SCHMITT8_R {
        SCHMITT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt9(&self) -> SCHMITT9_R {
        SCHMITT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt10(&self) -> SCHMITT10_R {
        SCHMITT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt11(&self) -> SCHMITT11_R {
        SCHMITT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt12(&self) -> SCHMITT12_R {
        SCHMITT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt13(&self) -> SCHMITT13_R {
        SCHMITT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt14(&self) -> SCHMITT14_R {
        SCHMITT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt15(&self) -> SCHMITT15_R {
        SCHMITT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt16(&self) -> SCHMITT16_R {
        SCHMITT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt17(&self) -> SCHMITT17_R {
        SCHMITT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt18(&self) -> SCHMITT18_R {
        SCHMITT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt19(&self) -> SCHMITT19_R {
        SCHMITT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt20(&self) -> SCHMITT20_R {
        SCHMITT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt21(&self) -> SCHMITT21_R {
        SCHMITT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt22(&self) -> SCHMITT22_R {
        SCHMITT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt23(&self) -> SCHMITT23_R {
        SCHMITT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt24(&self) -> SCHMITT24_R {
        SCHMITT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt25(&self) -> SCHMITT25_R {
        SCHMITT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt26(&self) -> SCHMITT26_R {
        SCHMITT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt27(&self) -> SCHMITT27_R {
        SCHMITT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt28(&self) -> SCHMITT28_R {
        SCHMITT28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt29(&self) -> SCHMITT29_R {
        SCHMITT29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt30(&self) -> SCHMITT30_R {
        SCHMITT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt31(&self) -> SCHMITT31_R {
        SCHMITT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt0(&mut self) -> SCHMITT0_W {
        SCHMITT0_W { w: self }
    }
    #[doc = "Bit 1 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt1(&mut self) -> SCHMITT1_W {
        SCHMITT1_W { w: self }
    }
    #[doc = "Bit 2 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt2(&mut self) -> SCHMITT2_W {
        SCHMITT2_W { w: self }
    }
    #[doc = "Bit 3 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt3(&mut self) -> SCHMITT3_W {
        SCHMITT3_W { w: self }
    }
    #[doc = "Bit 4 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt4(&mut self) -> SCHMITT4_W {
        SCHMITT4_W { w: self }
    }
    #[doc = "Bit 5 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt5(&mut self) -> SCHMITT5_W {
        SCHMITT5_W { w: self }
    }
    #[doc = "Bit 6 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt6(&mut self) -> SCHMITT6_W {
        SCHMITT6_W { w: self }
    }
    #[doc = "Bit 7 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt7(&mut self) -> SCHMITT7_W {
        SCHMITT7_W { w: self }
    }
    #[doc = "Bit 8 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt8(&mut self) -> SCHMITT8_W {
        SCHMITT8_W { w: self }
    }
    #[doc = "Bit 9 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt9(&mut self) -> SCHMITT9_W {
        SCHMITT9_W { w: self }
    }
    #[doc = "Bit 10 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt10(&mut self) -> SCHMITT10_W {
        SCHMITT10_W { w: self }
    }
    #[doc = "Bit 11 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt11(&mut self) -> SCHMITT11_W {
        SCHMITT11_W { w: self }
    }
    #[doc = "Bit 12 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt12(&mut self) -> SCHMITT12_W {
        SCHMITT12_W { w: self }
    }
    #[doc = "Bit 13 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt13(&mut self) -> SCHMITT13_W {
        SCHMITT13_W { w: self }
    }
    #[doc = "Bit 14 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt14(&mut self) -> SCHMITT14_W {
        SCHMITT14_W { w: self }
    }
    #[doc = "Bit 15 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt15(&mut self) -> SCHMITT15_W {
        SCHMITT15_W { w: self }
    }
    #[doc = "Bit 16 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt16(&mut self) -> SCHMITT16_W {
        SCHMITT16_W { w: self }
    }
    #[doc = "Bit 17 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt17(&mut self) -> SCHMITT17_W {
        SCHMITT17_W { w: self }
    }
    #[doc = "Bit 18 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt18(&mut self) -> SCHMITT18_W {
        SCHMITT18_W { w: self }
    }
    #[doc = "Bit 19 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt19(&mut self) -> SCHMITT19_W {
        SCHMITT19_W { w: self }
    }
    #[doc = "Bit 20 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt20(&mut self) -> SCHMITT20_W {
        SCHMITT20_W { w: self }
    }
    #[doc = "Bit 21 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt21(&mut self) -> SCHMITT21_W {
        SCHMITT21_W { w: self }
    }
    #[doc = "Bit 22 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt22(&mut self) -> SCHMITT22_W {
        SCHMITT22_W { w: self }
    }
    #[doc = "Bit 23 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt23(&mut self) -> SCHMITT23_W {
        SCHMITT23_W { w: self }
    }
    #[doc = "Bit 24 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt24(&mut self) -> SCHMITT24_W {
        SCHMITT24_W { w: self }
    }
    #[doc = "Bit 25 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt25(&mut self) -> SCHMITT25_W {
        SCHMITT25_W { w: self }
    }
    #[doc = "Bit 26 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt26(&mut self) -> SCHMITT26_W {
        SCHMITT26_W { w: self }
    }
    #[doc = "Bit 27 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt27(&mut self) -> SCHMITT27_W {
        SCHMITT27_W { w: self }
    }
    #[doc = "Bit 28 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt28(&mut self) -> SCHMITT28_W {
        SCHMITT28_W { w: self }
    }
    #[doc = "Bit 29 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt29(&mut self) -> SCHMITT29_W {
        SCHMITT29_W { w: self }
    }
    #[doc = "Bit 30 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt30(&mut self) -> SCHMITT30_W {
        SCHMITT30_W { w: self }
    }
    #[doc = "Bit 31 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt31(&mut self) -> SCHMITT31_W {
        SCHMITT31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Schmitt Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_schmitt](index.html) module"]
pub struct PIO_SCHMITT_SPEC;
impl crate::RegisterSpec for PIO_SCHMITT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_schmitt::R](R) reader structure"]
impl crate::Readable for PIO_SCHMITT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio_schmitt::W](W) writer structure"]
impl crate::Writable for PIO_SCHMITT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO_SCHMITT to value 0"]
impl crate::Resettable for PIO_SCHMITT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
