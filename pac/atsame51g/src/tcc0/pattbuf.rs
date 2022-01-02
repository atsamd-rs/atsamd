#[doc = "Register `PATTBUF` reader"]
pub struct R(crate::R<PATTBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATTBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATTBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATTBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATTBUF` writer"]
pub struct W(crate::W<PATTBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATTBUF_SPEC>;
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
impl From<crate::W<PATTBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATTBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGEB0` reader - Pattern Generator 0 Output Enable Buffer"]
pub struct PGEB0_R(crate::FieldReader<bool, bool>);
impl PGEB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGEB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGEB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGEB0` writer - Pattern Generator 0 Output Enable Buffer"]
pub struct PGEB0_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB0_W<'a> {
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
#[doc = "Field `PGEB1` reader - Pattern Generator 1 Output Enable Buffer"]
pub struct PGEB1_R(crate::FieldReader<bool, bool>);
impl PGEB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGEB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGEB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGEB1` writer - Pattern Generator 1 Output Enable Buffer"]
pub struct PGEB1_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB1_W<'a> {
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
#[doc = "Field `PGEB2` reader - Pattern Generator 2 Output Enable Buffer"]
pub struct PGEB2_R(crate::FieldReader<bool, bool>);
impl PGEB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGEB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGEB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGEB2` writer - Pattern Generator 2 Output Enable Buffer"]
pub struct PGEB2_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB2_W<'a> {
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
#[doc = "Field `PGEB3` reader - Pattern Generator 3 Output Enable Buffer"]
pub struct PGEB3_R(crate::FieldReader<bool, bool>);
impl PGEB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGEB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGEB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGEB3` writer - Pattern Generator 3 Output Enable Buffer"]
pub struct PGEB3_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB3_W<'a> {
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
#[doc = "Field `PGEB4` reader - Pattern Generator 4 Output Enable Buffer"]
pub struct PGEB4_R(crate::FieldReader<bool, bool>);
impl PGEB4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGEB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGEB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGEB4` writer - Pattern Generator 4 Output Enable Buffer"]
pub struct PGEB4_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB4_W<'a> {
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
#[doc = "Field `PGEB5` reader - Pattern Generator 5 Output Enable Buffer"]
pub struct PGEB5_R(crate::FieldReader<bool, bool>);
impl PGEB5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGEB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGEB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGEB5` writer - Pattern Generator 5 Output Enable Buffer"]
pub struct PGEB5_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB5_W<'a> {
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
#[doc = "Field `PGEB6` reader - Pattern Generator 6 Output Enable Buffer"]
pub struct PGEB6_R(crate::FieldReader<bool, bool>);
impl PGEB6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGEB6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGEB6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGEB6` writer - Pattern Generator 6 Output Enable Buffer"]
pub struct PGEB6_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PGEB7` reader - Pattern Generator 7 Output Enable Buffer"]
pub struct PGEB7_R(crate::FieldReader<bool, bool>);
impl PGEB7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGEB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGEB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGEB7` writer - Pattern Generator 7 Output Enable Buffer"]
pub struct PGEB7_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PGVB0` reader - Pattern Generator 0 Output Enable"]
pub struct PGVB0_R(crate::FieldReader<bool, bool>);
impl PGVB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGVB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGVB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGVB0` writer - Pattern Generator 0 Output Enable"]
pub struct PGVB0_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB0_W<'a> {
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
#[doc = "Field `PGVB1` reader - Pattern Generator 1 Output Enable"]
pub struct PGVB1_R(crate::FieldReader<bool, bool>);
impl PGVB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGVB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGVB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGVB1` writer - Pattern Generator 1 Output Enable"]
pub struct PGVB1_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB1_W<'a> {
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
#[doc = "Field `PGVB2` reader - Pattern Generator 2 Output Enable"]
pub struct PGVB2_R(crate::FieldReader<bool, bool>);
impl PGVB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGVB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGVB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGVB2` writer - Pattern Generator 2 Output Enable"]
pub struct PGVB2_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB2_W<'a> {
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
#[doc = "Field `PGVB3` reader - Pattern Generator 3 Output Enable"]
pub struct PGVB3_R(crate::FieldReader<bool, bool>);
impl PGVB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGVB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGVB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGVB3` writer - Pattern Generator 3 Output Enable"]
pub struct PGVB3_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB3_W<'a> {
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
#[doc = "Field `PGVB4` reader - Pattern Generator 4 Output Enable"]
pub struct PGVB4_R(crate::FieldReader<bool, bool>);
impl PGVB4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGVB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGVB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGVB4` writer - Pattern Generator 4 Output Enable"]
pub struct PGVB4_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB4_W<'a> {
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
#[doc = "Field `PGVB5` reader - Pattern Generator 5 Output Enable"]
pub struct PGVB5_R(crate::FieldReader<bool, bool>);
impl PGVB5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGVB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGVB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGVB5` writer - Pattern Generator 5 Output Enable"]
pub struct PGVB5_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB5_W<'a> {
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
#[doc = "Field `PGVB6` reader - Pattern Generator 6 Output Enable"]
pub struct PGVB6_R(crate::FieldReader<bool, bool>);
impl PGVB6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGVB6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGVB6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGVB6` writer - Pattern Generator 6 Output Enable"]
pub struct PGVB6_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB6_W<'a> {
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
#[doc = "Field `PGVB7` reader - Pattern Generator 7 Output Enable"]
pub struct PGVB7_R(crate::FieldReader<bool, bool>);
impl PGVB7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGVB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGVB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGVB7` writer - Pattern Generator 7 Output Enable"]
pub struct PGVB7_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB7_W<'a> {
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
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb0(&self) -> PGEB0_R {
        PGEB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb1(&self) -> PGEB1_R {
        PGEB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb2(&self) -> PGEB2_R {
        PGEB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb3(&self) -> PGEB3_R {
        PGEB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb4(&self) -> PGEB4_R {
        PGEB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb5(&self) -> PGEB5_R {
        PGEB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb6(&self) -> PGEB6_R {
        PGEB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb7(&self) -> PGEB7_R {
        PGEB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pgvb0(&self) -> PGVB0_R {
        PGVB0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pgvb1(&self) -> PGVB1_R {
        PGVB1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pgvb2(&self) -> PGVB2_R {
        PGVB2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pgvb3(&self) -> PGVB3_R {
        PGVB3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pgvb4(&self) -> PGVB4_R {
        PGVB4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pgvb5(&self) -> PGVB5_R {
        PGVB5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pgvb6(&self) -> PGVB6_R {
        PGVB6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pgvb7(&self) -> PGVB7_R {
        PGVB7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb0(&mut self) -> PGEB0_W {
        PGEB0_W { w: self }
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb1(&mut self) -> PGEB1_W {
        PGEB1_W { w: self }
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb2(&mut self) -> PGEB2_W {
        PGEB2_W { w: self }
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb3(&mut self) -> PGEB3_W {
        PGEB3_W { w: self }
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb4(&mut self) -> PGEB4_W {
        PGEB4_W { w: self }
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb5(&mut self) -> PGEB5_W {
        PGEB5_W { w: self }
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb6(&mut self) -> PGEB6_W {
        PGEB6_W { w: self }
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb7(&mut self) -> PGEB7_W {
        PGEB7_W { w: self }
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pgvb0(&mut self) -> PGVB0_W {
        PGVB0_W { w: self }
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pgvb1(&mut self) -> PGVB1_W {
        PGVB1_W { w: self }
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pgvb2(&mut self) -> PGVB2_W {
        PGVB2_W { w: self }
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pgvb3(&mut self) -> PGVB3_W {
        PGVB3_W { w: self }
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pgvb4(&mut self) -> PGVB4_W {
        PGVB4_W { w: self }
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pgvb5(&mut self) -> PGVB5_W {
        PGVB5_W { w: self }
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pgvb6(&mut self) -> PGVB6_W {
        PGVB6_W { w: self }
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pgvb7(&mut self) -> PGVB7_W {
        PGVB7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pattern Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pattbuf](index.html) module"]
pub struct PATTBUF_SPEC;
impl crate::RegisterSpec for PATTBUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pattbuf::R](R) reader structure"]
impl crate::Readable for PATTBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pattbuf::W](W) writer structure"]
impl crate::Writable for PATTBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PATTBUF to value 0"]
impl crate::Resettable for PATTBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
