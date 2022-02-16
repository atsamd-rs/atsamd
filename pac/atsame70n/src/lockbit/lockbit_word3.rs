#[doc = "Register `LOCKBIT_WORD3` reader"]
pub struct R(crate::R<LOCKBIT_WORD3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKBIT_WORD3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKBIT_WORD3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKBIT_WORD3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKBIT_WORD3` writer"]
pub struct W(crate::W<LOCKBIT_WORD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKBIT_WORD3_SPEC>;
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
impl From<crate::W<LOCKBIT_WORD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKBIT_WORD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_REGION_96` reader - Lock Region 96"]
pub struct LOCK_REGION_96_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_96_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_96_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_96_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_96` writer - Lock Region 96"]
pub struct LOCK_REGION_96_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_96_W<'a> {
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
#[doc = "Field `LOCK_REGION_97` reader - Lock Region 97"]
pub struct LOCK_REGION_97_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_97_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_97_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_97_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_97` writer - Lock Region 97"]
pub struct LOCK_REGION_97_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_97_W<'a> {
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
#[doc = "Field `LOCK_REGION_98` reader - Lock Region 98"]
pub struct LOCK_REGION_98_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_98_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_98_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_98_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_98` writer - Lock Region 98"]
pub struct LOCK_REGION_98_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_98_W<'a> {
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
#[doc = "Field `LOCK_REGION_99` reader - Lock Region 99"]
pub struct LOCK_REGION_99_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_99_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_99_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_99_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_99` writer - Lock Region 99"]
pub struct LOCK_REGION_99_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_99_W<'a> {
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
#[doc = "Field `LOCK_REGION_100` reader - Lock Region 100"]
pub struct LOCK_REGION_100_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_100_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_100_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_100_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_100` writer - Lock Region 100"]
pub struct LOCK_REGION_100_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_100_W<'a> {
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
#[doc = "Field `LOCK_REGION_101` reader - Lock Region 101"]
pub struct LOCK_REGION_101_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_101_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_101_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_101_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_101` writer - Lock Region 101"]
pub struct LOCK_REGION_101_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_101_W<'a> {
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
#[doc = "Field `LOCK_REGION_102` reader - Lock Region 102"]
pub struct LOCK_REGION_102_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_102_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_102_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_102_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_102` writer - Lock Region 102"]
pub struct LOCK_REGION_102_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_102_W<'a> {
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
#[doc = "Field `LOCK_REGION_103` reader - Lock Region 103"]
pub struct LOCK_REGION_103_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_103_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_103_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_103_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_103` writer - Lock Region 103"]
pub struct LOCK_REGION_103_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_103_W<'a> {
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
#[doc = "Field `LOCK_REGION_104` reader - Lock Region 104"]
pub struct LOCK_REGION_104_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_104_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_104_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_104_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_104` writer - Lock Region 104"]
pub struct LOCK_REGION_104_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_104_W<'a> {
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
#[doc = "Field `LOCK_REGION_105` reader - Lock Region 105"]
pub struct LOCK_REGION_105_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_105_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_105_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_105_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_105` writer - Lock Region 105"]
pub struct LOCK_REGION_105_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_105_W<'a> {
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
#[doc = "Field `LOCK_REGION_106` reader - Lock Region 106"]
pub struct LOCK_REGION_106_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_106_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_106_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_106_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_106` writer - Lock Region 106"]
pub struct LOCK_REGION_106_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_106_W<'a> {
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
#[doc = "Field `LOCK_REGION_107` reader - Lock Region 107"]
pub struct LOCK_REGION_107_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_107_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_107_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_107_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_107` writer - Lock Region 107"]
pub struct LOCK_REGION_107_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_107_W<'a> {
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
#[doc = "Field `LOCK_REGION_108` reader - Lock Region 108"]
pub struct LOCK_REGION_108_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_108_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_108_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_108_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_108` writer - Lock Region 108"]
pub struct LOCK_REGION_108_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_108_W<'a> {
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
#[doc = "Field `LOCK_REGION_109` reader - Lock Region 109"]
pub struct LOCK_REGION_109_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_109_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_109_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_109_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_109` writer - Lock Region 109"]
pub struct LOCK_REGION_109_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_109_W<'a> {
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
#[doc = "Field `LOCK_REGION_110` reader - Lock Region 110"]
pub struct LOCK_REGION_110_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_110_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_110_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_110_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_110` writer - Lock Region 110"]
pub struct LOCK_REGION_110_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_110_W<'a> {
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
#[doc = "Field `LOCK_REGION_111` reader - Lock Region 111"]
pub struct LOCK_REGION_111_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_111_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_111_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_111_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_111` writer - Lock Region 111"]
pub struct LOCK_REGION_111_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_111_W<'a> {
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
#[doc = "Field `LOCK_REGION_112` reader - Lock Region 112"]
pub struct LOCK_REGION_112_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_112_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_112_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_112_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_112` writer - Lock Region 112"]
pub struct LOCK_REGION_112_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_112_W<'a> {
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
#[doc = "Field `LOCK_REGION_113` reader - Lock Region 113"]
pub struct LOCK_REGION_113_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_113_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_113_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_113_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_113` writer - Lock Region 113"]
pub struct LOCK_REGION_113_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_113_W<'a> {
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
#[doc = "Field `LOCK_REGION_114` reader - Lock Region 114"]
pub struct LOCK_REGION_114_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_114_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_114_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_114_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_114` writer - Lock Region 114"]
pub struct LOCK_REGION_114_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_114_W<'a> {
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
#[doc = "Field `LOCK_REGION_115` reader - Lock Region 115"]
pub struct LOCK_REGION_115_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_115_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_115_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_115_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_115` writer - Lock Region 115"]
pub struct LOCK_REGION_115_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_115_W<'a> {
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
#[doc = "Field `LOCK_REGION_116` reader - Lock Region 116"]
pub struct LOCK_REGION_116_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_116_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_116_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_116_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_116` writer - Lock Region 116"]
pub struct LOCK_REGION_116_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_116_W<'a> {
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
#[doc = "Field `LOCK_REGION_117` reader - Lock Region 117"]
pub struct LOCK_REGION_117_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_117_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_117_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_117_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_117` writer - Lock Region 117"]
pub struct LOCK_REGION_117_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_117_W<'a> {
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
#[doc = "Field `LOCK_REGION_118` reader - Lock Region 118"]
pub struct LOCK_REGION_118_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_118_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_118_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_118_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_118` writer - Lock Region 118"]
pub struct LOCK_REGION_118_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_118_W<'a> {
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
#[doc = "Field `LOCK_REGION_119` reader - Lock Region 119"]
pub struct LOCK_REGION_119_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_119_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_119_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_119_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_119` writer - Lock Region 119"]
pub struct LOCK_REGION_119_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_119_W<'a> {
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
#[doc = "Field `LOCK_REGION_120` reader - Lock Region 120"]
pub struct LOCK_REGION_120_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_120_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_120_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_120_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_120` writer - Lock Region 120"]
pub struct LOCK_REGION_120_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_120_W<'a> {
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
#[doc = "Field `LOCK_REGION_121` reader - Lock Region 121"]
pub struct LOCK_REGION_121_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_121_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_121_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_121_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_121` writer - Lock Region 121"]
pub struct LOCK_REGION_121_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_121_W<'a> {
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
#[doc = "Field `LOCK_REGION_122` reader - Lock Region 122"]
pub struct LOCK_REGION_122_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_122_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_122_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_122_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_122` writer - Lock Region 122"]
pub struct LOCK_REGION_122_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_122_W<'a> {
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
#[doc = "Field `LOCK_REGION_123` reader - Lock Region 123"]
pub struct LOCK_REGION_123_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_123_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_123_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_123_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_123` writer - Lock Region 123"]
pub struct LOCK_REGION_123_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_123_W<'a> {
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
#[doc = "Field `LOCK_REGION_124` reader - Lock Region 124"]
pub struct LOCK_REGION_124_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_124_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_124_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_124_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_124` writer - Lock Region 124"]
pub struct LOCK_REGION_124_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_124_W<'a> {
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
#[doc = "Field `LOCK_REGION_125` reader - Lock Region 125"]
pub struct LOCK_REGION_125_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_125_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_125_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_125_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_125` writer - Lock Region 125"]
pub struct LOCK_REGION_125_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_125_W<'a> {
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
#[doc = "Field `LOCK_REGION_126` reader - Lock Region 126"]
pub struct LOCK_REGION_126_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_126_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_126_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_126_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_126` writer - Lock Region 126"]
pub struct LOCK_REGION_126_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_126_W<'a> {
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
#[doc = "Field `LOCK_REGION_127` reader - Lock Region 127"]
pub struct LOCK_REGION_127_R(crate::FieldReader<bool, bool>);
impl LOCK_REGION_127_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_REGION_127_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_REGION_127_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_REGION_127` writer - Lock Region 127"]
pub struct LOCK_REGION_127_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_127_W<'a> {
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
    #[doc = "Bit 0 - Lock Region 96"]
    #[inline(always)]
    pub fn lock_region_96(&self) -> LOCK_REGION_96_R {
        LOCK_REGION_96_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Region 97"]
    #[inline(always)]
    pub fn lock_region_97(&self) -> LOCK_REGION_97_R {
        LOCK_REGION_97_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Region 98"]
    #[inline(always)]
    pub fn lock_region_98(&self) -> LOCK_REGION_98_R {
        LOCK_REGION_98_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Region 99"]
    #[inline(always)]
    pub fn lock_region_99(&self) -> LOCK_REGION_99_R {
        LOCK_REGION_99_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock Region 100"]
    #[inline(always)]
    pub fn lock_region_100(&self) -> LOCK_REGION_100_R {
        LOCK_REGION_100_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lock Region 101"]
    #[inline(always)]
    pub fn lock_region_101(&self) -> LOCK_REGION_101_R {
        LOCK_REGION_101_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Region 102"]
    #[inline(always)]
    pub fn lock_region_102(&self) -> LOCK_REGION_102_R {
        LOCK_REGION_102_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock Region 103"]
    #[inline(always)]
    pub fn lock_region_103(&self) -> LOCK_REGION_103_R {
        LOCK_REGION_103_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Lock Region 104"]
    #[inline(always)]
    pub fn lock_region_104(&self) -> LOCK_REGION_104_R {
        LOCK_REGION_104_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Lock Region 105"]
    #[inline(always)]
    pub fn lock_region_105(&self) -> LOCK_REGION_105_R {
        LOCK_REGION_105_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Lock Region 106"]
    #[inline(always)]
    pub fn lock_region_106(&self) -> LOCK_REGION_106_R {
        LOCK_REGION_106_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Lock Region 107"]
    #[inline(always)]
    pub fn lock_region_107(&self) -> LOCK_REGION_107_R {
        LOCK_REGION_107_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Lock Region 108"]
    #[inline(always)]
    pub fn lock_region_108(&self) -> LOCK_REGION_108_R {
        LOCK_REGION_108_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Lock Region 109"]
    #[inline(always)]
    pub fn lock_region_109(&self) -> LOCK_REGION_109_R {
        LOCK_REGION_109_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Lock Region 110"]
    #[inline(always)]
    pub fn lock_region_110(&self) -> LOCK_REGION_110_R {
        LOCK_REGION_110_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lock Region 111"]
    #[inline(always)]
    pub fn lock_region_111(&self) -> LOCK_REGION_111_R {
        LOCK_REGION_111_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Lock Region 112"]
    #[inline(always)]
    pub fn lock_region_112(&self) -> LOCK_REGION_112_R {
        LOCK_REGION_112_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock Region 113"]
    #[inline(always)]
    pub fn lock_region_113(&self) -> LOCK_REGION_113_R {
        LOCK_REGION_113_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock Region 114"]
    #[inline(always)]
    pub fn lock_region_114(&self) -> LOCK_REGION_114_R {
        LOCK_REGION_114_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock Region 115"]
    #[inline(always)]
    pub fn lock_region_115(&self) -> LOCK_REGION_115_R {
        LOCK_REGION_115_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock Region 116"]
    #[inline(always)]
    pub fn lock_region_116(&self) -> LOCK_REGION_116_R {
        LOCK_REGION_116_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock Region 117"]
    #[inline(always)]
    pub fn lock_region_117(&self) -> LOCK_REGION_117_R {
        LOCK_REGION_117_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Lock Region 118"]
    #[inline(always)]
    pub fn lock_region_118(&self) -> LOCK_REGION_118_R {
        LOCK_REGION_118_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Region 119"]
    #[inline(always)]
    pub fn lock_region_119(&self) -> LOCK_REGION_119_R {
        LOCK_REGION_119_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock Region 120"]
    #[inline(always)]
    pub fn lock_region_120(&self) -> LOCK_REGION_120_R {
        LOCK_REGION_120_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Region 121"]
    #[inline(always)]
    pub fn lock_region_121(&self) -> LOCK_REGION_121_R {
        LOCK_REGION_121_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Lock Region 122"]
    #[inline(always)]
    pub fn lock_region_122(&self) -> LOCK_REGION_122_R {
        LOCK_REGION_122_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock Region 123"]
    #[inline(always)]
    pub fn lock_region_123(&self) -> LOCK_REGION_123_R {
        LOCK_REGION_123_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Lock Region 124"]
    #[inline(always)]
    pub fn lock_region_124(&self) -> LOCK_REGION_124_R {
        LOCK_REGION_124_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lock Region 125"]
    #[inline(always)]
    pub fn lock_region_125(&self) -> LOCK_REGION_125_R {
        LOCK_REGION_125_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lock Region 126"]
    #[inline(always)]
    pub fn lock_region_126(&self) -> LOCK_REGION_126_R {
        LOCK_REGION_126_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Region 127"]
    #[inline(always)]
    pub fn lock_region_127(&self) -> LOCK_REGION_127_R {
        LOCK_REGION_127_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock Region 96"]
    #[inline(always)]
    pub fn lock_region_96(&mut self) -> LOCK_REGION_96_W {
        LOCK_REGION_96_W { w: self }
    }
    #[doc = "Bit 1 - Lock Region 97"]
    #[inline(always)]
    pub fn lock_region_97(&mut self) -> LOCK_REGION_97_W {
        LOCK_REGION_97_W { w: self }
    }
    #[doc = "Bit 2 - Lock Region 98"]
    #[inline(always)]
    pub fn lock_region_98(&mut self) -> LOCK_REGION_98_W {
        LOCK_REGION_98_W { w: self }
    }
    #[doc = "Bit 3 - Lock Region 99"]
    #[inline(always)]
    pub fn lock_region_99(&mut self) -> LOCK_REGION_99_W {
        LOCK_REGION_99_W { w: self }
    }
    #[doc = "Bit 4 - Lock Region 100"]
    #[inline(always)]
    pub fn lock_region_100(&mut self) -> LOCK_REGION_100_W {
        LOCK_REGION_100_W { w: self }
    }
    #[doc = "Bit 5 - Lock Region 101"]
    #[inline(always)]
    pub fn lock_region_101(&mut self) -> LOCK_REGION_101_W {
        LOCK_REGION_101_W { w: self }
    }
    #[doc = "Bit 6 - Lock Region 102"]
    #[inline(always)]
    pub fn lock_region_102(&mut self) -> LOCK_REGION_102_W {
        LOCK_REGION_102_W { w: self }
    }
    #[doc = "Bit 7 - Lock Region 103"]
    #[inline(always)]
    pub fn lock_region_103(&mut self) -> LOCK_REGION_103_W {
        LOCK_REGION_103_W { w: self }
    }
    #[doc = "Bit 8 - Lock Region 104"]
    #[inline(always)]
    pub fn lock_region_104(&mut self) -> LOCK_REGION_104_W {
        LOCK_REGION_104_W { w: self }
    }
    #[doc = "Bit 9 - Lock Region 105"]
    #[inline(always)]
    pub fn lock_region_105(&mut self) -> LOCK_REGION_105_W {
        LOCK_REGION_105_W { w: self }
    }
    #[doc = "Bit 10 - Lock Region 106"]
    #[inline(always)]
    pub fn lock_region_106(&mut self) -> LOCK_REGION_106_W {
        LOCK_REGION_106_W { w: self }
    }
    #[doc = "Bit 11 - Lock Region 107"]
    #[inline(always)]
    pub fn lock_region_107(&mut self) -> LOCK_REGION_107_W {
        LOCK_REGION_107_W { w: self }
    }
    #[doc = "Bit 12 - Lock Region 108"]
    #[inline(always)]
    pub fn lock_region_108(&mut self) -> LOCK_REGION_108_W {
        LOCK_REGION_108_W { w: self }
    }
    #[doc = "Bit 13 - Lock Region 109"]
    #[inline(always)]
    pub fn lock_region_109(&mut self) -> LOCK_REGION_109_W {
        LOCK_REGION_109_W { w: self }
    }
    #[doc = "Bit 14 - Lock Region 110"]
    #[inline(always)]
    pub fn lock_region_110(&mut self) -> LOCK_REGION_110_W {
        LOCK_REGION_110_W { w: self }
    }
    #[doc = "Bit 15 - Lock Region 111"]
    #[inline(always)]
    pub fn lock_region_111(&mut self) -> LOCK_REGION_111_W {
        LOCK_REGION_111_W { w: self }
    }
    #[doc = "Bit 16 - Lock Region 112"]
    #[inline(always)]
    pub fn lock_region_112(&mut self) -> LOCK_REGION_112_W {
        LOCK_REGION_112_W { w: self }
    }
    #[doc = "Bit 17 - Lock Region 113"]
    #[inline(always)]
    pub fn lock_region_113(&mut self) -> LOCK_REGION_113_W {
        LOCK_REGION_113_W { w: self }
    }
    #[doc = "Bit 18 - Lock Region 114"]
    #[inline(always)]
    pub fn lock_region_114(&mut self) -> LOCK_REGION_114_W {
        LOCK_REGION_114_W { w: self }
    }
    #[doc = "Bit 19 - Lock Region 115"]
    #[inline(always)]
    pub fn lock_region_115(&mut self) -> LOCK_REGION_115_W {
        LOCK_REGION_115_W { w: self }
    }
    #[doc = "Bit 20 - Lock Region 116"]
    #[inline(always)]
    pub fn lock_region_116(&mut self) -> LOCK_REGION_116_W {
        LOCK_REGION_116_W { w: self }
    }
    #[doc = "Bit 21 - Lock Region 117"]
    #[inline(always)]
    pub fn lock_region_117(&mut self) -> LOCK_REGION_117_W {
        LOCK_REGION_117_W { w: self }
    }
    #[doc = "Bit 22 - Lock Region 118"]
    #[inline(always)]
    pub fn lock_region_118(&mut self) -> LOCK_REGION_118_W {
        LOCK_REGION_118_W { w: self }
    }
    #[doc = "Bit 23 - Lock Region 119"]
    #[inline(always)]
    pub fn lock_region_119(&mut self) -> LOCK_REGION_119_W {
        LOCK_REGION_119_W { w: self }
    }
    #[doc = "Bit 24 - Lock Region 120"]
    #[inline(always)]
    pub fn lock_region_120(&mut self) -> LOCK_REGION_120_W {
        LOCK_REGION_120_W { w: self }
    }
    #[doc = "Bit 25 - Lock Region 121"]
    #[inline(always)]
    pub fn lock_region_121(&mut self) -> LOCK_REGION_121_W {
        LOCK_REGION_121_W { w: self }
    }
    #[doc = "Bit 26 - Lock Region 122"]
    #[inline(always)]
    pub fn lock_region_122(&mut self) -> LOCK_REGION_122_W {
        LOCK_REGION_122_W { w: self }
    }
    #[doc = "Bit 27 - Lock Region 123"]
    #[inline(always)]
    pub fn lock_region_123(&mut self) -> LOCK_REGION_123_W {
        LOCK_REGION_123_W { w: self }
    }
    #[doc = "Bit 28 - Lock Region 124"]
    #[inline(always)]
    pub fn lock_region_124(&mut self) -> LOCK_REGION_124_W {
        LOCK_REGION_124_W { w: self }
    }
    #[doc = "Bit 29 - Lock Region 125"]
    #[inline(always)]
    pub fn lock_region_125(&mut self) -> LOCK_REGION_125_W {
        LOCK_REGION_125_W { w: self }
    }
    #[doc = "Bit 30 - Lock Region 126"]
    #[inline(always)]
    pub fn lock_region_126(&mut self) -> LOCK_REGION_126_W {
        LOCK_REGION_126_W { w: self }
    }
    #[doc = "Bit 31 - Lock Region 127"]
    #[inline(always)]
    pub fn lock_region_127(&mut self) -> LOCK_REGION_127_W {
        LOCK_REGION_127_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock Bits Word 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockbit_word3](index.html) module"]
pub struct LOCKBIT_WORD3_SPEC;
impl crate::RegisterSpec for LOCKBIT_WORD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockbit_word3::R](R) reader structure"]
impl crate::Readable for LOCKBIT_WORD3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockbit_word3::W](W) writer structure"]
impl crate::Writable for LOCKBIT_WORD3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCKBIT_WORD3 to value 0"]
impl crate::Resettable for LOCKBIT_WORD3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
