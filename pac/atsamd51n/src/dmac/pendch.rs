#[doc = "Register `PENDCH` reader"]
pub type R = crate::R<PendchSpec>;
#[doc = "Field `PENDCH0` reader - Pending Channel 0"]
pub type Pendch0R = crate::BitReader;
#[doc = "Field `PENDCH1` reader - Pending Channel 1"]
pub type Pendch1R = crate::BitReader;
#[doc = "Field `PENDCH2` reader - Pending Channel 2"]
pub type Pendch2R = crate::BitReader;
#[doc = "Field `PENDCH3` reader - Pending Channel 3"]
pub type Pendch3R = crate::BitReader;
#[doc = "Field `PENDCH4` reader - Pending Channel 4"]
pub type Pendch4R = crate::BitReader;
#[doc = "Field `PENDCH5` reader - Pending Channel 5"]
pub type Pendch5R = crate::BitReader;
#[doc = "Field `PENDCH6` reader - Pending Channel 6"]
pub type Pendch6R = crate::BitReader;
#[doc = "Field `PENDCH7` reader - Pending Channel 7"]
pub type Pendch7R = crate::BitReader;
#[doc = "Field `PENDCH8` reader - Pending Channel 8"]
pub type Pendch8R = crate::BitReader;
#[doc = "Field `PENDCH9` reader - Pending Channel 9"]
pub type Pendch9R = crate::BitReader;
#[doc = "Field `PENDCH10` reader - Pending Channel 10"]
pub type Pendch10R = crate::BitReader;
#[doc = "Field `PENDCH11` reader - Pending Channel 11"]
pub type Pendch11R = crate::BitReader;
#[doc = "Field `PENDCH12` reader - Pending Channel 12"]
pub type Pendch12R = crate::BitReader;
#[doc = "Field `PENDCH13` reader - Pending Channel 13"]
pub type Pendch13R = crate::BitReader;
#[doc = "Field `PENDCH14` reader - Pending Channel 14"]
pub type Pendch14R = crate::BitReader;
#[doc = "Field `PENDCH15` reader - Pending Channel 15"]
pub type Pendch15R = crate::BitReader;
#[doc = "Field `PENDCH16` reader - Pending Channel 16"]
pub type Pendch16R = crate::BitReader;
#[doc = "Field `PENDCH17` reader - Pending Channel 17"]
pub type Pendch17R = crate::BitReader;
#[doc = "Field `PENDCH18` reader - Pending Channel 18"]
pub type Pendch18R = crate::BitReader;
#[doc = "Field `PENDCH19` reader - Pending Channel 19"]
pub type Pendch19R = crate::BitReader;
#[doc = "Field `PENDCH20` reader - Pending Channel 20"]
pub type Pendch20R = crate::BitReader;
#[doc = "Field `PENDCH21` reader - Pending Channel 21"]
pub type Pendch21R = crate::BitReader;
#[doc = "Field `PENDCH22` reader - Pending Channel 22"]
pub type Pendch22R = crate::BitReader;
#[doc = "Field `PENDCH23` reader - Pending Channel 23"]
pub type Pendch23R = crate::BitReader;
#[doc = "Field `PENDCH24` reader - Pending Channel 24"]
pub type Pendch24R = crate::BitReader;
#[doc = "Field `PENDCH25` reader - Pending Channel 25"]
pub type Pendch25R = crate::BitReader;
#[doc = "Field `PENDCH26` reader - Pending Channel 26"]
pub type Pendch26R = crate::BitReader;
#[doc = "Field `PENDCH27` reader - Pending Channel 27"]
pub type Pendch27R = crate::BitReader;
#[doc = "Field `PENDCH28` reader - Pending Channel 28"]
pub type Pendch28R = crate::BitReader;
#[doc = "Field `PENDCH29` reader - Pending Channel 29"]
pub type Pendch29R = crate::BitReader;
#[doc = "Field `PENDCH30` reader - Pending Channel 30"]
pub type Pendch30R = crate::BitReader;
#[doc = "Field `PENDCH31` reader - Pending Channel 31"]
pub type Pendch31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pending Channel 0"]
    #[inline(always)]
    pub fn pendch0(&self) -> Pendch0R {
        Pendch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending Channel 1"]
    #[inline(always)]
    pub fn pendch1(&self) -> Pendch1R {
        Pendch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending Channel 2"]
    #[inline(always)]
    pub fn pendch2(&self) -> Pendch2R {
        Pendch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending Channel 3"]
    #[inline(always)]
    pub fn pendch3(&self) -> Pendch3R {
        Pendch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending Channel 4"]
    #[inline(always)]
    pub fn pendch4(&self) -> Pendch4R {
        Pendch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending Channel 5"]
    #[inline(always)]
    pub fn pendch5(&self) -> Pendch5R {
        Pendch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending Channel 6"]
    #[inline(always)]
    pub fn pendch6(&self) -> Pendch6R {
        Pendch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending Channel 7"]
    #[inline(always)]
    pub fn pendch7(&self) -> Pendch7R {
        Pendch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending Channel 8"]
    #[inline(always)]
    pub fn pendch8(&self) -> Pendch8R {
        Pendch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending Channel 9"]
    #[inline(always)]
    pub fn pendch9(&self) -> Pendch9R {
        Pendch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending Channel 10"]
    #[inline(always)]
    pub fn pendch10(&self) -> Pendch10R {
        Pendch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending Channel 11"]
    #[inline(always)]
    pub fn pendch11(&self) -> Pendch11R {
        Pendch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending Channel 12"]
    #[inline(always)]
    pub fn pendch12(&self) -> Pendch12R {
        Pendch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending Channel 13"]
    #[inline(always)]
    pub fn pendch13(&self) -> Pendch13R {
        Pendch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending Channel 14"]
    #[inline(always)]
    pub fn pendch14(&self) -> Pendch14R {
        Pendch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending Channel 15"]
    #[inline(always)]
    pub fn pendch15(&self) -> Pendch15R {
        Pendch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pending Channel 16"]
    #[inline(always)]
    pub fn pendch16(&self) -> Pendch16R {
        Pendch16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pending Channel 17"]
    #[inline(always)]
    pub fn pendch17(&self) -> Pendch17R {
        Pendch17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pending Channel 18"]
    #[inline(always)]
    pub fn pendch18(&self) -> Pendch18R {
        Pendch18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pending Channel 19"]
    #[inline(always)]
    pub fn pendch19(&self) -> Pendch19R {
        Pendch19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pending Channel 20"]
    #[inline(always)]
    pub fn pendch20(&self) -> Pendch20R {
        Pendch20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pending Channel 21"]
    #[inline(always)]
    pub fn pendch21(&self) -> Pendch21R {
        Pendch21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pending Channel 22"]
    #[inline(always)]
    pub fn pendch22(&self) -> Pendch22R {
        Pendch22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Pending Channel 23"]
    #[inline(always)]
    pub fn pendch23(&self) -> Pendch23R {
        Pendch23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Pending Channel 24"]
    #[inline(always)]
    pub fn pendch24(&self) -> Pendch24R {
        Pendch24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pending Channel 25"]
    #[inline(always)]
    pub fn pendch25(&self) -> Pendch25R {
        Pendch25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pending Channel 26"]
    #[inline(always)]
    pub fn pendch26(&self) -> Pendch26R {
        Pendch26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Pending Channel 27"]
    #[inline(always)]
    pub fn pendch27(&self) -> Pendch27R {
        Pendch27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pending Channel 28"]
    #[inline(always)]
    pub fn pendch28(&self) -> Pendch28R {
        Pendch28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Pending Channel 29"]
    #[inline(always)]
    pub fn pendch29(&self) -> Pendch29R {
        Pendch29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Pending Channel 30"]
    #[inline(always)]
    pub fn pendch30(&self) -> Pendch30R {
        Pendch30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Pending Channel 31"]
    #[inline(always)]
    pub fn pendch31(&self) -> Pendch31R {
        Pendch31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Pending Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`pendch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PendchSpec;
impl crate::RegisterSpec for PendchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pendch::R`](R) reader structure"]
impl crate::Readable for PendchSpec {}
#[doc = "`reset()` method sets PENDCH to value 0"]
impl crate::Resettable for PendchSpec {}
