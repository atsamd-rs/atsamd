#[doc = "Register `PENDCH` reader"]
pub struct R(crate::R<PENDCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PENDCH0` reader - Pending Channel 0"]
pub struct PENDCH0_R(crate::FieldReader<bool, bool>);
impl PENDCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH1` reader - Pending Channel 1"]
pub struct PENDCH1_R(crate::FieldReader<bool, bool>);
impl PENDCH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH2` reader - Pending Channel 2"]
pub struct PENDCH2_R(crate::FieldReader<bool, bool>);
impl PENDCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH3` reader - Pending Channel 3"]
pub struct PENDCH3_R(crate::FieldReader<bool, bool>);
impl PENDCH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH4` reader - Pending Channel 4"]
pub struct PENDCH4_R(crate::FieldReader<bool, bool>);
impl PENDCH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH5` reader - Pending Channel 5"]
pub struct PENDCH5_R(crate::FieldReader<bool, bool>);
impl PENDCH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH6` reader - Pending Channel 6"]
pub struct PENDCH6_R(crate::FieldReader<bool, bool>);
impl PENDCH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH7` reader - Pending Channel 7"]
pub struct PENDCH7_R(crate::FieldReader<bool, bool>);
impl PENDCH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH8` reader - Pending Channel 8"]
pub struct PENDCH8_R(crate::FieldReader<bool, bool>);
impl PENDCH8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH9` reader - Pending Channel 9"]
pub struct PENDCH9_R(crate::FieldReader<bool, bool>);
impl PENDCH9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH10` reader - Pending Channel 10"]
pub struct PENDCH10_R(crate::FieldReader<bool, bool>);
impl PENDCH10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH11` reader - Pending Channel 11"]
pub struct PENDCH11_R(crate::FieldReader<bool, bool>);
impl PENDCH11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH12` reader - Pending Channel 12"]
pub struct PENDCH12_R(crate::FieldReader<bool, bool>);
impl PENDCH12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH13` reader - Pending Channel 13"]
pub struct PENDCH13_R(crate::FieldReader<bool, bool>);
impl PENDCH13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH14` reader - Pending Channel 14"]
pub struct PENDCH14_R(crate::FieldReader<bool, bool>);
impl PENDCH14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH15` reader - Pending Channel 15"]
pub struct PENDCH15_R(crate::FieldReader<bool, bool>);
impl PENDCH15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH16` reader - Pending Channel 16"]
pub struct PENDCH16_R(crate::FieldReader<bool, bool>);
impl PENDCH16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH17` reader - Pending Channel 17"]
pub struct PENDCH17_R(crate::FieldReader<bool, bool>);
impl PENDCH17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH18` reader - Pending Channel 18"]
pub struct PENDCH18_R(crate::FieldReader<bool, bool>);
impl PENDCH18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH19` reader - Pending Channel 19"]
pub struct PENDCH19_R(crate::FieldReader<bool, bool>);
impl PENDCH19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH20` reader - Pending Channel 20"]
pub struct PENDCH20_R(crate::FieldReader<bool, bool>);
impl PENDCH20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH21` reader - Pending Channel 21"]
pub struct PENDCH21_R(crate::FieldReader<bool, bool>);
impl PENDCH21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH22` reader - Pending Channel 22"]
pub struct PENDCH22_R(crate::FieldReader<bool, bool>);
impl PENDCH22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH23` reader - Pending Channel 23"]
pub struct PENDCH23_R(crate::FieldReader<bool, bool>);
impl PENDCH23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH24` reader - Pending Channel 24"]
pub struct PENDCH24_R(crate::FieldReader<bool, bool>);
impl PENDCH24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH25` reader - Pending Channel 25"]
pub struct PENDCH25_R(crate::FieldReader<bool, bool>);
impl PENDCH25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH26` reader - Pending Channel 26"]
pub struct PENDCH26_R(crate::FieldReader<bool, bool>);
impl PENDCH26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH27` reader - Pending Channel 27"]
pub struct PENDCH27_R(crate::FieldReader<bool, bool>);
impl PENDCH27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH28` reader - Pending Channel 28"]
pub struct PENDCH28_R(crate::FieldReader<bool, bool>);
impl PENDCH28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH29` reader - Pending Channel 29"]
pub struct PENDCH29_R(crate::FieldReader<bool, bool>);
impl PENDCH29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH30` reader - Pending Channel 30"]
pub struct PENDCH30_R(crate::FieldReader<bool, bool>);
impl PENDCH30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDCH31` reader - Pending Channel 31"]
pub struct PENDCH31_R(crate::FieldReader<bool, bool>);
impl PENDCH31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDCH31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDCH31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Pending Channel 0"]
    #[inline(always)]
    pub fn pendch0(&self) -> PENDCH0_R {
        PENDCH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pending Channel 1"]
    #[inline(always)]
    pub fn pendch1(&self) -> PENDCH1_R {
        PENDCH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pending Channel 2"]
    #[inline(always)]
    pub fn pendch2(&self) -> PENDCH2_R {
        PENDCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pending Channel 3"]
    #[inline(always)]
    pub fn pendch3(&self) -> PENDCH3_R {
        PENDCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pending Channel 4"]
    #[inline(always)]
    pub fn pendch4(&self) -> PENDCH4_R {
        PENDCH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pending Channel 5"]
    #[inline(always)]
    pub fn pendch5(&self) -> PENDCH5_R {
        PENDCH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pending Channel 6"]
    #[inline(always)]
    pub fn pendch6(&self) -> PENDCH6_R {
        PENDCH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pending Channel 7"]
    #[inline(always)]
    pub fn pendch7(&self) -> PENDCH7_R {
        PENDCH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pending Channel 8"]
    #[inline(always)]
    pub fn pendch8(&self) -> PENDCH8_R {
        PENDCH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pending Channel 9"]
    #[inline(always)]
    pub fn pendch9(&self) -> PENDCH9_R {
        PENDCH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pending Channel 10"]
    #[inline(always)]
    pub fn pendch10(&self) -> PENDCH10_R {
        PENDCH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pending Channel 11"]
    #[inline(always)]
    pub fn pendch11(&self) -> PENDCH11_R {
        PENDCH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pending Channel 12"]
    #[inline(always)]
    pub fn pendch12(&self) -> PENDCH12_R {
        PENDCH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pending Channel 13"]
    #[inline(always)]
    pub fn pendch13(&self) -> PENDCH13_R {
        PENDCH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pending Channel 14"]
    #[inline(always)]
    pub fn pendch14(&self) -> PENDCH14_R {
        PENDCH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pending Channel 15"]
    #[inline(always)]
    pub fn pendch15(&self) -> PENDCH15_R {
        PENDCH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pending Channel 16"]
    #[inline(always)]
    pub fn pendch16(&self) -> PENDCH16_R {
        PENDCH16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pending Channel 17"]
    #[inline(always)]
    pub fn pendch17(&self) -> PENDCH17_R {
        PENDCH17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pending Channel 18"]
    #[inline(always)]
    pub fn pendch18(&self) -> PENDCH18_R {
        PENDCH18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Pending Channel 19"]
    #[inline(always)]
    pub fn pendch19(&self) -> PENDCH19_R {
        PENDCH19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pending Channel 20"]
    #[inline(always)]
    pub fn pendch20(&self) -> PENDCH20_R {
        PENDCH20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Pending Channel 21"]
    #[inline(always)]
    pub fn pendch21(&self) -> PENDCH21_R {
        PENDCH21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Pending Channel 22"]
    #[inline(always)]
    pub fn pendch22(&self) -> PENDCH22_R {
        PENDCH22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Pending Channel 23"]
    #[inline(always)]
    pub fn pendch23(&self) -> PENDCH23_R {
        PENDCH23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pending Channel 24"]
    #[inline(always)]
    pub fn pendch24(&self) -> PENDCH24_R {
        PENDCH24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pending Channel 25"]
    #[inline(always)]
    pub fn pendch25(&self) -> PENDCH25_R {
        PENDCH25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Pending Channel 26"]
    #[inline(always)]
    pub fn pendch26(&self) -> PENDCH26_R {
        PENDCH26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Pending Channel 27"]
    #[inline(always)]
    pub fn pendch27(&self) -> PENDCH27_R {
        PENDCH27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Pending Channel 28"]
    #[inline(always)]
    pub fn pendch28(&self) -> PENDCH28_R {
        PENDCH28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Pending Channel 29"]
    #[inline(always)]
    pub fn pendch29(&self) -> PENDCH29_R {
        PENDCH29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Pending Channel 30"]
    #[inline(always)]
    pub fn pendch30(&self) -> PENDCH30_R {
        PENDCH30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Pending Channel 31"]
    #[inline(always)]
    pub fn pendch31(&self) -> PENDCH31_R {
        PENDCH31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Pending Channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pendch](index.html) module"]
pub struct PENDCH_SPEC;
impl crate::RegisterSpec for PENDCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pendch::R](R) reader structure"]
impl crate::Readable for PENDCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PENDCH to value 0"]
impl crate::Resettable for PENDCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
