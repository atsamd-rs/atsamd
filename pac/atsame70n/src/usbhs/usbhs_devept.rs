#[doc = "Register `USBHS_DEVEPT` reader"]
pub struct R(crate::R<USBHS_DEVEPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_DEVEPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_DEVEPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_DEVEPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_DEVEPT` writer"]
pub struct W(crate::W<USBHS_DEVEPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVEPT_SPEC>;
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
impl From<crate::W<USBHS_DEVEPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVEPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPEN0` reader - Endpoint 0 Enable"]
pub struct EPEN0_R(crate::FieldReader<bool, bool>);
impl EPEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN0` writer - Endpoint 0 Enable"]
pub struct EPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPEN1` reader - Endpoint 1 Enable"]
pub struct EPEN1_R(crate::FieldReader<bool, bool>);
impl EPEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN1` writer - Endpoint 1 Enable"]
pub struct EPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPEN2` reader - Endpoint 2 Enable"]
pub struct EPEN2_R(crate::FieldReader<bool, bool>);
impl EPEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN2` writer - Endpoint 2 Enable"]
pub struct EPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPEN3` reader - Endpoint 3 Enable"]
pub struct EPEN3_R(crate::FieldReader<bool, bool>);
impl EPEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN3` writer - Endpoint 3 Enable"]
pub struct EPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPEN4` reader - Endpoint 4 Enable"]
pub struct EPEN4_R(crate::FieldReader<bool, bool>);
impl EPEN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN4` writer - Endpoint 4 Enable"]
pub struct EPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPEN5` reader - Endpoint 5 Enable"]
pub struct EPEN5_R(crate::FieldReader<bool, bool>);
impl EPEN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN5` writer - Endpoint 5 Enable"]
pub struct EPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPEN6` reader - Endpoint 6 Enable"]
pub struct EPEN6_R(crate::FieldReader<bool, bool>);
impl EPEN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN6` writer - Endpoint 6 Enable"]
pub struct EPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPEN7` reader - Endpoint 7 Enable"]
pub struct EPEN7_R(crate::FieldReader<bool, bool>);
impl EPEN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN7` writer - Endpoint 7 Enable"]
pub struct EPEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPEN8` reader - Endpoint 8 Enable"]
pub struct EPEN8_R(crate::FieldReader<bool, bool>);
impl EPEN8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPEN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN8` writer - Endpoint 8 Enable"]
pub struct EPEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPEN9` reader - Endpoint 9 Enable"]
pub struct EPEN9_R(crate::FieldReader<bool, bool>);
impl EPEN9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPEN9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN9` writer - Endpoint 9 Enable"]
pub struct EPEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPRST0` reader - Endpoint 0 Reset"]
pub struct EPRST0_R(crate::FieldReader<bool, bool>);
impl EPRST0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST0` writer - Endpoint 0 Reset"]
pub struct EPRST0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPRST1` reader - Endpoint 1 Reset"]
pub struct EPRST1_R(crate::FieldReader<bool, bool>);
impl EPRST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST1` writer - Endpoint 1 Reset"]
pub struct EPRST1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPRST2` reader - Endpoint 2 Reset"]
pub struct EPRST2_R(crate::FieldReader<bool, bool>);
impl EPRST2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST2` writer - Endpoint 2 Reset"]
pub struct EPRST2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPRST3` reader - Endpoint 3 Reset"]
pub struct EPRST3_R(crate::FieldReader<bool, bool>);
impl EPRST3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST3` writer - Endpoint 3 Reset"]
pub struct EPRST3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPRST4` reader - Endpoint 4 Reset"]
pub struct EPRST4_R(crate::FieldReader<bool, bool>);
impl EPRST4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST4` writer - Endpoint 4 Reset"]
pub struct EPRST4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPRST5` reader - Endpoint 5 Reset"]
pub struct EPRST5_R(crate::FieldReader<bool, bool>);
impl EPRST5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST5` writer - Endpoint 5 Reset"]
pub struct EPRST5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPRST6` reader - Endpoint 6 Reset"]
pub struct EPRST6_R(crate::FieldReader<bool, bool>);
impl EPRST6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRST6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST6` writer - Endpoint 6 Reset"]
pub struct EPRST6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPRST7` reader - Endpoint 7 Reset"]
pub struct EPRST7_R(crate::FieldReader<bool, bool>);
impl EPRST7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRST7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST7` writer - Endpoint 7 Reset"]
pub struct EPRST7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPRST8` reader - Endpoint 8 Reset"]
pub struct EPRST8_R(crate::FieldReader<bool, bool>);
impl EPRST8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRST8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST8` writer - Endpoint 8 Reset"]
pub struct EPRST8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPRST9` reader - Endpoint 9 Reset"]
pub struct EPRST9_R(crate::FieldReader<bool, bool>);
impl EPRST9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPRST9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRST9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRST9` writer - Endpoint 9 Reset"]
pub struct EPRST9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - Endpoint 0 Enable"]
    #[inline(always)]
    pub fn epen0(&self) -> EPEN0_R {
        EPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Enable"]
    #[inline(always)]
    pub fn epen1(&self) -> EPEN1_R {
        EPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Enable"]
    #[inline(always)]
    pub fn epen2(&self) -> EPEN2_R {
        EPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Enable"]
    #[inline(always)]
    pub fn epen3(&self) -> EPEN3_R {
        EPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Enable"]
    #[inline(always)]
    pub fn epen4(&self) -> EPEN4_R {
        EPEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Enable"]
    #[inline(always)]
    pub fn epen5(&self) -> EPEN5_R {
        EPEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Enable"]
    #[inline(always)]
    pub fn epen6(&self) -> EPEN6_R {
        EPEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 Enable"]
    #[inline(always)]
    pub fn epen7(&self) -> EPEN7_R {
        EPEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 Enable"]
    #[inline(always)]
    pub fn epen8(&self) -> EPEN8_R {
        EPEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 Enable"]
    #[inline(always)]
    pub fn epen9(&self) -> EPEN9_R {
        EPEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint 0 Reset"]
    #[inline(always)]
    pub fn eprst0(&self) -> EPRST0_R {
        EPRST0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint 1 Reset"]
    #[inline(always)]
    pub fn eprst1(&self) -> EPRST1_R {
        EPRST1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint 2 Reset"]
    #[inline(always)]
    pub fn eprst2(&self) -> EPRST2_R {
        EPRST2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint 3 Reset"]
    #[inline(always)]
    pub fn eprst3(&self) -> EPRST3_R {
        EPRST3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint 4 Reset"]
    #[inline(always)]
    pub fn eprst4(&self) -> EPRST4_R {
        EPRST4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint 5 Reset"]
    #[inline(always)]
    pub fn eprst5(&self) -> EPRST5_R {
        EPRST5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint 6 Reset"]
    #[inline(always)]
    pub fn eprst6(&self) -> EPRST6_R {
        EPRST6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint 7 Reset"]
    #[inline(always)]
    pub fn eprst7(&self) -> EPRST7_R {
        EPRST7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint 8 Reset"]
    #[inline(always)]
    pub fn eprst8(&self) -> EPRST8_R {
        EPRST8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint 9 Reset"]
    #[inline(always)]
    pub fn eprst9(&self) -> EPRST9_R {
        EPRST9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 Enable"]
    #[inline(always)]
    pub fn epen0(&mut self) -> EPEN0_W {
        EPEN0_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 Enable"]
    #[inline(always)]
    pub fn epen1(&mut self) -> EPEN1_W {
        EPEN1_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 Enable"]
    #[inline(always)]
    pub fn epen2(&mut self) -> EPEN2_W {
        EPEN2_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 Enable"]
    #[inline(always)]
    pub fn epen3(&mut self) -> EPEN3_W {
        EPEN3_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 Enable"]
    #[inline(always)]
    pub fn epen4(&mut self) -> EPEN4_W {
        EPEN4_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 Enable"]
    #[inline(always)]
    pub fn epen5(&mut self) -> EPEN5_W {
        EPEN5_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 Enable"]
    #[inline(always)]
    pub fn epen6(&mut self) -> EPEN6_W {
        EPEN6_W { w: self }
    }
    #[doc = "Bit 7 - Endpoint 7 Enable"]
    #[inline(always)]
    pub fn epen7(&mut self) -> EPEN7_W {
        EPEN7_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint 8 Enable"]
    #[inline(always)]
    pub fn epen8(&mut self) -> EPEN8_W {
        EPEN8_W { w: self }
    }
    #[doc = "Bit 9 - Endpoint 9 Enable"]
    #[inline(always)]
    pub fn epen9(&mut self) -> EPEN9_W {
        EPEN9_W { w: self }
    }
    #[doc = "Bit 16 - Endpoint 0 Reset"]
    #[inline(always)]
    pub fn eprst0(&mut self) -> EPRST0_W {
        EPRST0_W { w: self }
    }
    #[doc = "Bit 17 - Endpoint 1 Reset"]
    #[inline(always)]
    pub fn eprst1(&mut self) -> EPRST1_W {
        EPRST1_W { w: self }
    }
    #[doc = "Bit 18 - Endpoint 2 Reset"]
    #[inline(always)]
    pub fn eprst2(&mut self) -> EPRST2_W {
        EPRST2_W { w: self }
    }
    #[doc = "Bit 19 - Endpoint 3 Reset"]
    #[inline(always)]
    pub fn eprst3(&mut self) -> EPRST3_W {
        EPRST3_W { w: self }
    }
    #[doc = "Bit 20 - Endpoint 4 Reset"]
    #[inline(always)]
    pub fn eprst4(&mut self) -> EPRST4_W {
        EPRST4_W { w: self }
    }
    #[doc = "Bit 21 - Endpoint 5 Reset"]
    #[inline(always)]
    pub fn eprst5(&mut self) -> EPRST5_W {
        EPRST5_W { w: self }
    }
    #[doc = "Bit 22 - Endpoint 6 Reset"]
    #[inline(always)]
    pub fn eprst6(&mut self) -> EPRST6_W {
        EPRST6_W { w: self }
    }
    #[doc = "Bit 23 - Endpoint 7 Reset"]
    #[inline(always)]
    pub fn eprst7(&mut self) -> EPRST7_W {
        EPRST7_W { w: self }
    }
    #[doc = "Bit 24 - Endpoint 8 Reset"]
    #[inline(always)]
    pub fn eprst8(&mut self) -> EPRST8_W {
        EPRST8_W { w: self }
    }
    #[doc = "Bit 25 - Endpoint 9 Reset"]
    #[inline(always)]
    pub fn eprst9(&mut self) -> EPRST9_W {
        EPRST9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devept](index.html) module"]
pub struct USBHS_DEVEPT_SPEC;
impl crate::RegisterSpec for USBHS_DEVEPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_devept::R](R) reader structure"]
impl crate::Readable for USBHS_DEVEPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_devept::W](W) writer structure"]
impl crate::Writable for USBHS_DEVEPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVEPT to value 0"]
impl crate::Resettable for USBHS_DEVEPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
