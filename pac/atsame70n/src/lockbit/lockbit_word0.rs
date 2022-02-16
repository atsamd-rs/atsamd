#[doc = "Register `LOCKBIT_WORD0` reader"]
pub struct R(crate::R<LOCKBIT_WORD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKBIT_WORD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKBIT_WORD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKBIT_WORD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKBIT_WORD0` writer"]
pub struct W(crate::W<LOCKBIT_WORD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKBIT_WORD0_SPEC>;
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
impl From<crate::W<LOCKBIT_WORD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKBIT_WORD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_REGION_0` reader - Lock Region 0"]
pub struct LOCK_REGION_0_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_0` writer - Lock Region 0"]
pub struct LOCK_REGION_0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_0_W<'a> {
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
#[doc = "Field `LOCK_REGION_1` reader - Lock Region 1"]
pub struct LOCK_REGION_1_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_1` writer - Lock Region 1"]
pub struct LOCK_REGION_1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_1_W<'a> {
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
#[doc = "Field `LOCK_REGION_2` reader - Lock Region 2"]
pub struct LOCK_REGION_2_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_2` writer - Lock Region 2"]
pub struct LOCK_REGION_2_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_2_W<'a> {
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
#[doc = "Field `LOCK_REGION_3` reader - Lock Region 3"]
pub struct LOCK_REGION_3_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_3` writer - Lock Region 3"]
pub struct LOCK_REGION_3_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_3_W<'a> {
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
#[doc = "Field `LOCK_REGION_4` reader - Lock Region 4"]
pub struct LOCK_REGION_4_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_4` writer - Lock Region 4"]
pub struct LOCK_REGION_4_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_4_W<'a> {
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
#[doc = "Field `LOCK_REGION_5` reader - Lock Region 5"]
pub struct LOCK_REGION_5_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_5` writer - Lock Region 5"]
pub struct LOCK_REGION_5_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_5_W<'a> {
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
#[doc = "Field `LOCK_REGION_6` reader - Lock Region 6"]
pub struct LOCK_REGION_6_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_6` writer - Lock Region 6"]
pub struct LOCK_REGION_6_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_6_W<'a> {
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
#[doc = "Field `LOCK_REGION_7` reader - Lock Region 7"]
pub struct LOCK_REGION_7_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_7` writer - Lock Region 7"]
pub struct LOCK_REGION_7_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_7_W<'a> {
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
#[doc = "Field `LOCK_REGION_8` reader - Lock Region 8"]
pub struct LOCK_REGION_8_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_8` writer - Lock Region 8"]
pub struct LOCK_REGION_8_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_8_W<'a> {
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
#[doc = "Field `LOCK_REGION_9` reader - Lock Region 9"]
pub struct LOCK_REGION_9_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_9` writer - Lock Region 9"]
pub struct LOCK_REGION_9_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_9_W<'a> {
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
#[doc = "Field `LOCK_REGION_10` reader - Lock Region 10"]
pub struct LOCK_REGION_10_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_10` writer - Lock Region 10"]
pub struct LOCK_REGION_10_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_10_W<'a> {
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
#[doc = "Field `LOCK_REGION_11` reader - Lock Region 11"]
pub struct LOCK_REGION_11_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_11` writer - Lock Region 11"]
pub struct LOCK_REGION_11_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_11_W<'a> {
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
#[doc = "Field `LOCK_REGION_12` reader - Lock Region 12"]
pub struct LOCK_REGION_12_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_12` writer - Lock Region 12"]
pub struct LOCK_REGION_12_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_12_W<'a> {
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
#[doc = "Field `LOCK_REGION_13` reader - Lock Region 13"]
pub struct LOCK_REGION_13_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_13` writer - Lock Region 13"]
pub struct LOCK_REGION_13_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_13_W<'a> {
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
#[doc = "Field `LOCK_REGION_14` reader - Lock Region 14"]
pub struct LOCK_REGION_14_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_14` writer - Lock Region 14"]
pub struct LOCK_REGION_14_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_14_W<'a> {
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
#[doc = "Field `LOCK_REGION_15` reader - Lock Region 15"]
pub struct LOCK_REGION_15_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_15` writer - Lock Region 15"]
pub struct LOCK_REGION_15_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_15_W<'a> {
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
#[doc = "Field `LOCK_REGION_16` reader - Lock Region 16"]
pub struct LOCK_REGION_16_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_16` writer - Lock Region 16"]
pub struct LOCK_REGION_16_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_16_W<'a> {
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
#[doc = "Field `LOCK_REGION_17` reader - Lock Region 17"]
pub struct LOCK_REGION_17_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_17` writer - Lock Region 17"]
pub struct LOCK_REGION_17_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_17_W<'a> {
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
#[doc = "Field `LOCK_REGION_18` reader - Lock Region 18"]
pub struct LOCK_REGION_18_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_18` writer - Lock Region 18"]
pub struct LOCK_REGION_18_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_18_W<'a> {
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
#[doc = "Field `LOCK_REGION_19` reader - Lock Region 19"]
pub struct LOCK_REGION_19_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_19` writer - Lock Region 19"]
pub struct LOCK_REGION_19_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_19_W<'a> {
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
#[doc = "Field `LOCK_REGION_20` reader - Lock Region 20"]
pub struct LOCK_REGION_20_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_20` writer - Lock Region 20"]
pub struct LOCK_REGION_20_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_20_W<'a> {
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
#[doc = "Field `LOCK_REGION_21` reader - Lock Region 21"]
pub struct LOCK_REGION_21_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_21` writer - Lock Region 21"]
pub struct LOCK_REGION_21_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_21_W<'a> {
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
#[doc = "Field `LOCK_REGION_22` reader - Lock Region 22"]
pub struct LOCK_REGION_22_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_22` writer - Lock Region 22"]
pub struct LOCK_REGION_22_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_22_W<'a> {
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
#[doc = "Field `LOCK_REGION_23` reader - Lock Region 23"]
pub struct LOCK_REGION_23_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_23` writer - Lock Region 23"]
pub struct LOCK_REGION_23_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_23_W<'a> {
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
#[doc = "Field `LOCK_REGION_24` reader - Lock Region 24"]
pub struct LOCK_REGION_24_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_24` writer - Lock Region 24"]
pub struct LOCK_REGION_24_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_24_W<'a> {
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
#[doc = "Field `LOCK_REGION_25` reader - Lock Region 25"]
pub struct LOCK_REGION_25_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_25` writer - Lock Region 25"]
pub struct LOCK_REGION_25_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_25_W<'a> {
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
#[doc = "Field `LOCK_REGION_26` reader - Lock Region 26"]
pub struct LOCK_REGION_26_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_26` writer - Lock Region 26"]
pub struct LOCK_REGION_26_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_26_W<'a> {
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
#[doc = "Field `LOCK_REGION_27` reader - Lock Region 27"]
pub struct LOCK_REGION_27_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_27` writer - Lock Region 27"]
pub struct LOCK_REGION_27_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_27_W<'a> {
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
#[doc = "Field `LOCK_REGION_28` reader - Lock Region 28"]
pub struct LOCK_REGION_28_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_28` writer - Lock Region 28"]
pub struct LOCK_REGION_28_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_28_W<'a> {
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
#[doc = "Field `LOCK_REGION_29` reader - Lock Region 29"]
pub struct LOCK_REGION_29_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_29` writer - Lock Region 29"]
pub struct LOCK_REGION_29_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_29_W<'a> {
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
#[doc = "Field `LOCK_REGION_30` reader - Lock Region 30"]
pub struct LOCK_REGION_30_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_30` writer - Lock Region 30"]
pub struct LOCK_REGION_30_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_30_W<'a> {
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
#[doc = "Field `LOCK_REGION_31` reader - Lock Region 31"]
pub struct LOCK_REGION_31_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_31` writer - Lock Region 31"]
pub struct LOCK_REGION_31_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_31_W<'a> {
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
    #[doc = "Bit 0 - Lock Region 0"]
    #[inline(always)]
    pub fn lock_region_0(&self) -> LOCK_REGION_0_R {
        LOCK_REGION_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Region 1"]
    #[inline(always)]
    pub fn lock_region_1(&self) -> LOCK_REGION_1_R {
        LOCK_REGION_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Region 2"]
    #[inline(always)]
    pub fn lock_region_2(&self) -> LOCK_REGION_2_R {
        LOCK_REGION_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Region 3"]
    #[inline(always)]
    pub fn lock_region_3(&self) -> LOCK_REGION_3_R {
        LOCK_REGION_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock Region 4"]
    #[inline(always)]
    pub fn lock_region_4(&self) -> LOCK_REGION_4_R {
        LOCK_REGION_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lock Region 5"]
    #[inline(always)]
    pub fn lock_region_5(&self) -> LOCK_REGION_5_R {
        LOCK_REGION_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Region 6"]
    #[inline(always)]
    pub fn lock_region_6(&self) -> LOCK_REGION_6_R {
        LOCK_REGION_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock Region 7"]
    #[inline(always)]
    pub fn lock_region_7(&self) -> LOCK_REGION_7_R {
        LOCK_REGION_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Lock Region 8"]
    #[inline(always)]
    pub fn lock_region_8(&self) -> LOCK_REGION_8_R {
        LOCK_REGION_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Lock Region 9"]
    #[inline(always)]
    pub fn lock_region_9(&self) -> LOCK_REGION_9_R {
        LOCK_REGION_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Lock Region 10"]
    #[inline(always)]
    pub fn lock_region_10(&self) -> LOCK_REGION_10_R {
        LOCK_REGION_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Lock Region 11"]
    #[inline(always)]
    pub fn lock_region_11(&self) -> LOCK_REGION_11_R {
        LOCK_REGION_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Lock Region 12"]
    #[inline(always)]
    pub fn lock_region_12(&self) -> LOCK_REGION_12_R {
        LOCK_REGION_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Lock Region 13"]
    #[inline(always)]
    pub fn lock_region_13(&self) -> LOCK_REGION_13_R {
        LOCK_REGION_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Lock Region 14"]
    #[inline(always)]
    pub fn lock_region_14(&self) -> LOCK_REGION_14_R {
        LOCK_REGION_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lock Region 15"]
    #[inline(always)]
    pub fn lock_region_15(&self) -> LOCK_REGION_15_R {
        LOCK_REGION_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Lock Region 16"]
    #[inline(always)]
    pub fn lock_region_16(&self) -> LOCK_REGION_16_R {
        LOCK_REGION_16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock Region 17"]
    #[inline(always)]
    pub fn lock_region_17(&self) -> LOCK_REGION_17_R {
        LOCK_REGION_17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock Region 18"]
    #[inline(always)]
    pub fn lock_region_18(&self) -> LOCK_REGION_18_R {
        LOCK_REGION_18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock Region 19"]
    #[inline(always)]
    pub fn lock_region_19(&self) -> LOCK_REGION_19_R {
        LOCK_REGION_19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock Region 20"]
    #[inline(always)]
    pub fn lock_region_20(&self) -> LOCK_REGION_20_R {
        LOCK_REGION_20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock Region 21"]
    #[inline(always)]
    pub fn lock_region_21(&self) -> LOCK_REGION_21_R {
        LOCK_REGION_21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Lock Region 22"]
    #[inline(always)]
    pub fn lock_region_22(&self) -> LOCK_REGION_22_R {
        LOCK_REGION_22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Region 23"]
    #[inline(always)]
    pub fn lock_region_23(&self) -> LOCK_REGION_23_R {
        LOCK_REGION_23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock Region 24"]
    #[inline(always)]
    pub fn lock_region_24(&self) -> LOCK_REGION_24_R {
        LOCK_REGION_24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Region 25"]
    #[inline(always)]
    pub fn lock_region_25(&self) -> LOCK_REGION_25_R {
        LOCK_REGION_25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Lock Region 26"]
    #[inline(always)]
    pub fn lock_region_26(&self) -> LOCK_REGION_26_R {
        LOCK_REGION_26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock Region 27"]
    #[inline(always)]
    pub fn lock_region_27(&self) -> LOCK_REGION_27_R {
        LOCK_REGION_27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Lock Region 28"]
    #[inline(always)]
    pub fn lock_region_28(&self) -> LOCK_REGION_28_R {
        LOCK_REGION_28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lock Region 29"]
    #[inline(always)]
    pub fn lock_region_29(&self) -> LOCK_REGION_29_R {
        LOCK_REGION_29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lock Region 30"]
    #[inline(always)]
    pub fn lock_region_30(&self) -> LOCK_REGION_30_R {
        LOCK_REGION_30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Region 31"]
    #[inline(always)]
    pub fn lock_region_31(&self) -> LOCK_REGION_31_R {
        LOCK_REGION_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock Region 0"]
    #[inline(always)]
    pub fn lock_region_0(&mut self) -> LOCK_REGION_0_W {
        LOCK_REGION_0_W { w: self }
    }
    #[doc = "Bit 1 - Lock Region 1"]
    #[inline(always)]
    pub fn lock_region_1(&mut self) -> LOCK_REGION_1_W {
        LOCK_REGION_1_W { w: self }
    }
    #[doc = "Bit 2 - Lock Region 2"]
    #[inline(always)]
    pub fn lock_region_2(&mut self) -> LOCK_REGION_2_W {
        LOCK_REGION_2_W { w: self }
    }
    #[doc = "Bit 3 - Lock Region 3"]
    #[inline(always)]
    pub fn lock_region_3(&mut self) -> LOCK_REGION_3_W {
        LOCK_REGION_3_W { w: self }
    }
    #[doc = "Bit 4 - Lock Region 4"]
    #[inline(always)]
    pub fn lock_region_4(&mut self) -> LOCK_REGION_4_W {
        LOCK_REGION_4_W { w: self }
    }
    #[doc = "Bit 5 - Lock Region 5"]
    #[inline(always)]
    pub fn lock_region_5(&mut self) -> LOCK_REGION_5_W {
        LOCK_REGION_5_W { w: self }
    }
    #[doc = "Bit 6 - Lock Region 6"]
    #[inline(always)]
    pub fn lock_region_6(&mut self) -> LOCK_REGION_6_W {
        LOCK_REGION_6_W { w: self }
    }
    #[doc = "Bit 7 - Lock Region 7"]
    #[inline(always)]
    pub fn lock_region_7(&mut self) -> LOCK_REGION_7_W {
        LOCK_REGION_7_W { w: self }
    }
    #[doc = "Bit 8 - Lock Region 8"]
    #[inline(always)]
    pub fn lock_region_8(&mut self) -> LOCK_REGION_8_W {
        LOCK_REGION_8_W { w: self }
    }
    #[doc = "Bit 9 - Lock Region 9"]
    #[inline(always)]
    pub fn lock_region_9(&mut self) -> LOCK_REGION_9_W {
        LOCK_REGION_9_W { w: self }
    }
    #[doc = "Bit 10 - Lock Region 10"]
    #[inline(always)]
    pub fn lock_region_10(&mut self) -> LOCK_REGION_10_W {
        LOCK_REGION_10_W { w: self }
    }
    #[doc = "Bit 11 - Lock Region 11"]
    #[inline(always)]
    pub fn lock_region_11(&mut self) -> LOCK_REGION_11_W {
        LOCK_REGION_11_W { w: self }
    }
    #[doc = "Bit 12 - Lock Region 12"]
    #[inline(always)]
    pub fn lock_region_12(&mut self) -> LOCK_REGION_12_W {
        LOCK_REGION_12_W { w: self }
    }
    #[doc = "Bit 13 - Lock Region 13"]
    #[inline(always)]
    pub fn lock_region_13(&mut self) -> LOCK_REGION_13_W {
        LOCK_REGION_13_W { w: self }
    }
    #[doc = "Bit 14 - Lock Region 14"]
    #[inline(always)]
    pub fn lock_region_14(&mut self) -> LOCK_REGION_14_W {
        LOCK_REGION_14_W { w: self }
    }
    #[doc = "Bit 15 - Lock Region 15"]
    #[inline(always)]
    pub fn lock_region_15(&mut self) -> LOCK_REGION_15_W {
        LOCK_REGION_15_W { w: self }
    }
    #[doc = "Bit 16 - Lock Region 16"]
    #[inline(always)]
    pub fn lock_region_16(&mut self) -> LOCK_REGION_16_W {
        LOCK_REGION_16_W { w: self }
    }
    #[doc = "Bit 17 - Lock Region 17"]
    #[inline(always)]
    pub fn lock_region_17(&mut self) -> LOCK_REGION_17_W {
        LOCK_REGION_17_W { w: self }
    }
    #[doc = "Bit 18 - Lock Region 18"]
    #[inline(always)]
    pub fn lock_region_18(&mut self) -> LOCK_REGION_18_W {
        LOCK_REGION_18_W { w: self }
    }
    #[doc = "Bit 19 - Lock Region 19"]
    #[inline(always)]
    pub fn lock_region_19(&mut self) -> LOCK_REGION_19_W {
        LOCK_REGION_19_W { w: self }
    }
    #[doc = "Bit 20 - Lock Region 20"]
    #[inline(always)]
    pub fn lock_region_20(&mut self) -> LOCK_REGION_20_W {
        LOCK_REGION_20_W { w: self }
    }
    #[doc = "Bit 21 - Lock Region 21"]
    #[inline(always)]
    pub fn lock_region_21(&mut self) -> LOCK_REGION_21_W {
        LOCK_REGION_21_W { w: self }
    }
    #[doc = "Bit 22 - Lock Region 22"]
    #[inline(always)]
    pub fn lock_region_22(&mut self) -> LOCK_REGION_22_W {
        LOCK_REGION_22_W { w: self }
    }
    #[doc = "Bit 23 - Lock Region 23"]
    #[inline(always)]
    pub fn lock_region_23(&mut self) -> LOCK_REGION_23_W {
        LOCK_REGION_23_W { w: self }
    }
    #[doc = "Bit 24 - Lock Region 24"]
    #[inline(always)]
    pub fn lock_region_24(&mut self) -> LOCK_REGION_24_W {
        LOCK_REGION_24_W { w: self }
    }
    #[doc = "Bit 25 - Lock Region 25"]
    #[inline(always)]
    pub fn lock_region_25(&mut self) -> LOCK_REGION_25_W {
        LOCK_REGION_25_W { w: self }
    }
    #[doc = "Bit 26 - Lock Region 26"]
    #[inline(always)]
    pub fn lock_region_26(&mut self) -> LOCK_REGION_26_W {
        LOCK_REGION_26_W { w: self }
    }
    #[doc = "Bit 27 - Lock Region 27"]
    #[inline(always)]
    pub fn lock_region_27(&mut self) -> LOCK_REGION_27_W {
        LOCK_REGION_27_W { w: self }
    }
    #[doc = "Bit 28 - Lock Region 28"]
    #[inline(always)]
    pub fn lock_region_28(&mut self) -> LOCK_REGION_28_W {
        LOCK_REGION_28_W { w: self }
    }
    #[doc = "Bit 29 - Lock Region 29"]
    #[inline(always)]
    pub fn lock_region_29(&mut self) -> LOCK_REGION_29_W {
        LOCK_REGION_29_W { w: self }
    }
    #[doc = "Bit 30 - Lock Region 30"]
    #[inline(always)]
    pub fn lock_region_30(&mut self) -> LOCK_REGION_30_W {
        LOCK_REGION_30_W { w: self }
    }
    #[doc = "Bit 31 - Lock Region 31"]
    #[inline(always)]
    pub fn lock_region_31(&mut self) -> LOCK_REGION_31_W {
        LOCK_REGION_31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock Bits Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockbit_word0](index.html) module"]
pub struct LOCKBIT_WORD0_SPEC;
impl crate::RegisterSpec for LOCKBIT_WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockbit_word0::R](R) reader structure"]
impl crate::Readable for LOCKBIT_WORD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockbit_word0::W](W) writer structure"]
impl crate::Writable for LOCKBIT_WORD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCKBIT_WORD0 to value 0"]
impl crate::Resettable for LOCKBIT_WORD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
