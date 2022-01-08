#[doc = "Register `WAKEUP` reader"]
pub struct R(crate::R<WAKEUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP` writer"]
pub struct W(crate::W<WAKEUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_SPEC>;
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
impl From<crate::W<WAKEUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUPEN0` reader - External Interrupt 0 Wake-up Enable"]
pub struct WAKEUPEN0_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN0` writer - External Interrupt 0 Wake-up Enable"]
pub struct WAKEUPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN0_W<'a> {
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
#[doc = "Field `WAKEUPEN1` reader - External Interrupt 1 Wake-up Enable"]
pub struct WAKEUPEN1_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN1` writer - External Interrupt 1 Wake-up Enable"]
pub struct WAKEUPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN1_W<'a> {
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
#[doc = "Field `WAKEUPEN2` reader - External Interrupt 2 Wake-up Enable"]
pub struct WAKEUPEN2_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN2` writer - External Interrupt 2 Wake-up Enable"]
pub struct WAKEUPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN2_W<'a> {
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
#[doc = "Field `WAKEUPEN3` reader - External Interrupt 3 Wake-up Enable"]
pub struct WAKEUPEN3_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN3` writer - External Interrupt 3 Wake-up Enable"]
pub struct WAKEUPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN3_W<'a> {
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
#[doc = "Field `WAKEUPEN4` reader - External Interrupt 4 Wake-up Enable"]
pub struct WAKEUPEN4_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN4` writer - External Interrupt 4 Wake-up Enable"]
pub struct WAKEUPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN4_W<'a> {
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
#[doc = "Field `WAKEUPEN5` reader - External Interrupt 5 Wake-up Enable"]
pub struct WAKEUPEN5_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN5` writer - External Interrupt 5 Wake-up Enable"]
pub struct WAKEUPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN5_W<'a> {
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
#[doc = "Field `WAKEUPEN6` reader - External Interrupt 6 Wake-up Enable"]
pub struct WAKEUPEN6_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN6` writer - External Interrupt 6 Wake-up Enable"]
pub struct WAKEUPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN6_W<'a> {
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
#[doc = "Field `WAKEUPEN7` reader - External Interrupt 7 Wake-up Enable"]
pub struct WAKEUPEN7_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN7` writer - External Interrupt 7 Wake-up Enable"]
pub struct WAKEUPEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN7_W<'a> {
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
#[doc = "Field `WAKEUPEN8` reader - External Interrupt 8 Wake-up Enable"]
pub struct WAKEUPEN8_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN8` writer - External Interrupt 8 Wake-up Enable"]
pub struct WAKEUPEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN8_W<'a> {
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
#[doc = "Field `WAKEUPEN9` reader - External Interrupt 9 Wake-up Enable"]
pub struct WAKEUPEN9_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN9` writer - External Interrupt 9 Wake-up Enable"]
pub struct WAKEUPEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN9_W<'a> {
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
#[doc = "Field `WAKEUPEN10` reader - External Interrupt 10 Wake-up Enable"]
pub struct WAKEUPEN10_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN10` writer - External Interrupt 10 Wake-up Enable"]
pub struct WAKEUPEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN10_W<'a> {
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
#[doc = "Field `WAKEUPEN11` reader - External Interrupt 11 Wake-up Enable"]
pub struct WAKEUPEN11_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN11` writer - External Interrupt 11 Wake-up Enable"]
pub struct WAKEUPEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN11_W<'a> {
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
#[doc = "Field `WAKEUPEN12` reader - External Interrupt 12 Wake-up Enable"]
pub struct WAKEUPEN12_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN12` writer - External Interrupt 12 Wake-up Enable"]
pub struct WAKEUPEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN12_W<'a> {
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
#[doc = "Field `WAKEUPEN13` reader - External Interrupt 13 Wake-up Enable"]
pub struct WAKEUPEN13_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN13` writer - External Interrupt 13 Wake-up Enable"]
pub struct WAKEUPEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN13_W<'a> {
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
#[doc = "Field `WAKEUPEN14` reader - External Interrupt 14 Wake-up Enable"]
pub struct WAKEUPEN14_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN14` writer - External Interrupt 14 Wake-up Enable"]
pub struct WAKEUPEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN14_W<'a> {
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
#[doc = "Field `WAKEUPEN15` reader - External Interrupt 15 Wake-up Enable"]
pub struct WAKEUPEN15_R(crate::FieldReader<bool, bool>);
impl WAKEUPEN15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPEN15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPEN15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPEN15` writer - External Interrupt 15 Wake-up Enable"]
pub struct WAKEUPEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN15_W<'a> {
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
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen0(&self) -> WAKEUPEN0_R {
        WAKEUPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen1(&self) -> WAKEUPEN1_R {
        WAKEUPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen2(&self) -> WAKEUPEN2_R {
        WAKEUPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen3(&self) -> WAKEUPEN3_R {
        WAKEUPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen4(&self) -> WAKEUPEN4_R {
        WAKEUPEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen5(&self) -> WAKEUPEN5_R {
        WAKEUPEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen6(&self) -> WAKEUPEN6_R {
        WAKEUPEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen7(&self) -> WAKEUPEN7_R {
        WAKEUPEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen8(&self) -> WAKEUPEN8_R {
        WAKEUPEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen9(&self) -> WAKEUPEN9_R {
        WAKEUPEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen10(&self) -> WAKEUPEN10_R {
        WAKEUPEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen11(&self) -> WAKEUPEN11_R {
        WAKEUPEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen12(&self) -> WAKEUPEN12_R {
        WAKEUPEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen13(&self) -> WAKEUPEN13_R {
        WAKEUPEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen14(&self) -> WAKEUPEN14_R {
        WAKEUPEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen15(&self) -> WAKEUPEN15_R {
        WAKEUPEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen0(&mut self) -> WAKEUPEN0_W {
        WAKEUPEN0_W { w: self }
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen1(&mut self) -> WAKEUPEN1_W {
        WAKEUPEN1_W { w: self }
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen2(&mut self) -> WAKEUPEN2_W {
        WAKEUPEN2_W { w: self }
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen3(&mut self) -> WAKEUPEN3_W {
        WAKEUPEN3_W { w: self }
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen4(&mut self) -> WAKEUPEN4_W {
        WAKEUPEN4_W { w: self }
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen5(&mut self) -> WAKEUPEN5_W {
        WAKEUPEN5_W { w: self }
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen6(&mut self) -> WAKEUPEN6_W {
        WAKEUPEN6_W { w: self }
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen7(&mut self) -> WAKEUPEN7_W {
        WAKEUPEN7_W { w: self }
    }
    #[doc = "Bit 8 - External Interrupt 8 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen8(&mut self) -> WAKEUPEN8_W {
        WAKEUPEN8_W { w: self }
    }
    #[doc = "Bit 9 - External Interrupt 9 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen9(&mut self) -> WAKEUPEN9_W {
        WAKEUPEN9_W { w: self }
    }
    #[doc = "Bit 10 - External Interrupt 10 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen10(&mut self) -> WAKEUPEN10_W {
        WAKEUPEN10_W { w: self }
    }
    #[doc = "Bit 11 - External Interrupt 11 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen11(&mut self) -> WAKEUPEN11_W {
        WAKEUPEN11_W { w: self }
    }
    #[doc = "Bit 12 - External Interrupt 12 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen12(&mut self) -> WAKEUPEN12_W {
        WAKEUPEN12_W { w: self }
    }
    #[doc = "Bit 13 - External Interrupt 13 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen13(&mut self) -> WAKEUPEN13_W {
        WAKEUPEN13_W { w: self }
    }
    #[doc = "Bit 14 - External Interrupt 14 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen14(&mut self) -> WAKEUPEN14_W {
        WAKEUPEN14_W { w: self }
    }
    #[doc = "Bit 15 - External Interrupt 15 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen15(&mut self) -> WAKEUPEN15_W {
        WAKEUPEN15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-Up Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup](index.html) module"]
pub struct WAKEUP_SPEC;
impl crate::RegisterSpec for WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup::R](R) reader structure"]
impl crate::Readable for WAKEUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup::W](W) writer structure"]
impl crate::Writable for WAKEUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEUP to value 0"]
impl crate::Resettable for WAKEUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
