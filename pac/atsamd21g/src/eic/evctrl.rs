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
#[doc = "Field `EXTINTEO0` reader - External Interrupt 0 Event Output Enable"]
pub struct EXTINTEO0_R(crate::FieldReader<bool, bool>);
impl EXTINTEO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO0` writer - External Interrupt 0 Event Output Enable"]
pub struct EXTINTEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO0_W<'a> {
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
#[doc = "Field `EXTINTEO1` reader - External Interrupt 1 Event Output Enable"]
pub struct EXTINTEO1_R(crate::FieldReader<bool, bool>);
impl EXTINTEO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO1` writer - External Interrupt 1 Event Output Enable"]
pub struct EXTINTEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO1_W<'a> {
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
#[doc = "Field `EXTINTEO2` reader - External Interrupt 2 Event Output Enable"]
pub struct EXTINTEO2_R(crate::FieldReader<bool, bool>);
impl EXTINTEO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO2` writer - External Interrupt 2 Event Output Enable"]
pub struct EXTINTEO2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO2_W<'a> {
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
#[doc = "Field `EXTINTEO3` reader - External Interrupt 3 Event Output Enable"]
pub struct EXTINTEO3_R(crate::FieldReader<bool, bool>);
impl EXTINTEO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO3` writer - External Interrupt 3 Event Output Enable"]
pub struct EXTINTEO3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO3_W<'a> {
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
#[doc = "Field `EXTINTEO4` reader - External Interrupt 4 Event Output Enable"]
pub struct EXTINTEO4_R(crate::FieldReader<bool, bool>);
impl EXTINTEO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO4` writer - External Interrupt 4 Event Output Enable"]
pub struct EXTINTEO4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO4_W<'a> {
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
#[doc = "Field `EXTINTEO5` reader - External Interrupt 5 Event Output Enable"]
pub struct EXTINTEO5_R(crate::FieldReader<bool, bool>);
impl EXTINTEO5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO5` writer - External Interrupt 5 Event Output Enable"]
pub struct EXTINTEO5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO5_W<'a> {
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
#[doc = "Field `EXTINTEO6` reader - External Interrupt 6 Event Output Enable"]
pub struct EXTINTEO6_R(crate::FieldReader<bool, bool>);
impl EXTINTEO6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO6` writer - External Interrupt 6 Event Output Enable"]
pub struct EXTINTEO6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO6_W<'a> {
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
#[doc = "Field `EXTINTEO7` reader - External Interrupt 7 Event Output Enable"]
pub struct EXTINTEO7_R(crate::FieldReader<bool, bool>);
impl EXTINTEO7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO7` writer - External Interrupt 7 Event Output Enable"]
pub struct EXTINTEO7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO7_W<'a> {
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
#[doc = "Field `EXTINTEO8` reader - External Interrupt 8 Event Output Enable"]
pub struct EXTINTEO8_R(crate::FieldReader<bool, bool>);
impl EXTINTEO8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO8` writer - External Interrupt 8 Event Output Enable"]
pub struct EXTINTEO8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO8_W<'a> {
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
#[doc = "Field `EXTINTEO9` reader - External Interrupt 9 Event Output Enable"]
pub struct EXTINTEO9_R(crate::FieldReader<bool, bool>);
impl EXTINTEO9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO9` writer - External Interrupt 9 Event Output Enable"]
pub struct EXTINTEO9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO9_W<'a> {
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
#[doc = "Field `EXTINTEO10` reader - External Interrupt 10 Event Output Enable"]
pub struct EXTINTEO10_R(crate::FieldReader<bool, bool>);
impl EXTINTEO10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO10` writer - External Interrupt 10 Event Output Enable"]
pub struct EXTINTEO10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO10_W<'a> {
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
#[doc = "Field `EXTINTEO11` reader - External Interrupt 11 Event Output Enable"]
pub struct EXTINTEO11_R(crate::FieldReader<bool, bool>);
impl EXTINTEO11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO11` writer - External Interrupt 11 Event Output Enable"]
pub struct EXTINTEO11_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO11_W<'a> {
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
#[doc = "Field `EXTINTEO12` reader - External Interrupt 12 Event Output Enable"]
pub struct EXTINTEO12_R(crate::FieldReader<bool, bool>);
impl EXTINTEO12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO12` writer - External Interrupt 12 Event Output Enable"]
pub struct EXTINTEO12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO12_W<'a> {
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
#[doc = "Field `EXTINTEO13` reader - External Interrupt 13 Event Output Enable"]
pub struct EXTINTEO13_R(crate::FieldReader<bool, bool>);
impl EXTINTEO13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO13` writer - External Interrupt 13 Event Output Enable"]
pub struct EXTINTEO13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO13_W<'a> {
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
#[doc = "Field `EXTINTEO14` reader - External Interrupt 14 Event Output Enable"]
pub struct EXTINTEO14_R(crate::FieldReader<bool, bool>);
impl EXTINTEO14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO14` writer - External Interrupt 14 Event Output Enable"]
pub struct EXTINTEO14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO14_W<'a> {
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
#[doc = "Field `EXTINTEO15` reader - External Interrupt 15 Event Output Enable"]
pub struct EXTINTEO15_R(crate::FieldReader<bool, bool>);
impl EXTINTEO15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTINTEO15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTINTEO15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTINTEO15` writer - External Interrupt 15 Event Output Enable"]
pub struct EXTINTEO15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO15_W<'a> {
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
    #[doc = "Bit 0 - External Interrupt 0 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo0(&self) -> EXTINTEO0_R {
        EXTINTEO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo1(&self) -> EXTINTEO1_R {
        EXTINTEO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo2(&self) -> EXTINTEO2_R {
        EXTINTEO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo3(&self) -> EXTINTEO3_R {
        EXTINTEO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo4(&self) -> EXTINTEO4_R {
        EXTINTEO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo5(&self) -> EXTINTEO5_R {
        EXTINTEO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo6(&self) -> EXTINTEO6_R {
        EXTINTEO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo7(&self) -> EXTINTEO7_R {
        EXTINTEO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo8(&self) -> EXTINTEO8_R {
        EXTINTEO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo9(&self) -> EXTINTEO9_R {
        EXTINTEO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo10(&self) -> EXTINTEO10_R {
        EXTINTEO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo11(&self) -> EXTINTEO11_R {
        EXTINTEO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo12(&self) -> EXTINTEO12_R {
        EXTINTEO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo13(&self) -> EXTINTEO13_R {
        EXTINTEO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo14(&self) -> EXTINTEO14_R {
        EXTINTEO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo15(&self) -> EXTINTEO15_R {
        EXTINTEO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo0(&mut self) -> EXTINTEO0_W {
        EXTINTEO0_W { w: self }
    }
    #[doc = "Bit 1 - External Interrupt 1 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo1(&mut self) -> EXTINTEO1_W {
        EXTINTEO1_W { w: self }
    }
    #[doc = "Bit 2 - External Interrupt 2 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo2(&mut self) -> EXTINTEO2_W {
        EXTINTEO2_W { w: self }
    }
    #[doc = "Bit 3 - External Interrupt 3 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo3(&mut self) -> EXTINTEO3_W {
        EXTINTEO3_W { w: self }
    }
    #[doc = "Bit 4 - External Interrupt 4 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo4(&mut self) -> EXTINTEO4_W {
        EXTINTEO4_W { w: self }
    }
    #[doc = "Bit 5 - External Interrupt 5 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo5(&mut self) -> EXTINTEO5_W {
        EXTINTEO5_W { w: self }
    }
    #[doc = "Bit 6 - External Interrupt 6 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo6(&mut self) -> EXTINTEO6_W {
        EXTINTEO6_W { w: self }
    }
    #[doc = "Bit 7 - External Interrupt 7 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo7(&mut self) -> EXTINTEO7_W {
        EXTINTEO7_W { w: self }
    }
    #[doc = "Bit 8 - External Interrupt 8 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo8(&mut self) -> EXTINTEO8_W {
        EXTINTEO8_W { w: self }
    }
    #[doc = "Bit 9 - External Interrupt 9 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo9(&mut self) -> EXTINTEO9_W {
        EXTINTEO9_W { w: self }
    }
    #[doc = "Bit 10 - External Interrupt 10 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo10(&mut self) -> EXTINTEO10_W {
        EXTINTEO10_W { w: self }
    }
    #[doc = "Bit 11 - External Interrupt 11 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo11(&mut self) -> EXTINTEO11_W {
        EXTINTEO11_W { w: self }
    }
    #[doc = "Bit 12 - External Interrupt 12 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo12(&mut self) -> EXTINTEO12_W {
        EXTINTEO12_W { w: self }
    }
    #[doc = "Bit 13 - External Interrupt 13 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo13(&mut self) -> EXTINTEO13_W {
        EXTINTEO13_W { w: self }
    }
    #[doc = "Bit 14 - External Interrupt 14 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo14(&mut self) -> EXTINTEO14_W {
        EXTINTEO14_W { w: self }
    }
    #[doc = "Bit 15 - External Interrupt 15 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo15(&mut self) -> EXTINTEO15_W {
        EXTINTEO15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
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
