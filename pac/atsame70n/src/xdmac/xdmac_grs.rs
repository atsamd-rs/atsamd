#[doc = "Register `XDMAC_GRS` reader"]
pub struct R(crate::R<XDMAC_GRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_GRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_GRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_GRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XDMAC_GRS` writer"]
pub struct W(crate::W<XDMAC_GRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_GRS_SPEC>;
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
impl From<crate::W<XDMAC_GRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_GRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS0` reader - XDMAC Channel 0 Read Suspend Bit"]
pub struct RS0_R(crate::FieldReader<bool, bool>);
impl RS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS0` writer - XDMAC Channel 0 Read Suspend Bit"]
pub struct RS0_W<'a> {
    w: &'a mut W,
}
impl<'a> RS0_W<'a> {
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
#[doc = "Field `RS1` reader - XDMAC Channel 1 Read Suspend Bit"]
pub struct RS1_R(crate::FieldReader<bool, bool>);
impl RS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS1` writer - XDMAC Channel 1 Read Suspend Bit"]
pub struct RS1_W<'a> {
    w: &'a mut W,
}
impl<'a> RS1_W<'a> {
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
#[doc = "Field `RS2` reader - XDMAC Channel 2 Read Suspend Bit"]
pub struct RS2_R(crate::FieldReader<bool, bool>);
impl RS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS2` writer - XDMAC Channel 2 Read Suspend Bit"]
pub struct RS2_W<'a> {
    w: &'a mut W,
}
impl<'a> RS2_W<'a> {
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
#[doc = "Field `RS3` reader - XDMAC Channel 3 Read Suspend Bit"]
pub struct RS3_R(crate::FieldReader<bool, bool>);
impl RS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS3` writer - XDMAC Channel 3 Read Suspend Bit"]
pub struct RS3_W<'a> {
    w: &'a mut W,
}
impl<'a> RS3_W<'a> {
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
#[doc = "Field `RS4` reader - XDMAC Channel 4 Read Suspend Bit"]
pub struct RS4_R(crate::FieldReader<bool, bool>);
impl RS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS4` writer - XDMAC Channel 4 Read Suspend Bit"]
pub struct RS4_W<'a> {
    w: &'a mut W,
}
impl<'a> RS4_W<'a> {
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
#[doc = "Field `RS5` reader - XDMAC Channel 5 Read Suspend Bit"]
pub struct RS5_R(crate::FieldReader<bool, bool>);
impl RS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS5` writer - XDMAC Channel 5 Read Suspend Bit"]
pub struct RS5_W<'a> {
    w: &'a mut W,
}
impl<'a> RS5_W<'a> {
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
#[doc = "Field `RS6` reader - XDMAC Channel 6 Read Suspend Bit"]
pub struct RS6_R(crate::FieldReader<bool, bool>);
impl RS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS6` writer - XDMAC Channel 6 Read Suspend Bit"]
pub struct RS6_W<'a> {
    w: &'a mut W,
}
impl<'a> RS6_W<'a> {
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
#[doc = "Field `RS7` reader - XDMAC Channel 7 Read Suspend Bit"]
pub struct RS7_R(crate::FieldReader<bool, bool>);
impl RS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS7` writer - XDMAC Channel 7 Read Suspend Bit"]
pub struct RS7_W<'a> {
    w: &'a mut W,
}
impl<'a> RS7_W<'a> {
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
#[doc = "Field `RS8` reader - XDMAC Channel 8 Read Suspend Bit"]
pub struct RS8_R(crate::FieldReader<bool, bool>);
impl RS8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS8` writer - XDMAC Channel 8 Read Suspend Bit"]
pub struct RS8_W<'a> {
    w: &'a mut W,
}
impl<'a> RS8_W<'a> {
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
#[doc = "Field `RS9` reader - XDMAC Channel 9 Read Suspend Bit"]
pub struct RS9_R(crate::FieldReader<bool, bool>);
impl RS9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS9` writer - XDMAC Channel 9 Read Suspend Bit"]
pub struct RS9_W<'a> {
    w: &'a mut W,
}
impl<'a> RS9_W<'a> {
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
#[doc = "Field `RS10` reader - XDMAC Channel 10 Read Suspend Bit"]
pub struct RS10_R(crate::FieldReader<bool, bool>);
impl RS10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS10` writer - XDMAC Channel 10 Read Suspend Bit"]
pub struct RS10_W<'a> {
    w: &'a mut W,
}
impl<'a> RS10_W<'a> {
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
#[doc = "Field `RS11` reader - XDMAC Channel 11 Read Suspend Bit"]
pub struct RS11_R(crate::FieldReader<bool, bool>);
impl RS11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS11` writer - XDMAC Channel 11 Read Suspend Bit"]
pub struct RS11_W<'a> {
    w: &'a mut W,
}
impl<'a> RS11_W<'a> {
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
#[doc = "Field `RS12` reader - XDMAC Channel 12 Read Suspend Bit"]
pub struct RS12_R(crate::FieldReader<bool, bool>);
impl RS12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS12` writer - XDMAC Channel 12 Read Suspend Bit"]
pub struct RS12_W<'a> {
    w: &'a mut W,
}
impl<'a> RS12_W<'a> {
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
#[doc = "Field `RS13` reader - XDMAC Channel 13 Read Suspend Bit"]
pub struct RS13_R(crate::FieldReader<bool, bool>);
impl RS13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS13` writer - XDMAC Channel 13 Read Suspend Bit"]
pub struct RS13_W<'a> {
    w: &'a mut W,
}
impl<'a> RS13_W<'a> {
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
#[doc = "Field `RS14` reader - XDMAC Channel 14 Read Suspend Bit"]
pub struct RS14_R(crate::FieldReader<bool, bool>);
impl RS14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS14` writer - XDMAC Channel 14 Read Suspend Bit"]
pub struct RS14_W<'a> {
    w: &'a mut W,
}
impl<'a> RS14_W<'a> {
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
#[doc = "Field `RS15` reader - XDMAC Channel 15 Read Suspend Bit"]
pub struct RS15_R(crate::FieldReader<bool, bool>);
impl RS15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS15` writer - XDMAC Channel 15 Read Suspend Bit"]
pub struct RS15_W<'a> {
    w: &'a mut W,
}
impl<'a> RS15_W<'a> {
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
#[doc = "Field `RS16` reader - XDMAC Channel 16 Read Suspend Bit"]
pub struct RS16_R(crate::FieldReader<bool, bool>);
impl RS16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS16` writer - XDMAC Channel 16 Read Suspend Bit"]
pub struct RS16_W<'a> {
    w: &'a mut W,
}
impl<'a> RS16_W<'a> {
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
#[doc = "Field `RS17` reader - XDMAC Channel 17 Read Suspend Bit"]
pub struct RS17_R(crate::FieldReader<bool, bool>);
impl RS17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS17` writer - XDMAC Channel 17 Read Suspend Bit"]
pub struct RS17_W<'a> {
    w: &'a mut W,
}
impl<'a> RS17_W<'a> {
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
#[doc = "Field `RS18` reader - XDMAC Channel 18 Read Suspend Bit"]
pub struct RS18_R(crate::FieldReader<bool, bool>);
impl RS18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS18` writer - XDMAC Channel 18 Read Suspend Bit"]
pub struct RS18_W<'a> {
    w: &'a mut W,
}
impl<'a> RS18_W<'a> {
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
#[doc = "Field `RS19` reader - XDMAC Channel 19 Read Suspend Bit"]
pub struct RS19_R(crate::FieldReader<bool, bool>);
impl RS19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS19` writer - XDMAC Channel 19 Read Suspend Bit"]
pub struct RS19_W<'a> {
    w: &'a mut W,
}
impl<'a> RS19_W<'a> {
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
#[doc = "Field `RS20` reader - XDMAC Channel 20 Read Suspend Bit"]
pub struct RS20_R(crate::FieldReader<bool, bool>);
impl RS20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS20` writer - XDMAC Channel 20 Read Suspend Bit"]
pub struct RS20_W<'a> {
    w: &'a mut W,
}
impl<'a> RS20_W<'a> {
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
#[doc = "Field `RS21` reader - XDMAC Channel 21 Read Suspend Bit"]
pub struct RS21_R(crate::FieldReader<bool, bool>);
impl RS21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS21` writer - XDMAC Channel 21 Read Suspend Bit"]
pub struct RS21_W<'a> {
    w: &'a mut W,
}
impl<'a> RS21_W<'a> {
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
#[doc = "Field `RS22` reader - XDMAC Channel 22 Read Suspend Bit"]
pub struct RS22_R(crate::FieldReader<bool, bool>);
impl RS22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS22` writer - XDMAC Channel 22 Read Suspend Bit"]
pub struct RS22_W<'a> {
    w: &'a mut W,
}
impl<'a> RS22_W<'a> {
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
#[doc = "Field `RS23` reader - XDMAC Channel 23 Read Suspend Bit"]
pub struct RS23_R(crate::FieldReader<bool, bool>);
impl RS23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS23` writer - XDMAC Channel 23 Read Suspend Bit"]
pub struct RS23_W<'a> {
    w: &'a mut W,
}
impl<'a> RS23_W<'a> {
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
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs0(&self) -> RS0_R {
        RS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs1(&self) -> RS1_R {
        RS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs2(&self) -> RS2_R {
        RS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs3(&self) -> RS3_R {
        RS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs4(&self) -> RS4_R {
        RS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs5(&self) -> RS5_R {
        RS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs6(&self) -> RS6_R {
        RS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs7(&self) -> RS7_R {
        RS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs8(&self) -> RS8_R {
        RS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs9(&self) -> RS9_R {
        RS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs10(&self) -> RS10_R {
        RS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs11(&self) -> RS11_R {
        RS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs12(&self) -> RS12_R {
        RS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs13(&self) -> RS13_R {
        RS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs14(&self) -> RS14_R {
        RS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs15(&self) -> RS15_R {
        RS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs16(&self) -> RS16_R {
        RS16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs17(&self) -> RS17_R {
        RS17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs18(&self) -> RS18_R {
        RS18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs19(&self) -> RS19_R {
        RS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs20(&self) -> RS20_R {
        RS20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs21(&self) -> RS21_R {
        RS21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs22(&self) -> RS22_R {
        RS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs23(&self) -> RS23_R {
        RS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs0(&mut self) -> RS0_W {
        RS0_W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs1(&mut self) -> RS1_W {
        RS1_W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs2(&mut self) -> RS2_W {
        RS2_W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs3(&mut self) -> RS3_W {
        RS3_W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs4(&mut self) -> RS4_W {
        RS4_W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs5(&mut self) -> RS5_W {
        RS5_W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs6(&mut self) -> RS6_W {
        RS6_W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs7(&mut self) -> RS7_W {
        RS7_W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs8(&mut self) -> RS8_W {
        RS8_W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs9(&mut self) -> RS9_W {
        RS9_W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs10(&mut self) -> RS10_W {
        RS10_W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs11(&mut self) -> RS11_W {
        RS11_W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs12(&mut self) -> RS12_W {
        RS12_W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs13(&mut self) -> RS13_W {
        RS13_W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs14(&mut self) -> RS14_W {
        RS14_W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs15(&mut self) -> RS15_W {
        RS15_W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs16(&mut self) -> RS16_W {
        RS16_W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs17(&mut self) -> RS17_W {
        RS17_W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs18(&mut self) -> RS18_W {
        RS18_W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs19(&mut self) -> RS19_W {
        RS19_W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs20(&mut self) -> RS20_W {
        RS20_W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs21(&mut self) -> RS21_W {
        RS21_W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs22(&mut self) -> RS22_W {
        RS22_W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs23(&mut self) -> RS23_W {
        RS23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Read Suspend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_grs](index.html) module"]
pub struct XDMAC_GRS_SPEC;
impl crate::RegisterSpec for XDMAC_GRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_grs::R](R) reader structure"]
impl crate::Readable for XDMAC_GRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xdmac_grs::W](W) writer structure"]
impl crate::Writable for XDMAC_GRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_GRS to value 0"]
impl crate::Resettable for XDMAC_GRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
