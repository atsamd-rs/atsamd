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
#[doc = "Field `COMPEO0` reader - Comparator 0 Event Output Enable"]
pub struct COMPEO0_R(crate::FieldReader<bool, bool>);
impl COMPEO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPEO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPEO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPEO0` writer - Comparator 0 Event Output Enable"]
pub struct COMPEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `COMPEO1` reader - Comparator 1 Event Output Enable"]
pub struct COMPEO1_R(crate::FieldReader<bool, bool>);
impl COMPEO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPEO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPEO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPEO1` writer - Comparator 1 Event Output Enable"]
pub struct COMPEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `COMPEO2` reader - Comparator 2 Event Output Enable"]
pub struct COMPEO2_R(crate::FieldReader<bool, bool>);
impl COMPEO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPEO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPEO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPEO2` writer - Comparator 2 Event Output Enable"]
pub struct COMPEO2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `COMPEO3` reader - Comparator 3 Event Output Enable"]
pub struct COMPEO3_R(crate::FieldReader<bool, bool>);
impl COMPEO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPEO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPEO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPEO3` writer - Comparator 3 Event Output Enable"]
pub struct COMPEO3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `WINEO0` reader - Window 0 Event Output Enable"]
pub struct WINEO0_R(crate::FieldReader<bool, bool>);
impl WINEO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINEO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINEO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINEO0` writer - Window 0 Event Output Enable"]
pub struct WINEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> WINEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `WINEO1` reader - Window 1 Event Output Enable"]
pub struct WINEO1_R(crate::FieldReader<bool, bool>);
impl WINEO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINEO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINEO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINEO1` writer - Window 1 Event Output Enable"]
pub struct WINEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> WINEO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `COMPEI0` reader - Comparator 0 Event Input Enable"]
pub struct COMPEI0_R(crate::FieldReader<bool, bool>);
impl COMPEI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPEI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPEI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPEI0` writer - Comparator 0 Event Input Enable"]
pub struct COMPEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEI0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `COMPEI1` reader - Comparator 1 Event Input Enable"]
pub struct COMPEI1_R(crate::FieldReader<bool, bool>);
impl COMPEI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPEI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPEI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPEI1` writer - Comparator 1 Event Input Enable"]
pub struct COMPEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `COMPEI2` reader - Comparator 2 Event Input Enable"]
pub struct COMPEI2_R(crate::FieldReader<bool, bool>);
impl COMPEI2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPEI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPEI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPEI2` writer - Comparator 2 Event Input Enable"]
pub struct COMPEI2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEI2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `COMPEI3` reader - Comparator 3 Event Input Enable"]
pub struct COMPEI3_R(crate::FieldReader<bool, bool>);
impl COMPEI3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPEI3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPEI3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPEI3` writer - Comparator 3 Event Input Enable"]
pub struct COMPEI3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEI3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `INVEI0` reader - Comparator 0 Input Event Invert Enable"]
pub struct INVEI0_R(crate::FieldReader<bool, bool>);
impl INVEI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEI0` writer - Comparator 0 Input Event Invert Enable"]
pub struct INVEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `INVEI1` reader - Comparator 1 Input Event Invert Enable"]
pub struct INVEI1_R(crate::FieldReader<bool, bool>);
impl INVEI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEI1` writer - Comparator 1 Input Event Invert Enable"]
pub struct INVEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `INVEI2` reader - Comparator 2 Input Event Invert Enable"]
pub struct INVEI2_R(crate::FieldReader<bool, bool>);
impl INVEI2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEI2` writer - Comparator 2 Input Event Invert Enable"]
pub struct INVEI2_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u16 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `INVEI3` reader - Comparator 3 Input Event Invert Enable"]
pub struct INVEI3_R(crate::FieldReader<bool, bool>);
impl INVEI3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEI3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEI3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEI3` writer - Comparator 3 Input Event Invert Enable"]
pub struct INVEI3_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&self) -> COMPEO0_R {
        COMPEO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&self) -> COMPEO1_R {
        COMPEO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Event Output Enable"]
    #[inline(always)]
    pub fn compeo2(&self) -> COMPEO2_R {
        COMPEO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator 3 Event Output Enable"]
    #[inline(always)]
    pub fn compeo3(&self) -> COMPEO3_R {
        COMPEO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&self) -> WINEO0_R {
        WINEO0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Window 1 Event Output Enable"]
    #[inline(always)]
    pub fn wineo1(&self) -> WINEO1_R {
        WINEO1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    pub fn compei0(&self) -> COMPEI0_R {
        COMPEI0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    pub fn compei1(&self) -> COMPEI1_R {
        COMPEI1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Comparator 2 Event Input Enable"]
    #[inline(always)]
    pub fn compei2(&self) -> COMPEI2_R {
        COMPEI2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Comparator 3 Event Input Enable"]
    #[inline(always)]
    pub fn compei3(&self) -> COMPEI3_R {
        COMPEI3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei0(&self) -> INVEI0_R {
        INVEI0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei1(&self) -> INVEI1_R {
        INVEI1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Comparator 2 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei2(&self) -> INVEI2_R {
        INVEI2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 3 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei3(&self) -> INVEI3_R {
        INVEI3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&mut self) -> COMPEO0_W {
        COMPEO0_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&mut self) -> COMPEO1_W {
        COMPEO1_W { w: self }
    }
    #[doc = "Bit 2 - Comparator 2 Event Output Enable"]
    #[inline(always)]
    pub fn compeo2(&mut self) -> COMPEO2_W {
        COMPEO2_W { w: self }
    }
    #[doc = "Bit 3 - Comparator 3 Event Output Enable"]
    #[inline(always)]
    pub fn compeo3(&mut self) -> COMPEO3_W {
        COMPEO3_W { w: self }
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&mut self) -> WINEO0_W {
        WINEO0_W { w: self }
    }
    #[doc = "Bit 5 - Window 1 Event Output Enable"]
    #[inline(always)]
    pub fn wineo1(&mut self) -> WINEO1_W {
        WINEO1_W { w: self }
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    pub fn compei0(&mut self) -> COMPEI0_W {
        COMPEI0_W { w: self }
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    pub fn compei1(&mut self) -> COMPEI1_W {
        COMPEI1_W { w: self }
    }
    #[doc = "Bit 10 - Comparator 2 Event Input Enable"]
    #[inline(always)]
    pub fn compei2(&mut self) -> COMPEI2_W {
        COMPEI2_W { w: self }
    }
    #[doc = "Bit 11 - Comparator 3 Event Input Enable"]
    #[inline(always)]
    pub fn compei3(&mut self) -> COMPEI3_W {
        COMPEI3_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei0(&mut self) -> INVEI0_W {
        INVEI0_W { w: self }
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei1(&mut self) -> INVEI1_W {
        INVEI1_W { w: self }
    }
    #[doc = "Bit 14 - Comparator 2 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei2(&mut self) -> INVEI2_W {
        INVEI2_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 3 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei3(&mut self) -> INVEI3_W {
        INVEI3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u16;
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
