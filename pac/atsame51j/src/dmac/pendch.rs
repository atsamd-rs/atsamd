#[doc = "Register `PENDCH` reader"]
pub type R = crate::R<PENDCH_SPEC>;
#[doc = "Field `PENDCH0` reader - Pending Channel 0"]
pub type PENDCH0_R = crate::BitReader;
#[doc = "Field `PENDCH1` reader - Pending Channel 1"]
pub type PENDCH1_R = crate::BitReader;
#[doc = "Field `PENDCH2` reader - Pending Channel 2"]
pub type PENDCH2_R = crate::BitReader;
#[doc = "Field `PENDCH3` reader - Pending Channel 3"]
pub type PENDCH3_R = crate::BitReader;
#[doc = "Field `PENDCH4` reader - Pending Channel 4"]
pub type PENDCH4_R = crate::BitReader;
#[doc = "Field `PENDCH5` reader - Pending Channel 5"]
pub type PENDCH5_R = crate::BitReader;
#[doc = "Field `PENDCH6` reader - Pending Channel 6"]
pub type PENDCH6_R = crate::BitReader;
#[doc = "Field `PENDCH7` reader - Pending Channel 7"]
pub type PENDCH7_R = crate::BitReader;
#[doc = "Field `PENDCH8` reader - Pending Channel 8"]
pub type PENDCH8_R = crate::BitReader;
#[doc = "Field `PENDCH9` reader - Pending Channel 9"]
pub type PENDCH9_R = crate::BitReader;
#[doc = "Field `PENDCH10` reader - Pending Channel 10"]
pub type PENDCH10_R = crate::BitReader;
#[doc = "Field `PENDCH11` reader - Pending Channel 11"]
pub type PENDCH11_R = crate::BitReader;
#[doc = "Field `PENDCH12` reader - Pending Channel 12"]
pub type PENDCH12_R = crate::BitReader;
#[doc = "Field `PENDCH13` reader - Pending Channel 13"]
pub type PENDCH13_R = crate::BitReader;
#[doc = "Field `PENDCH14` reader - Pending Channel 14"]
pub type PENDCH14_R = crate::BitReader;
#[doc = "Field `PENDCH15` reader - Pending Channel 15"]
pub type PENDCH15_R = crate::BitReader;
#[doc = "Field `PENDCH16` reader - Pending Channel 16"]
pub type PENDCH16_R = crate::BitReader;
#[doc = "Field `PENDCH17` reader - Pending Channel 17"]
pub type PENDCH17_R = crate::BitReader;
#[doc = "Field `PENDCH18` reader - Pending Channel 18"]
pub type PENDCH18_R = crate::BitReader;
#[doc = "Field `PENDCH19` reader - Pending Channel 19"]
pub type PENDCH19_R = crate::BitReader;
#[doc = "Field `PENDCH20` reader - Pending Channel 20"]
pub type PENDCH20_R = crate::BitReader;
#[doc = "Field `PENDCH21` reader - Pending Channel 21"]
pub type PENDCH21_R = crate::BitReader;
#[doc = "Field `PENDCH22` reader - Pending Channel 22"]
pub type PENDCH22_R = crate::BitReader;
#[doc = "Field `PENDCH23` reader - Pending Channel 23"]
pub type PENDCH23_R = crate::BitReader;
#[doc = "Field `PENDCH24` reader - Pending Channel 24"]
pub type PENDCH24_R = crate::BitReader;
#[doc = "Field `PENDCH25` reader - Pending Channel 25"]
pub type PENDCH25_R = crate::BitReader;
#[doc = "Field `PENDCH26` reader - Pending Channel 26"]
pub type PENDCH26_R = crate::BitReader;
#[doc = "Field `PENDCH27` reader - Pending Channel 27"]
pub type PENDCH27_R = crate::BitReader;
#[doc = "Field `PENDCH28` reader - Pending Channel 28"]
pub type PENDCH28_R = crate::BitReader;
#[doc = "Field `PENDCH29` reader - Pending Channel 29"]
pub type PENDCH29_R = crate::BitReader;
#[doc = "Field `PENDCH30` reader - Pending Channel 30"]
pub type PENDCH30_R = crate::BitReader;
#[doc = "Field `PENDCH31` reader - Pending Channel 31"]
pub type PENDCH31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pending Channel 0"]
    #[inline(always)]
    pub fn pendch0(&self) -> PENDCH0_R {
        PENDCH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending Channel 1"]
    #[inline(always)]
    pub fn pendch1(&self) -> PENDCH1_R {
        PENDCH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending Channel 2"]
    #[inline(always)]
    pub fn pendch2(&self) -> PENDCH2_R {
        PENDCH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending Channel 3"]
    #[inline(always)]
    pub fn pendch3(&self) -> PENDCH3_R {
        PENDCH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending Channel 4"]
    #[inline(always)]
    pub fn pendch4(&self) -> PENDCH4_R {
        PENDCH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending Channel 5"]
    #[inline(always)]
    pub fn pendch5(&self) -> PENDCH5_R {
        PENDCH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending Channel 6"]
    #[inline(always)]
    pub fn pendch6(&self) -> PENDCH6_R {
        PENDCH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending Channel 7"]
    #[inline(always)]
    pub fn pendch7(&self) -> PENDCH7_R {
        PENDCH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending Channel 8"]
    #[inline(always)]
    pub fn pendch8(&self) -> PENDCH8_R {
        PENDCH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending Channel 9"]
    #[inline(always)]
    pub fn pendch9(&self) -> PENDCH9_R {
        PENDCH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending Channel 10"]
    #[inline(always)]
    pub fn pendch10(&self) -> PENDCH10_R {
        PENDCH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending Channel 11"]
    #[inline(always)]
    pub fn pendch11(&self) -> PENDCH11_R {
        PENDCH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending Channel 12"]
    #[inline(always)]
    pub fn pendch12(&self) -> PENDCH12_R {
        PENDCH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending Channel 13"]
    #[inline(always)]
    pub fn pendch13(&self) -> PENDCH13_R {
        PENDCH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending Channel 14"]
    #[inline(always)]
    pub fn pendch14(&self) -> PENDCH14_R {
        PENDCH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending Channel 15"]
    #[inline(always)]
    pub fn pendch15(&self) -> PENDCH15_R {
        PENDCH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pending Channel 16"]
    #[inline(always)]
    pub fn pendch16(&self) -> PENDCH16_R {
        PENDCH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pending Channel 17"]
    #[inline(always)]
    pub fn pendch17(&self) -> PENDCH17_R {
        PENDCH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pending Channel 18"]
    #[inline(always)]
    pub fn pendch18(&self) -> PENDCH18_R {
        PENDCH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pending Channel 19"]
    #[inline(always)]
    pub fn pendch19(&self) -> PENDCH19_R {
        PENDCH19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pending Channel 20"]
    #[inline(always)]
    pub fn pendch20(&self) -> PENDCH20_R {
        PENDCH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pending Channel 21"]
    #[inline(always)]
    pub fn pendch21(&self) -> PENDCH21_R {
        PENDCH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pending Channel 22"]
    #[inline(always)]
    pub fn pendch22(&self) -> PENDCH22_R {
        PENDCH22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Pending Channel 23"]
    #[inline(always)]
    pub fn pendch23(&self) -> PENDCH23_R {
        PENDCH23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Pending Channel 24"]
    #[inline(always)]
    pub fn pendch24(&self) -> PENDCH24_R {
        PENDCH24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pending Channel 25"]
    #[inline(always)]
    pub fn pendch25(&self) -> PENDCH25_R {
        PENDCH25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pending Channel 26"]
    #[inline(always)]
    pub fn pendch26(&self) -> PENDCH26_R {
        PENDCH26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Pending Channel 27"]
    #[inline(always)]
    pub fn pendch27(&self) -> PENDCH27_R {
        PENDCH27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pending Channel 28"]
    #[inline(always)]
    pub fn pendch28(&self) -> PENDCH28_R {
        PENDCH28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Pending Channel 29"]
    #[inline(always)]
    pub fn pendch29(&self) -> PENDCH29_R {
        PENDCH29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Pending Channel 30"]
    #[inline(always)]
    pub fn pendch30(&self) -> PENDCH30_R {
        PENDCH30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Pending Channel 31"]
    #[inline(always)]
    pub fn pendch31(&self) -> PENDCH31_R {
        PENDCH31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Pending Channels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pendch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PENDCH_SPEC;
impl crate::RegisterSpec for PENDCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pendch::R`](R) reader structure"]
impl crate::Readable for PENDCH_SPEC {}
#[doc = "`reset()` method sets PENDCH to value 0"]
impl crate::Resettable for PENDCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
