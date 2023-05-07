#[doc = "Register `TXBCF` reader"]
pub struct R(crate::R<TXBCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CF0` reader - Tx Buffer Cancellation Finished 0"]
pub type CF0_R = crate::BitReader<bool>;
#[doc = "Field `CF1` reader - Tx Buffer Cancellation Finished 1"]
pub type CF1_R = crate::BitReader<bool>;
#[doc = "Field `CF2` reader - Tx Buffer Cancellation Finished 2"]
pub type CF2_R = crate::BitReader<bool>;
#[doc = "Field `CF3` reader - Tx Buffer Cancellation Finished 3"]
pub type CF3_R = crate::BitReader<bool>;
#[doc = "Field `CF4` reader - Tx Buffer Cancellation Finished 4"]
pub type CF4_R = crate::BitReader<bool>;
#[doc = "Field `CF5` reader - Tx Buffer Cancellation Finished 5"]
pub type CF5_R = crate::BitReader<bool>;
#[doc = "Field `CF6` reader - Tx Buffer Cancellation Finished 6"]
pub type CF6_R = crate::BitReader<bool>;
#[doc = "Field `CF7` reader - Tx Buffer Cancellation Finished 7"]
pub type CF7_R = crate::BitReader<bool>;
#[doc = "Field `CF8` reader - Tx Buffer Cancellation Finished 8"]
pub type CF8_R = crate::BitReader<bool>;
#[doc = "Field `CF9` reader - Tx Buffer Cancellation Finished 9"]
pub type CF9_R = crate::BitReader<bool>;
#[doc = "Field `CF10` reader - Tx Buffer Cancellation Finished 10"]
pub type CF10_R = crate::BitReader<bool>;
#[doc = "Field `CF11` reader - Tx Buffer Cancellation Finished 11"]
pub type CF11_R = crate::BitReader<bool>;
#[doc = "Field `CF12` reader - Tx Buffer Cancellation Finished 12"]
pub type CF12_R = crate::BitReader<bool>;
#[doc = "Field `CF13` reader - Tx Buffer Cancellation Finished 13"]
pub type CF13_R = crate::BitReader<bool>;
#[doc = "Field `CF14` reader - Tx Buffer Cancellation Finished 14"]
pub type CF14_R = crate::BitReader<bool>;
#[doc = "Field `CF15` reader - Tx Buffer Cancellation Finished 15"]
pub type CF15_R = crate::BitReader<bool>;
#[doc = "Field `CF16` reader - Tx Buffer Cancellation Finished 16"]
pub type CF16_R = crate::BitReader<bool>;
#[doc = "Field `CF17` reader - Tx Buffer Cancellation Finished 17"]
pub type CF17_R = crate::BitReader<bool>;
#[doc = "Field `CF18` reader - Tx Buffer Cancellation Finished 18"]
pub type CF18_R = crate::BitReader<bool>;
#[doc = "Field `CF19` reader - Tx Buffer Cancellation Finished 19"]
pub type CF19_R = crate::BitReader<bool>;
#[doc = "Field `CF20` reader - Tx Buffer Cancellation Finished 20"]
pub type CF20_R = crate::BitReader<bool>;
#[doc = "Field `CF21` reader - Tx Buffer Cancellation Finished 21"]
pub type CF21_R = crate::BitReader<bool>;
#[doc = "Field `CF22` reader - Tx Buffer Cancellation Finished 22"]
pub type CF22_R = crate::BitReader<bool>;
#[doc = "Field `CF23` reader - Tx Buffer Cancellation Finished 23"]
pub type CF23_R = crate::BitReader<bool>;
#[doc = "Field `CF24` reader - Tx Buffer Cancellation Finished 24"]
pub type CF24_R = crate::BitReader<bool>;
#[doc = "Field `CF25` reader - Tx Buffer Cancellation Finished 25"]
pub type CF25_R = crate::BitReader<bool>;
#[doc = "Field `CF26` reader - Tx Buffer Cancellation Finished 26"]
pub type CF26_R = crate::BitReader<bool>;
#[doc = "Field `CF27` reader - Tx Buffer Cancellation Finished 27"]
pub type CF27_R = crate::BitReader<bool>;
#[doc = "Field `CF28` reader - Tx Buffer Cancellation Finished 28"]
pub type CF28_R = crate::BitReader<bool>;
#[doc = "Field `CF29` reader - Tx Buffer Cancellation Finished 29"]
pub type CF29_R = crate::BitReader<bool>;
#[doc = "Field `CF30` reader - Tx Buffer Cancellation Finished 30"]
pub type CF30_R = crate::BitReader<bool>;
#[doc = "Field `CF31` reader - Tx Buffer Cancellation Finished 31"]
pub type CF31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Tx Buffer Cancellation Finished 0"]
    #[inline(always)]
    pub fn cf0(&self) -> CF0_R {
        CF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx Buffer Cancellation Finished 1"]
    #[inline(always)]
    pub fn cf1(&self) -> CF1_R {
        CF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tx Buffer Cancellation Finished 2"]
    #[inline(always)]
    pub fn cf2(&self) -> CF2_R {
        CF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx Buffer Cancellation Finished 3"]
    #[inline(always)]
    pub fn cf3(&self) -> CF3_R {
        CF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx Buffer Cancellation Finished 4"]
    #[inline(always)]
    pub fn cf4(&self) -> CF4_R {
        CF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx Buffer Cancellation Finished 5"]
    #[inline(always)]
    pub fn cf5(&self) -> CF5_R {
        CF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tx Buffer Cancellation Finished 6"]
    #[inline(always)]
    pub fn cf6(&self) -> CF6_R {
        CF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx Buffer Cancellation Finished 7"]
    #[inline(always)]
    pub fn cf7(&self) -> CF7_R {
        CF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx Buffer Cancellation Finished 8"]
    #[inline(always)]
    pub fn cf8(&self) -> CF8_R {
        CF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx Buffer Cancellation Finished 9"]
    #[inline(always)]
    pub fn cf9(&self) -> CF9_R {
        CF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Buffer Cancellation Finished 10"]
    #[inline(always)]
    pub fn cf10(&self) -> CF10_R {
        CF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Buffer Cancellation Finished 11"]
    #[inline(always)]
    pub fn cf11(&self) -> CF11_R {
        CF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Buffer Cancellation Finished 12"]
    #[inline(always)]
    pub fn cf12(&self) -> CF12_R {
        CF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Buffer Cancellation Finished 13"]
    #[inline(always)]
    pub fn cf13(&self) -> CF13_R {
        CF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Buffer Cancellation Finished 14"]
    #[inline(always)]
    pub fn cf14(&self) -> CF14_R {
        CF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Buffer Cancellation Finished 15"]
    #[inline(always)]
    pub fn cf15(&self) -> CF15_R {
        CF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx Buffer Cancellation Finished 16"]
    #[inline(always)]
    pub fn cf16(&self) -> CF16_R {
        CF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tx Buffer Cancellation Finished 17"]
    #[inline(always)]
    pub fn cf17(&self) -> CF17_R {
        CF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx Buffer Cancellation Finished 18"]
    #[inline(always)]
    pub fn cf18(&self) -> CF18_R {
        CF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Tx Buffer Cancellation Finished 19"]
    #[inline(always)]
    pub fn cf19(&self) -> CF19_R {
        CF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tx Buffer Cancellation Finished 20"]
    #[inline(always)]
    pub fn cf20(&self) -> CF20_R {
        CF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Tx Buffer Cancellation Finished 21"]
    #[inline(always)]
    pub fn cf21(&self) -> CF21_R {
        CF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Tx Buffer Cancellation Finished 22"]
    #[inline(always)]
    pub fn cf22(&self) -> CF22_R {
        CF22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tx Buffer Cancellation Finished 23"]
    #[inline(always)]
    pub fn cf23(&self) -> CF23_R {
        CF23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Tx Buffer Cancellation Finished 24"]
    #[inline(always)]
    pub fn cf24(&self) -> CF24_R {
        CF24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Buffer Cancellation Finished 25"]
    #[inline(always)]
    pub fn cf25(&self) -> CF25_R {
        CF25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tx Buffer Cancellation Finished 26"]
    #[inline(always)]
    pub fn cf26(&self) -> CF26_R {
        CF26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Tx Buffer Cancellation Finished 27"]
    #[inline(always)]
    pub fn cf27(&self) -> CF27_R {
        CF27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Tx Buffer Cancellation Finished 28"]
    #[inline(always)]
    pub fn cf28(&self) -> CF28_R {
        CF28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Tx Buffer Cancellation Finished 29"]
    #[inline(always)]
    pub fn cf29(&self) -> CF29_R {
        CF29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Tx Buffer Cancellation Finished 30"]
    #[inline(always)]
    pub fn cf30(&self) -> CF30_R {
        CF30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Tx Buffer Cancellation Finished 31"]
    #[inline(always)]
    pub fn cf31(&self) -> CF31_R {
        CF31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Tx Buffer Cancellation Finished\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcf](index.html) module"]
pub struct TXBCF_SPEC;
impl crate::RegisterSpec for TXBCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbcf::R](R) reader structure"]
impl crate::Readable for TXBCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TXBCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
