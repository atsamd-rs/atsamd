#[doc = "Register `PMC_FSPR` reader"]
pub struct R(crate::R<PMC_FSPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_FSPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_FSPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_FSPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_FSPR` writer"]
pub struct W(crate::W<PMC_FSPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_FSPR_SPEC>;
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
impl From<crate::W<PMC_FSPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_FSPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSTP0` reader - Fast Startup Input Polarity 0"]
pub struct FSTP0_R(crate::FieldReader<bool, bool>);
impl FSTP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP0` writer - Fast Startup Input Polarity 0"]
pub struct FSTP0_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP1` reader - Fast Startup Input Polarity 1"]
pub struct FSTP1_R(crate::FieldReader<bool, bool>);
impl FSTP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP1` writer - Fast Startup Input Polarity 1"]
pub struct FSTP1_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP2` reader - Fast Startup Input Polarity 2"]
pub struct FSTP2_R(crate::FieldReader<bool, bool>);
impl FSTP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP2` writer - Fast Startup Input Polarity 2"]
pub struct FSTP2_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP3` reader - Fast Startup Input Polarity 3"]
pub struct FSTP3_R(crate::FieldReader<bool, bool>);
impl FSTP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP3` writer - Fast Startup Input Polarity 3"]
pub struct FSTP3_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP4` reader - Fast Startup Input Polarity 4"]
pub struct FSTP4_R(crate::FieldReader<bool, bool>);
impl FSTP4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP4` writer - Fast Startup Input Polarity 4"]
pub struct FSTP4_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP5` reader - Fast Startup Input Polarity 5"]
pub struct FSTP5_R(crate::FieldReader<bool, bool>);
impl FSTP5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP5` writer - Fast Startup Input Polarity 5"]
pub struct FSTP5_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP6` reader - Fast Startup Input Polarity 6"]
pub struct FSTP6_R(crate::FieldReader<bool, bool>);
impl FSTP6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP6` writer - Fast Startup Input Polarity 6"]
pub struct FSTP6_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP7` reader - Fast Startup Input Polarity 7"]
pub struct FSTP7_R(crate::FieldReader<bool, bool>);
impl FSTP7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP7` writer - Fast Startup Input Polarity 7"]
pub struct FSTP7_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP8` reader - Fast Startup Input Polarity 8"]
pub struct FSTP8_R(crate::FieldReader<bool, bool>);
impl FSTP8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP8` writer - Fast Startup Input Polarity 8"]
pub struct FSTP8_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP9` reader - Fast Startup Input Polarity 9"]
pub struct FSTP9_R(crate::FieldReader<bool, bool>);
impl FSTP9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP9` writer - Fast Startup Input Polarity 9"]
pub struct FSTP9_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP10` reader - Fast Startup Input Polarity 10"]
pub struct FSTP10_R(crate::FieldReader<bool, bool>);
impl FSTP10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP10` writer - Fast Startup Input Polarity 10"]
pub struct FSTP10_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP11` reader - Fast Startup Input Polarity 11"]
pub struct FSTP11_R(crate::FieldReader<bool, bool>);
impl FSTP11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP11` writer - Fast Startup Input Polarity 11"]
pub struct FSTP11_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP12` reader - Fast Startup Input Polarity 12"]
pub struct FSTP12_R(crate::FieldReader<bool, bool>);
impl FSTP12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP12` writer - Fast Startup Input Polarity 12"]
pub struct FSTP12_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP13` reader - Fast Startup Input Polarity 13"]
pub struct FSTP13_R(crate::FieldReader<bool, bool>);
impl FSTP13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP13` writer - Fast Startup Input Polarity 13"]
pub struct FSTP13_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP14` reader - Fast Startup Input Polarity 14"]
pub struct FSTP14_R(crate::FieldReader<bool, bool>);
impl FSTP14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP14` writer - Fast Startup Input Polarity 14"]
pub struct FSTP14_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FSTP15` reader - Fast Startup Input Polarity 15"]
pub struct FSTP15_R(crate::FieldReader<bool, bool>);
impl FSTP15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTP15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTP15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTP15` writer - Fast Startup Input Polarity 15"]
pub struct FSTP15_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - Fast Startup Input Polarity 0"]
    #[inline(always)]
    pub fn fstp0(&self) -> FSTP0_R {
        FSTP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Polarity 1"]
    #[inline(always)]
    pub fn fstp1(&self) -> FSTP1_R {
        FSTP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast Startup Input Polarity 2"]
    #[inline(always)]
    pub fn fstp2(&self) -> FSTP2_R {
        FSTP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast Startup Input Polarity 3"]
    #[inline(always)]
    pub fn fstp3(&self) -> FSTP3_R {
        FSTP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast Startup Input Polarity 4"]
    #[inline(always)]
    pub fn fstp4(&self) -> FSTP4_R {
        FSTP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast Startup Input Polarity 5"]
    #[inline(always)]
    pub fn fstp5(&self) -> FSTP5_R {
        FSTP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast Startup Input Polarity 6"]
    #[inline(always)]
    pub fn fstp6(&self) -> FSTP6_R {
        FSTP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast Startup Input Polarity 7"]
    #[inline(always)]
    pub fn fstp7(&self) -> FSTP7_R {
        FSTP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast Startup Input Polarity 8"]
    #[inline(always)]
    pub fn fstp8(&self) -> FSTP8_R {
        FSTP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast Startup Input Polarity 9"]
    #[inline(always)]
    pub fn fstp9(&self) -> FSTP9_R {
        FSTP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast Startup Input Polarity 10"]
    #[inline(always)]
    pub fn fstp10(&self) -> FSTP10_R {
        FSTP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast Startup Input Polarity 11"]
    #[inline(always)]
    pub fn fstp11(&self) -> FSTP11_R {
        FSTP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast Startup Input Polarity 12"]
    #[inline(always)]
    pub fn fstp12(&self) -> FSTP12_R {
        FSTP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast Startup Input Polarity 13"]
    #[inline(always)]
    pub fn fstp13(&self) -> FSTP13_R {
        FSTP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast Startup Input Polarity 14"]
    #[inline(always)]
    pub fn fstp14(&self) -> FSTP14_R {
        FSTP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast Startup Input Polarity 15"]
    #[inline(always)]
    pub fn fstp15(&self) -> FSTP15_R {
        FSTP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Startup Input Polarity 0"]
    #[inline(always)]
    pub fn fstp0(&mut self) -> FSTP0_W {
        FSTP0_W { w: self }
    }
    #[doc = "Bit 1 - Fast Startup Input Polarity 1"]
    #[inline(always)]
    pub fn fstp1(&mut self) -> FSTP1_W {
        FSTP1_W { w: self }
    }
    #[doc = "Bit 2 - Fast Startup Input Polarity 2"]
    #[inline(always)]
    pub fn fstp2(&mut self) -> FSTP2_W {
        FSTP2_W { w: self }
    }
    #[doc = "Bit 3 - Fast Startup Input Polarity 3"]
    #[inline(always)]
    pub fn fstp3(&mut self) -> FSTP3_W {
        FSTP3_W { w: self }
    }
    #[doc = "Bit 4 - Fast Startup Input Polarity 4"]
    #[inline(always)]
    pub fn fstp4(&mut self) -> FSTP4_W {
        FSTP4_W { w: self }
    }
    #[doc = "Bit 5 - Fast Startup Input Polarity 5"]
    #[inline(always)]
    pub fn fstp5(&mut self) -> FSTP5_W {
        FSTP5_W { w: self }
    }
    #[doc = "Bit 6 - Fast Startup Input Polarity 6"]
    #[inline(always)]
    pub fn fstp6(&mut self) -> FSTP6_W {
        FSTP6_W { w: self }
    }
    #[doc = "Bit 7 - Fast Startup Input Polarity 7"]
    #[inline(always)]
    pub fn fstp7(&mut self) -> FSTP7_W {
        FSTP7_W { w: self }
    }
    #[doc = "Bit 8 - Fast Startup Input Polarity 8"]
    #[inline(always)]
    pub fn fstp8(&mut self) -> FSTP8_W {
        FSTP8_W { w: self }
    }
    #[doc = "Bit 9 - Fast Startup Input Polarity 9"]
    #[inline(always)]
    pub fn fstp9(&mut self) -> FSTP9_W {
        FSTP9_W { w: self }
    }
    #[doc = "Bit 10 - Fast Startup Input Polarity 10"]
    #[inline(always)]
    pub fn fstp10(&mut self) -> FSTP10_W {
        FSTP10_W { w: self }
    }
    #[doc = "Bit 11 - Fast Startup Input Polarity 11"]
    #[inline(always)]
    pub fn fstp11(&mut self) -> FSTP11_W {
        FSTP11_W { w: self }
    }
    #[doc = "Bit 12 - Fast Startup Input Polarity 12"]
    #[inline(always)]
    pub fn fstp12(&mut self) -> FSTP12_W {
        FSTP12_W { w: self }
    }
    #[doc = "Bit 13 - Fast Startup Input Polarity 13"]
    #[inline(always)]
    pub fn fstp13(&mut self) -> FSTP13_W {
        FSTP13_W { w: self }
    }
    #[doc = "Bit 14 - Fast Startup Input Polarity 14"]
    #[inline(always)]
    pub fn fstp14(&mut self) -> FSTP14_W {
        FSTP14_W { w: self }
    }
    #[doc = "Bit 15 - Fast Startup Input Polarity 15"]
    #[inline(always)]
    pub fn fstp15(&mut self) -> FSTP15_W {
        FSTP15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Startup Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_fspr](index.html) module"]
pub struct PMC_FSPR_SPEC;
impl crate::RegisterSpec for PMC_FSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_fspr::R](R) reader structure"]
impl crate::Readable for PMC_FSPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_fspr::W](W) writer structure"]
impl crate::Writable for PMC_FSPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_FSPR to value 0"]
impl crate::Resettable for PMC_FSPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
