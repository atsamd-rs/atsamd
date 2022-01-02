#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVR0` reader - Channel 0 Overrun Interrupt Enable"]
pub struct OVR0_R(crate::FieldReader<bool, bool>);
impl OVR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR0` writer - Channel 0 Overrun Interrupt Enable"]
pub struct OVR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR0_W<'a> {
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
#[doc = "Field `OVR1` reader - Channel 1 Overrun Interrupt Enable"]
pub struct OVR1_R(crate::FieldReader<bool, bool>);
impl OVR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR1` writer - Channel 1 Overrun Interrupt Enable"]
pub struct OVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR1_W<'a> {
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
#[doc = "Field `OVR2` reader - Channel 2 Overrun Interrupt Enable"]
pub struct OVR2_R(crate::FieldReader<bool, bool>);
impl OVR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR2` writer - Channel 2 Overrun Interrupt Enable"]
pub struct OVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR2_W<'a> {
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
#[doc = "Field `OVR3` reader - Channel 3 Overrun Interrupt Enable"]
pub struct OVR3_R(crate::FieldReader<bool, bool>);
impl OVR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR3` writer - Channel 3 Overrun Interrupt Enable"]
pub struct OVR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR3_W<'a> {
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
#[doc = "Field `OVR4` reader - Channel 4 Overrun Interrupt Enable"]
pub struct OVR4_R(crate::FieldReader<bool, bool>);
impl OVR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR4` writer - Channel 4 Overrun Interrupt Enable"]
pub struct OVR4_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR4_W<'a> {
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
#[doc = "Field `OVR5` reader - Channel 5 Overrun Interrupt Enable"]
pub struct OVR5_R(crate::FieldReader<bool, bool>);
impl OVR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR5` writer - Channel 5 Overrun Interrupt Enable"]
pub struct OVR5_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR5_W<'a> {
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
#[doc = "Field `OVR6` reader - Channel 6 Overrun Interrupt Enable"]
pub struct OVR6_R(crate::FieldReader<bool, bool>);
impl OVR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR6` writer - Channel 6 Overrun Interrupt Enable"]
pub struct OVR6_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR6_W<'a> {
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
#[doc = "Field `OVR7` reader - Channel 7 Overrun Interrupt Enable"]
pub struct OVR7_R(crate::FieldReader<bool, bool>);
impl OVR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR7` writer - Channel 7 Overrun Interrupt Enable"]
pub struct OVR7_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR7_W<'a> {
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
#[doc = "Field `EVD0` reader - Channel 0 Event Detection Interrupt Enable"]
pub struct EVD0_R(crate::FieldReader<bool, bool>);
impl EVD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD0` writer - Channel 0 Event Detection Interrupt Enable"]
pub struct EVD0_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD0_W<'a> {
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
#[doc = "Field `EVD1` reader - Channel 1 Event Detection Interrupt Enable"]
pub struct EVD1_R(crate::FieldReader<bool, bool>);
impl EVD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD1` writer - Channel 1 Event Detection Interrupt Enable"]
pub struct EVD1_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD1_W<'a> {
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
#[doc = "Field `EVD2` reader - Channel 2 Event Detection Interrupt Enable"]
pub struct EVD2_R(crate::FieldReader<bool, bool>);
impl EVD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD2` writer - Channel 2 Event Detection Interrupt Enable"]
pub struct EVD2_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD2_W<'a> {
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
#[doc = "Field `EVD3` reader - Channel 3 Event Detection Interrupt Enable"]
pub struct EVD3_R(crate::FieldReader<bool, bool>);
impl EVD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD3` writer - Channel 3 Event Detection Interrupt Enable"]
pub struct EVD3_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD3_W<'a> {
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
#[doc = "Field `EVD4` reader - Channel 4 Event Detection Interrupt Enable"]
pub struct EVD4_R(crate::FieldReader<bool, bool>);
impl EVD4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD4` writer - Channel 4 Event Detection Interrupt Enable"]
pub struct EVD4_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD4_W<'a> {
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
#[doc = "Field `EVD5` reader - Channel 5 Event Detection Interrupt Enable"]
pub struct EVD5_R(crate::FieldReader<bool, bool>);
impl EVD5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD5` writer - Channel 5 Event Detection Interrupt Enable"]
pub struct EVD5_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD5_W<'a> {
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
#[doc = "Field `EVD6` reader - Channel 6 Event Detection Interrupt Enable"]
pub struct EVD6_R(crate::FieldReader<bool, bool>);
impl EVD6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD6` writer - Channel 6 Event Detection Interrupt Enable"]
pub struct EVD6_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD6_W<'a> {
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
#[doc = "Field `EVD7` reader - Channel 7 Event Detection Interrupt Enable"]
pub struct EVD7_R(crate::FieldReader<bool, bool>);
impl EVD7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD7` writer - Channel 7 Event Detection Interrupt Enable"]
pub struct EVD7_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD7_W<'a> {
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
#[doc = "Field `OVR8` reader - Channel 8 Overrun Interrupt Enable"]
pub struct OVR8_R(crate::FieldReader<bool, bool>);
impl OVR8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR8` writer - Channel 8 Overrun Interrupt Enable"]
pub struct OVR8_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR8_W<'a> {
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
#[doc = "Field `OVR9` reader - Channel 9 Overrun Interrupt Enable"]
pub struct OVR9_R(crate::FieldReader<bool, bool>);
impl OVR9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR9` writer - Channel 9 Overrun Interrupt Enable"]
pub struct OVR9_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR9_W<'a> {
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
#[doc = "Field `OVR10` reader - Channel 10 Overrun Interrupt Enable"]
pub struct OVR10_R(crate::FieldReader<bool, bool>);
impl OVR10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR10` writer - Channel 10 Overrun Interrupt Enable"]
pub struct OVR10_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR10_W<'a> {
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
#[doc = "Field `OVR11` reader - Channel 11 Overrun Interrupt Enable"]
pub struct OVR11_R(crate::FieldReader<bool, bool>);
impl OVR11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR11` writer - Channel 11 Overrun Interrupt Enable"]
pub struct OVR11_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR11_W<'a> {
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
#[doc = "Field `EVD8` reader - Channel 8 Event Detection Interrupt Enable"]
pub struct EVD8_R(crate::FieldReader<bool, bool>);
impl EVD8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD8` writer - Channel 8 Event Detection Interrupt Enable"]
pub struct EVD8_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD8_W<'a> {
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
#[doc = "Field `EVD9` reader - Channel 9 Event Detection Interrupt Enable"]
pub struct EVD9_R(crate::FieldReader<bool, bool>);
impl EVD9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD9` writer - Channel 9 Event Detection Interrupt Enable"]
pub struct EVD9_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD9_W<'a> {
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
#[doc = "Field `EVD10` reader - Channel 10 Event Detection Interrupt Enable"]
pub struct EVD10_R(crate::FieldReader<bool, bool>);
impl EVD10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD10` writer - Channel 10 Event Detection Interrupt Enable"]
pub struct EVD10_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD10_W<'a> {
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
#[doc = "Field `EVD11` reader - Channel 11 Event Detection Interrupt Enable"]
pub struct EVD11_R(crate::FieldReader<bool, bool>);
impl EVD11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVD11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVD11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVD11` writer - Channel 11 Event Detection Interrupt Enable"]
pub struct EVD11_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD11_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr0(&self) -> OVR0_R {
        OVR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr4(&self) -> OVR4_R {
        OVR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr5(&self) -> OVR5_R {
        OVR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr6(&self) -> OVR6_R {
        OVR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr7(&self) -> OVR7_R {
        OVR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd0(&self) -> EVD0_R {
        EVD0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd1(&self) -> EVD1_R {
        EVD1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd2(&self) -> EVD2_R {
        EVD2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd3(&self) -> EVD3_R {
        EVD3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd4(&self) -> EVD4_R {
        EVD4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd5(&self) -> EVD5_R {
        EVD5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd6(&self) -> EVD6_R {
        EVD6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd7(&self) -> EVD7_R {
        EVD7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 8 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr8(&self) -> OVR8_R {
        OVR8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 9 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr9(&self) -> OVR9_R {
        OVR9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 10 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr10(&self) -> OVR10_R {
        OVR10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 11 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr11(&self) -> OVR11_R {
        OVR11_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel 8 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd8(&self) -> EVD8_R {
        EVD8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel 9 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd9(&self) -> EVD9_R {
        EVD9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel 10 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd10(&self) -> EVD10_R {
        EVD10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel 11 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd11(&self) -> EVD11_R {
        EVD11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr0(&mut self) -> OVR0_W {
        OVR0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr1(&mut self) -> OVR1_W {
        OVR1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr2(&mut self) -> OVR2_W {
        OVR2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr3(&mut self) -> OVR3_W {
        OVR3_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr4(&mut self) -> OVR4_W {
        OVR4_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr5(&mut self) -> OVR5_W {
        OVR5_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr6(&mut self) -> OVR6_W {
        OVR6_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr7(&mut self) -> OVR7_W {
        OVR7_W { w: self }
    }
    #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd0(&mut self) -> EVD0_W {
        EVD0_W { w: self }
    }
    #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd1(&mut self) -> EVD1_W {
        EVD1_W { w: self }
    }
    #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd2(&mut self) -> EVD2_W {
        EVD2_W { w: self }
    }
    #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd3(&mut self) -> EVD3_W {
        EVD3_W { w: self }
    }
    #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd4(&mut self) -> EVD4_W {
        EVD4_W { w: self }
    }
    #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd5(&mut self) -> EVD5_W {
        EVD5_W { w: self }
    }
    #[doc = "Bit 14 - Channel 6 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd6(&mut self) -> EVD6_W {
        EVD6_W { w: self }
    }
    #[doc = "Bit 15 - Channel 7 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd7(&mut self) -> EVD7_W {
        EVD7_W { w: self }
    }
    #[doc = "Bit 16 - Channel 8 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr8(&mut self) -> OVR8_W {
        OVR8_W { w: self }
    }
    #[doc = "Bit 17 - Channel 9 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr9(&mut self) -> OVR9_W {
        OVR9_W { w: self }
    }
    #[doc = "Bit 18 - Channel 10 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr10(&mut self) -> OVR10_W {
        OVR10_W { w: self }
    }
    #[doc = "Bit 19 - Channel 11 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr11(&mut self) -> OVR11_W {
        OVR11_W { w: self }
    }
    #[doc = "Bit 24 - Channel 8 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd8(&mut self) -> EVD8_W {
        EVD8_W { w: self }
    }
    #[doc = "Bit 25 - Channel 9 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd9(&mut self) -> EVD9_W {
        EVD9_W { w: self }
    }
    #[doc = "Bit 26 - Channel 10 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd10(&mut self) -> EVD10_W {
        EVD10_W { w: self }
    }
    #[doc = "Bit 27 - Channel 11 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd11(&mut self) -> EVD11_W {
        EVD11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
