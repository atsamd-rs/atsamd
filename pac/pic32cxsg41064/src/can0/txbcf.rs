#[doc = "Register `TXBCF` reader"]
pub type R = crate::R<TxbcfSpec>;
#[doc = "Field `CF0` reader - Tx Buffer Cancellation Finished 0"]
pub type Cf0R = crate::BitReader;
#[doc = "Field `CF1` reader - Tx Buffer Cancellation Finished 1"]
pub type Cf1R = crate::BitReader;
#[doc = "Field `CF2` reader - Tx Buffer Cancellation Finished 2"]
pub type Cf2R = crate::BitReader;
#[doc = "Field `CF3` reader - Tx Buffer Cancellation Finished 3"]
pub type Cf3R = crate::BitReader;
#[doc = "Field `CF4` reader - Tx Buffer Cancellation Finished 4"]
pub type Cf4R = crate::BitReader;
#[doc = "Field `CF5` reader - Tx Buffer Cancellation Finished 5"]
pub type Cf5R = crate::BitReader;
#[doc = "Field `CF6` reader - Tx Buffer Cancellation Finished 6"]
pub type Cf6R = crate::BitReader;
#[doc = "Field `CF7` reader - Tx Buffer Cancellation Finished 7"]
pub type Cf7R = crate::BitReader;
#[doc = "Field `CF8` reader - Tx Buffer Cancellation Finished 8"]
pub type Cf8R = crate::BitReader;
#[doc = "Field `CF9` reader - Tx Buffer Cancellation Finished 9"]
pub type Cf9R = crate::BitReader;
#[doc = "Field `CF10` reader - Tx Buffer Cancellation Finished 10"]
pub type Cf10R = crate::BitReader;
#[doc = "Field `CF11` reader - Tx Buffer Cancellation Finished 11"]
pub type Cf11R = crate::BitReader;
#[doc = "Field `CF12` reader - Tx Buffer Cancellation Finished 12"]
pub type Cf12R = crate::BitReader;
#[doc = "Field `CF13` reader - Tx Buffer Cancellation Finished 13"]
pub type Cf13R = crate::BitReader;
#[doc = "Field `CF14` reader - Tx Buffer Cancellation Finished 14"]
pub type Cf14R = crate::BitReader;
#[doc = "Field `CF15` reader - Tx Buffer Cancellation Finished 15"]
pub type Cf15R = crate::BitReader;
#[doc = "Field `CF16` reader - Tx Buffer Cancellation Finished 16"]
pub type Cf16R = crate::BitReader;
#[doc = "Field `CF17` reader - Tx Buffer Cancellation Finished 17"]
pub type Cf17R = crate::BitReader;
#[doc = "Field `CF18` reader - Tx Buffer Cancellation Finished 18"]
pub type Cf18R = crate::BitReader;
#[doc = "Field `CF19` reader - Tx Buffer Cancellation Finished 19"]
pub type Cf19R = crate::BitReader;
#[doc = "Field `CF20` reader - Tx Buffer Cancellation Finished 20"]
pub type Cf20R = crate::BitReader;
#[doc = "Field `CF21` reader - Tx Buffer Cancellation Finished 21"]
pub type Cf21R = crate::BitReader;
#[doc = "Field `CF22` reader - Tx Buffer Cancellation Finished 22"]
pub type Cf22R = crate::BitReader;
#[doc = "Field `CF23` reader - Tx Buffer Cancellation Finished 23"]
pub type Cf23R = crate::BitReader;
#[doc = "Field `CF24` reader - Tx Buffer Cancellation Finished 24"]
pub type Cf24R = crate::BitReader;
#[doc = "Field `CF25` reader - Tx Buffer Cancellation Finished 25"]
pub type Cf25R = crate::BitReader;
#[doc = "Field `CF26` reader - Tx Buffer Cancellation Finished 26"]
pub type Cf26R = crate::BitReader;
#[doc = "Field `CF27` reader - Tx Buffer Cancellation Finished 27"]
pub type Cf27R = crate::BitReader;
#[doc = "Field `CF28` reader - Tx Buffer Cancellation Finished 28"]
pub type Cf28R = crate::BitReader;
#[doc = "Field `CF29` reader - Tx Buffer Cancellation Finished 29"]
pub type Cf29R = crate::BitReader;
#[doc = "Field `CF30` reader - Tx Buffer Cancellation Finished 30"]
pub type Cf30R = crate::BitReader;
#[doc = "Field `CF31` reader - Tx Buffer Cancellation Finished 31"]
pub type Cf31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Tx Buffer Cancellation Finished 0"]
    #[inline(always)]
    pub fn cf0(&self) -> Cf0R {
        Cf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx Buffer Cancellation Finished 1"]
    #[inline(always)]
    pub fn cf1(&self) -> Cf1R {
        Cf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tx Buffer Cancellation Finished 2"]
    #[inline(always)]
    pub fn cf2(&self) -> Cf2R {
        Cf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx Buffer Cancellation Finished 3"]
    #[inline(always)]
    pub fn cf3(&self) -> Cf3R {
        Cf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx Buffer Cancellation Finished 4"]
    #[inline(always)]
    pub fn cf4(&self) -> Cf4R {
        Cf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx Buffer Cancellation Finished 5"]
    #[inline(always)]
    pub fn cf5(&self) -> Cf5R {
        Cf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tx Buffer Cancellation Finished 6"]
    #[inline(always)]
    pub fn cf6(&self) -> Cf6R {
        Cf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx Buffer Cancellation Finished 7"]
    #[inline(always)]
    pub fn cf7(&self) -> Cf7R {
        Cf7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx Buffer Cancellation Finished 8"]
    #[inline(always)]
    pub fn cf8(&self) -> Cf8R {
        Cf8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx Buffer Cancellation Finished 9"]
    #[inline(always)]
    pub fn cf9(&self) -> Cf9R {
        Cf9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Buffer Cancellation Finished 10"]
    #[inline(always)]
    pub fn cf10(&self) -> Cf10R {
        Cf10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Buffer Cancellation Finished 11"]
    #[inline(always)]
    pub fn cf11(&self) -> Cf11R {
        Cf11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Buffer Cancellation Finished 12"]
    #[inline(always)]
    pub fn cf12(&self) -> Cf12R {
        Cf12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Buffer Cancellation Finished 13"]
    #[inline(always)]
    pub fn cf13(&self) -> Cf13R {
        Cf13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Buffer Cancellation Finished 14"]
    #[inline(always)]
    pub fn cf14(&self) -> Cf14R {
        Cf14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Buffer Cancellation Finished 15"]
    #[inline(always)]
    pub fn cf15(&self) -> Cf15R {
        Cf15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx Buffer Cancellation Finished 16"]
    #[inline(always)]
    pub fn cf16(&self) -> Cf16R {
        Cf16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tx Buffer Cancellation Finished 17"]
    #[inline(always)]
    pub fn cf17(&self) -> Cf17R {
        Cf17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx Buffer Cancellation Finished 18"]
    #[inline(always)]
    pub fn cf18(&self) -> Cf18R {
        Cf18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Tx Buffer Cancellation Finished 19"]
    #[inline(always)]
    pub fn cf19(&self) -> Cf19R {
        Cf19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tx Buffer Cancellation Finished 20"]
    #[inline(always)]
    pub fn cf20(&self) -> Cf20R {
        Cf20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Tx Buffer Cancellation Finished 21"]
    #[inline(always)]
    pub fn cf21(&self) -> Cf21R {
        Cf21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Tx Buffer Cancellation Finished 22"]
    #[inline(always)]
    pub fn cf22(&self) -> Cf22R {
        Cf22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tx Buffer Cancellation Finished 23"]
    #[inline(always)]
    pub fn cf23(&self) -> Cf23R {
        Cf23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Tx Buffer Cancellation Finished 24"]
    #[inline(always)]
    pub fn cf24(&self) -> Cf24R {
        Cf24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Buffer Cancellation Finished 25"]
    #[inline(always)]
    pub fn cf25(&self) -> Cf25R {
        Cf25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tx Buffer Cancellation Finished 26"]
    #[inline(always)]
    pub fn cf26(&self) -> Cf26R {
        Cf26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Tx Buffer Cancellation Finished 27"]
    #[inline(always)]
    pub fn cf27(&self) -> Cf27R {
        Cf27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Tx Buffer Cancellation Finished 28"]
    #[inline(always)]
    pub fn cf28(&self) -> Cf28R {
        Cf28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Tx Buffer Cancellation Finished 29"]
    #[inline(always)]
    pub fn cf29(&self) -> Cf29R {
        Cf29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Tx Buffer Cancellation Finished 30"]
    #[inline(always)]
    pub fn cf30(&self) -> Cf30R {
        Cf30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Tx Buffer Cancellation Finished 31"]
    #[inline(always)]
    pub fn cf31(&self) -> Cf31R {
        Cf31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Tx Buffer Cancellation Finished\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcfSpec;
impl crate::RegisterSpec for TxbcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcf::R`](R) reader structure"]
impl crate::Readable for TxbcfSpec {}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TxbcfSpec {
    const RESET_VALUE: u32 = 0;
}
