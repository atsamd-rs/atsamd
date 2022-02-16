#[doc = "Register `MCAN_TXBAR` reader"]
pub struct R(crate::R<MCAN_TXBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_TXBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_TXBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_TXBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_TXBAR` writer"]
pub struct W(crate::W<MCAN_TXBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_TXBAR_SPEC>;
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
impl From<crate::W<MCAN_TXBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_TXBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR0` reader - Add Request for Transmit Buffer 0"]
pub struct AR0_R(crate::FieldReader<bool, bool>);
impl AR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR0` writer - Add Request for Transmit Buffer 0"]
pub struct AR0_W<'a> {
    w: &'a mut W,
}
impl<'a> AR0_W<'a> {
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
#[doc = "Field `AR1` reader - Add Request for Transmit Buffer 1"]
pub struct AR1_R(crate::FieldReader<bool, bool>);
impl AR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR1` writer - Add Request for Transmit Buffer 1"]
pub struct AR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AR1_W<'a> {
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
#[doc = "Field `AR2` reader - Add Request for Transmit Buffer 2"]
pub struct AR2_R(crate::FieldReader<bool, bool>);
impl AR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR2` writer - Add Request for Transmit Buffer 2"]
pub struct AR2_W<'a> {
    w: &'a mut W,
}
impl<'a> AR2_W<'a> {
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
#[doc = "Field `AR3` reader - Add Request for Transmit Buffer 3"]
pub struct AR3_R(crate::FieldReader<bool, bool>);
impl AR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR3` writer - Add Request for Transmit Buffer 3"]
pub struct AR3_W<'a> {
    w: &'a mut W,
}
impl<'a> AR3_W<'a> {
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
#[doc = "Field `AR4` reader - Add Request for Transmit Buffer 4"]
pub struct AR4_R(crate::FieldReader<bool, bool>);
impl AR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR4` writer - Add Request for Transmit Buffer 4"]
pub struct AR4_W<'a> {
    w: &'a mut W,
}
impl<'a> AR4_W<'a> {
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
#[doc = "Field `AR5` reader - Add Request for Transmit Buffer 5"]
pub struct AR5_R(crate::FieldReader<bool, bool>);
impl AR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR5` writer - Add Request for Transmit Buffer 5"]
pub struct AR5_W<'a> {
    w: &'a mut W,
}
impl<'a> AR5_W<'a> {
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
#[doc = "Field `AR6` reader - Add Request for Transmit Buffer 6"]
pub struct AR6_R(crate::FieldReader<bool, bool>);
impl AR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR6` writer - Add Request for Transmit Buffer 6"]
pub struct AR6_W<'a> {
    w: &'a mut W,
}
impl<'a> AR6_W<'a> {
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
#[doc = "Field `AR7` reader - Add Request for Transmit Buffer 7"]
pub struct AR7_R(crate::FieldReader<bool, bool>);
impl AR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR7` writer - Add Request for Transmit Buffer 7"]
pub struct AR7_W<'a> {
    w: &'a mut W,
}
impl<'a> AR7_W<'a> {
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
#[doc = "Field `AR8` reader - Add Request for Transmit Buffer 8"]
pub struct AR8_R(crate::FieldReader<bool, bool>);
impl AR8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR8` writer - Add Request for Transmit Buffer 8"]
pub struct AR8_W<'a> {
    w: &'a mut W,
}
impl<'a> AR8_W<'a> {
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
#[doc = "Field `AR9` reader - Add Request for Transmit Buffer 9"]
pub struct AR9_R(crate::FieldReader<bool, bool>);
impl AR9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR9` writer - Add Request for Transmit Buffer 9"]
pub struct AR9_W<'a> {
    w: &'a mut W,
}
impl<'a> AR9_W<'a> {
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
#[doc = "Field `AR10` reader - Add Request for Transmit Buffer 10"]
pub struct AR10_R(crate::FieldReader<bool, bool>);
impl AR10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR10` writer - Add Request for Transmit Buffer 10"]
pub struct AR10_W<'a> {
    w: &'a mut W,
}
impl<'a> AR10_W<'a> {
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
#[doc = "Field `AR11` reader - Add Request for Transmit Buffer 11"]
pub struct AR11_R(crate::FieldReader<bool, bool>);
impl AR11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR11` writer - Add Request for Transmit Buffer 11"]
pub struct AR11_W<'a> {
    w: &'a mut W,
}
impl<'a> AR11_W<'a> {
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
#[doc = "Field `AR12` reader - Add Request for Transmit Buffer 12"]
pub struct AR12_R(crate::FieldReader<bool, bool>);
impl AR12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR12` writer - Add Request for Transmit Buffer 12"]
pub struct AR12_W<'a> {
    w: &'a mut W,
}
impl<'a> AR12_W<'a> {
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
#[doc = "Field `AR13` reader - Add Request for Transmit Buffer 13"]
pub struct AR13_R(crate::FieldReader<bool, bool>);
impl AR13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR13` writer - Add Request for Transmit Buffer 13"]
pub struct AR13_W<'a> {
    w: &'a mut W,
}
impl<'a> AR13_W<'a> {
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
#[doc = "Field `AR14` reader - Add Request for Transmit Buffer 14"]
pub struct AR14_R(crate::FieldReader<bool, bool>);
impl AR14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR14` writer - Add Request for Transmit Buffer 14"]
pub struct AR14_W<'a> {
    w: &'a mut W,
}
impl<'a> AR14_W<'a> {
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
#[doc = "Field `AR15` reader - Add Request for Transmit Buffer 15"]
pub struct AR15_R(crate::FieldReader<bool, bool>);
impl AR15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR15` writer - Add Request for Transmit Buffer 15"]
pub struct AR15_W<'a> {
    w: &'a mut W,
}
impl<'a> AR15_W<'a> {
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
#[doc = "Field `AR16` reader - Add Request for Transmit Buffer 16"]
pub struct AR16_R(crate::FieldReader<bool, bool>);
impl AR16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR16` writer - Add Request for Transmit Buffer 16"]
pub struct AR16_W<'a> {
    w: &'a mut W,
}
impl<'a> AR16_W<'a> {
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
#[doc = "Field `AR17` reader - Add Request for Transmit Buffer 17"]
pub struct AR17_R(crate::FieldReader<bool, bool>);
impl AR17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR17` writer - Add Request for Transmit Buffer 17"]
pub struct AR17_W<'a> {
    w: &'a mut W,
}
impl<'a> AR17_W<'a> {
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
#[doc = "Field `AR18` reader - Add Request for Transmit Buffer 18"]
pub struct AR18_R(crate::FieldReader<bool, bool>);
impl AR18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR18` writer - Add Request for Transmit Buffer 18"]
pub struct AR18_W<'a> {
    w: &'a mut W,
}
impl<'a> AR18_W<'a> {
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
#[doc = "Field `AR19` reader - Add Request for Transmit Buffer 19"]
pub struct AR19_R(crate::FieldReader<bool, bool>);
impl AR19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR19` writer - Add Request for Transmit Buffer 19"]
pub struct AR19_W<'a> {
    w: &'a mut W,
}
impl<'a> AR19_W<'a> {
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
#[doc = "Field `AR20` reader - Add Request for Transmit Buffer 20"]
pub struct AR20_R(crate::FieldReader<bool, bool>);
impl AR20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR20` writer - Add Request for Transmit Buffer 20"]
pub struct AR20_W<'a> {
    w: &'a mut W,
}
impl<'a> AR20_W<'a> {
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
#[doc = "Field `AR21` reader - Add Request for Transmit Buffer 21"]
pub struct AR21_R(crate::FieldReader<bool, bool>);
impl AR21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR21` writer - Add Request for Transmit Buffer 21"]
pub struct AR21_W<'a> {
    w: &'a mut W,
}
impl<'a> AR21_W<'a> {
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
#[doc = "Field `AR22` reader - Add Request for Transmit Buffer 22"]
pub struct AR22_R(crate::FieldReader<bool, bool>);
impl AR22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR22` writer - Add Request for Transmit Buffer 22"]
pub struct AR22_W<'a> {
    w: &'a mut W,
}
impl<'a> AR22_W<'a> {
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
#[doc = "Field `AR23` reader - Add Request for Transmit Buffer 23"]
pub struct AR23_R(crate::FieldReader<bool, bool>);
impl AR23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR23` writer - Add Request for Transmit Buffer 23"]
pub struct AR23_W<'a> {
    w: &'a mut W,
}
impl<'a> AR23_W<'a> {
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
#[doc = "Field `AR24` reader - Add Request for Transmit Buffer 24"]
pub struct AR24_R(crate::FieldReader<bool, bool>);
impl AR24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR24` writer - Add Request for Transmit Buffer 24"]
pub struct AR24_W<'a> {
    w: &'a mut W,
}
impl<'a> AR24_W<'a> {
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
#[doc = "Field `AR25` reader - Add Request for Transmit Buffer 25"]
pub struct AR25_R(crate::FieldReader<bool, bool>);
impl AR25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR25` writer - Add Request for Transmit Buffer 25"]
pub struct AR25_W<'a> {
    w: &'a mut W,
}
impl<'a> AR25_W<'a> {
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
#[doc = "Field `AR26` reader - Add Request for Transmit Buffer 26"]
pub struct AR26_R(crate::FieldReader<bool, bool>);
impl AR26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR26` writer - Add Request for Transmit Buffer 26"]
pub struct AR26_W<'a> {
    w: &'a mut W,
}
impl<'a> AR26_W<'a> {
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
#[doc = "Field `AR27` reader - Add Request for Transmit Buffer 27"]
pub struct AR27_R(crate::FieldReader<bool, bool>);
impl AR27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR27` writer - Add Request for Transmit Buffer 27"]
pub struct AR27_W<'a> {
    w: &'a mut W,
}
impl<'a> AR27_W<'a> {
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
#[doc = "Field `AR28` reader - Add Request for Transmit Buffer 28"]
pub struct AR28_R(crate::FieldReader<bool, bool>);
impl AR28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR28` writer - Add Request for Transmit Buffer 28"]
pub struct AR28_W<'a> {
    w: &'a mut W,
}
impl<'a> AR28_W<'a> {
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
#[doc = "Field `AR29` reader - Add Request for Transmit Buffer 29"]
pub struct AR29_R(crate::FieldReader<bool, bool>);
impl AR29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR29` writer - Add Request for Transmit Buffer 29"]
pub struct AR29_W<'a> {
    w: &'a mut W,
}
impl<'a> AR29_W<'a> {
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
#[doc = "Field `AR30` reader - Add Request for Transmit Buffer 30"]
pub struct AR30_R(crate::FieldReader<bool, bool>);
impl AR30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR30` writer - Add Request for Transmit Buffer 30"]
pub struct AR30_W<'a> {
    w: &'a mut W,
}
impl<'a> AR30_W<'a> {
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
#[doc = "Field `AR31` reader - Add Request for Transmit Buffer 31"]
pub struct AR31_R(crate::FieldReader<bool, bool>);
impl AR31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AR31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR31` writer - Add Request for Transmit Buffer 31"]
pub struct AR31_W<'a> {
    w: &'a mut W,
}
impl<'a> AR31_W<'a> {
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
    #[doc = "Bit 0 - Add Request for Transmit Buffer 0"]
    #[inline(always)]
    pub fn ar0(&self) -> AR0_R {
        AR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Add Request for Transmit Buffer 1"]
    #[inline(always)]
    pub fn ar1(&self) -> AR1_R {
        AR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Add Request for Transmit Buffer 2"]
    #[inline(always)]
    pub fn ar2(&self) -> AR2_R {
        AR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Add Request for Transmit Buffer 3"]
    #[inline(always)]
    pub fn ar3(&self) -> AR3_R {
        AR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Add Request for Transmit Buffer 4"]
    #[inline(always)]
    pub fn ar4(&self) -> AR4_R {
        AR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Add Request for Transmit Buffer 5"]
    #[inline(always)]
    pub fn ar5(&self) -> AR5_R {
        AR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Add Request for Transmit Buffer 6"]
    #[inline(always)]
    pub fn ar6(&self) -> AR6_R {
        AR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Add Request for Transmit Buffer 7"]
    #[inline(always)]
    pub fn ar7(&self) -> AR7_R {
        AR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Add Request for Transmit Buffer 8"]
    #[inline(always)]
    pub fn ar8(&self) -> AR8_R {
        AR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Add Request for Transmit Buffer 9"]
    #[inline(always)]
    pub fn ar9(&self) -> AR9_R {
        AR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Add Request for Transmit Buffer 10"]
    #[inline(always)]
    pub fn ar10(&self) -> AR10_R {
        AR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Add Request for Transmit Buffer 11"]
    #[inline(always)]
    pub fn ar11(&self) -> AR11_R {
        AR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Add Request for Transmit Buffer 12"]
    #[inline(always)]
    pub fn ar12(&self) -> AR12_R {
        AR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Add Request for Transmit Buffer 13"]
    #[inline(always)]
    pub fn ar13(&self) -> AR13_R {
        AR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Add Request for Transmit Buffer 14"]
    #[inline(always)]
    pub fn ar14(&self) -> AR14_R {
        AR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Add Request for Transmit Buffer 15"]
    #[inline(always)]
    pub fn ar15(&self) -> AR15_R {
        AR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Add Request for Transmit Buffer 16"]
    #[inline(always)]
    pub fn ar16(&self) -> AR16_R {
        AR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Add Request for Transmit Buffer 17"]
    #[inline(always)]
    pub fn ar17(&self) -> AR17_R {
        AR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Add Request for Transmit Buffer 18"]
    #[inline(always)]
    pub fn ar18(&self) -> AR18_R {
        AR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Add Request for Transmit Buffer 19"]
    #[inline(always)]
    pub fn ar19(&self) -> AR19_R {
        AR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Add Request for Transmit Buffer 20"]
    #[inline(always)]
    pub fn ar20(&self) -> AR20_R {
        AR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Add Request for Transmit Buffer 21"]
    #[inline(always)]
    pub fn ar21(&self) -> AR21_R {
        AR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Add Request for Transmit Buffer 22"]
    #[inline(always)]
    pub fn ar22(&self) -> AR22_R {
        AR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Add Request for Transmit Buffer 23"]
    #[inline(always)]
    pub fn ar23(&self) -> AR23_R {
        AR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Add Request for Transmit Buffer 24"]
    #[inline(always)]
    pub fn ar24(&self) -> AR24_R {
        AR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Add Request for Transmit Buffer 25"]
    #[inline(always)]
    pub fn ar25(&self) -> AR25_R {
        AR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Add Request for Transmit Buffer 26"]
    #[inline(always)]
    pub fn ar26(&self) -> AR26_R {
        AR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Add Request for Transmit Buffer 27"]
    #[inline(always)]
    pub fn ar27(&self) -> AR27_R {
        AR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Add Request for Transmit Buffer 28"]
    #[inline(always)]
    pub fn ar28(&self) -> AR28_R {
        AR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Add Request for Transmit Buffer 29"]
    #[inline(always)]
    pub fn ar29(&self) -> AR29_R {
        AR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Add Request for Transmit Buffer 30"]
    #[inline(always)]
    pub fn ar30(&self) -> AR30_R {
        AR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Add Request for Transmit Buffer 31"]
    #[inline(always)]
    pub fn ar31(&self) -> AR31_R {
        AR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Add Request for Transmit Buffer 0"]
    #[inline(always)]
    pub fn ar0(&mut self) -> AR0_W {
        AR0_W { w: self }
    }
    #[doc = "Bit 1 - Add Request for Transmit Buffer 1"]
    #[inline(always)]
    pub fn ar1(&mut self) -> AR1_W {
        AR1_W { w: self }
    }
    #[doc = "Bit 2 - Add Request for Transmit Buffer 2"]
    #[inline(always)]
    pub fn ar2(&mut self) -> AR2_W {
        AR2_W { w: self }
    }
    #[doc = "Bit 3 - Add Request for Transmit Buffer 3"]
    #[inline(always)]
    pub fn ar3(&mut self) -> AR3_W {
        AR3_W { w: self }
    }
    #[doc = "Bit 4 - Add Request for Transmit Buffer 4"]
    #[inline(always)]
    pub fn ar4(&mut self) -> AR4_W {
        AR4_W { w: self }
    }
    #[doc = "Bit 5 - Add Request for Transmit Buffer 5"]
    #[inline(always)]
    pub fn ar5(&mut self) -> AR5_W {
        AR5_W { w: self }
    }
    #[doc = "Bit 6 - Add Request for Transmit Buffer 6"]
    #[inline(always)]
    pub fn ar6(&mut self) -> AR6_W {
        AR6_W { w: self }
    }
    #[doc = "Bit 7 - Add Request for Transmit Buffer 7"]
    #[inline(always)]
    pub fn ar7(&mut self) -> AR7_W {
        AR7_W { w: self }
    }
    #[doc = "Bit 8 - Add Request for Transmit Buffer 8"]
    #[inline(always)]
    pub fn ar8(&mut self) -> AR8_W {
        AR8_W { w: self }
    }
    #[doc = "Bit 9 - Add Request for Transmit Buffer 9"]
    #[inline(always)]
    pub fn ar9(&mut self) -> AR9_W {
        AR9_W { w: self }
    }
    #[doc = "Bit 10 - Add Request for Transmit Buffer 10"]
    #[inline(always)]
    pub fn ar10(&mut self) -> AR10_W {
        AR10_W { w: self }
    }
    #[doc = "Bit 11 - Add Request for Transmit Buffer 11"]
    #[inline(always)]
    pub fn ar11(&mut self) -> AR11_W {
        AR11_W { w: self }
    }
    #[doc = "Bit 12 - Add Request for Transmit Buffer 12"]
    #[inline(always)]
    pub fn ar12(&mut self) -> AR12_W {
        AR12_W { w: self }
    }
    #[doc = "Bit 13 - Add Request for Transmit Buffer 13"]
    #[inline(always)]
    pub fn ar13(&mut self) -> AR13_W {
        AR13_W { w: self }
    }
    #[doc = "Bit 14 - Add Request for Transmit Buffer 14"]
    #[inline(always)]
    pub fn ar14(&mut self) -> AR14_W {
        AR14_W { w: self }
    }
    #[doc = "Bit 15 - Add Request for Transmit Buffer 15"]
    #[inline(always)]
    pub fn ar15(&mut self) -> AR15_W {
        AR15_W { w: self }
    }
    #[doc = "Bit 16 - Add Request for Transmit Buffer 16"]
    #[inline(always)]
    pub fn ar16(&mut self) -> AR16_W {
        AR16_W { w: self }
    }
    #[doc = "Bit 17 - Add Request for Transmit Buffer 17"]
    #[inline(always)]
    pub fn ar17(&mut self) -> AR17_W {
        AR17_W { w: self }
    }
    #[doc = "Bit 18 - Add Request for Transmit Buffer 18"]
    #[inline(always)]
    pub fn ar18(&mut self) -> AR18_W {
        AR18_W { w: self }
    }
    #[doc = "Bit 19 - Add Request for Transmit Buffer 19"]
    #[inline(always)]
    pub fn ar19(&mut self) -> AR19_W {
        AR19_W { w: self }
    }
    #[doc = "Bit 20 - Add Request for Transmit Buffer 20"]
    #[inline(always)]
    pub fn ar20(&mut self) -> AR20_W {
        AR20_W { w: self }
    }
    #[doc = "Bit 21 - Add Request for Transmit Buffer 21"]
    #[inline(always)]
    pub fn ar21(&mut self) -> AR21_W {
        AR21_W { w: self }
    }
    #[doc = "Bit 22 - Add Request for Transmit Buffer 22"]
    #[inline(always)]
    pub fn ar22(&mut self) -> AR22_W {
        AR22_W { w: self }
    }
    #[doc = "Bit 23 - Add Request for Transmit Buffer 23"]
    #[inline(always)]
    pub fn ar23(&mut self) -> AR23_W {
        AR23_W { w: self }
    }
    #[doc = "Bit 24 - Add Request for Transmit Buffer 24"]
    #[inline(always)]
    pub fn ar24(&mut self) -> AR24_W {
        AR24_W { w: self }
    }
    #[doc = "Bit 25 - Add Request for Transmit Buffer 25"]
    #[inline(always)]
    pub fn ar25(&mut self) -> AR25_W {
        AR25_W { w: self }
    }
    #[doc = "Bit 26 - Add Request for Transmit Buffer 26"]
    #[inline(always)]
    pub fn ar26(&mut self) -> AR26_W {
        AR26_W { w: self }
    }
    #[doc = "Bit 27 - Add Request for Transmit Buffer 27"]
    #[inline(always)]
    pub fn ar27(&mut self) -> AR27_W {
        AR27_W { w: self }
    }
    #[doc = "Bit 28 - Add Request for Transmit Buffer 28"]
    #[inline(always)]
    pub fn ar28(&mut self) -> AR28_W {
        AR28_W { w: self }
    }
    #[doc = "Bit 29 - Add Request for Transmit Buffer 29"]
    #[inline(always)]
    pub fn ar29(&mut self) -> AR29_W {
        AR29_W { w: self }
    }
    #[doc = "Bit 30 - Add Request for Transmit Buffer 30"]
    #[inline(always)]
    pub fn ar30(&mut self) -> AR30_W {
        AR30_W { w: self }
    }
    #[doc = "Bit 31 - Add Request for Transmit Buffer 31"]
    #[inline(always)]
    pub fn ar31(&mut self) -> AR31_W {
        AR31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Buffer Add Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txbar](index.html) module"]
pub struct MCAN_TXBAR_SPEC;
impl crate::RegisterSpec for MCAN_TXBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_txbar::R](R) reader structure"]
impl crate::Readable for MCAN_TXBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_txbar::W](W) writer structure"]
impl crate::Writable for MCAN_TXBAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_TXBAR to value 0"]
impl crate::Resettable for MCAN_TXBAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
