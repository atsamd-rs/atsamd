#[doc = "Register `XDMAC_GIS` reader"]
pub struct R(crate::R<XDMAC_GIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_GIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_GIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_GIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IS0` reader - XDMAC Channel 0 Interrupt Status Bit"]
pub struct IS0_R(crate::FieldReader<bool, bool>);
impl IS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS1` reader - XDMAC Channel 1 Interrupt Status Bit"]
pub struct IS1_R(crate::FieldReader<bool, bool>);
impl IS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS2` reader - XDMAC Channel 2 Interrupt Status Bit"]
pub struct IS2_R(crate::FieldReader<bool, bool>);
impl IS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS3` reader - XDMAC Channel 3 Interrupt Status Bit"]
pub struct IS3_R(crate::FieldReader<bool, bool>);
impl IS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS4` reader - XDMAC Channel 4 Interrupt Status Bit"]
pub struct IS4_R(crate::FieldReader<bool, bool>);
impl IS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS5` reader - XDMAC Channel 5 Interrupt Status Bit"]
pub struct IS5_R(crate::FieldReader<bool, bool>);
impl IS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS6` reader - XDMAC Channel 6 Interrupt Status Bit"]
pub struct IS6_R(crate::FieldReader<bool, bool>);
impl IS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS7` reader - XDMAC Channel 7 Interrupt Status Bit"]
pub struct IS7_R(crate::FieldReader<bool, bool>);
impl IS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS8` reader - XDMAC Channel 8 Interrupt Status Bit"]
pub struct IS8_R(crate::FieldReader<bool, bool>);
impl IS8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS9` reader - XDMAC Channel 9 Interrupt Status Bit"]
pub struct IS9_R(crate::FieldReader<bool, bool>);
impl IS9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS10` reader - XDMAC Channel 10 Interrupt Status Bit"]
pub struct IS10_R(crate::FieldReader<bool, bool>);
impl IS10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS11` reader - XDMAC Channel 11 Interrupt Status Bit"]
pub struct IS11_R(crate::FieldReader<bool, bool>);
impl IS11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS12` reader - XDMAC Channel 12 Interrupt Status Bit"]
pub struct IS12_R(crate::FieldReader<bool, bool>);
impl IS12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS13` reader - XDMAC Channel 13 Interrupt Status Bit"]
pub struct IS13_R(crate::FieldReader<bool, bool>);
impl IS13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS14` reader - XDMAC Channel 14 Interrupt Status Bit"]
pub struct IS14_R(crate::FieldReader<bool, bool>);
impl IS14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS15` reader - XDMAC Channel 15 Interrupt Status Bit"]
pub struct IS15_R(crate::FieldReader<bool, bool>);
impl IS15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS16` reader - XDMAC Channel 16 Interrupt Status Bit"]
pub struct IS16_R(crate::FieldReader<bool, bool>);
impl IS16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS17` reader - XDMAC Channel 17 Interrupt Status Bit"]
pub struct IS17_R(crate::FieldReader<bool, bool>);
impl IS17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS18` reader - XDMAC Channel 18 Interrupt Status Bit"]
pub struct IS18_R(crate::FieldReader<bool, bool>);
impl IS18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS19` reader - XDMAC Channel 19 Interrupt Status Bit"]
pub struct IS19_R(crate::FieldReader<bool, bool>);
impl IS19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS20` reader - XDMAC Channel 20 Interrupt Status Bit"]
pub struct IS20_R(crate::FieldReader<bool, bool>);
impl IS20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS21` reader - XDMAC Channel 21 Interrupt Status Bit"]
pub struct IS21_R(crate::FieldReader<bool, bool>);
impl IS21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS22` reader - XDMAC Channel 22 Interrupt Status Bit"]
pub struct IS22_R(crate::FieldReader<bool, bool>);
impl IS22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS23` reader - XDMAC Channel 23 Interrupt Status Bit"]
pub struct IS23_R(crate::FieldReader<bool, bool>);
impl IS23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is0(&self) -> IS0_R {
        IS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is1(&self) -> IS1_R {
        IS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is2(&self) -> IS2_R {
        IS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is3(&self) -> IS3_R {
        IS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is4(&self) -> IS4_R {
        IS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is5(&self) -> IS5_R {
        IS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is6(&self) -> IS6_R {
        IS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is7(&self) -> IS7_R {
        IS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is8(&self) -> IS8_R {
        IS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is9(&self) -> IS9_R {
        IS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is10(&self) -> IS10_R {
        IS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is11(&self) -> IS11_R {
        IS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is12(&self) -> IS12_R {
        IS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is13(&self) -> IS13_R {
        IS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is14(&self) -> IS14_R {
        IS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is15(&self) -> IS15_R {
        IS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is16(&self) -> IS16_R {
        IS16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is17(&self) -> IS17_R {
        IS17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is18(&self) -> IS18_R {
        IS18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is19(&self) -> IS19_R {
        IS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is20(&self) -> IS20_R {
        IS20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is21(&self) -> IS21_R {
        IS21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is22(&self) -> IS22_R {
        IS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is23(&self) -> IS23_R {
        IS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gis](index.html) module"]
pub struct XDMAC_GIS_SPEC;
impl crate::RegisterSpec for XDMAC_GIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_gis::R](R) reader structure"]
impl crate::Readable for XDMAC_GIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XDMAC_GIS to value 0"]
impl crate::Resettable for XDMAC_GIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
