#[doc = "Register `NDAT1` reader"]
pub struct R(crate::R<NDAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NDAT1` writer"]
pub struct W(crate::W<NDAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDAT1_SPEC>;
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
impl From<crate::W<NDAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NDAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ND0` reader - New Data 0"]
pub struct ND0_R(crate::FieldReader<bool, bool>);
impl ND0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND0` writer - New Data 0"]
pub struct ND0_W<'a> {
    w: &'a mut W,
}
impl<'a> ND0_W<'a> {
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
#[doc = "Field `ND1` reader - New Data 1"]
pub struct ND1_R(crate::FieldReader<bool, bool>);
impl ND1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND1` writer - New Data 1"]
pub struct ND1_W<'a> {
    w: &'a mut W,
}
impl<'a> ND1_W<'a> {
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
#[doc = "Field `ND2` reader - New Data 2"]
pub struct ND2_R(crate::FieldReader<bool, bool>);
impl ND2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND2` writer - New Data 2"]
pub struct ND2_W<'a> {
    w: &'a mut W,
}
impl<'a> ND2_W<'a> {
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
#[doc = "Field `ND3` reader - New Data 3"]
pub struct ND3_R(crate::FieldReader<bool, bool>);
impl ND3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND3` writer - New Data 3"]
pub struct ND3_W<'a> {
    w: &'a mut W,
}
impl<'a> ND3_W<'a> {
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
#[doc = "Field `ND4` reader - New Data 4"]
pub struct ND4_R(crate::FieldReader<bool, bool>);
impl ND4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND4` writer - New Data 4"]
pub struct ND4_W<'a> {
    w: &'a mut W,
}
impl<'a> ND4_W<'a> {
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
#[doc = "Field `ND5` reader - New Data 5"]
pub struct ND5_R(crate::FieldReader<bool, bool>);
impl ND5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND5` writer - New Data 5"]
pub struct ND5_W<'a> {
    w: &'a mut W,
}
impl<'a> ND5_W<'a> {
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
#[doc = "Field `ND6` reader - New Data 6"]
pub struct ND6_R(crate::FieldReader<bool, bool>);
impl ND6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND6` writer - New Data 6"]
pub struct ND6_W<'a> {
    w: &'a mut W,
}
impl<'a> ND6_W<'a> {
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
#[doc = "Field `ND7` reader - New Data 7"]
pub struct ND7_R(crate::FieldReader<bool, bool>);
impl ND7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND7` writer - New Data 7"]
pub struct ND7_W<'a> {
    w: &'a mut W,
}
impl<'a> ND7_W<'a> {
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
#[doc = "Field `ND8` reader - New Data 8"]
pub struct ND8_R(crate::FieldReader<bool, bool>);
impl ND8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND8` writer - New Data 8"]
pub struct ND8_W<'a> {
    w: &'a mut W,
}
impl<'a> ND8_W<'a> {
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
#[doc = "Field `ND9` reader - New Data 9"]
pub struct ND9_R(crate::FieldReader<bool, bool>);
impl ND9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND9` writer - New Data 9"]
pub struct ND9_W<'a> {
    w: &'a mut W,
}
impl<'a> ND9_W<'a> {
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
#[doc = "Field `ND10` reader - New Data 10"]
pub struct ND10_R(crate::FieldReader<bool, bool>);
impl ND10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND10` writer - New Data 10"]
pub struct ND10_W<'a> {
    w: &'a mut W,
}
impl<'a> ND10_W<'a> {
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
#[doc = "Field `ND11` reader - New Data 11"]
pub struct ND11_R(crate::FieldReader<bool, bool>);
impl ND11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND11` writer - New Data 11"]
pub struct ND11_W<'a> {
    w: &'a mut W,
}
impl<'a> ND11_W<'a> {
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
#[doc = "Field `ND12` reader - New Data 12"]
pub struct ND12_R(crate::FieldReader<bool, bool>);
impl ND12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND12` writer - New Data 12"]
pub struct ND12_W<'a> {
    w: &'a mut W,
}
impl<'a> ND12_W<'a> {
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
#[doc = "Field `ND13` reader - New Data 13"]
pub struct ND13_R(crate::FieldReader<bool, bool>);
impl ND13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND13` writer - New Data 13"]
pub struct ND13_W<'a> {
    w: &'a mut W,
}
impl<'a> ND13_W<'a> {
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
#[doc = "Field `ND14` reader - New Data 14"]
pub struct ND14_R(crate::FieldReader<bool, bool>);
impl ND14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND14` writer - New Data 14"]
pub struct ND14_W<'a> {
    w: &'a mut W,
}
impl<'a> ND14_W<'a> {
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
#[doc = "Field `ND15` reader - New Data 15"]
pub struct ND15_R(crate::FieldReader<bool, bool>);
impl ND15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND15` writer - New Data 15"]
pub struct ND15_W<'a> {
    w: &'a mut W,
}
impl<'a> ND15_W<'a> {
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
#[doc = "Field `ND16` reader - New Data 16"]
pub struct ND16_R(crate::FieldReader<bool, bool>);
impl ND16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND16` writer - New Data 16"]
pub struct ND16_W<'a> {
    w: &'a mut W,
}
impl<'a> ND16_W<'a> {
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
#[doc = "Field `ND17` reader - New Data 17"]
pub struct ND17_R(crate::FieldReader<bool, bool>);
impl ND17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND17` writer - New Data 17"]
pub struct ND17_W<'a> {
    w: &'a mut W,
}
impl<'a> ND17_W<'a> {
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
#[doc = "Field `ND18` reader - New Data 18"]
pub struct ND18_R(crate::FieldReader<bool, bool>);
impl ND18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND18` writer - New Data 18"]
pub struct ND18_W<'a> {
    w: &'a mut W,
}
impl<'a> ND18_W<'a> {
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
#[doc = "Field `ND19` reader - New Data 19"]
pub struct ND19_R(crate::FieldReader<bool, bool>);
impl ND19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND19` writer - New Data 19"]
pub struct ND19_W<'a> {
    w: &'a mut W,
}
impl<'a> ND19_W<'a> {
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
#[doc = "Field `ND20` reader - New Data 20"]
pub struct ND20_R(crate::FieldReader<bool, bool>);
impl ND20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND20` writer - New Data 20"]
pub struct ND20_W<'a> {
    w: &'a mut W,
}
impl<'a> ND20_W<'a> {
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
#[doc = "Field `ND21` reader - New Data 21"]
pub struct ND21_R(crate::FieldReader<bool, bool>);
impl ND21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND21` writer - New Data 21"]
pub struct ND21_W<'a> {
    w: &'a mut W,
}
impl<'a> ND21_W<'a> {
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
#[doc = "Field `ND22` reader - New Data 22"]
pub struct ND22_R(crate::FieldReader<bool, bool>);
impl ND22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND22` writer - New Data 22"]
pub struct ND22_W<'a> {
    w: &'a mut W,
}
impl<'a> ND22_W<'a> {
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
#[doc = "Field `ND23` reader - New Data 23"]
pub struct ND23_R(crate::FieldReader<bool, bool>);
impl ND23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND23` writer - New Data 23"]
pub struct ND23_W<'a> {
    w: &'a mut W,
}
impl<'a> ND23_W<'a> {
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
#[doc = "Field `ND24` reader - New Data 24"]
pub struct ND24_R(crate::FieldReader<bool, bool>);
impl ND24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND24` writer - New Data 24"]
pub struct ND24_W<'a> {
    w: &'a mut W,
}
impl<'a> ND24_W<'a> {
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
#[doc = "Field `ND25` reader - New Data 25"]
pub struct ND25_R(crate::FieldReader<bool, bool>);
impl ND25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND25` writer - New Data 25"]
pub struct ND25_W<'a> {
    w: &'a mut W,
}
impl<'a> ND25_W<'a> {
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
#[doc = "Field `ND26` reader - New Data 26"]
pub struct ND26_R(crate::FieldReader<bool, bool>);
impl ND26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND26` writer - New Data 26"]
pub struct ND26_W<'a> {
    w: &'a mut W,
}
impl<'a> ND26_W<'a> {
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
#[doc = "Field `ND27` reader - New Data 27"]
pub struct ND27_R(crate::FieldReader<bool, bool>);
impl ND27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND27` writer - New Data 27"]
pub struct ND27_W<'a> {
    w: &'a mut W,
}
impl<'a> ND27_W<'a> {
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
#[doc = "Field `ND28` reader - New Data 28"]
pub struct ND28_R(crate::FieldReader<bool, bool>);
impl ND28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND28` writer - New Data 28"]
pub struct ND28_W<'a> {
    w: &'a mut W,
}
impl<'a> ND28_W<'a> {
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
#[doc = "Field `ND29` reader - New Data 29"]
pub struct ND29_R(crate::FieldReader<bool, bool>);
impl ND29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND29` writer - New Data 29"]
pub struct ND29_W<'a> {
    w: &'a mut W,
}
impl<'a> ND29_W<'a> {
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
#[doc = "Field `ND30` reader - New Data 30"]
pub struct ND30_R(crate::FieldReader<bool, bool>);
impl ND30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND30` writer - New Data 30"]
pub struct ND30_W<'a> {
    w: &'a mut W,
}
impl<'a> ND30_W<'a> {
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
#[doc = "Field `ND31` reader - New Data 31"]
pub struct ND31_R(crate::FieldReader<bool, bool>);
impl ND31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ND31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND31` writer - New Data 31"]
pub struct ND31_W<'a> {
    w: &'a mut W,
}
impl<'a> ND31_W<'a> {
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
    #[doc = "Bit 0 - New Data 0"]
    #[inline(always)]
    pub fn nd0(&self) -> ND0_R {
        ND0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - New Data 1"]
    #[inline(always)]
    pub fn nd1(&self) -> ND1_R {
        ND1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - New Data 2"]
    #[inline(always)]
    pub fn nd2(&self) -> ND2_R {
        ND2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - New Data 3"]
    #[inline(always)]
    pub fn nd3(&self) -> ND3_R {
        ND3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - New Data 4"]
    #[inline(always)]
    pub fn nd4(&self) -> ND4_R {
        ND4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - New Data 5"]
    #[inline(always)]
    pub fn nd5(&self) -> ND5_R {
        ND5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - New Data 6"]
    #[inline(always)]
    pub fn nd6(&self) -> ND6_R {
        ND6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - New Data 7"]
    #[inline(always)]
    pub fn nd7(&self) -> ND7_R {
        ND7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - New Data 8"]
    #[inline(always)]
    pub fn nd8(&self) -> ND8_R {
        ND8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - New Data 9"]
    #[inline(always)]
    pub fn nd9(&self) -> ND9_R {
        ND9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - New Data 10"]
    #[inline(always)]
    pub fn nd10(&self) -> ND10_R {
        ND10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - New Data 11"]
    #[inline(always)]
    pub fn nd11(&self) -> ND11_R {
        ND11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - New Data 12"]
    #[inline(always)]
    pub fn nd12(&self) -> ND12_R {
        ND12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - New Data 13"]
    #[inline(always)]
    pub fn nd13(&self) -> ND13_R {
        ND13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - New Data 14"]
    #[inline(always)]
    pub fn nd14(&self) -> ND14_R {
        ND14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - New Data 15"]
    #[inline(always)]
    pub fn nd15(&self) -> ND15_R {
        ND15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - New Data 16"]
    #[inline(always)]
    pub fn nd16(&self) -> ND16_R {
        ND16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - New Data 17"]
    #[inline(always)]
    pub fn nd17(&self) -> ND17_R {
        ND17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - New Data 18"]
    #[inline(always)]
    pub fn nd18(&self) -> ND18_R {
        ND18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - New Data 19"]
    #[inline(always)]
    pub fn nd19(&self) -> ND19_R {
        ND19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - New Data 20"]
    #[inline(always)]
    pub fn nd20(&self) -> ND20_R {
        ND20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - New Data 21"]
    #[inline(always)]
    pub fn nd21(&self) -> ND21_R {
        ND21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - New Data 22"]
    #[inline(always)]
    pub fn nd22(&self) -> ND22_R {
        ND22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - New Data 23"]
    #[inline(always)]
    pub fn nd23(&self) -> ND23_R {
        ND23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - New Data 24"]
    #[inline(always)]
    pub fn nd24(&self) -> ND24_R {
        ND24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - New Data 25"]
    #[inline(always)]
    pub fn nd25(&self) -> ND25_R {
        ND25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - New Data 26"]
    #[inline(always)]
    pub fn nd26(&self) -> ND26_R {
        ND26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - New Data 27"]
    #[inline(always)]
    pub fn nd27(&self) -> ND27_R {
        ND27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - New Data 28"]
    #[inline(always)]
    pub fn nd28(&self) -> ND28_R {
        ND28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - New Data 29"]
    #[inline(always)]
    pub fn nd29(&self) -> ND29_R {
        ND29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - New Data 30"]
    #[inline(always)]
    pub fn nd30(&self) -> ND30_R {
        ND30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - New Data 31"]
    #[inline(always)]
    pub fn nd31(&self) -> ND31_R {
        ND31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - New Data 0"]
    #[inline(always)]
    pub fn nd0(&mut self) -> ND0_W {
        ND0_W { w: self }
    }
    #[doc = "Bit 1 - New Data 1"]
    #[inline(always)]
    pub fn nd1(&mut self) -> ND1_W {
        ND1_W { w: self }
    }
    #[doc = "Bit 2 - New Data 2"]
    #[inline(always)]
    pub fn nd2(&mut self) -> ND2_W {
        ND2_W { w: self }
    }
    #[doc = "Bit 3 - New Data 3"]
    #[inline(always)]
    pub fn nd3(&mut self) -> ND3_W {
        ND3_W { w: self }
    }
    #[doc = "Bit 4 - New Data 4"]
    #[inline(always)]
    pub fn nd4(&mut self) -> ND4_W {
        ND4_W { w: self }
    }
    #[doc = "Bit 5 - New Data 5"]
    #[inline(always)]
    pub fn nd5(&mut self) -> ND5_W {
        ND5_W { w: self }
    }
    #[doc = "Bit 6 - New Data 6"]
    #[inline(always)]
    pub fn nd6(&mut self) -> ND6_W {
        ND6_W { w: self }
    }
    #[doc = "Bit 7 - New Data 7"]
    #[inline(always)]
    pub fn nd7(&mut self) -> ND7_W {
        ND7_W { w: self }
    }
    #[doc = "Bit 8 - New Data 8"]
    #[inline(always)]
    pub fn nd8(&mut self) -> ND8_W {
        ND8_W { w: self }
    }
    #[doc = "Bit 9 - New Data 9"]
    #[inline(always)]
    pub fn nd9(&mut self) -> ND9_W {
        ND9_W { w: self }
    }
    #[doc = "Bit 10 - New Data 10"]
    #[inline(always)]
    pub fn nd10(&mut self) -> ND10_W {
        ND10_W { w: self }
    }
    #[doc = "Bit 11 - New Data 11"]
    #[inline(always)]
    pub fn nd11(&mut self) -> ND11_W {
        ND11_W { w: self }
    }
    #[doc = "Bit 12 - New Data 12"]
    #[inline(always)]
    pub fn nd12(&mut self) -> ND12_W {
        ND12_W { w: self }
    }
    #[doc = "Bit 13 - New Data 13"]
    #[inline(always)]
    pub fn nd13(&mut self) -> ND13_W {
        ND13_W { w: self }
    }
    #[doc = "Bit 14 - New Data 14"]
    #[inline(always)]
    pub fn nd14(&mut self) -> ND14_W {
        ND14_W { w: self }
    }
    #[doc = "Bit 15 - New Data 15"]
    #[inline(always)]
    pub fn nd15(&mut self) -> ND15_W {
        ND15_W { w: self }
    }
    #[doc = "Bit 16 - New Data 16"]
    #[inline(always)]
    pub fn nd16(&mut self) -> ND16_W {
        ND16_W { w: self }
    }
    #[doc = "Bit 17 - New Data 17"]
    #[inline(always)]
    pub fn nd17(&mut self) -> ND17_W {
        ND17_W { w: self }
    }
    #[doc = "Bit 18 - New Data 18"]
    #[inline(always)]
    pub fn nd18(&mut self) -> ND18_W {
        ND18_W { w: self }
    }
    #[doc = "Bit 19 - New Data 19"]
    #[inline(always)]
    pub fn nd19(&mut self) -> ND19_W {
        ND19_W { w: self }
    }
    #[doc = "Bit 20 - New Data 20"]
    #[inline(always)]
    pub fn nd20(&mut self) -> ND20_W {
        ND20_W { w: self }
    }
    #[doc = "Bit 21 - New Data 21"]
    #[inline(always)]
    pub fn nd21(&mut self) -> ND21_W {
        ND21_W { w: self }
    }
    #[doc = "Bit 22 - New Data 22"]
    #[inline(always)]
    pub fn nd22(&mut self) -> ND22_W {
        ND22_W { w: self }
    }
    #[doc = "Bit 23 - New Data 23"]
    #[inline(always)]
    pub fn nd23(&mut self) -> ND23_W {
        ND23_W { w: self }
    }
    #[doc = "Bit 24 - New Data 24"]
    #[inline(always)]
    pub fn nd24(&mut self) -> ND24_W {
        ND24_W { w: self }
    }
    #[doc = "Bit 25 - New Data 25"]
    #[inline(always)]
    pub fn nd25(&mut self) -> ND25_W {
        ND25_W { w: self }
    }
    #[doc = "Bit 26 - New Data 26"]
    #[inline(always)]
    pub fn nd26(&mut self) -> ND26_W {
        ND26_W { w: self }
    }
    #[doc = "Bit 27 - New Data 27"]
    #[inline(always)]
    pub fn nd27(&mut self) -> ND27_W {
        ND27_W { w: self }
    }
    #[doc = "Bit 28 - New Data 28"]
    #[inline(always)]
    pub fn nd28(&mut self) -> ND28_W {
        ND28_W { w: self }
    }
    #[doc = "Bit 29 - New Data 29"]
    #[inline(always)]
    pub fn nd29(&mut self) -> ND29_W {
        ND29_W { w: self }
    }
    #[doc = "Bit 30 - New Data 30"]
    #[inline(always)]
    pub fn nd30(&mut self) -> ND30_W {
        ND30_W { w: self }
    }
    #[doc = "Bit 31 - New Data 31"]
    #[inline(always)]
    pub fn nd31(&mut self) -> ND31_W {
        ND31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "New Data 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat1](index.html) module"]
pub struct NDAT1_SPEC;
impl crate::RegisterSpec for NDAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ndat1::R](R) reader structure"]
impl crate::Readable for NDAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ndat1::W](W) writer structure"]
impl crate::Writable for NDAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NDAT1 to value 0"]
impl crate::Resettable for NDAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
