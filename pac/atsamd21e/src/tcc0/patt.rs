#[doc = "Register `PATT` reader"]
pub struct R(crate::R<PATT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATT` writer"]
pub struct W(crate::W<PATT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATT_SPEC>;
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
impl From<crate::W<PATT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGE0` reader - Pattern Generator 0 Output Enable"]
pub struct PGE0_R(crate::FieldReader<bool, bool>);
impl PGE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGE0` writer - Pattern Generator 0 Output Enable"]
pub struct PGE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE0_W<'a> {
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
#[doc = "Field `PGE1` reader - Pattern Generator 1 Output Enable"]
pub struct PGE1_R(crate::FieldReader<bool, bool>);
impl PGE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGE1` writer - Pattern Generator 1 Output Enable"]
pub struct PGE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE1_W<'a> {
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
#[doc = "Field `PGE2` reader - Pattern Generator 2 Output Enable"]
pub struct PGE2_R(crate::FieldReader<bool, bool>);
impl PGE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGE2` writer - Pattern Generator 2 Output Enable"]
pub struct PGE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE2_W<'a> {
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
#[doc = "Field `PGE3` reader - Pattern Generator 3 Output Enable"]
pub struct PGE3_R(crate::FieldReader<bool, bool>);
impl PGE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGE3` writer - Pattern Generator 3 Output Enable"]
pub struct PGE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE3_W<'a> {
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
#[doc = "Field `PGE4` reader - Pattern Generator 4 Output Enable"]
pub struct PGE4_R(crate::FieldReader<bool, bool>);
impl PGE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGE4` writer - Pattern Generator 4 Output Enable"]
pub struct PGE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE4_W<'a> {
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
#[doc = "Field `PGE5` reader - Pattern Generator 5 Output Enable"]
pub struct PGE5_R(crate::FieldReader<bool, bool>);
impl PGE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGE5` writer - Pattern Generator 5 Output Enable"]
pub struct PGE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE5_W<'a> {
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
#[doc = "Field `PGE6` reader - Pattern Generator 6 Output Enable"]
pub struct PGE6_R(crate::FieldReader<bool, bool>);
impl PGE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGE6` writer - Pattern Generator 6 Output Enable"]
pub struct PGE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE6_W<'a> {
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
#[doc = "Field `PGE7` reader - Pattern Generator 7 Output Enable"]
pub struct PGE7_R(crate::FieldReader<bool, bool>);
impl PGE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGE7` writer - Pattern Generator 7 Output Enable"]
pub struct PGE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE7_W<'a> {
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
#[doc = "Field `PGV0` reader - Pattern Generator 0 Output Value"]
pub struct PGV0_R(crate::FieldReader<bool, bool>);
impl PGV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGV0` writer - Pattern Generator 0 Output Value"]
pub struct PGV0_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV0_W<'a> {
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
#[doc = "Field `PGV1` reader - Pattern Generator 1 Output Value"]
pub struct PGV1_R(crate::FieldReader<bool, bool>);
impl PGV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGV1` writer - Pattern Generator 1 Output Value"]
pub struct PGV1_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV1_W<'a> {
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
#[doc = "Field `PGV2` reader - Pattern Generator 2 Output Value"]
pub struct PGV2_R(crate::FieldReader<bool, bool>);
impl PGV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGV2` writer - Pattern Generator 2 Output Value"]
pub struct PGV2_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV2_W<'a> {
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
#[doc = "Field `PGV3` reader - Pattern Generator 3 Output Value"]
pub struct PGV3_R(crate::FieldReader<bool, bool>);
impl PGV3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGV3` writer - Pattern Generator 3 Output Value"]
pub struct PGV3_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV3_W<'a> {
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
#[doc = "Field `PGV4` reader - Pattern Generator 4 Output Value"]
pub struct PGV4_R(crate::FieldReader<bool, bool>);
impl PGV4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGV4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGV4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGV4` writer - Pattern Generator 4 Output Value"]
pub struct PGV4_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV4_W<'a> {
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
#[doc = "Field `PGV5` reader - Pattern Generator 5 Output Value"]
pub struct PGV5_R(crate::FieldReader<bool, bool>);
impl PGV5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGV5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGV5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGV5` writer - Pattern Generator 5 Output Value"]
pub struct PGV5_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV5_W<'a> {
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
#[doc = "Field `PGV6` reader - Pattern Generator 6 Output Value"]
pub struct PGV6_R(crate::FieldReader<bool, bool>);
impl PGV6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGV6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGV6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGV6` writer - Pattern Generator 6 Output Value"]
pub struct PGV6_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV6_W<'a> {
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
#[doc = "Field `PGV7` reader - Pattern Generator 7 Output Value"]
pub struct PGV7_R(crate::FieldReader<bool, bool>);
impl PGV7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGV7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGV7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGV7` writer - Pattern Generator 7 Output Value"]
pub struct PGV7_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV7_W<'a> {
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
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pge0(&self) -> PGE0_R {
        PGE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pge1(&self) -> PGE1_R {
        PGE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pge2(&self) -> PGE2_R {
        PGE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pge3(&self) -> PGE3_R {
        PGE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pge4(&self) -> PGE4_R {
        PGE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pge5(&self) -> PGE5_R {
        PGE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pge6(&self) -> PGE6_R {
        PGE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pge7(&self) -> PGE7_R {
        PGE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Value"]
    #[inline(always)]
    pub fn pgv0(&self) -> PGV0_R {
        PGV0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Value"]
    #[inline(always)]
    pub fn pgv1(&self) -> PGV1_R {
        PGV1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Value"]
    #[inline(always)]
    pub fn pgv2(&self) -> PGV2_R {
        PGV2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Value"]
    #[inline(always)]
    pub fn pgv3(&self) -> PGV3_R {
        PGV3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Value"]
    #[inline(always)]
    pub fn pgv4(&self) -> PGV4_R {
        PGV4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Value"]
    #[inline(always)]
    pub fn pgv5(&self) -> PGV5_R {
        PGV5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Value"]
    #[inline(always)]
    pub fn pgv6(&self) -> PGV6_R {
        PGV6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Value"]
    #[inline(always)]
    pub fn pgv7(&self) -> PGV7_R {
        PGV7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pge0(&mut self) -> PGE0_W {
        PGE0_W { w: self }
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pge1(&mut self) -> PGE1_W {
        PGE1_W { w: self }
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pge2(&mut self) -> PGE2_W {
        PGE2_W { w: self }
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pge3(&mut self) -> PGE3_W {
        PGE3_W { w: self }
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pge4(&mut self) -> PGE4_W {
        PGE4_W { w: self }
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pge5(&mut self) -> PGE5_W {
        PGE5_W { w: self }
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pge6(&mut self) -> PGE6_W {
        PGE6_W { w: self }
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pge7(&mut self) -> PGE7_W {
        PGE7_W { w: self }
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Value"]
    #[inline(always)]
    pub fn pgv0(&mut self) -> PGV0_W {
        PGV0_W { w: self }
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Value"]
    #[inline(always)]
    pub fn pgv1(&mut self) -> PGV1_W {
        PGV1_W { w: self }
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Value"]
    #[inline(always)]
    pub fn pgv2(&mut self) -> PGV2_W {
        PGV2_W { w: self }
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Value"]
    #[inline(always)]
    pub fn pgv3(&mut self) -> PGV3_W {
        PGV3_W { w: self }
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Value"]
    #[inline(always)]
    pub fn pgv4(&mut self) -> PGV4_W {
        PGV4_W { w: self }
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Value"]
    #[inline(always)]
    pub fn pgv5(&mut self) -> PGV5_W {
        PGV5_W { w: self }
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Value"]
    #[inline(always)]
    pub fn pgv6(&mut self) -> PGV6_W {
        PGV6_W { w: self }
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Value"]
    #[inline(always)]
    pub fn pgv7(&mut self) -> PGV7_W {
        PGV7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pattern\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt](index.html) module"]
pub struct PATT_SPEC;
impl crate::RegisterSpec for PATT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [patt::R](R) reader structure"]
impl crate::Readable for PATT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [patt::W](W) writer structure"]
impl crate::Writable for PATT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PATT to value 0"]
impl crate::Resettable for PATT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
