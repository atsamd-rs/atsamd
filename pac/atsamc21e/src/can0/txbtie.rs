#[doc = "Register `TXBTIE` reader"]
pub struct R(crate::R<TXBTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBTIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBTIE` writer"]
pub struct W(crate::W<TXBTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBTIE_SPEC>;
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
impl From<crate::W<TXBTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBTIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE0` reader - Transmission Interrupt Enable 0"]
pub struct TIE0_R(crate::FieldReader<bool, bool>);
impl TIE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE0` writer - Transmission Interrupt Enable 0"]
pub struct TIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE0_W<'a> {
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
#[doc = "Field `TIE1` reader - Transmission Interrupt Enable 1"]
pub struct TIE1_R(crate::FieldReader<bool, bool>);
impl TIE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE1` writer - Transmission Interrupt Enable 1"]
pub struct TIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE1_W<'a> {
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
#[doc = "Field `TIE2` reader - Transmission Interrupt Enable 2"]
pub struct TIE2_R(crate::FieldReader<bool, bool>);
impl TIE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE2` writer - Transmission Interrupt Enable 2"]
pub struct TIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE2_W<'a> {
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
#[doc = "Field `TIE3` reader - Transmission Interrupt Enable 3"]
pub struct TIE3_R(crate::FieldReader<bool, bool>);
impl TIE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE3` writer - Transmission Interrupt Enable 3"]
pub struct TIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE3_W<'a> {
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
#[doc = "Field `TIE4` reader - Transmission Interrupt Enable 4"]
pub struct TIE4_R(crate::FieldReader<bool, bool>);
impl TIE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE4` writer - Transmission Interrupt Enable 4"]
pub struct TIE4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE4_W<'a> {
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
#[doc = "Field `TIE5` reader - Transmission Interrupt Enable 5"]
pub struct TIE5_R(crate::FieldReader<bool, bool>);
impl TIE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE5` writer - Transmission Interrupt Enable 5"]
pub struct TIE5_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE5_W<'a> {
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
#[doc = "Field `TIE6` reader - Transmission Interrupt Enable 6"]
pub struct TIE6_R(crate::FieldReader<bool, bool>);
impl TIE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE6` writer - Transmission Interrupt Enable 6"]
pub struct TIE6_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE6_W<'a> {
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
#[doc = "Field `TIE7` reader - Transmission Interrupt Enable 7"]
pub struct TIE7_R(crate::FieldReader<bool, bool>);
impl TIE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE7` writer - Transmission Interrupt Enable 7"]
pub struct TIE7_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE7_W<'a> {
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
#[doc = "Field `TIE8` reader - Transmission Interrupt Enable 8"]
pub struct TIE8_R(crate::FieldReader<bool, bool>);
impl TIE8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE8` writer - Transmission Interrupt Enable 8"]
pub struct TIE8_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE8_W<'a> {
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
#[doc = "Field `TIE9` reader - Transmission Interrupt Enable 9"]
pub struct TIE9_R(crate::FieldReader<bool, bool>);
impl TIE9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE9` writer - Transmission Interrupt Enable 9"]
pub struct TIE9_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE9_W<'a> {
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
#[doc = "Field `TIE10` reader - Transmission Interrupt Enable 10"]
pub struct TIE10_R(crate::FieldReader<bool, bool>);
impl TIE10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE10` writer - Transmission Interrupt Enable 10"]
pub struct TIE10_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE10_W<'a> {
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
#[doc = "Field `TIE11` reader - Transmission Interrupt Enable 11"]
pub struct TIE11_R(crate::FieldReader<bool, bool>);
impl TIE11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE11` writer - Transmission Interrupt Enable 11"]
pub struct TIE11_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE11_W<'a> {
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
#[doc = "Field `TIE12` reader - Transmission Interrupt Enable 12"]
pub struct TIE12_R(crate::FieldReader<bool, bool>);
impl TIE12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE12` writer - Transmission Interrupt Enable 12"]
pub struct TIE12_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE12_W<'a> {
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
#[doc = "Field `TIE13` reader - Transmission Interrupt Enable 13"]
pub struct TIE13_R(crate::FieldReader<bool, bool>);
impl TIE13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE13` writer - Transmission Interrupt Enable 13"]
pub struct TIE13_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE13_W<'a> {
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
#[doc = "Field `TIE14` reader - Transmission Interrupt Enable 14"]
pub struct TIE14_R(crate::FieldReader<bool, bool>);
impl TIE14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE14` writer - Transmission Interrupt Enable 14"]
pub struct TIE14_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE14_W<'a> {
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
#[doc = "Field `TIE15` reader - Transmission Interrupt Enable 15"]
pub struct TIE15_R(crate::FieldReader<bool, bool>);
impl TIE15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE15` writer - Transmission Interrupt Enable 15"]
pub struct TIE15_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE15_W<'a> {
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
#[doc = "Field `TIE16` reader - Transmission Interrupt Enable 16"]
pub struct TIE16_R(crate::FieldReader<bool, bool>);
impl TIE16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE16` writer - Transmission Interrupt Enable 16"]
pub struct TIE16_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE16_W<'a> {
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
#[doc = "Field `TIE17` reader - Transmission Interrupt Enable 17"]
pub struct TIE17_R(crate::FieldReader<bool, bool>);
impl TIE17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE17` writer - Transmission Interrupt Enable 17"]
pub struct TIE17_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE17_W<'a> {
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
#[doc = "Field `TIE18` reader - Transmission Interrupt Enable 18"]
pub struct TIE18_R(crate::FieldReader<bool, bool>);
impl TIE18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE18` writer - Transmission Interrupt Enable 18"]
pub struct TIE18_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE18_W<'a> {
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
#[doc = "Field `TIE19` reader - Transmission Interrupt Enable 19"]
pub struct TIE19_R(crate::FieldReader<bool, bool>);
impl TIE19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE19` writer - Transmission Interrupt Enable 19"]
pub struct TIE19_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE19_W<'a> {
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
#[doc = "Field `TIE20` reader - Transmission Interrupt Enable 20"]
pub struct TIE20_R(crate::FieldReader<bool, bool>);
impl TIE20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE20` writer - Transmission Interrupt Enable 20"]
pub struct TIE20_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE20_W<'a> {
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
#[doc = "Field `TIE21` reader - Transmission Interrupt Enable 21"]
pub struct TIE21_R(crate::FieldReader<bool, bool>);
impl TIE21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE21` writer - Transmission Interrupt Enable 21"]
pub struct TIE21_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE21_W<'a> {
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
#[doc = "Field `TIE22` reader - Transmission Interrupt Enable 22"]
pub struct TIE22_R(crate::FieldReader<bool, bool>);
impl TIE22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE22` writer - Transmission Interrupt Enable 22"]
pub struct TIE22_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE22_W<'a> {
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
#[doc = "Field `TIE23` reader - Transmission Interrupt Enable 23"]
pub struct TIE23_R(crate::FieldReader<bool, bool>);
impl TIE23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE23` writer - Transmission Interrupt Enable 23"]
pub struct TIE23_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE23_W<'a> {
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
#[doc = "Field `TIE24` reader - Transmission Interrupt Enable 24"]
pub struct TIE24_R(crate::FieldReader<bool, bool>);
impl TIE24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE24` writer - Transmission Interrupt Enable 24"]
pub struct TIE24_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE24_W<'a> {
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
#[doc = "Field `TIE25` reader - Transmission Interrupt Enable 25"]
pub struct TIE25_R(crate::FieldReader<bool, bool>);
impl TIE25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE25` writer - Transmission Interrupt Enable 25"]
pub struct TIE25_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE25_W<'a> {
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
#[doc = "Field `TIE26` reader - Transmission Interrupt Enable 26"]
pub struct TIE26_R(crate::FieldReader<bool, bool>);
impl TIE26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE26` writer - Transmission Interrupt Enable 26"]
pub struct TIE26_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE26_W<'a> {
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
#[doc = "Field `TIE27` reader - Transmission Interrupt Enable 27"]
pub struct TIE27_R(crate::FieldReader<bool, bool>);
impl TIE27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE27` writer - Transmission Interrupt Enable 27"]
pub struct TIE27_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE27_W<'a> {
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
#[doc = "Field `TIE28` reader - Transmission Interrupt Enable 28"]
pub struct TIE28_R(crate::FieldReader<bool, bool>);
impl TIE28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE28` writer - Transmission Interrupt Enable 28"]
pub struct TIE28_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE28_W<'a> {
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
#[doc = "Field `TIE29` reader - Transmission Interrupt Enable 29"]
pub struct TIE29_R(crate::FieldReader<bool, bool>);
impl TIE29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE29` writer - Transmission Interrupt Enable 29"]
pub struct TIE29_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE29_W<'a> {
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
#[doc = "Field `TIE30` reader - Transmission Interrupt Enable 30"]
pub struct TIE30_R(crate::FieldReader<bool, bool>);
impl TIE30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE30` writer - Transmission Interrupt Enable 30"]
pub struct TIE30_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE30_W<'a> {
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
#[doc = "Field `TIE31` reader - Transmission Interrupt Enable 31"]
pub struct TIE31_R(crate::FieldReader<bool, bool>);
impl TIE31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE31` writer - Transmission Interrupt Enable 31"]
pub struct TIE31_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE31_W<'a> {
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
    #[doc = "Bit 0 - Transmission Interrupt Enable 0"]
    #[inline(always)]
    pub fn tie0(&self) -> TIE0_R {
        TIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmission Interrupt Enable 1"]
    #[inline(always)]
    pub fn tie1(&self) -> TIE1_R {
        TIE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmission Interrupt Enable 2"]
    #[inline(always)]
    pub fn tie2(&self) -> TIE2_R {
        TIE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission Interrupt Enable 3"]
    #[inline(always)]
    pub fn tie3(&self) -> TIE3_R {
        TIE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmission Interrupt Enable 4"]
    #[inline(always)]
    pub fn tie4(&self) -> TIE4_R {
        TIE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmission Interrupt Enable 5"]
    #[inline(always)]
    pub fn tie5(&self) -> TIE5_R {
        TIE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmission Interrupt Enable 6"]
    #[inline(always)]
    pub fn tie6(&self) -> TIE6_R {
        TIE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmission Interrupt Enable 7"]
    #[inline(always)]
    pub fn tie7(&self) -> TIE7_R {
        TIE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmission Interrupt Enable 8"]
    #[inline(always)]
    pub fn tie8(&self) -> TIE8_R {
        TIE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Interrupt Enable 9"]
    #[inline(always)]
    pub fn tie9(&self) -> TIE9_R {
        TIE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Interrupt Enable 10"]
    #[inline(always)]
    pub fn tie10(&self) -> TIE10_R {
        TIE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmission Interrupt Enable 11"]
    #[inline(always)]
    pub fn tie11(&self) -> TIE11_R {
        TIE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmission Interrupt Enable 12"]
    #[inline(always)]
    pub fn tie12(&self) -> TIE12_R {
        TIE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmission Interrupt Enable 13"]
    #[inline(always)]
    pub fn tie13(&self) -> TIE13_R {
        TIE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmission Interrupt Enable 14"]
    #[inline(always)]
    pub fn tie14(&self) -> TIE14_R {
        TIE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmission Interrupt Enable 15"]
    #[inline(always)]
    pub fn tie15(&self) -> TIE15_R {
        TIE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmission Interrupt Enable 16"]
    #[inline(always)]
    pub fn tie16(&self) -> TIE16_R {
        TIE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmission Interrupt Enable 17"]
    #[inline(always)]
    pub fn tie17(&self) -> TIE17_R {
        TIE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmission Interrupt Enable 18"]
    #[inline(always)]
    pub fn tie18(&self) -> TIE18_R {
        TIE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmission Interrupt Enable 19"]
    #[inline(always)]
    pub fn tie19(&self) -> TIE19_R {
        TIE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Transmission Interrupt Enable 20"]
    #[inline(always)]
    pub fn tie20(&self) -> TIE20_R {
        TIE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmission Interrupt Enable 21"]
    #[inline(always)]
    pub fn tie21(&self) -> TIE21_R {
        TIE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Transmission Interrupt Enable 22"]
    #[inline(always)]
    pub fn tie22(&self) -> TIE22_R {
        TIE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmission Interrupt Enable 23"]
    #[inline(always)]
    pub fn tie23(&self) -> TIE23_R {
        TIE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmission Interrupt Enable 24"]
    #[inline(always)]
    pub fn tie24(&self) -> TIE24_R {
        TIE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Transmission Interrupt Enable 25"]
    #[inline(always)]
    pub fn tie25(&self) -> TIE25_R {
        TIE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Transmission Interrupt Enable 26"]
    #[inline(always)]
    pub fn tie26(&self) -> TIE26_R {
        TIE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmission Interrupt Enable 27"]
    #[inline(always)]
    pub fn tie27(&self) -> TIE27_R {
        TIE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Transmission Interrupt Enable 28"]
    #[inline(always)]
    pub fn tie28(&self) -> TIE28_R {
        TIE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Transmission Interrupt Enable 29"]
    #[inline(always)]
    pub fn tie29(&self) -> TIE29_R {
        TIE29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Transmission Interrupt Enable 30"]
    #[inline(always)]
    pub fn tie30(&self) -> TIE30_R {
        TIE30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transmission Interrupt Enable 31"]
    #[inline(always)]
    pub fn tie31(&self) -> TIE31_R {
        TIE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission Interrupt Enable 0"]
    #[inline(always)]
    pub fn tie0(&mut self) -> TIE0_W {
        TIE0_W { w: self }
    }
    #[doc = "Bit 1 - Transmission Interrupt Enable 1"]
    #[inline(always)]
    pub fn tie1(&mut self) -> TIE1_W {
        TIE1_W { w: self }
    }
    #[doc = "Bit 2 - Transmission Interrupt Enable 2"]
    #[inline(always)]
    pub fn tie2(&mut self) -> TIE2_W {
        TIE2_W { w: self }
    }
    #[doc = "Bit 3 - Transmission Interrupt Enable 3"]
    #[inline(always)]
    pub fn tie3(&mut self) -> TIE3_W {
        TIE3_W { w: self }
    }
    #[doc = "Bit 4 - Transmission Interrupt Enable 4"]
    #[inline(always)]
    pub fn tie4(&mut self) -> TIE4_W {
        TIE4_W { w: self }
    }
    #[doc = "Bit 5 - Transmission Interrupt Enable 5"]
    #[inline(always)]
    pub fn tie5(&mut self) -> TIE5_W {
        TIE5_W { w: self }
    }
    #[doc = "Bit 6 - Transmission Interrupt Enable 6"]
    #[inline(always)]
    pub fn tie6(&mut self) -> TIE6_W {
        TIE6_W { w: self }
    }
    #[doc = "Bit 7 - Transmission Interrupt Enable 7"]
    #[inline(always)]
    pub fn tie7(&mut self) -> TIE7_W {
        TIE7_W { w: self }
    }
    #[doc = "Bit 8 - Transmission Interrupt Enable 8"]
    #[inline(always)]
    pub fn tie8(&mut self) -> TIE8_W {
        TIE8_W { w: self }
    }
    #[doc = "Bit 9 - Transmission Interrupt Enable 9"]
    #[inline(always)]
    pub fn tie9(&mut self) -> TIE9_W {
        TIE9_W { w: self }
    }
    #[doc = "Bit 10 - Transmission Interrupt Enable 10"]
    #[inline(always)]
    pub fn tie10(&mut self) -> TIE10_W {
        TIE10_W { w: self }
    }
    #[doc = "Bit 11 - Transmission Interrupt Enable 11"]
    #[inline(always)]
    pub fn tie11(&mut self) -> TIE11_W {
        TIE11_W { w: self }
    }
    #[doc = "Bit 12 - Transmission Interrupt Enable 12"]
    #[inline(always)]
    pub fn tie12(&mut self) -> TIE12_W {
        TIE12_W { w: self }
    }
    #[doc = "Bit 13 - Transmission Interrupt Enable 13"]
    #[inline(always)]
    pub fn tie13(&mut self) -> TIE13_W {
        TIE13_W { w: self }
    }
    #[doc = "Bit 14 - Transmission Interrupt Enable 14"]
    #[inline(always)]
    pub fn tie14(&mut self) -> TIE14_W {
        TIE14_W { w: self }
    }
    #[doc = "Bit 15 - Transmission Interrupt Enable 15"]
    #[inline(always)]
    pub fn tie15(&mut self) -> TIE15_W {
        TIE15_W { w: self }
    }
    #[doc = "Bit 16 - Transmission Interrupt Enable 16"]
    #[inline(always)]
    pub fn tie16(&mut self) -> TIE16_W {
        TIE16_W { w: self }
    }
    #[doc = "Bit 17 - Transmission Interrupt Enable 17"]
    #[inline(always)]
    pub fn tie17(&mut self) -> TIE17_W {
        TIE17_W { w: self }
    }
    #[doc = "Bit 18 - Transmission Interrupt Enable 18"]
    #[inline(always)]
    pub fn tie18(&mut self) -> TIE18_W {
        TIE18_W { w: self }
    }
    #[doc = "Bit 19 - Transmission Interrupt Enable 19"]
    #[inline(always)]
    pub fn tie19(&mut self) -> TIE19_W {
        TIE19_W { w: self }
    }
    #[doc = "Bit 20 - Transmission Interrupt Enable 20"]
    #[inline(always)]
    pub fn tie20(&mut self) -> TIE20_W {
        TIE20_W { w: self }
    }
    #[doc = "Bit 21 - Transmission Interrupt Enable 21"]
    #[inline(always)]
    pub fn tie21(&mut self) -> TIE21_W {
        TIE21_W { w: self }
    }
    #[doc = "Bit 22 - Transmission Interrupt Enable 22"]
    #[inline(always)]
    pub fn tie22(&mut self) -> TIE22_W {
        TIE22_W { w: self }
    }
    #[doc = "Bit 23 - Transmission Interrupt Enable 23"]
    #[inline(always)]
    pub fn tie23(&mut self) -> TIE23_W {
        TIE23_W { w: self }
    }
    #[doc = "Bit 24 - Transmission Interrupt Enable 24"]
    #[inline(always)]
    pub fn tie24(&mut self) -> TIE24_W {
        TIE24_W { w: self }
    }
    #[doc = "Bit 25 - Transmission Interrupt Enable 25"]
    #[inline(always)]
    pub fn tie25(&mut self) -> TIE25_W {
        TIE25_W { w: self }
    }
    #[doc = "Bit 26 - Transmission Interrupt Enable 26"]
    #[inline(always)]
    pub fn tie26(&mut self) -> TIE26_W {
        TIE26_W { w: self }
    }
    #[doc = "Bit 27 - Transmission Interrupt Enable 27"]
    #[inline(always)]
    pub fn tie27(&mut self) -> TIE27_W {
        TIE27_W { w: self }
    }
    #[doc = "Bit 28 - Transmission Interrupt Enable 28"]
    #[inline(always)]
    pub fn tie28(&mut self) -> TIE28_W {
        TIE28_W { w: self }
    }
    #[doc = "Bit 29 - Transmission Interrupt Enable 29"]
    #[inline(always)]
    pub fn tie29(&mut self) -> TIE29_W {
        TIE29_W { w: self }
    }
    #[doc = "Bit 30 - Transmission Interrupt Enable 30"]
    #[inline(always)]
    pub fn tie30(&mut self) -> TIE30_W {
        TIE30_W { w: self }
    }
    #[doc = "Bit 31 - Transmission Interrupt Enable 31"]
    #[inline(always)]
    pub fn tie31(&mut self) -> TIE31_W {
        TIE31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Transmission Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbtie](index.html) module"]
pub struct TXBTIE_SPEC;
impl crate::RegisterSpec for TXBTIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbtie::R](R) reader structure"]
impl crate::Readable for TXBTIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbtie::W](W) writer structure"]
impl crate::Writable for TXBTIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBTIE to value 0"]
impl crate::Resettable for TXBTIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
