#[doc = "Register `PMC_FSMR` reader"]
pub struct R(crate::R<PMC_FSMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_FSMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_FSMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_FSMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_FSMR` writer"]
pub struct W(crate::W<PMC_FSMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_FSMR_SPEC>;
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
impl From<crate::W<PMC_FSMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_FSMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSTT0` reader - Fast Startup Input Enable 0"]
pub struct FSTT0_R(crate::FieldReader<bool, bool>);
impl FSTT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT0` writer - Fast Startup Input Enable 0"]
pub struct FSTT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT1` reader - Fast Startup Input Enable 1"]
pub struct FSTT1_R(crate::FieldReader<bool, bool>);
impl FSTT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT1` writer - Fast Startup Input Enable 1"]
pub struct FSTT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT2` reader - Fast Startup Input Enable 2"]
pub struct FSTT2_R(crate::FieldReader<bool, bool>);
impl FSTT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT2` writer - Fast Startup Input Enable 2"]
pub struct FSTT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT3` reader - Fast Startup Input Enable 3"]
pub struct FSTT3_R(crate::FieldReader<bool, bool>);
impl FSTT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT3` writer - Fast Startup Input Enable 3"]
pub struct FSTT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT4` reader - Fast Startup Input Enable 4"]
pub struct FSTT4_R(crate::FieldReader<bool, bool>);
impl FSTT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT4` writer - Fast Startup Input Enable 4"]
pub struct FSTT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT5` reader - Fast Startup Input Enable 5"]
pub struct FSTT5_R(crate::FieldReader<bool, bool>);
impl FSTT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT5` writer - Fast Startup Input Enable 5"]
pub struct FSTT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT6` reader - Fast Startup Input Enable 6"]
pub struct FSTT6_R(crate::FieldReader<bool, bool>);
impl FSTT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT6` writer - Fast Startup Input Enable 6"]
pub struct FSTT6_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT7` reader - Fast Startup Input Enable 7"]
pub struct FSTT7_R(crate::FieldReader<bool, bool>);
impl FSTT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT7` writer - Fast Startup Input Enable 7"]
pub struct FSTT7_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT8` reader - Fast Startup Input Enable 8"]
pub struct FSTT8_R(crate::FieldReader<bool, bool>);
impl FSTT8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT8` writer - Fast Startup Input Enable 8"]
pub struct FSTT8_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT9` reader - Fast Startup Input Enable 9"]
pub struct FSTT9_R(crate::FieldReader<bool, bool>);
impl FSTT9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT9` writer - Fast Startup Input Enable 9"]
pub struct FSTT9_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT10` reader - Fast Startup Input Enable 10"]
pub struct FSTT10_R(crate::FieldReader<bool, bool>);
impl FSTT10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT10` writer - Fast Startup Input Enable 10"]
pub struct FSTT10_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT11` reader - Fast Startup Input Enable 11"]
pub struct FSTT11_R(crate::FieldReader<bool, bool>);
impl FSTT11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT11` writer - Fast Startup Input Enable 11"]
pub struct FSTT11_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT12` reader - Fast Startup Input Enable 12"]
pub struct FSTT12_R(crate::FieldReader<bool, bool>);
impl FSTT12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT12` writer - Fast Startup Input Enable 12"]
pub struct FSTT12_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT13` reader - Fast Startup Input Enable 13"]
pub struct FSTT13_R(crate::FieldReader<bool, bool>);
impl FSTT13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT13` writer - Fast Startup Input Enable 13"]
pub struct FSTT13_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT14` reader - Fast Startup Input Enable 14"]
pub struct FSTT14_R(crate::FieldReader<bool, bool>);
impl FSTT14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT14` writer - Fast Startup Input Enable 14"]
pub struct FSTT14_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTT15` reader - Fast Startup Input Enable 15"]
pub struct FSTT15_R(crate::FieldReader<bool, bool>);
impl FSTT15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTT15` writer - Fast Startup Input Enable 15"]
pub struct FSTT15_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RTTAL` reader - RTT Alarm Enable"]
pub struct RTTAL_R(crate::FieldReader<bool, bool>);
impl RTTAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTTAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTTAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTTAL` writer - RTT Alarm Enable"]
pub struct RTTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTAL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RTCAL` reader - RTC Alarm Enable"]
pub struct RTCAL_R(crate::FieldReader<bool, bool>);
impl RTCAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCAL` writer - RTC Alarm Enable"]
pub struct RTCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `USBAL` reader - USB Alarm Enable"]
pub struct USBAL_R(crate::FieldReader<bool, bool>);
impl USBAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBAL` writer - USB Alarm Enable"]
pub struct USBAL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBAL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LPM` reader - Low-power Mode"]
pub struct LPM_R(crate::FieldReader<bool, bool>);
impl LPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM` writer - Low-power Mode"]
pub struct LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Flash Low-power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLPM_A {
    #[doc = "0: Flash is in Standby Mode when system enters Wait Mode"]
    FLASH_STANDBY = 0,
    #[doc = "1: Flash is in Deep-power-down mode when system enters Wait Mode"]
    FLASH_DEEP_POWERDOWN = 1,
    #[doc = "2: Idle mode"]
    FLASH_IDLE = 2,
}
impl From<FLPM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLPM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLPM` reader - Flash Low-power Mode"]
pub struct FLPM_R(crate::FieldReader<u8, FLPM_A>);
impl FLPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLPM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLPM_A> {
        match self.bits {
            0 => Some(FLPM_A::FLASH_STANDBY),
            1 => Some(FLPM_A::FLASH_DEEP_POWERDOWN),
            2 => Some(FLPM_A::FLASH_IDLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_STANDBY`"]
    #[inline(always)]
    pub fn is_flash_standby(&self) -> bool {
        **self == FLPM_A::FLASH_STANDBY
    }
    #[doc = "Checks if the value of the field is `FLASH_DEEP_POWERDOWN`"]
    #[inline(always)]
    pub fn is_flash_deep_powerdown(&self) -> bool {
        **self == FLPM_A::FLASH_DEEP_POWERDOWN
    }
    #[doc = "Checks if the value of the field is `FLASH_IDLE`"]
    #[inline(always)]
    pub fn is_flash_idle(&self) -> bool {
        **self == FLPM_A::FLASH_IDLE
    }
}
impl core::ops::Deref for FLPM_R {
    type Target = crate::FieldReader<u8, FLPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLPM` writer - Flash Low-power Mode"]
pub struct FLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLPM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash is in Standby Mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn flash_standby(self) -> &'a mut W {
        self.variant(FLPM_A::FLASH_STANDBY)
    }
    #[doc = "Flash is in Deep-power-down mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn flash_deep_powerdown(self) -> &'a mut W {
        self.variant(FLPM_A::FLASH_DEEP_POWERDOWN)
    }
    #[doc = "Idle mode"]
    #[inline(always)]
    pub fn flash_idle(self) -> &'a mut W {
        self.variant(FLPM_A::FLASH_IDLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `FFLPM` reader - Force Flash Low-power Mode"]
pub struct FFLPM_R(crate::FieldReader<bool, bool>);
impl FFLPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFLPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFLPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFLPM` writer - Force Flash Low-power Mode"]
pub struct FFLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLPM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&self) -> FSTT0_R {
        FSTT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&self) -> FSTT1_R {
        FSTT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&self) -> FSTT2_R {
        FSTT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&self) -> FSTT3_R {
        FSTT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&self) -> FSTT4_R {
        FSTT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&self) -> FSTT5_R {
        FSTT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&self) -> FSTT6_R {
        FSTT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&self) -> FSTT7_R {
        FSTT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&self) -> FSTT8_R {
        FSTT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&self) -> FSTT9_R {
        FSTT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&self) -> FSTT10_R {
        FSTT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&self) -> FSTT11_R {
        FSTT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&self) -> FSTT12_R {
        FSTT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&self) -> FSTT13_R {
        FSTT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&self) -> FSTT14_R {
        FSTT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&self) -> FSTT15_R {
        FSTT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&self) -> RTTAL_R {
        RTTAL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&self) -> RTCAL_R {
        RTCAL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&self) -> USBAL_R {
        USBAL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline(always)]
    pub fn flpm(&self) -> FLPM_R {
        FLPM_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline(always)]
    pub fn fflpm(&self) -> FFLPM_R {
        FFLPM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&mut self) -> FSTT0_W {
        FSTT0_W { w: self }
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&mut self) -> FSTT1_W {
        FSTT1_W { w: self }
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&mut self) -> FSTT2_W {
        FSTT2_W { w: self }
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&mut self) -> FSTT3_W {
        FSTT3_W { w: self }
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&mut self) -> FSTT4_W {
        FSTT4_W { w: self }
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&mut self) -> FSTT5_W {
        FSTT5_W { w: self }
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&mut self) -> FSTT6_W {
        FSTT6_W { w: self }
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&mut self) -> FSTT7_W {
        FSTT7_W { w: self }
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&mut self) -> FSTT8_W {
        FSTT8_W { w: self }
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&mut self) -> FSTT9_W {
        FSTT9_W { w: self }
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&mut self) -> FSTT10_W {
        FSTT10_W { w: self }
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&mut self) -> FSTT11_W {
        FSTT11_W { w: self }
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&mut self) -> FSTT12_W {
        FSTT12_W { w: self }
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&mut self) -> FSTT13_W {
        FSTT13_W { w: self }
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&mut self) -> FSTT14_W {
        FSTT14_W { w: self }
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&mut self) -> FSTT15_W {
        FSTT15_W { w: self }
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&mut self) -> RTTAL_W {
        RTTAL_W { w: self }
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&mut self) -> RTCAL_W {
        RTCAL_W { w: self }
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&mut self) -> USBAL_W {
        USBAL_W { w: self }
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline(always)]
    pub fn lpm(&mut self) -> LPM_W {
        LPM_W { w: self }
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline(always)]
    pub fn flpm(&mut self) -> FLPM_W {
        FLPM_W { w: self }
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline(always)]
    pub fn fflpm(&mut self) -> FFLPM_W {
        FFLPM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Startup Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_fsmr](index.html) module"]
pub struct PMC_FSMR_SPEC;
impl crate::RegisterSpec for PMC_FSMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_fsmr::R](R) reader structure"]
impl crate::Readable for PMC_FSMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_fsmr::W](W) writer structure"]
impl crate::Writable for PMC_FSMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_FSMR to value 0"]
impl crate::Resettable for PMC_FSMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
