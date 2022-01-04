#[doc = "Register `SWTRIGCTRL` reader"]
pub struct R(crate::R<SWTRIGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWTRIGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWTRIGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWTRIGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWTRIGCTRL` writer"]
pub struct W(crate::W<SWTRIGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRIGCTRL_SPEC>;
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
impl From<crate::W<SWTRIGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRIGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWTRIG0` reader - Channel 0 Software Trigger"]
pub struct SWTRIG0_R(crate::FieldReader<bool, bool>);
impl SWTRIG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG0` writer - Channel 0 Software Trigger"]
pub struct SWTRIG0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG1` reader - Channel 1 Software Trigger"]
pub struct SWTRIG1_R(crate::FieldReader<bool, bool>);
impl SWTRIG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG1` writer - Channel 1 Software Trigger"]
pub struct SWTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG2` reader - Channel 2 Software Trigger"]
pub struct SWTRIG2_R(crate::FieldReader<bool, bool>);
impl SWTRIG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG2` writer - Channel 2 Software Trigger"]
pub struct SWTRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG3` reader - Channel 3 Software Trigger"]
pub struct SWTRIG3_R(crate::FieldReader<bool, bool>);
impl SWTRIG3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG3` writer - Channel 3 Software Trigger"]
pub struct SWTRIG3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG4` reader - Channel 4 Software Trigger"]
pub struct SWTRIG4_R(crate::FieldReader<bool, bool>);
impl SWTRIG4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG4` writer - Channel 4 Software Trigger"]
pub struct SWTRIG4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG5` reader - Channel 5 Software Trigger"]
pub struct SWTRIG5_R(crate::FieldReader<bool, bool>);
impl SWTRIG5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG5` writer - Channel 5 Software Trigger"]
pub struct SWTRIG5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG6` reader - Channel 6 Software Trigger"]
pub struct SWTRIG6_R(crate::FieldReader<bool, bool>);
impl SWTRIG6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG6` writer - Channel 6 Software Trigger"]
pub struct SWTRIG6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG7` reader - Channel 7 Software Trigger"]
pub struct SWTRIG7_R(crate::FieldReader<bool, bool>);
impl SWTRIG7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG7` writer - Channel 7 Software Trigger"]
pub struct SWTRIG7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG8` reader - Channel 8 Software Trigger"]
pub struct SWTRIG8_R(crate::FieldReader<bool, bool>);
impl SWTRIG8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG8` writer - Channel 8 Software Trigger"]
pub struct SWTRIG8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG9` reader - Channel 9 Software Trigger"]
pub struct SWTRIG9_R(crate::FieldReader<bool, bool>);
impl SWTRIG9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG9` writer - Channel 9 Software Trigger"]
pub struct SWTRIG9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG10` reader - Channel 10 Software Trigger"]
pub struct SWTRIG10_R(crate::FieldReader<bool, bool>);
impl SWTRIG10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG10` writer - Channel 10 Software Trigger"]
pub struct SWTRIG10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG11` reader - Channel 11 Software Trigger"]
pub struct SWTRIG11_R(crate::FieldReader<bool, bool>);
impl SWTRIG11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG11` writer - Channel 11 Software Trigger"]
pub struct SWTRIG11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG12` reader - Channel 12 Software Trigger"]
pub struct SWTRIG12_R(crate::FieldReader<bool, bool>);
impl SWTRIG12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG12` writer - Channel 12 Software Trigger"]
pub struct SWTRIG12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG13` reader - Channel 13 Software Trigger"]
pub struct SWTRIG13_R(crate::FieldReader<bool, bool>);
impl SWTRIG13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG13` writer - Channel 13 Software Trigger"]
pub struct SWTRIG13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG14` reader - Channel 14 Software Trigger"]
pub struct SWTRIG14_R(crate::FieldReader<bool, bool>);
impl SWTRIG14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG14` writer - Channel 14 Software Trigger"]
pub struct SWTRIG14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG15` reader - Channel 15 Software Trigger"]
pub struct SWTRIG15_R(crate::FieldReader<bool, bool>);
impl SWTRIG15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG15` writer - Channel 15 Software Trigger"]
pub struct SWTRIG15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG16` reader - Channel 16 Software Trigger"]
pub struct SWTRIG16_R(crate::FieldReader<bool, bool>);
impl SWTRIG16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG16` writer - Channel 16 Software Trigger"]
pub struct SWTRIG16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG17` reader - Channel 17 Software Trigger"]
pub struct SWTRIG17_R(crate::FieldReader<bool, bool>);
impl SWTRIG17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG17` writer - Channel 17 Software Trigger"]
pub struct SWTRIG17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG18` reader - Channel 18 Software Trigger"]
pub struct SWTRIG18_R(crate::FieldReader<bool, bool>);
impl SWTRIG18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG18` writer - Channel 18 Software Trigger"]
pub struct SWTRIG18_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG19` reader - Channel 19 Software Trigger"]
pub struct SWTRIG19_R(crate::FieldReader<bool, bool>);
impl SWTRIG19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG19` writer - Channel 19 Software Trigger"]
pub struct SWTRIG19_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG20` reader - Channel 20 Software Trigger"]
pub struct SWTRIG20_R(crate::FieldReader<bool, bool>);
impl SWTRIG20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG20` writer - Channel 20 Software Trigger"]
pub struct SWTRIG20_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG21` reader - Channel 21 Software Trigger"]
pub struct SWTRIG21_R(crate::FieldReader<bool, bool>);
impl SWTRIG21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG21` writer - Channel 21 Software Trigger"]
pub struct SWTRIG21_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG22` reader - Channel 22 Software Trigger"]
pub struct SWTRIG22_R(crate::FieldReader<bool, bool>);
impl SWTRIG22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG22` writer - Channel 22 Software Trigger"]
pub struct SWTRIG22_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG23` reader - Channel 23 Software Trigger"]
pub struct SWTRIG23_R(crate::FieldReader<bool, bool>);
impl SWTRIG23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG23` writer - Channel 23 Software Trigger"]
pub struct SWTRIG23_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG24` reader - Channel 24 Software Trigger"]
pub struct SWTRIG24_R(crate::FieldReader<bool, bool>);
impl SWTRIG24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG24` writer - Channel 24 Software Trigger"]
pub struct SWTRIG24_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG25` reader - Channel 25 Software Trigger"]
pub struct SWTRIG25_R(crate::FieldReader<bool, bool>);
impl SWTRIG25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG25` writer - Channel 25 Software Trigger"]
pub struct SWTRIG25_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG26` reader - Channel 26 Software Trigger"]
pub struct SWTRIG26_R(crate::FieldReader<bool, bool>);
impl SWTRIG26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG26` writer - Channel 26 Software Trigger"]
pub struct SWTRIG26_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG27` reader - Channel 27 Software Trigger"]
pub struct SWTRIG27_R(crate::FieldReader<bool, bool>);
impl SWTRIG27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG27` writer - Channel 27 Software Trigger"]
pub struct SWTRIG27_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG28` reader - Channel 28 Software Trigger"]
pub struct SWTRIG28_R(crate::FieldReader<bool, bool>);
impl SWTRIG28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG28` writer - Channel 28 Software Trigger"]
pub struct SWTRIG28_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG29` reader - Channel 29 Software Trigger"]
pub struct SWTRIG29_R(crate::FieldReader<bool, bool>);
impl SWTRIG29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG29` writer - Channel 29 Software Trigger"]
pub struct SWTRIG29_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG30` reader - Channel 30 Software Trigger"]
pub struct SWTRIG30_R(crate::FieldReader<bool, bool>);
impl SWTRIG30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG30` writer - Channel 30 Software Trigger"]
pub struct SWTRIG30_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWTRIG31` reader - Channel 31 Software Trigger"]
pub struct SWTRIG31_R(crate::FieldReader<bool, bool>);
impl SWTRIG31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG31` writer - Channel 31 Software Trigger"]
pub struct SWTRIG31_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&self) -> SWTRIG0_R {
        SWTRIG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&self) -> SWTRIG1_R {
        SWTRIG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    pub fn swtrig2(&self) -> SWTRIG2_R {
        SWTRIG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    pub fn swtrig3(&self) -> SWTRIG3_R {
        SWTRIG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    pub fn swtrig4(&self) -> SWTRIG4_R {
        SWTRIG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    pub fn swtrig5(&self) -> SWTRIG5_R {
        SWTRIG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    pub fn swtrig6(&self) -> SWTRIG6_R {
        SWTRIG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    pub fn swtrig7(&self) -> SWTRIG7_R {
        SWTRIG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    pub fn swtrig8(&self) -> SWTRIG8_R {
        SWTRIG8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    pub fn swtrig9(&self) -> SWTRIG9_R {
        SWTRIG9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    pub fn swtrig10(&self) -> SWTRIG10_R {
        SWTRIG10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    pub fn swtrig11(&self) -> SWTRIG11_R {
        SWTRIG11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Software Trigger"]
    #[inline(always)]
    pub fn swtrig12(&self) -> SWTRIG12_R {
        SWTRIG12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Software Trigger"]
    #[inline(always)]
    pub fn swtrig13(&self) -> SWTRIG13_R {
        SWTRIG13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Software Trigger"]
    #[inline(always)]
    pub fn swtrig14(&self) -> SWTRIG14_R {
        SWTRIG14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Software Trigger"]
    #[inline(always)]
    pub fn swtrig15(&self) -> SWTRIG15_R {
        SWTRIG15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 16 Software Trigger"]
    #[inline(always)]
    pub fn swtrig16(&self) -> SWTRIG16_R {
        SWTRIG16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 17 Software Trigger"]
    #[inline(always)]
    pub fn swtrig17(&self) -> SWTRIG17_R {
        SWTRIG17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 18 Software Trigger"]
    #[inline(always)]
    pub fn swtrig18(&self) -> SWTRIG18_R {
        SWTRIG18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 19 Software Trigger"]
    #[inline(always)]
    pub fn swtrig19(&self) -> SWTRIG19_R {
        SWTRIG19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 20 Software Trigger"]
    #[inline(always)]
    pub fn swtrig20(&self) -> SWTRIG20_R {
        SWTRIG20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 21 Software Trigger"]
    #[inline(always)]
    pub fn swtrig21(&self) -> SWTRIG21_R {
        SWTRIG21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 22 Software Trigger"]
    #[inline(always)]
    pub fn swtrig22(&self) -> SWTRIG22_R {
        SWTRIG22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 23 Software Trigger"]
    #[inline(always)]
    pub fn swtrig23(&self) -> SWTRIG23_R {
        SWTRIG23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel 24 Software Trigger"]
    #[inline(always)]
    pub fn swtrig24(&self) -> SWTRIG24_R {
        SWTRIG24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel 25 Software Trigger"]
    #[inline(always)]
    pub fn swtrig25(&self) -> SWTRIG25_R {
        SWTRIG25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel 26 Software Trigger"]
    #[inline(always)]
    pub fn swtrig26(&self) -> SWTRIG26_R {
        SWTRIG26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel 27 Software Trigger"]
    #[inline(always)]
    pub fn swtrig27(&self) -> SWTRIG27_R {
        SWTRIG27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Channel 28 Software Trigger"]
    #[inline(always)]
    pub fn swtrig28(&self) -> SWTRIG28_R {
        SWTRIG28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel 29 Software Trigger"]
    #[inline(always)]
    pub fn swtrig29(&self) -> SWTRIG29_R {
        SWTRIG29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel 30 Software Trigger"]
    #[inline(always)]
    pub fn swtrig30(&self) -> SWTRIG30_R {
        SWTRIG30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel 31 Software Trigger"]
    #[inline(always)]
    pub fn swtrig31(&self) -> SWTRIG31_R {
        SWTRIG31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&mut self) -> SWTRIG0_W {
        SWTRIG0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W {
        SWTRIG1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W {
        SWTRIG2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    pub fn swtrig3(&mut self) -> SWTRIG3_W {
        SWTRIG3_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    pub fn swtrig4(&mut self) -> SWTRIG4_W {
        SWTRIG4_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    pub fn swtrig5(&mut self) -> SWTRIG5_W {
        SWTRIG5_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    pub fn swtrig6(&mut self) -> SWTRIG6_W {
        SWTRIG6_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    pub fn swtrig7(&mut self) -> SWTRIG7_W {
        SWTRIG7_W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    pub fn swtrig8(&mut self) -> SWTRIG8_W {
        SWTRIG8_W { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    pub fn swtrig9(&mut self) -> SWTRIG9_W {
        SWTRIG9_W { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    pub fn swtrig10(&mut self) -> SWTRIG10_W {
        SWTRIG10_W { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    pub fn swtrig11(&mut self) -> SWTRIG11_W {
        SWTRIG11_W { w: self }
    }
    #[doc = "Bit 12 - Channel 12 Software Trigger"]
    #[inline(always)]
    pub fn swtrig12(&mut self) -> SWTRIG12_W {
        SWTRIG12_W { w: self }
    }
    #[doc = "Bit 13 - Channel 13 Software Trigger"]
    #[inline(always)]
    pub fn swtrig13(&mut self) -> SWTRIG13_W {
        SWTRIG13_W { w: self }
    }
    #[doc = "Bit 14 - Channel 14 Software Trigger"]
    #[inline(always)]
    pub fn swtrig14(&mut self) -> SWTRIG14_W {
        SWTRIG14_W { w: self }
    }
    #[doc = "Bit 15 - Channel 15 Software Trigger"]
    #[inline(always)]
    pub fn swtrig15(&mut self) -> SWTRIG15_W {
        SWTRIG15_W { w: self }
    }
    #[doc = "Bit 16 - Channel 16 Software Trigger"]
    #[inline(always)]
    pub fn swtrig16(&mut self) -> SWTRIG16_W {
        SWTRIG16_W { w: self }
    }
    #[doc = "Bit 17 - Channel 17 Software Trigger"]
    #[inline(always)]
    pub fn swtrig17(&mut self) -> SWTRIG17_W {
        SWTRIG17_W { w: self }
    }
    #[doc = "Bit 18 - Channel 18 Software Trigger"]
    #[inline(always)]
    pub fn swtrig18(&mut self) -> SWTRIG18_W {
        SWTRIG18_W { w: self }
    }
    #[doc = "Bit 19 - Channel 19 Software Trigger"]
    #[inline(always)]
    pub fn swtrig19(&mut self) -> SWTRIG19_W {
        SWTRIG19_W { w: self }
    }
    #[doc = "Bit 20 - Channel 20 Software Trigger"]
    #[inline(always)]
    pub fn swtrig20(&mut self) -> SWTRIG20_W {
        SWTRIG20_W { w: self }
    }
    #[doc = "Bit 21 - Channel 21 Software Trigger"]
    #[inline(always)]
    pub fn swtrig21(&mut self) -> SWTRIG21_W {
        SWTRIG21_W { w: self }
    }
    #[doc = "Bit 22 - Channel 22 Software Trigger"]
    #[inline(always)]
    pub fn swtrig22(&mut self) -> SWTRIG22_W {
        SWTRIG22_W { w: self }
    }
    #[doc = "Bit 23 - Channel 23 Software Trigger"]
    #[inline(always)]
    pub fn swtrig23(&mut self) -> SWTRIG23_W {
        SWTRIG23_W { w: self }
    }
    #[doc = "Bit 24 - Channel 24 Software Trigger"]
    #[inline(always)]
    pub fn swtrig24(&mut self) -> SWTRIG24_W {
        SWTRIG24_W { w: self }
    }
    #[doc = "Bit 25 - Channel 25 Software Trigger"]
    #[inline(always)]
    pub fn swtrig25(&mut self) -> SWTRIG25_W {
        SWTRIG25_W { w: self }
    }
    #[doc = "Bit 26 - Channel 26 Software Trigger"]
    #[inline(always)]
    pub fn swtrig26(&mut self) -> SWTRIG26_W {
        SWTRIG26_W { w: self }
    }
    #[doc = "Bit 27 - Channel 27 Software Trigger"]
    #[inline(always)]
    pub fn swtrig27(&mut self) -> SWTRIG27_W {
        SWTRIG27_W { w: self }
    }
    #[doc = "Bit 28 - Channel 28 Software Trigger"]
    #[inline(always)]
    pub fn swtrig28(&mut self) -> SWTRIG28_W {
        SWTRIG28_W { w: self }
    }
    #[doc = "Bit 29 - Channel 29 Software Trigger"]
    #[inline(always)]
    pub fn swtrig29(&mut self) -> SWTRIG29_W {
        SWTRIG29_W { w: self }
    }
    #[doc = "Bit 30 - Channel 30 Software Trigger"]
    #[inline(always)]
    pub fn swtrig30(&mut self) -> SWTRIG30_W {
        SWTRIG30_W { w: self }
    }
    #[doc = "Bit 31 - Channel 31 Software Trigger"]
    #[inline(always)]
    pub fn swtrig31(&mut self) -> SWTRIG31_W {
        SWTRIG31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Trigger Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrigctrl](index.html) module"]
pub struct SWTRIGCTRL_SPEC;
impl crate::RegisterSpec for SWTRIGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swtrigctrl::R](R) reader structure"]
impl crate::Readable for SWTRIGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swtrigctrl::W](W) writer structure"]
impl crate::Writable for SWTRIGCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWTRIGCTRL to value 0"]
impl crate::Resettable for SWTRIGCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
