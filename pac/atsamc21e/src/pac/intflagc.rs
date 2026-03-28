#[doc = "Register `INTFLAGC` reader"]
pub struct R(crate::R<INTFLAGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGC` writer"]
pub struct W(crate::W<INTFLAGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGC_SPEC>;
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
impl From<crate::W<INTFLAGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS"]
pub struct EVSYS__R(crate::FieldReader<bool, bool>);
impl EVSYS__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVSYS__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVSYS__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVSYS_` writer - EVSYS"]
pub struct EVSYS__W<'a> {
    w: &'a mut W,
}
impl<'a> EVSYS__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SERCOM0_` reader - SERCOM0"]
pub struct SERCOM0__R(crate::FieldReader<bool, bool>);
impl SERCOM0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM0_` writer - SERCOM0"]
pub struct SERCOM0__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SERCOM1_` reader - SERCOM1"]
pub struct SERCOM1__R(crate::FieldReader<bool, bool>);
impl SERCOM1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM1_` writer - SERCOM1"]
pub struct SERCOM1__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM1__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SERCOM2_` reader - SERCOM2"]
pub struct SERCOM2__R(crate::FieldReader<bool, bool>);
impl SERCOM2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM2_` writer - SERCOM2"]
pub struct SERCOM2__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM2__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SERCOM3_` reader - SERCOM3"]
pub struct SERCOM3__R(crate::FieldReader<bool, bool>);
impl SERCOM3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM3_` writer - SERCOM3"]
pub struct SERCOM3__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM3__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `CAN0_` reader - CAN0"]
pub struct CAN0__R(crate::FieldReader<bool, bool>);
impl CAN0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAN0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN0_` writer - CAN0"]
pub struct CAN0__W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TCC0_` reader - TCC0"]
pub struct TCC0__R(crate::FieldReader<bool, bool>);
impl TCC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC0_` writer - TCC0"]
pub struct TCC0__W<'a> {
    w: &'a mut W,
}
impl<'a> TCC0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TCC1_` reader - TCC1"]
pub struct TCC1__R(crate::FieldReader<bool, bool>);
impl TCC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC1_` writer - TCC1"]
pub struct TCC1__W<'a> {
    w: &'a mut W,
}
impl<'a> TCC1__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TCC2_` reader - TCC2"]
pub struct TCC2__R(crate::FieldReader<bool, bool>);
impl TCC2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC2_` writer - TCC2"]
pub struct TCC2__W<'a> {
    w: &'a mut W,
}
impl<'a> TCC2__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TC0_` reader - TC0"]
pub struct TC0__R(crate::FieldReader<bool, bool>);
impl TC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC0_` writer - TC0"]
pub struct TC0__W<'a> {
    w: &'a mut W,
}
impl<'a> TC0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TC1_` reader - TC1"]
pub struct TC1__R(crate::FieldReader<bool, bool>);
impl TC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC1_` writer - TC1"]
pub struct TC1__W<'a> {
    w: &'a mut W,
}
impl<'a> TC1__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TC2_` reader - TC2"]
pub struct TC2__R(crate::FieldReader<bool, bool>);
impl TC2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC2_` writer - TC2"]
pub struct TC2__W<'a> {
    w: &'a mut W,
}
impl<'a> TC2__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TC3_` reader - TC3"]
pub struct TC3__R(crate::FieldReader<bool, bool>);
impl TC3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC3_` writer - TC3"]
pub struct TC3__W<'a> {
    w: &'a mut W,
}
impl<'a> TC3__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TC4_` reader - TC4"]
pub struct TC4__R(crate::FieldReader<bool, bool>);
impl TC4__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC4__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC4__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC4_` writer - TC4"]
pub struct TC4__W<'a> {
    w: &'a mut W,
}
impl<'a> TC4__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ADC0_` reader - ADC0"]
pub struct ADC0__R(crate::FieldReader<bool, bool>);
impl ADC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_` writer - ADC0"]
pub struct ADC0__W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ADC1_` reader - ADC1"]
pub struct ADC1__R(crate::FieldReader<bool, bool>);
impl ADC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_` writer - ADC1"]
pub struct ADC1__W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SDADC_` reader - SDADC"]
pub struct SDADC__R(crate::FieldReader<bool, bool>);
impl SDADC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDADC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDADC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDADC_` writer - SDADC"]
pub struct SDADC__W<'a> {
    w: &'a mut W,
}
impl<'a> SDADC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `AC_` reader - AC"]
pub struct AC__R(crate::FieldReader<bool, bool>);
impl AC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AC_` writer - AC"]
pub struct AC__W<'a> {
    w: &'a mut W,
}
impl<'a> AC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `DAC_` reader - DAC"]
pub struct DAC__R(crate::FieldReader<bool, bool>);
impl DAC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_` writer - DAC"]
pub struct DAC__W<'a> {
    w: &'a mut W,
}
impl<'a> DAC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PTC_` reader - PTC"]
pub struct PTC__R(crate::FieldReader<bool, bool>);
impl PTC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PTC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTC_` writer - PTC"]
pub struct PTC__W<'a> {
    w: &'a mut W,
}
impl<'a> PTC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `CCL_` reader - CCL"]
pub struct CCL__R(crate::FieldReader<bool, bool>);
impl CCL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCL_` writer - CCL"]
pub struct CCL__W<'a> {
    w: &'a mut W,
}
impl<'a> CCL__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - EVSYS"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM0"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SERCOM1"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SERCOM2"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SERCOM3"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CAN0"]
    #[inline(always)]
    pub fn can0_(&self) -> CAN0__R {
        CAN0__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TCC0"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TCC1"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TCC2"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TC0"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TC1"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TC2"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TC3"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TC4"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC0"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC1"]
    #[inline(always)]
    pub fn adc1_(&self) -> ADC1__R {
        ADC1__R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SDADC"]
    #[inline(always)]
    pub fn sdadc_(&self) -> SDADC__R {
        SDADC__R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - AC"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DAC"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PTC"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CCL"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EVSYS"]
    #[inline(always)]
    pub fn evsys_(&mut self) -> EVSYS__W {
        EVSYS__W { w: self }
    }
    #[doc = "Bit 1 - SERCOM0"]
    #[inline(always)]
    pub fn sercom0_(&mut self) -> SERCOM0__W {
        SERCOM0__W { w: self }
    }
    #[doc = "Bit 2 - SERCOM1"]
    #[inline(always)]
    pub fn sercom1_(&mut self) -> SERCOM1__W {
        SERCOM1__W { w: self }
    }
    #[doc = "Bit 3 - SERCOM2"]
    #[inline(always)]
    pub fn sercom2_(&mut self) -> SERCOM2__W {
        SERCOM2__W { w: self }
    }
    #[doc = "Bit 4 - SERCOM3"]
    #[inline(always)]
    pub fn sercom3_(&mut self) -> SERCOM3__W {
        SERCOM3__W { w: self }
    }
    #[doc = "Bit 7 - CAN0"]
    #[inline(always)]
    pub fn can0_(&mut self) -> CAN0__W {
        CAN0__W { w: self }
    }
    #[doc = "Bit 9 - TCC0"]
    #[inline(always)]
    pub fn tcc0_(&mut self) -> TCC0__W {
        TCC0__W { w: self }
    }
    #[doc = "Bit 10 - TCC1"]
    #[inline(always)]
    pub fn tcc1_(&mut self) -> TCC1__W {
        TCC1__W { w: self }
    }
    #[doc = "Bit 11 - TCC2"]
    #[inline(always)]
    pub fn tcc2_(&mut self) -> TCC2__W {
        TCC2__W { w: self }
    }
    #[doc = "Bit 12 - TC0"]
    #[inline(always)]
    pub fn tc0_(&mut self) -> TC0__W {
        TC0__W { w: self }
    }
    #[doc = "Bit 13 - TC1"]
    #[inline(always)]
    pub fn tc1_(&mut self) -> TC1__W {
        TC1__W { w: self }
    }
    #[doc = "Bit 14 - TC2"]
    #[inline(always)]
    pub fn tc2_(&mut self) -> TC2__W {
        TC2__W { w: self }
    }
    #[doc = "Bit 15 - TC3"]
    #[inline(always)]
    pub fn tc3_(&mut self) -> TC3__W {
        TC3__W { w: self }
    }
    #[doc = "Bit 16 - TC4"]
    #[inline(always)]
    pub fn tc4_(&mut self) -> TC4__W {
        TC4__W { w: self }
    }
    #[doc = "Bit 17 - ADC0"]
    #[inline(always)]
    pub fn adc0_(&mut self) -> ADC0__W {
        ADC0__W { w: self }
    }
    #[doc = "Bit 18 - ADC1"]
    #[inline(always)]
    pub fn adc1_(&mut self) -> ADC1__W {
        ADC1__W { w: self }
    }
    #[doc = "Bit 19 - SDADC"]
    #[inline(always)]
    pub fn sdadc_(&mut self) -> SDADC__W {
        SDADC__W { w: self }
    }
    #[doc = "Bit 20 - AC"]
    #[inline(always)]
    pub fn ac_(&mut self) -> AC__W {
        AC__W { w: self }
    }
    #[doc = "Bit 21 - DAC"]
    #[inline(always)]
    pub fn dac_(&mut self) -> DAC__W {
        DAC__W { w: self }
    }
    #[doc = "Bit 22 - PTC"]
    #[inline(always)]
    pub fn ptc_(&mut self) -> PTC__W {
        PTC__W { w: self }
    }
    #[doc = "Bit 23 - CCL"]
    #[inline(always)]
    pub fn ccl_(&mut self) -> CCL__W {
        CCL__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagc](index.html) module"]
pub struct INTFLAGC_SPEC;
impl crate::RegisterSpec for INTFLAGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagc::R](R) reader structure"]
impl crate::Readable for INTFLAGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagc::W](W) writer structure"]
impl crate::Writable for INTFLAGC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGC to value 0"]
impl crate::Resettable for INTFLAGC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
