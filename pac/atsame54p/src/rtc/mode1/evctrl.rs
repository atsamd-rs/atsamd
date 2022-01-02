#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEREO0` reader - Periodic Interval 0 Event Output Enable"]
pub struct PEREO0_R(crate::FieldReader<bool, bool>);
impl PEREO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO0` writer - Periodic Interval 0 Event Output Enable"]
pub struct PEREO0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO0_W<'a> {
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
#[doc = "Field `PEREO1` reader - Periodic Interval 1 Event Output Enable"]
pub struct PEREO1_R(crate::FieldReader<bool, bool>);
impl PEREO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO1` writer - Periodic Interval 1 Event Output Enable"]
pub struct PEREO1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO1_W<'a> {
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
#[doc = "Field `PEREO2` reader - Periodic Interval 2 Event Output Enable"]
pub struct PEREO2_R(crate::FieldReader<bool, bool>);
impl PEREO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO2` writer - Periodic Interval 2 Event Output Enable"]
pub struct PEREO2_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO2_W<'a> {
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
#[doc = "Field `PEREO3` reader - Periodic Interval 3 Event Output Enable"]
pub struct PEREO3_R(crate::FieldReader<bool, bool>);
impl PEREO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO3` writer - Periodic Interval 3 Event Output Enable"]
pub struct PEREO3_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO3_W<'a> {
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
#[doc = "Field `PEREO4` reader - Periodic Interval 4 Event Output Enable"]
pub struct PEREO4_R(crate::FieldReader<bool, bool>);
impl PEREO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO4` writer - Periodic Interval 4 Event Output Enable"]
pub struct PEREO4_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO4_W<'a> {
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
#[doc = "Field `PEREO5` reader - Periodic Interval 5 Event Output Enable"]
pub struct PEREO5_R(crate::FieldReader<bool, bool>);
impl PEREO5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO5` writer - Periodic Interval 5 Event Output Enable"]
pub struct PEREO5_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO5_W<'a> {
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
#[doc = "Field `PEREO6` reader - Periodic Interval 6 Event Output Enable"]
pub struct PEREO6_R(crate::FieldReader<bool, bool>);
impl PEREO6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO6` writer - Periodic Interval 6 Event Output Enable"]
pub struct PEREO6_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO6_W<'a> {
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
#[doc = "Field `PEREO7` reader - Periodic Interval 7 Event Output Enable"]
pub struct PEREO7_R(crate::FieldReader<bool, bool>);
impl PEREO7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEREO7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREO7` writer - Periodic Interval 7 Event Output Enable"]
pub struct PEREO7_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO7_W<'a> {
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
#[doc = "Field `CMPEO0` reader - Compare 0 Event Output Enable"]
pub struct CMPEO0_R(crate::FieldReader<bool, bool>);
impl CMPEO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPEO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPEO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPEO0` writer - Compare 0 Event Output Enable"]
pub struct CMPEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEO0_W<'a> {
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
#[doc = "Field `CMPEO1` reader - Compare 1 Event Output Enable"]
pub struct CMPEO1_R(crate::FieldReader<bool, bool>);
impl CMPEO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPEO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPEO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPEO1` writer - Compare 1 Event Output Enable"]
pub struct CMPEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEO1_W<'a> {
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
#[doc = "Field `CMPEO2` reader - Compare 2 Event Output Enable"]
pub struct CMPEO2_R(crate::FieldReader<bool, bool>);
impl CMPEO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPEO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPEO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPEO2` writer - Compare 2 Event Output Enable"]
pub struct CMPEO2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEO2_W<'a> {
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
#[doc = "Field `CMPEO3` reader - Compare 3 Event Output Enable"]
pub struct CMPEO3_R(crate::FieldReader<bool, bool>);
impl CMPEO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPEO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPEO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPEO3` writer - Compare 3 Event Output Enable"]
pub struct CMPEO3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEO3_W<'a> {
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
#[doc = "Field `TAMPEREO` reader - Tamper Event Output Enable"]
pub struct TAMPEREO_R(crate::FieldReader<bool, bool>);
impl TAMPEREO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAMPEREO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPEREO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPEREO` writer - Tamper Event Output Enable"]
pub struct TAMPEREO_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPEREO_W<'a> {
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
#[doc = "Field `OVFEO` reader - Overflow Event Output Enable"]
pub struct OVFEO_R(crate::FieldReader<bool, bool>);
impl OVFEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVFEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFEO` writer - Overflow Event Output Enable"]
pub struct OVFEO_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFEO_W<'a> {
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
#[doc = "Field `TAMPEVEI` reader - Tamper Event Input Enable"]
pub struct TAMPEVEI_R(crate::FieldReader<bool, bool>);
impl TAMPEVEI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAMPEVEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPEVEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPEVEI` writer - Tamper Event Input Enable"]
pub struct TAMPEVEI_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPEVEI_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline(always)]
    pub fn pereo0(&self) -> PEREO0_R {
        PEREO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline(always)]
    pub fn pereo1(&self) -> PEREO1_R {
        PEREO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline(always)]
    pub fn pereo2(&self) -> PEREO2_R {
        PEREO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline(always)]
    pub fn pereo3(&self) -> PEREO3_R {
        PEREO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline(always)]
    pub fn pereo4(&self) -> PEREO4_R {
        PEREO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline(always)]
    pub fn pereo5(&self) -> PEREO5_R {
        PEREO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline(always)]
    pub fn pereo6(&self) -> PEREO6_R {
        PEREO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline(always)]
    pub fn pereo7(&self) -> PEREO7_R {
        PEREO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Compare 0 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo0(&self) -> CMPEO0_R {
        CMPEO0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Compare 1 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo1(&self) -> CMPEO1_R {
        CMPEO1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Compare 2 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo2(&self) -> CMPEO2_R {
        CMPEO2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Compare 3 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo3(&self) -> CMPEO3_R {
        CMPEO3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tamper Event Output Enable"]
    #[inline(always)]
    pub fn tampereo(&self) -> TAMPEREO_R {
        TAMPEREO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Tamper Event Input Enable"]
    #[inline(always)]
    pub fn tampevei(&self) -> TAMPEVEI_R {
        TAMPEVEI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline(always)]
    pub fn pereo0(&mut self) -> PEREO0_W {
        PEREO0_W { w: self }
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline(always)]
    pub fn pereo1(&mut self) -> PEREO1_W {
        PEREO1_W { w: self }
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline(always)]
    pub fn pereo2(&mut self) -> PEREO2_W {
        PEREO2_W { w: self }
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline(always)]
    pub fn pereo3(&mut self) -> PEREO3_W {
        PEREO3_W { w: self }
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline(always)]
    pub fn pereo4(&mut self) -> PEREO4_W {
        PEREO4_W { w: self }
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline(always)]
    pub fn pereo5(&mut self) -> PEREO5_W {
        PEREO5_W { w: self }
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline(always)]
    pub fn pereo6(&mut self) -> PEREO6_W {
        PEREO6_W { w: self }
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline(always)]
    pub fn pereo7(&mut self) -> PEREO7_W {
        PEREO7_W { w: self }
    }
    #[doc = "Bit 8 - Compare 0 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo0(&mut self) -> CMPEO0_W {
        CMPEO0_W { w: self }
    }
    #[doc = "Bit 9 - Compare 1 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo1(&mut self) -> CMPEO1_W {
        CMPEO1_W { w: self }
    }
    #[doc = "Bit 10 - Compare 2 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo2(&mut self) -> CMPEO2_W {
        CMPEO2_W { w: self }
    }
    #[doc = "Bit 11 - Compare 3 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo3(&mut self) -> CMPEO3_W {
        CMPEO3_W { w: self }
    }
    #[doc = "Bit 14 - Tamper Event Output Enable"]
    #[inline(always)]
    pub fn tampereo(&mut self) -> TAMPEREO_W {
        TAMPEREO_W { w: self }
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&mut self) -> OVFEO_W {
        OVFEO_W { w: self }
    }
    #[doc = "Bit 16 - Tamper Event Input Enable"]
    #[inline(always)]
    pub fn tampevei(&mut self) -> TAMPEVEI_W {
        TAMPEVEI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE1 Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
