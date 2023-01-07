#[doc = "Register `DRVCTRL` reader"]
pub struct R(crate::R<DRVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRVCTRL` writer"]
pub struct W(crate::W<DRVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRVCTRL_SPEC>;
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
impl From<crate::W<DRVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NRE0` reader - Non-Recoverable State 0 Output Enable"]
pub struct NRE0_R(crate::FieldReader<bool, bool>);
impl NRE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRE0` writer - Non-Recoverable State 0 Output Enable"]
pub struct NRE0_W<'a> {
    w: &'a mut W,
}
impl<'a> NRE0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRE1` reader - Non-Recoverable State 1 Output Enable"]
pub struct NRE1_R(crate::FieldReader<bool, bool>);
impl NRE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRE1` writer - Non-Recoverable State 1 Output Enable"]
pub struct NRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> NRE1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRE2` reader - Non-Recoverable State 2 Output Enable"]
pub struct NRE2_R(crate::FieldReader<bool, bool>);
impl NRE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRE2` writer - Non-Recoverable State 2 Output Enable"]
pub struct NRE2_W<'a> {
    w: &'a mut W,
}
impl<'a> NRE2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRE3` reader - Non-Recoverable State 3 Output Enable"]
pub struct NRE3_R(crate::FieldReader<bool, bool>);
impl NRE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRE3` writer - Non-Recoverable State 3 Output Enable"]
pub struct NRE3_W<'a> {
    w: &'a mut W,
}
impl<'a> NRE3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRE4` reader - Non-Recoverable State 4 Output Enable"]
pub struct NRE4_R(crate::FieldReader<bool, bool>);
impl NRE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRE4` writer - Non-Recoverable State 4 Output Enable"]
pub struct NRE4_W<'a> {
    w: &'a mut W,
}
impl<'a> NRE4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRE5` reader - Non-Recoverable State 5 Output Enable"]
pub struct NRE5_R(crate::FieldReader<bool, bool>);
impl NRE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRE5` writer - Non-Recoverable State 5 Output Enable"]
pub struct NRE5_W<'a> {
    w: &'a mut W,
}
impl<'a> NRE5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRE6` reader - Non-Recoverable State 6 Output Enable"]
pub struct NRE6_R(crate::FieldReader<bool, bool>);
impl NRE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRE6` writer - Non-Recoverable State 6 Output Enable"]
pub struct NRE6_W<'a> {
    w: &'a mut W,
}
impl<'a> NRE6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRE7` reader - Non-Recoverable State 7 Output Enable"]
pub struct NRE7_R(crate::FieldReader<bool, bool>);
impl NRE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRE7` writer - Non-Recoverable State 7 Output Enable"]
pub struct NRE7_W<'a> {
    w: &'a mut W,
}
impl<'a> NRE7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRV0` reader - Non-Recoverable State 0 Output Value"]
pub struct NRV0_R(crate::FieldReader<bool, bool>);
impl NRV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRV0` writer - Non-Recoverable State 0 Output Value"]
pub struct NRV0_W<'a> {
    w: &'a mut W,
}
impl<'a> NRV0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRV1` reader - Non-Recoverable State 1 Output Value"]
pub struct NRV1_R(crate::FieldReader<bool, bool>);
impl NRV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRV1` writer - Non-Recoverable State 1 Output Value"]
pub struct NRV1_W<'a> {
    w: &'a mut W,
}
impl<'a> NRV1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRV2` reader - Non-Recoverable State 2 Output Value"]
pub struct NRV2_R(crate::FieldReader<bool, bool>);
impl NRV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRV2` writer - Non-Recoverable State 2 Output Value"]
pub struct NRV2_W<'a> {
    w: &'a mut W,
}
impl<'a> NRV2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRV3` reader - Non-Recoverable State 3 Output Value"]
pub struct NRV3_R(crate::FieldReader<bool, bool>);
impl NRV3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRV3` writer - Non-Recoverable State 3 Output Value"]
pub struct NRV3_W<'a> {
    w: &'a mut W,
}
impl<'a> NRV3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRV4` reader - Non-Recoverable State 4 Output Value"]
pub struct NRV4_R(crate::FieldReader<bool, bool>);
impl NRV4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRV4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRV4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRV4` writer - Non-Recoverable State 4 Output Value"]
pub struct NRV4_W<'a> {
    w: &'a mut W,
}
impl<'a> NRV4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRV5` reader - Non-Recoverable State 5 Output Value"]
pub struct NRV5_R(crate::FieldReader<bool, bool>);
impl NRV5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRV5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRV5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRV5` writer - Non-Recoverable State 5 Output Value"]
pub struct NRV5_W<'a> {
    w: &'a mut W,
}
impl<'a> NRV5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRV6` reader - Non-Recoverable State 6 Output Value"]
pub struct NRV6_R(crate::FieldReader<bool, bool>);
impl NRV6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRV6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRV6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRV6` writer - Non-Recoverable State 6 Output Value"]
pub struct NRV6_W<'a> {
    w: &'a mut W,
}
impl<'a> NRV6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NRV7` reader - Non-Recoverable State 7 Output Value"]
pub struct NRV7_R(crate::FieldReader<bool, bool>);
impl NRV7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRV7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRV7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRV7` writer - Non-Recoverable State 7 Output Value"]
pub struct NRV7_W<'a> {
    w: &'a mut W,
}
impl<'a> NRV7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `INVEN0` reader - Output Waveform 0 Inversion"]
pub struct INVEN0_R(crate::FieldReader<bool, bool>);
impl INVEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEN0` writer - Output Waveform 0 Inversion"]
pub struct INVEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `INVEN1` reader - Output Waveform 1 Inversion"]
pub struct INVEN1_R(crate::FieldReader<bool, bool>);
impl INVEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEN1` writer - Output Waveform 1 Inversion"]
pub struct INVEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `INVEN2` reader - Output Waveform 2 Inversion"]
pub struct INVEN2_R(crate::FieldReader<bool, bool>);
impl INVEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEN2` writer - Output Waveform 2 Inversion"]
pub struct INVEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `INVEN3` reader - Output Waveform 3 Inversion"]
pub struct INVEN3_R(crate::FieldReader<bool, bool>);
impl INVEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEN3` writer - Output Waveform 3 Inversion"]
pub struct INVEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `INVEN4` reader - Output Waveform 4 Inversion"]
pub struct INVEN4_R(crate::FieldReader<bool, bool>);
impl INVEN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEN4` writer - Output Waveform 4 Inversion"]
pub struct INVEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `INVEN5` reader - Output Waveform 5 Inversion"]
pub struct INVEN5_R(crate::FieldReader<bool, bool>);
impl INVEN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEN5` writer - Output Waveform 5 Inversion"]
pub struct INVEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `INVEN6` reader - Output Waveform 6 Inversion"]
pub struct INVEN6_R(crate::FieldReader<bool, bool>);
impl INVEN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEN6` writer - Output Waveform 6 Inversion"]
pub struct INVEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `INVEN7` reader - Output Waveform 7 Inversion"]
pub struct INVEN7_R(crate::FieldReader<bool, bool>);
impl INVEN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEN7` writer - Output Waveform 7 Inversion"]
pub struct INVEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FILTERVAL0` reader - Non-Recoverable Fault Input 0 Filter Value"]
pub struct FILTERVAL0_R(crate::FieldReader<u8, u8>);
impl FILTERVAL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTERVAL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTERVAL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTERVAL0` writer - Non-Recoverable Fault Input 0 Filter Value"]
pub struct FILTERVAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTERVAL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `FILTERVAL1` reader - Non-Recoverable Fault Input 1 Filter Value"]
pub struct FILTERVAL1_R(crate::FieldReader<u8, u8>);
impl FILTERVAL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTERVAL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTERVAL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTERVAL1` writer - Non-Recoverable Fault Input 1 Filter Value"]
pub struct FILTERVAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTERVAL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Non-Recoverable State 0 Output Enable"]
    #[inline(always)]
    pub fn nre0(&self) -> NRE0_R {
        NRE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Non-Recoverable State 1 Output Enable"]
    #[inline(always)]
    pub fn nre1(&self) -> NRE1_R {
        NRE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Non-Recoverable State 2 Output Enable"]
    #[inline(always)]
    pub fn nre2(&self) -> NRE2_R {
        NRE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Non-Recoverable State 3 Output Enable"]
    #[inline(always)]
    pub fn nre3(&self) -> NRE3_R {
        NRE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Non-Recoverable State 4 Output Enable"]
    #[inline(always)]
    pub fn nre4(&self) -> NRE4_R {
        NRE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Non-Recoverable State 5 Output Enable"]
    #[inline(always)]
    pub fn nre5(&self) -> NRE5_R {
        NRE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Non-Recoverable State 6 Output Enable"]
    #[inline(always)]
    pub fn nre6(&self) -> NRE6_R {
        NRE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Non-Recoverable State 7 Output Enable"]
    #[inline(always)]
    pub fn nre7(&self) -> NRE7_R {
        NRE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Non-Recoverable State 0 Output Value"]
    #[inline(always)]
    pub fn nrv0(&self) -> NRV0_R {
        NRV0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Non-Recoverable State 1 Output Value"]
    #[inline(always)]
    pub fn nrv1(&self) -> NRV1_R {
        NRV1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Non-Recoverable State 2 Output Value"]
    #[inline(always)]
    pub fn nrv2(&self) -> NRV2_R {
        NRV2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable State 3 Output Value"]
    #[inline(always)]
    pub fn nrv3(&self) -> NRV3_R {
        NRV3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Non-Recoverable State 4 Output Value"]
    #[inline(always)]
    pub fn nrv4(&self) -> NRV4_R {
        NRV4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Non-Recoverable State 5 Output Value"]
    #[inline(always)]
    pub fn nrv5(&self) -> NRV5_R {
        NRV5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable State 6 Output Value"]
    #[inline(always)]
    pub fn nrv6(&self) -> NRV6_R {
        NRV6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable State 7 Output Value"]
    #[inline(always)]
    pub fn nrv7(&self) -> NRV7_R {
        NRV7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Output Waveform 0 Inversion"]
    #[inline(always)]
    pub fn inven0(&self) -> INVEN0_R {
        INVEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Output Waveform 1 Inversion"]
    #[inline(always)]
    pub fn inven1(&self) -> INVEN1_R {
        INVEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output Waveform 2 Inversion"]
    #[inline(always)]
    pub fn inven2(&self) -> INVEN2_R {
        INVEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Output Waveform 3 Inversion"]
    #[inline(always)]
    pub fn inven3(&self) -> INVEN3_R {
        INVEN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Output Waveform 4 Inversion"]
    #[inline(always)]
    pub fn inven4(&self) -> INVEN4_R {
        INVEN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Output Waveform 5 Inversion"]
    #[inline(always)]
    pub fn inven5(&self) -> INVEN5_R {
        INVEN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Output Waveform 6 Inversion"]
    #[inline(always)]
    pub fn inven6(&self) -> INVEN6_R {
        INVEN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Output Waveform 7 Inversion"]
    #[inline(always)]
    pub fn inven7(&self) -> INVEN7_R {
        INVEN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Non-Recoverable Fault Input 0 Filter Value"]
    #[inline(always)]
    pub fn filterval0(&self) -> FILTERVAL0_R {
        FILTERVAL0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Non-Recoverable Fault Input 1 Filter Value"]
    #[inline(always)]
    pub fn filterval1(&self) -> FILTERVAL1_R {
        FILTERVAL1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Recoverable State 0 Output Enable"]
    #[inline(always)]
    pub fn nre0(&mut self) -> NRE0_W {
        NRE0_W { w: self }
    }
    #[doc = "Bit 1 - Non-Recoverable State 1 Output Enable"]
    #[inline(always)]
    pub fn nre1(&mut self) -> NRE1_W {
        NRE1_W { w: self }
    }
    #[doc = "Bit 2 - Non-Recoverable State 2 Output Enable"]
    #[inline(always)]
    pub fn nre2(&mut self) -> NRE2_W {
        NRE2_W { w: self }
    }
    #[doc = "Bit 3 - Non-Recoverable State 3 Output Enable"]
    #[inline(always)]
    pub fn nre3(&mut self) -> NRE3_W {
        NRE3_W { w: self }
    }
    #[doc = "Bit 4 - Non-Recoverable State 4 Output Enable"]
    #[inline(always)]
    pub fn nre4(&mut self) -> NRE4_W {
        NRE4_W { w: self }
    }
    #[doc = "Bit 5 - Non-Recoverable State 5 Output Enable"]
    #[inline(always)]
    pub fn nre5(&mut self) -> NRE5_W {
        NRE5_W { w: self }
    }
    #[doc = "Bit 6 - Non-Recoverable State 6 Output Enable"]
    #[inline(always)]
    pub fn nre6(&mut self) -> NRE6_W {
        NRE6_W { w: self }
    }
    #[doc = "Bit 7 - Non-Recoverable State 7 Output Enable"]
    #[inline(always)]
    pub fn nre7(&mut self) -> NRE7_W {
        NRE7_W { w: self }
    }
    #[doc = "Bit 8 - Non-Recoverable State 0 Output Value"]
    #[inline(always)]
    pub fn nrv0(&mut self) -> NRV0_W {
        NRV0_W { w: self }
    }
    #[doc = "Bit 9 - Non-Recoverable State 1 Output Value"]
    #[inline(always)]
    pub fn nrv1(&mut self) -> NRV1_W {
        NRV1_W { w: self }
    }
    #[doc = "Bit 10 - Non-Recoverable State 2 Output Value"]
    #[inline(always)]
    pub fn nrv2(&mut self) -> NRV2_W {
        NRV2_W { w: self }
    }
    #[doc = "Bit 11 - Non-Recoverable State 3 Output Value"]
    #[inline(always)]
    pub fn nrv3(&mut self) -> NRV3_W {
        NRV3_W { w: self }
    }
    #[doc = "Bit 12 - Non-Recoverable State 4 Output Value"]
    #[inline(always)]
    pub fn nrv4(&mut self) -> NRV4_W {
        NRV4_W { w: self }
    }
    #[doc = "Bit 13 - Non-Recoverable State 5 Output Value"]
    #[inline(always)]
    pub fn nrv5(&mut self) -> NRV5_W {
        NRV5_W { w: self }
    }
    #[doc = "Bit 14 - Non-Recoverable State 6 Output Value"]
    #[inline(always)]
    pub fn nrv6(&mut self) -> NRV6_W {
        NRV6_W { w: self }
    }
    #[doc = "Bit 15 - Non-Recoverable State 7 Output Value"]
    #[inline(always)]
    pub fn nrv7(&mut self) -> NRV7_W {
        NRV7_W { w: self }
    }
    #[doc = "Bit 16 - Output Waveform 0 Inversion"]
    #[inline(always)]
    pub fn inven0(&mut self) -> INVEN0_W {
        INVEN0_W { w: self }
    }
    #[doc = "Bit 17 - Output Waveform 1 Inversion"]
    #[inline(always)]
    pub fn inven1(&mut self) -> INVEN1_W {
        INVEN1_W { w: self }
    }
    #[doc = "Bit 18 - Output Waveform 2 Inversion"]
    #[inline(always)]
    pub fn inven2(&mut self) -> INVEN2_W {
        INVEN2_W { w: self }
    }
    #[doc = "Bit 19 - Output Waveform 3 Inversion"]
    #[inline(always)]
    pub fn inven3(&mut self) -> INVEN3_W {
        INVEN3_W { w: self }
    }
    #[doc = "Bit 20 - Output Waveform 4 Inversion"]
    #[inline(always)]
    pub fn inven4(&mut self) -> INVEN4_W {
        INVEN4_W { w: self }
    }
    #[doc = "Bit 21 - Output Waveform 5 Inversion"]
    #[inline(always)]
    pub fn inven5(&mut self) -> INVEN5_W {
        INVEN5_W { w: self }
    }
    #[doc = "Bit 22 - Output Waveform 6 Inversion"]
    #[inline(always)]
    pub fn inven6(&mut self) -> INVEN6_W {
        INVEN6_W { w: self }
    }
    #[doc = "Bit 23 - Output Waveform 7 Inversion"]
    #[inline(always)]
    pub fn inven7(&mut self) -> INVEN7_W {
        INVEN7_W { w: self }
    }
    #[doc = "Bits 24:27 - Non-Recoverable Fault Input 0 Filter Value"]
    #[inline(always)]
    pub fn filterval0(&mut self) -> FILTERVAL0_W {
        FILTERVAL0_W { w: self }
    }
    #[doc = "Bits 28:31 - Non-Recoverable Fault Input 1 Filter Value"]
    #[inline(always)]
    pub fn filterval1(&mut self) -> FILTERVAL1_W {
        FILTERVAL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Driver Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drvctrl](index.html) module"]
pub struct DRVCTRL_SPEC;
impl crate::RegisterSpec for DRVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [drvctrl::R](R) reader structure"]
impl crate::Readable for DRVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drvctrl::W](W) writer structure"]
impl crate::Writable for DRVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRVCTRL to value 0"]
impl crate::Resettable for DRVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
