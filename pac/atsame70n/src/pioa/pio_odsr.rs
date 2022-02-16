#[doc = "Register `PIO_ODSR` reader"]
pub struct R(crate::R<PIO_ODSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_ODSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_ODSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_ODSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO_ODSR` writer"]
pub struct W(crate::W<PIO_ODSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO_ODSR_SPEC>;
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
impl From<crate::W<PIO_ODSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO_ODSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P0` reader - Output Data Status"]
pub struct P0_R(crate::FieldReader<bool, bool>);
impl P0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0` writer - Output Data Status"]
pub struct P0_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_W<'a> {
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
#[doc = "Field `P1` reader - Output Data Status"]
pub struct P1_R(crate::FieldReader<bool, bool>);
impl P1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1` writer - Output Data Status"]
pub struct P1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_W<'a> {
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
#[doc = "Field `P2` reader - Output Data Status"]
pub struct P2_R(crate::FieldReader<bool, bool>);
impl P2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2` writer - Output Data Status"]
pub struct P2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_W<'a> {
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
#[doc = "Field `P3` reader - Output Data Status"]
pub struct P3_R(crate::FieldReader<bool, bool>);
impl P3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3` writer - Output Data Status"]
pub struct P3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_W<'a> {
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
#[doc = "Field `P4` reader - Output Data Status"]
pub struct P4_R(crate::FieldReader<bool, bool>);
impl P4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4` writer - Output Data Status"]
pub struct P4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_W<'a> {
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
#[doc = "Field `P5` reader - Output Data Status"]
pub struct P5_R(crate::FieldReader<bool, bool>);
impl P5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5` writer - Output Data Status"]
pub struct P5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5_W<'a> {
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
#[doc = "Field `P6` reader - Output Data Status"]
pub struct P6_R(crate::FieldReader<bool, bool>);
impl P6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6` writer - Output Data Status"]
pub struct P6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6_W<'a> {
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
#[doc = "Field `P7` reader - Output Data Status"]
pub struct P7_R(crate::FieldReader<bool, bool>);
impl P7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7` writer - Output Data Status"]
pub struct P7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7_W<'a> {
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
#[doc = "Field `P8` reader - Output Data Status"]
pub struct P8_R(crate::FieldReader<bool, bool>);
impl P8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8` writer - Output Data Status"]
pub struct P8_W<'a> {
    w: &'a mut W,
}
impl<'a> P8_W<'a> {
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
#[doc = "Field `P9` reader - Output Data Status"]
pub struct P9_R(crate::FieldReader<bool, bool>);
impl P9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9` writer - Output Data Status"]
pub struct P9_W<'a> {
    w: &'a mut W,
}
impl<'a> P9_W<'a> {
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
#[doc = "Field `P10` reader - Output Data Status"]
pub struct P10_R(crate::FieldReader<bool, bool>);
impl P10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P10` writer - Output Data Status"]
pub struct P10_W<'a> {
    w: &'a mut W,
}
impl<'a> P10_W<'a> {
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
#[doc = "Field `P11` reader - Output Data Status"]
pub struct P11_R(crate::FieldReader<bool, bool>);
impl P11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P11` writer - Output Data Status"]
pub struct P11_W<'a> {
    w: &'a mut W,
}
impl<'a> P11_W<'a> {
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
#[doc = "Field `P12` reader - Output Data Status"]
pub struct P12_R(crate::FieldReader<bool, bool>);
impl P12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P12` writer - Output Data Status"]
pub struct P12_W<'a> {
    w: &'a mut W,
}
impl<'a> P12_W<'a> {
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
#[doc = "Field `P13` reader - Output Data Status"]
pub struct P13_R(crate::FieldReader<bool, bool>);
impl P13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P13` writer - Output Data Status"]
pub struct P13_W<'a> {
    w: &'a mut W,
}
impl<'a> P13_W<'a> {
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
#[doc = "Field `P14` reader - Output Data Status"]
pub struct P14_R(crate::FieldReader<bool, bool>);
impl P14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P14` writer - Output Data Status"]
pub struct P14_W<'a> {
    w: &'a mut W,
}
impl<'a> P14_W<'a> {
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
#[doc = "Field `P15` reader - Output Data Status"]
pub struct P15_R(crate::FieldReader<bool, bool>);
impl P15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P15` writer - Output Data Status"]
pub struct P15_W<'a> {
    w: &'a mut W,
}
impl<'a> P15_W<'a> {
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
#[doc = "Field `P16` reader - Output Data Status"]
pub struct P16_R(crate::FieldReader<bool, bool>);
impl P16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P16` writer - Output Data Status"]
pub struct P16_W<'a> {
    w: &'a mut W,
}
impl<'a> P16_W<'a> {
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
#[doc = "Field `P17` reader - Output Data Status"]
pub struct P17_R(crate::FieldReader<bool, bool>);
impl P17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P17` writer - Output Data Status"]
pub struct P17_W<'a> {
    w: &'a mut W,
}
impl<'a> P17_W<'a> {
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
#[doc = "Field `P18` reader - Output Data Status"]
pub struct P18_R(crate::FieldReader<bool, bool>);
impl P18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P18` writer - Output Data Status"]
pub struct P18_W<'a> {
    w: &'a mut W,
}
impl<'a> P18_W<'a> {
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
#[doc = "Field `P19` reader - Output Data Status"]
pub struct P19_R(crate::FieldReader<bool, bool>);
impl P19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P19` writer - Output Data Status"]
pub struct P19_W<'a> {
    w: &'a mut W,
}
impl<'a> P19_W<'a> {
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
#[doc = "Field `P20` reader - Output Data Status"]
pub struct P20_R(crate::FieldReader<bool, bool>);
impl P20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P20` writer - Output Data Status"]
pub struct P20_W<'a> {
    w: &'a mut W,
}
impl<'a> P20_W<'a> {
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
#[doc = "Field `P21` reader - Output Data Status"]
pub struct P21_R(crate::FieldReader<bool, bool>);
impl P21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P21` writer - Output Data Status"]
pub struct P21_W<'a> {
    w: &'a mut W,
}
impl<'a> P21_W<'a> {
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
#[doc = "Field `P22` reader - Output Data Status"]
pub struct P22_R(crate::FieldReader<bool, bool>);
impl P22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P22` writer - Output Data Status"]
pub struct P22_W<'a> {
    w: &'a mut W,
}
impl<'a> P22_W<'a> {
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
#[doc = "Field `P23` reader - Output Data Status"]
pub struct P23_R(crate::FieldReader<bool, bool>);
impl P23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P23` writer - Output Data Status"]
pub struct P23_W<'a> {
    w: &'a mut W,
}
impl<'a> P23_W<'a> {
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
#[doc = "Field `P24` reader - Output Data Status"]
pub struct P24_R(crate::FieldReader<bool, bool>);
impl P24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P24` writer - Output Data Status"]
pub struct P24_W<'a> {
    w: &'a mut W,
}
impl<'a> P24_W<'a> {
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
#[doc = "Field `P25` reader - Output Data Status"]
pub struct P25_R(crate::FieldReader<bool, bool>);
impl P25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P25` writer - Output Data Status"]
pub struct P25_W<'a> {
    w: &'a mut W,
}
impl<'a> P25_W<'a> {
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
#[doc = "Field `P26` reader - Output Data Status"]
pub struct P26_R(crate::FieldReader<bool, bool>);
impl P26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P26` writer - Output Data Status"]
pub struct P26_W<'a> {
    w: &'a mut W,
}
impl<'a> P26_W<'a> {
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
#[doc = "Field `P27` reader - Output Data Status"]
pub struct P27_R(crate::FieldReader<bool, bool>);
impl P27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P27` writer - Output Data Status"]
pub struct P27_W<'a> {
    w: &'a mut W,
}
impl<'a> P27_W<'a> {
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
#[doc = "Field `P28` reader - Output Data Status"]
pub struct P28_R(crate::FieldReader<bool, bool>);
impl P28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P28` writer - Output Data Status"]
pub struct P28_W<'a> {
    w: &'a mut W,
}
impl<'a> P28_W<'a> {
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
#[doc = "Field `P29` reader - Output Data Status"]
pub struct P29_R(crate::FieldReader<bool, bool>);
impl P29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P29` writer - Output Data Status"]
pub struct P29_W<'a> {
    w: &'a mut W,
}
impl<'a> P29_W<'a> {
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
#[doc = "Field `P30` reader - Output Data Status"]
pub struct P30_R(crate::FieldReader<bool, bool>);
impl P30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P30` writer - Output Data Status"]
pub struct P30_W<'a> {
    w: &'a mut W,
}
impl<'a> P30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `P31` reader - Output Data Status"]
pub struct P31_R(crate::FieldReader<bool, bool>);
impl P31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P31` writer - Output Data Status"]
pub struct P31_W<'a> {
    w: &'a mut W,
}
impl<'a> P31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Output Data Status"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Data Status"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output Data Status"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Data Status"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output Data Status"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Output Data Status"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Output Data Status"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Output Data Status"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Output Data Status"]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Output Data Status"]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output Data Status"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output Data Status"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Output Data Status"]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Output Data Status"]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Output Data Status"]
    #[inline(always)]
    pub fn p14(&self) -> P14_R {
        P14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Output Data Status"]
    #[inline(always)]
    pub fn p15(&self) -> P15_R {
        P15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Output Data Status"]
    #[inline(always)]
    pub fn p16(&self) -> P16_R {
        P16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Output Data Status"]
    #[inline(always)]
    pub fn p17(&self) -> P17_R {
        P17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output Data Status"]
    #[inline(always)]
    pub fn p18(&self) -> P18_R {
        P18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Output Data Status"]
    #[inline(always)]
    pub fn p19(&self) -> P19_R {
        P19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Output Data Status"]
    #[inline(always)]
    pub fn p20(&self) -> P20_R {
        P20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Output Data Status"]
    #[inline(always)]
    pub fn p21(&self) -> P21_R {
        P21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Output Data Status"]
    #[inline(always)]
    pub fn p22(&self) -> P22_R {
        P22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Output Data Status"]
    #[inline(always)]
    pub fn p23(&self) -> P23_R {
        P23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Output Data Status"]
    #[inline(always)]
    pub fn p24(&self) -> P24_R {
        P24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Output Data Status"]
    #[inline(always)]
    pub fn p25(&self) -> P25_R {
        P25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Output Data Status"]
    #[inline(always)]
    pub fn p26(&self) -> P26_R {
        P26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Output Data Status"]
    #[inline(always)]
    pub fn p27(&self) -> P27_R {
        P27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Output Data Status"]
    #[inline(always)]
    pub fn p28(&self) -> P28_R {
        P28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Output Data Status"]
    #[inline(always)]
    pub fn p29(&self) -> P29_R {
        P29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Output Data Status"]
    #[inline(always)]
    pub fn p30(&self) -> P30_R {
        P30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Output Data Status"]
    #[inline(always)]
    pub fn p31(&self) -> P31_R {
        P31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Data Status"]
    #[inline(always)]
    pub fn p0(&mut self) -> P0_W {
        P0_W { w: self }
    }
    #[doc = "Bit 1 - Output Data Status"]
    #[inline(always)]
    pub fn p1(&mut self) -> P1_W {
        P1_W { w: self }
    }
    #[doc = "Bit 2 - Output Data Status"]
    #[inline(always)]
    pub fn p2(&mut self) -> P2_W {
        P2_W { w: self }
    }
    #[doc = "Bit 3 - Output Data Status"]
    #[inline(always)]
    pub fn p3(&mut self) -> P3_W {
        P3_W { w: self }
    }
    #[doc = "Bit 4 - Output Data Status"]
    #[inline(always)]
    pub fn p4(&mut self) -> P4_W {
        P4_W { w: self }
    }
    #[doc = "Bit 5 - Output Data Status"]
    #[inline(always)]
    pub fn p5(&mut self) -> P5_W {
        P5_W { w: self }
    }
    #[doc = "Bit 6 - Output Data Status"]
    #[inline(always)]
    pub fn p6(&mut self) -> P6_W {
        P6_W { w: self }
    }
    #[doc = "Bit 7 - Output Data Status"]
    #[inline(always)]
    pub fn p7(&mut self) -> P7_W {
        P7_W { w: self }
    }
    #[doc = "Bit 8 - Output Data Status"]
    #[inline(always)]
    pub fn p8(&mut self) -> P8_W {
        P8_W { w: self }
    }
    #[doc = "Bit 9 - Output Data Status"]
    #[inline(always)]
    pub fn p9(&mut self) -> P9_W {
        P9_W { w: self }
    }
    #[doc = "Bit 10 - Output Data Status"]
    #[inline(always)]
    pub fn p10(&mut self) -> P10_W {
        P10_W { w: self }
    }
    #[doc = "Bit 11 - Output Data Status"]
    #[inline(always)]
    pub fn p11(&mut self) -> P11_W {
        P11_W { w: self }
    }
    #[doc = "Bit 12 - Output Data Status"]
    #[inline(always)]
    pub fn p12(&mut self) -> P12_W {
        P12_W { w: self }
    }
    #[doc = "Bit 13 - Output Data Status"]
    #[inline(always)]
    pub fn p13(&mut self) -> P13_W {
        P13_W { w: self }
    }
    #[doc = "Bit 14 - Output Data Status"]
    #[inline(always)]
    pub fn p14(&mut self) -> P14_W {
        P14_W { w: self }
    }
    #[doc = "Bit 15 - Output Data Status"]
    #[inline(always)]
    pub fn p15(&mut self) -> P15_W {
        P15_W { w: self }
    }
    #[doc = "Bit 16 - Output Data Status"]
    #[inline(always)]
    pub fn p16(&mut self) -> P16_W {
        P16_W { w: self }
    }
    #[doc = "Bit 17 - Output Data Status"]
    #[inline(always)]
    pub fn p17(&mut self) -> P17_W {
        P17_W { w: self }
    }
    #[doc = "Bit 18 - Output Data Status"]
    #[inline(always)]
    pub fn p18(&mut self) -> P18_W {
        P18_W { w: self }
    }
    #[doc = "Bit 19 - Output Data Status"]
    #[inline(always)]
    pub fn p19(&mut self) -> P19_W {
        P19_W { w: self }
    }
    #[doc = "Bit 20 - Output Data Status"]
    #[inline(always)]
    pub fn p20(&mut self) -> P20_W {
        P20_W { w: self }
    }
    #[doc = "Bit 21 - Output Data Status"]
    #[inline(always)]
    pub fn p21(&mut self) -> P21_W {
        P21_W { w: self }
    }
    #[doc = "Bit 22 - Output Data Status"]
    #[inline(always)]
    pub fn p22(&mut self) -> P22_W {
        P22_W { w: self }
    }
    #[doc = "Bit 23 - Output Data Status"]
    #[inline(always)]
    pub fn p23(&mut self) -> P23_W {
        P23_W { w: self }
    }
    #[doc = "Bit 24 - Output Data Status"]
    #[inline(always)]
    pub fn p24(&mut self) -> P24_W {
        P24_W { w: self }
    }
    #[doc = "Bit 25 - Output Data Status"]
    #[inline(always)]
    pub fn p25(&mut self) -> P25_W {
        P25_W { w: self }
    }
    #[doc = "Bit 26 - Output Data Status"]
    #[inline(always)]
    pub fn p26(&mut self) -> P26_W {
        P26_W { w: self }
    }
    #[doc = "Bit 27 - Output Data Status"]
    #[inline(always)]
    pub fn p27(&mut self) -> P27_W {
        P27_W { w: self }
    }
    #[doc = "Bit 28 - Output Data Status"]
    #[inline(always)]
    pub fn p28(&mut self) -> P28_W {
        P28_W { w: self }
    }
    #[doc = "Bit 29 - Output Data Status"]
    #[inline(always)]
    pub fn p29(&mut self) -> P29_W {
        P29_W { w: self }
    }
    #[doc = "Bit 30 - Output Data Status"]
    #[inline(always)]
    pub fn p30(&mut self) -> P30_W {
        P30_W { w: self }
    }
    #[doc = "Bit 31 - Output Data Status"]
    #[inline(always)]
    pub fn p31(&mut self) -> P31_W {
        P31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Data Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_odsr](index.html) module"]
pub struct PIO_ODSR_SPEC;
impl crate::RegisterSpec for PIO_ODSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_odsr::R](R) reader structure"]
impl crate::Readable for PIO_ODSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio_odsr::W](W) writer structure"]
impl crate::Writable for PIO_ODSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO_ODSR to value 0"]
impl crate::Resettable for PIO_ODSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
