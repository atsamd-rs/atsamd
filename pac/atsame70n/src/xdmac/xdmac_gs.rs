#[doc = "Register `XDMAC_GS` reader"]
pub struct R(crate::R<XDMAC_GS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_GS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_GS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_GS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ST0` reader - XDMAC Channel 0 Status Bit"]
pub struct ST0_R(crate::FieldReader<bool, bool>);
impl ST0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST1` reader - XDMAC Channel 1 Status Bit"]
pub struct ST1_R(crate::FieldReader<bool, bool>);
impl ST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST2` reader - XDMAC Channel 2 Status Bit"]
pub struct ST2_R(crate::FieldReader<bool, bool>);
impl ST2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST3` reader - XDMAC Channel 3 Status Bit"]
pub struct ST3_R(crate::FieldReader<bool, bool>);
impl ST3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST4` reader - XDMAC Channel 4 Status Bit"]
pub struct ST4_R(crate::FieldReader<bool, bool>);
impl ST4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST5` reader - XDMAC Channel 5 Status Bit"]
pub struct ST5_R(crate::FieldReader<bool, bool>);
impl ST5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST6` reader - XDMAC Channel 6 Status Bit"]
pub struct ST6_R(crate::FieldReader<bool, bool>);
impl ST6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST7` reader - XDMAC Channel 7 Status Bit"]
pub struct ST7_R(crate::FieldReader<bool, bool>);
impl ST7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST8` reader - XDMAC Channel 8 Status Bit"]
pub struct ST8_R(crate::FieldReader<bool, bool>);
impl ST8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST9` reader - XDMAC Channel 9 Status Bit"]
pub struct ST9_R(crate::FieldReader<bool, bool>);
impl ST9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST10` reader - XDMAC Channel 10 Status Bit"]
pub struct ST10_R(crate::FieldReader<bool, bool>);
impl ST10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST11` reader - XDMAC Channel 11 Status Bit"]
pub struct ST11_R(crate::FieldReader<bool, bool>);
impl ST11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST12` reader - XDMAC Channel 12 Status Bit"]
pub struct ST12_R(crate::FieldReader<bool, bool>);
impl ST12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST13` reader - XDMAC Channel 13 Status Bit"]
pub struct ST13_R(crate::FieldReader<bool, bool>);
impl ST13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST14` reader - XDMAC Channel 14 Status Bit"]
pub struct ST14_R(crate::FieldReader<bool, bool>);
impl ST14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST15` reader - XDMAC Channel 15 Status Bit"]
pub struct ST15_R(crate::FieldReader<bool, bool>);
impl ST15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST16` reader - XDMAC Channel 16 Status Bit"]
pub struct ST16_R(crate::FieldReader<bool, bool>);
impl ST16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST17` reader - XDMAC Channel 17 Status Bit"]
pub struct ST17_R(crate::FieldReader<bool, bool>);
impl ST17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST18` reader - XDMAC Channel 18 Status Bit"]
pub struct ST18_R(crate::FieldReader<bool, bool>);
impl ST18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST19` reader - XDMAC Channel 19 Status Bit"]
pub struct ST19_R(crate::FieldReader<bool, bool>);
impl ST19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST20` reader - XDMAC Channel 20 Status Bit"]
pub struct ST20_R(crate::FieldReader<bool, bool>);
impl ST20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST21` reader - XDMAC Channel 21 Status Bit"]
pub struct ST21_R(crate::FieldReader<bool, bool>);
impl ST21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST22` reader - XDMAC Channel 22 Status Bit"]
pub struct ST22_R(crate::FieldReader<bool, bool>);
impl ST22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST23` reader - XDMAC Channel 23 Status Bit"]
pub struct ST23_R(crate::FieldReader<bool, bool>);
impl ST23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Status Bit"]
    #[inline(always)]
    pub fn st0(&self) -> ST0_R {
        ST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Status Bit"]
    #[inline(always)]
    pub fn st1(&self) -> ST1_R {
        ST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Status Bit"]
    #[inline(always)]
    pub fn st2(&self) -> ST2_R {
        ST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Status Bit"]
    #[inline(always)]
    pub fn st3(&self) -> ST3_R {
        ST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Status Bit"]
    #[inline(always)]
    pub fn st4(&self) -> ST4_R {
        ST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Status Bit"]
    #[inline(always)]
    pub fn st5(&self) -> ST5_R {
        ST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Status Bit"]
    #[inline(always)]
    pub fn st6(&self) -> ST6_R {
        ST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Status Bit"]
    #[inline(always)]
    pub fn st7(&self) -> ST7_R {
        ST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Status Bit"]
    #[inline(always)]
    pub fn st8(&self) -> ST8_R {
        ST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Status Bit"]
    #[inline(always)]
    pub fn st9(&self) -> ST9_R {
        ST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Status Bit"]
    #[inline(always)]
    pub fn st10(&self) -> ST10_R {
        ST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Status Bit"]
    #[inline(always)]
    pub fn st11(&self) -> ST11_R {
        ST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Status Bit"]
    #[inline(always)]
    pub fn st12(&self) -> ST12_R {
        ST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Status Bit"]
    #[inline(always)]
    pub fn st13(&self) -> ST13_R {
        ST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Status Bit"]
    #[inline(always)]
    pub fn st14(&self) -> ST14_R {
        ST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Status Bit"]
    #[inline(always)]
    pub fn st15(&self) -> ST15_R {
        ST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Status Bit"]
    #[inline(always)]
    pub fn st16(&self) -> ST16_R {
        ST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Status Bit"]
    #[inline(always)]
    pub fn st17(&self) -> ST17_R {
        ST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Status Bit"]
    #[inline(always)]
    pub fn st18(&self) -> ST18_R {
        ST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Status Bit"]
    #[inline(always)]
    pub fn st19(&self) -> ST19_R {
        ST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Status Bit"]
    #[inline(always)]
    pub fn st20(&self) -> ST20_R {
        ST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Status Bit"]
    #[inline(always)]
    pub fn st21(&self) -> ST21_R {
        ST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Status Bit"]
    #[inline(always)]
    pub fn st22(&self) -> ST22_R {
        ST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Status Bit"]
    #[inline(always)]
    pub fn st23(&self) -> ST23_R {
        ST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "Global Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gs](index.html) module"]
pub struct XDMAC_GS_SPEC;
impl crate::RegisterSpec for XDMAC_GS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_gs::R](R) reader structure"]
impl crate::Readable for XDMAC_GS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XDMAC_GS to value 0"]
impl crate::Resettable for XDMAC_GS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
