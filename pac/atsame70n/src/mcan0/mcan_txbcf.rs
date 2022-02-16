#[doc = "Register `MCAN_TXBCF` reader"]
pub struct R(crate::R<MCAN_TXBCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_TXBCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_TXBCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_TXBCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CF0` reader - Cancellation Finished for Transmit Buffer 0"]
pub struct CF0_R(crate::FieldReader<bool, bool>);
impl CF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF1` reader - Cancellation Finished for Transmit Buffer 1"]
pub struct CF1_R(crate::FieldReader<bool, bool>);
impl CF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF2` reader - Cancellation Finished for Transmit Buffer 2"]
pub struct CF2_R(crate::FieldReader<bool, bool>);
impl CF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF3` reader - Cancellation Finished for Transmit Buffer 3"]
pub struct CF3_R(crate::FieldReader<bool, bool>);
impl CF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF4` reader - Cancellation Finished for Transmit Buffer 4"]
pub struct CF4_R(crate::FieldReader<bool, bool>);
impl CF4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF5` reader - Cancellation Finished for Transmit Buffer 5"]
pub struct CF5_R(crate::FieldReader<bool, bool>);
impl CF5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF6` reader - Cancellation Finished for Transmit Buffer 6"]
pub struct CF6_R(crate::FieldReader<bool, bool>);
impl CF6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF7` reader - Cancellation Finished for Transmit Buffer 7"]
pub struct CF7_R(crate::FieldReader<bool, bool>);
impl CF7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF8` reader - Cancellation Finished for Transmit Buffer 8"]
pub struct CF8_R(crate::FieldReader<bool, bool>);
impl CF8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF9` reader - Cancellation Finished for Transmit Buffer 9"]
pub struct CF9_R(crate::FieldReader<bool, bool>);
impl CF9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF10` reader - Cancellation Finished for Transmit Buffer 10"]
pub struct CF10_R(crate::FieldReader<bool, bool>);
impl CF10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF11` reader - Cancellation Finished for Transmit Buffer 11"]
pub struct CF11_R(crate::FieldReader<bool, bool>);
impl CF11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF12` reader - Cancellation Finished for Transmit Buffer 12"]
pub struct CF12_R(crate::FieldReader<bool, bool>);
impl CF12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF13` reader - Cancellation Finished for Transmit Buffer 13"]
pub struct CF13_R(crate::FieldReader<bool, bool>);
impl CF13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF14` reader - Cancellation Finished for Transmit Buffer 14"]
pub struct CF14_R(crate::FieldReader<bool, bool>);
impl CF14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF15` reader - Cancellation Finished for Transmit Buffer 15"]
pub struct CF15_R(crate::FieldReader<bool, bool>);
impl CF15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF16` reader - Cancellation Finished for Transmit Buffer 16"]
pub struct CF16_R(crate::FieldReader<bool, bool>);
impl CF16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF17` reader - Cancellation Finished for Transmit Buffer 17"]
pub struct CF17_R(crate::FieldReader<bool, bool>);
impl CF17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF18` reader - Cancellation Finished for Transmit Buffer 18"]
pub struct CF18_R(crate::FieldReader<bool, bool>);
impl CF18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF19` reader - Cancellation Finished for Transmit Buffer 19"]
pub struct CF19_R(crate::FieldReader<bool, bool>);
impl CF19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF20` reader - Cancellation Finished for Transmit Buffer 20"]
pub struct CF20_R(crate::FieldReader<bool, bool>);
impl CF20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF21` reader - Cancellation Finished for Transmit Buffer 21"]
pub struct CF21_R(crate::FieldReader<bool, bool>);
impl CF21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF22` reader - Cancellation Finished for Transmit Buffer 22"]
pub struct CF22_R(crate::FieldReader<bool, bool>);
impl CF22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF23` reader - Cancellation Finished for Transmit Buffer 23"]
pub struct CF23_R(crate::FieldReader<bool, bool>);
impl CF23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF24` reader - Cancellation Finished for Transmit Buffer 24"]
pub struct CF24_R(crate::FieldReader<bool, bool>);
impl CF24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF25` reader - Cancellation Finished for Transmit Buffer 25"]
pub struct CF25_R(crate::FieldReader<bool, bool>);
impl CF25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF26` reader - Cancellation Finished for Transmit Buffer 26"]
pub struct CF26_R(crate::FieldReader<bool, bool>);
impl CF26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF27` reader - Cancellation Finished for Transmit Buffer 27"]
pub struct CF27_R(crate::FieldReader<bool, bool>);
impl CF27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF28` reader - Cancellation Finished for Transmit Buffer 28"]
pub struct CF28_R(crate::FieldReader<bool, bool>);
impl CF28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF29` reader - Cancellation Finished for Transmit Buffer 29"]
pub struct CF29_R(crate::FieldReader<bool, bool>);
impl CF29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF30` reader - Cancellation Finished for Transmit Buffer 30"]
pub struct CF30_R(crate::FieldReader<bool, bool>);
impl CF30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF31` reader - Cancellation Finished for Transmit Buffer 31"]
pub struct CF31_R(crate::FieldReader<bool, bool>);
impl CF31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CF31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Cancellation Finished for Transmit Buffer 0"]
    #[inline(always)]
    pub fn cf0(&self) -> CF0_R {
        CF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Cancellation Finished for Transmit Buffer 1"]
    #[inline(always)]
    pub fn cf1(&self) -> CF1_R {
        CF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cancellation Finished for Transmit Buffer 2"]
    #[inline(always)]
    pub fn cf2(&self) -> CF2_R {
        CF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Cancellation Finished for Transmit Buffer 3"]
    #[inline(always)]
    pub fn cf3(&self) -> CF3_R {
        CF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Cancellation Finished for Transmit Buffer 4"]
    #[inline(always)]
    pub fn cf4(&self) -> CF4_R {
        CF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Cancellation Finished for Transmit Buffer 5"]
    #[inline(always)]
    pub fn cf5(&self) -> CF5_R {
        CF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Cancellation Finished for Transmit Buffer 6"]
    #[inline(always)]
    pub fn cf6(&self) -> CF6_R {
        CF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Cancellation Finished for Transmit Buffer 7"]
    #[inline(always)]
    pub fn cf7(&self) -> CF7_R {
        CF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Cancellation Finished for Transmit Buffer 8"]
    #[inline(always)]
    pub fn cf8(&self) -> CF8_R {
        CF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Cancellation Finished for Transmit Buffer 9"]
    #[inline(always)]
    pub fn cf9(&self) -> CF9_R {
        CF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Cancellation Finished for Transmit Buffer 10"]
    #[inline(always)]
    pub fn cf10(&self) -> CF10_R {
        CF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Cancellation Finished for Transmit Buffer 11"]
    #[inline(always)]
    pub fn cf11(&self) -> CF11_R {
        CF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Cancellation Finished for Transmit Buffer 12"]
    #[inline(always)]
    pub fn cf12(&self) -> CF12_R {
        CF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Cancellation Finished for Transmit Buffer 13"]
    #[inline(always)]
    pub fn cf13(&self) -> CF13_R {
        CF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Cancellation Finished for Transmit Buffer 14"]
    #[inline(always)]
    pub fn cf14(&self) -> CF14_R {
        CF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Cancellation Finished for Transmit Buffer 15"]
    #[inline(always)]
    pub fn cf15(&self) -> CF15_R {
        CF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Cancellation Finished for Transmit Buffer 16"]
    #[inline(always)]
    pub fn cf16(&self) -> CF16_R {
        CF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Cancellation Finished for Transmit Buffer 17"]
    #[inline(always)]
    pub fn cf17(&self) -> CF17_R {
        CF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Cancellation Finished for Transmit Buffer 18"]
    #[inline(always)]
    pub fn cf18(&self) -> CF18_R {
        CF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Cancellation Finished for Transmit Buffer 19"]
    #[inline(always)]
    pub fn cf19(&self) -> CF19_R {
        CF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Cancellation Finished for Transmit Buffer 20"]
    #[inline(always)]
    pub fn cf20(&self) -> CF20_R {
        CF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Cancellation Finished for Transmit Buffer 21"]
    #[inline(always)]
    pub fn cf21(&self) -> CF21_R {
        CF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Cancellation Finished for Transmit Buffer 22"]
    #[inline(always)]
    pub fn cf22(&self) -> CF22_R {
        CF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Cancellation Finished for Transmit Buffer 23"]
    #[inline(always)]
    pub fn cf23(&self) -> CF23_R {
        CF23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Cancellation Finished for Transmit Buffer 24"]
    #[inline(always)]
    pub fn cf24(&self) -> CF24_R {
        CF24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Cancellation Finished for Transmit Buffer 25"]
    #[inline(always)]
    pub fn cf25(&self) -> CF25_R {
        CF25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Cancellation Finished for Transmit Buffer 26"]
    #[inline(always)]
    pub fn cf26(&self) -> CF26_R {
        CF26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Cancellation Finished for Transmit Buffer 27"]
    #[inline(always)]
    pub fn cf27(&self) -> CF27_R {
        CF27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Cancellation Finished for Transmit Buffer 28"]
    #[inline(always)]
    pub fn cf28(&self) -> CF28_R {
        CF28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Cancellation Finished for Transmit Buffer 29"]
    #[inline(always)]
    pub fn cf29(&self) -> CF29_R {
        CF29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Cancellation Finished for Transmit Buffer 30"]
    #[inline(always)]
    pub fn cf30(&self) -> CF30_R {
        CF30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Cancellation Finished for Transmit Buffer 31"]
    #[inline(always)]
    pub fn cf31(&self) -> CF31_R {
        CF31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Transmit Buffer Cancellation Finished Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txbcf](index.html) module"]
pub struct MCAN_TXBCF_SPEC;
impl crate::RegisterSpec for MCAN_TXBCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_txbcf::R](R) reader structure"]
impl crate::Readable for MCAN_TXBCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCAN_TXBCF to value 0"]
impl crate::Resettable for MCAN_TXBCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
