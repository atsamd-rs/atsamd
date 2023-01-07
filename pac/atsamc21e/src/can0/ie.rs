#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF0NE` reader - Rx FIFO 0 New Message Interrupt Enable"]
pub struct RF0NE_R(crate::FieldReader<bool, bool>);
impl RF0NE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF0NE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0NE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0NE` writer - Rx FIFO 0 New Message Interrupt Enable"]
pub struct RF0NE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0NE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RF0WE` reader - Rx FIFO 0 Watermark Reached Interrupt Enable"]
pub struct RF0WE_R(crate::FieldReader<bool, bool>);
impl RF0WE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF0WE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0WE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0WE` writer - Rx FIFO 0 Watermark Reached Interrupt Enable"]
pub struct RF0WE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0WE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RF0FE` reader - Rx FIFO 0 Full Interrupt Enable"]
pub struct RF0FE_R(crate::FieldReader<bool, bool>);
impl RF0FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF0FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0FE` writer - Rx FIFO 0 Full Interrupt Enable"]
pub struct RF0FE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0FE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RF0LE` reader - Rx FIFO 0 Message Lost Interrupt Enable"]
pub struct RF0LE_R(crate::FieldReader<bool, bool>);
impl RF0LE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF0LE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0LE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0LE` writer - Rx FIFO 0 Message Lost Interrupt Enable"]
pub struct RF0LE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0LE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RF1NE` reader - Rx FIFO 1 New Message Interrupt Enable"]
pub struct RF1NE_R(crate::FieldReader<bool, bool>);
impl RF1NE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF1NE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1NE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1NE` writer - Rx FIFO 1 New Message Interrupt Enable"]
pub struct RF1NE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1NE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RF1WE` reader - Rx FIFO 1 Watermark Reached Interrupt Enable"]
pub struct RF1WE_R(crate::FieldReader<bool, bool>);
impl RF1WE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF1WE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1WE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1WE` writer - Rx FIFO 1 Watermark Reached Interrupt Enable"]
pub struct RF1WE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1WE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RF1FE` reader - Rx FIFO 1 FIFO Full Interrupt Enable"]
pub struct RF1FE_R(crate::FieldReader<bool, bool>);
impl RF1FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF1FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1FE` writer - Rx FIFO 1 FIFO Full Interrupt Enable"]
pub struct RF1FE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1FE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RF1LE` reader - Rx FIFO 1 Message Lost Interrupt Enable"]
pub struct RF1LE_R(crate::FieldReader<bool, bool>);
impl RF1LE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF1LE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1LE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1LE` writer - Rx FIFO 1 Message Lost Interrupt Enable"]
pub struct RF1LE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1LE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `HPME` reader - High Priority Message Interrupt Enable"]
pub struct HPME_R(crate::FieldReader<bool, bool>);
impl HPME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPME` writer - High Priority Message Interrupt Enable"]
pub struct HPME_W<'a> {
    w: &'a mut W,
}
impl<'a> HPME_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TCE` reader - Timestamp Completed Interrupt Enable"]
pub struct TCE_R(crate::FieldReader<bool, bool>);
impl TCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCE` writer - Timestamp Completed Interrupt Enable"]
pub struct TCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TCFE` reader - Transmission Cancellation Finished Interrupt Enable"]
pub struct TCFE_R(crate::FieldReader<bool, bool>);
impl TCFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCFE` writer - Transmission Cancellation Finished Interrupt Enable"]
pub struct TCFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TFEE` reader - Tx FIFO Empty Interrupt Enable"]
pub struct TFEE_R(crate::FieldReader<bool, bool>);
impl TFEE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFEE` writer - Tx FIFO Empty Interrupt Enable"]
pub struct TFEE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFEE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TEFNE` reader - Tx Event FIFO New Entry Interrupt Enable"]
pub struct TEFNE_R(crate::FieldReader<bool, bool>);
impl TEFNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEFNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFNE` writer - Tx Event FIFO New Entry Interrupt Enable"]
pub struct TEFNE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFNE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TEFWE` reader - Tx Event FIFO Watermark Reached Interrupt Enable"]
pub struct TEFWE_R(crate::FieldReader<bool, bool>);
impl TEFWE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEFWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFWE` writer - Tx Event FIFO Watermark Reached Interrupt Enable"]
pub struct TEFWE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFWE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TEFFE` reader - Tx Event FIFO Full Interrupt Enable"]
pub struct TEFFE_R(crate::FieldReader<bool, bool>);
impl TEFFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEFFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFFE` writer - Tx Event FIFO Full Interrupt Enable"]
pub struct TEFFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TEFLE` reader - Tx Event FIFO Element Lost Interrupt Enable"]
pub struct TEFLE_R(crate::FieldReader<bool, bool>);
impl TEFLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEFLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFLE` writer - Tx Event FIFO Element Lost Interrupt Enable"]
pub struct TEFLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TSWE` reader - Timestamp Wraparound Interrupt Enable"]
pub struct TSWE_R(crate::FieldReader<bool, bool>);
impl TSWE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSWE` writer - Timestamp Wraparound Interrupt Enable"]
pub struct TSWE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSWE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MRAFE` reader - Message RAM Access Failure Interrupt Enable"]
pub struct MRAFE_R(crate::FieldReader<bool, bool>);
impl MRAFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MRAFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRAFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRAFE` writer - Message RAM Access Failure Interrupt Enable"]
pub struct MRAFE_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TOOE` reader - Timeout Occurred Interrupt Enable"]
pub struct TOOE_R(crate::FieldReader<bool, bool>);
impl TOOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOOE` writer - Timeout Occurred Interrupt Enable"]
pub struct TOOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOOE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `DRXE` reader - Message stored to Dedicated Rx Buffer Interrupt Enable"]
pub struct DRXE_R(crate::FieldReader<bool, bool>);
impl DRXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRXE` writer - Message stored to Dedicated Rx Buffer Interrupt Enable"]
pub struct DRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRXE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `BECE` reader - Bit Error Corrected Interrupt Enable"]
pub struct BECE_R(crate::FieldReader<bool, bool>);
impl BECE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BECE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BECE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BECE` writer - Bit Error Corrected Interrupt Enable"]
pub struct BECE_W<'a> {
    w: &'a mut W,
}
impl<'a> BECE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `BEUE` reader - Bit Error Uncorrected Interrupt Enable"]
pub struct BEUE_R(crate::FieldReader<bool, bool>);
impl BEUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEUE` writer - Bit Error Uncorrected Interrupt Enable"]
pub struct BEUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BEUE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ELOE` reader - Error Logging Overflow Interrupt Enable"]
pub struct ELOE_R(crate::FieldReader<bool, bool>);
impl ELOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ELOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELOE` writer - Error Logging Overflow Interrupt Enable"]
pub struct ELOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ELOE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EPE` reader - Error Passive Interrupt Enable"]
pub struct EPE_R(crate::FieldReader<bool, bool>);
impl EPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPE` writer - Error Passive Interrupt Enable"]
pub struct EPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EWE` reader - Warning Status Interrupt Enable"]
pub struct EWE_R(crate::FieldReader<bool, bool>);
impl EWE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWE` writer - Warning Status Interrupt Enable"]
pub struct EWE_W<'a> {
    w: &'a mut W,
}
impl<'a> EWE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `BOE` reader - Bus_Off Status Interrupt Enable"]
pub struct BOE_R(crate::FieldReader<bool, bool>);
impl BOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOE` writer - Bus_Off Status Interrupt Enable"]
pub struct BOE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WDIE` reader - Watchdog Interrupt Interrupt Enable"]
pub struct WDIE_R(crate::FieldReader<bool, bool>);
impl WDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDIE` writer - Watchdog Interrupt Interrupt Enable"]
pub struct WDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PEAE` reader - Protocol Error in Arbitration Phase Enable"]
pub struct PEAE_R(crate::FieldReader<bool, bool>);
impl PEAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEAE` writer - Protocol Error in Arbitration Phase Enable"]
pub struct PEAE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PEDE` reader - Protocol Error in Data Phase Enable"]
pub struct PEDE_R(crate::FieldReader<bool, bool>);
impl PEDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEDE` writer - Protocol Error in Data Phase Enable"]
pub struct PEDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ARAE` reader - Access to Reserved Address Enable"]
pub struct ARAE_R(crate::FieldReader<bool, bool>);
impl ARAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARAE` writer - Access to Reserved Address Enable"]
pub struct ARAE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Interrupt Enable"]
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn rf0we(&self) -> RF0WE_R {
        RF0WE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Interrupt Enable"]
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Interrupt Enable"]
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn rf1we(&self) -> RF1WE_R {
        RF1WE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Interrupt Enable"]
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Completed Interrupt Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Interrupt Enable"]
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn tefwe(&self) -> TEFWE_R {
        TEFWE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Interrupt Enable"]
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Interrupt Enable"]
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Interrupt Enable"]
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Interrupt Enable"]
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn drxe(&self) -> DRXE_R {
        DRXE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Enable"]
    #[inline(always)]
    pub fn bece(&self) -> BECE_R {
        BECE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    pub fn beue(&self) -> BEUE_R {
        BEUE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Warning Status Interrupt Enable"]
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status Interrupt Enable"]
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Interrupt Enable"]
    #[inline(always)]
    pub fn rf0ne(&mut self) -> RF0NE_W {
        RF0NE_W { w: self }
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn rf0we(&mut self) -> RF0WE_W {
        RF0WE_W { w: self }
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Interrupt Enable"]
    #[inline(always)]
    pub fn rf0fe(&mut self) -> RF0FE_W {
        RF0FE_W { w: self }
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub fn rf0le(&mut self) -> RF0LE_W {
        RF0LE_W { w: self }
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Interrupt Enable"]
    #[inline(always)]
    pub fn rf1ne(&mut self) -> RF1NE_W {
        RF1NE_W { w: self }
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn rf1we(&mut self) -> RF1WE_W {
        RF1WE_W { w: self }
    }
    #[doc = "Bit 6 - Rx FIFO 1 FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn rf1fe(&mut self) -> RF1FE_W {
        RF1FE_W { w: self }
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub fn rf1le(&mut self) -> RF1LE_W {
        RF1LE_W { w: self }
    }
    #[doc = "Bit 8 - High Priority Message Interrupt Enable"]
    #[inline(always)]
    pub fn hpme(&mut self) -> HPME_W {
        HPME_W { w: self }
    }
    #[doc = "Bit 9 - Timestamp Completed Interrupt Enable"]
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W {
        TCE_W { w: self }
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn tcfe(&mut self) -> TCFE_W {
        TCFE_W { w: self }
    }
    #[doc = "Bit 11 - Tx FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tfee(&mut self) -> TFEE_W {
        TFEE_W { w: self }
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Interrupt Enable"]
    #[inline(always)]
    pub fn tefne(&mut self) -> TEFNE_W {
        TEFNE_W { w: self }
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn tefwe(&mut self) -> TEFWE_W {
        TEFWE_W { w: self }
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn teffe(&mut self) -> TEFFE_W {
        TEFFE_W { w: self }
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Interrupt Enable"]
    #[inline(always)]
    pub fn tefle(&mut self) -> TEFLE_W {
        TEFLE_W { w: self }
    }
    #[doc = "Bit 16 - Timestamp Wraparound Interrupt Enable"]
    #[inline(always)]
    pub fn tswe(&mut self) -> TSWE_W {
        TSWE_W { w: self }
    }
    #[doc = "Bit 17 - Message RAM Access Failure Interrupt Enable"]
    #[inline(always)]
    pub fn mrafe(&mut self) -> MRAFE_W {
        MRAFE_W { w: self }
    }
    #[doc = "Bit 18 - Timeout Occurred Interrupt Enable"]
    #[inline(always)]
    pub fn tooe(&mut self) -> TOOE_W {
        TOOE_W { w: self }
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn drxe(&mut self) -> DRXE_W {
        DRXE_W { w: self }
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Enable"]
    #[inline(always)]
    pub fn bece(&mut self) -> BECE_W {
        BECE_W { w: self }
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    pub fn beue(&mut self) -> BEUE_W {
        BEUE_W { w: self }
    }
    #[doc = "Bit 22 - Error Logging Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn eloe(&mut self) -> ELOE_W {
        ELOE_W { w: self }
    }
    #[doc = "Bit 23 - Error Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W {
        EPE_W { w: self }
    }
    #[doc = "Bit 24 - Warning Status Interrupt Enable"]
    #[inline(always)]
    pub fn ewe(&mut self) -> EWE_W {
        EWE_W { w: self }
    }
    #[doc = "Bit 25 - Bus_Off Status Interrupt Enable"]
    #[inline(always)]
    pub fn boe(&mut self) -> BOE_W {
        BOE_W { w: self }
    }
    #[doc = "Bit 26 - Watchdog Interrupt Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&mut self) -> WDIE_W {
        WDIE_W { w: self }
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn peae(&mut self) -> PEAE_W {
        PEAE_W { w: self }
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn pede(&mut self) -> PEDE_W {
        PEDE_W { w: self }
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    pub fn arae(&mut self) -> ARAE_W {
        ARAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
