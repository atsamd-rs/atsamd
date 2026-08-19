#[doc = "Register `BUSYCH` reader"]
pub type R = crate::R<BusychSpec>;
#[doc = "Field `BUSYCH0` reader - Busy Channel 0"]
pub type Busych0R = crate::BitReader;
#[doc = "Field `BUSYCH1` reader - Busy Channel 1"]
pub type Busych1R = crate::BitReader;
#[doc = "Field `BUSYCH2` reader - Busy Channel 2"]
pub type Busych2R = crate::BitReader;
#[doc = "Field `BUSYCH3` reader - Busy Channel 3"]
pub type Busych3R = crate::BitReader;
#[doc = "Field `BUSYCH4` reader - Busy Channel 4"]
pub type Busych4R = crate::BitReader;
#[doc = "Field `BUSYCH5` reader - Busy Channel 5"]
pub type Busych5R = crate::BitReader;
#[doc = "Field `BUSYCH6` reader - Busy Channel 6"]
pub type Busych6R = crate::BitReader;
#[doc = "Field `BUSYCH7` reader - Busy Channel 7"]
pub type Busych7R = crate::BitReader;
#[doc = "Field `BUSYCH8` reader - Busy Channel 8"]
pub type Busych8R = crate::BitReader;
#[doc = "Field `BUSYCH9` reader - Busy Channel 9"]
pub type Busych9R = crate::BitReader;
#[doc = "Field `BUSYCH10` reader - Busy Channel 10"]
pub type Busych10R = crate::BitReader;
#[doc = "Field `BUSYCH11` reader - Busy Channel 11"]
pub type Busych11R = crate::BitReader;
#[doc = "Field `BUSYCH12` reader - Busy Channel 12"]
pub type Busych12R = crate::BitReader;
#[doc = "Field `BUSYCH13` reader - Busy Channel 13"]
pub type Busych13R = crate::BitReader;
#[doc = "Field `BUSYCH14` reader - Busy Channel 14"]
pub type Busych14R = crate::BitReader;
#[doc = "Field `BUSYCH15` reader - Busy Channel 15"]
pub type Busych15R = crate::BitReader;
#[doc = "Field `BUSYCH16` reader - Busy Channel 16"]
pub type Busych16R = crate::BitReader;
#[doc = "Field `BUSYCH17` reader - Busy Channel 17"]
pub type Busych17R = crate::BitReader;
#[doc = "Field `BUSYCH18` reader - Busy Channel 18"]
pub type Busych18R = crate::BitReader;
#[doc = "Field `BUSYCH19` reader - Busy Channel 19"]
pub type Busych19R = crate::BitReader;
#[doc = "Field `BUSYCH20` reader - Busy Channel 20"]
pub type Busych20R = crate::BitReader;
#[doc = "Field `BUSYCH21` reader - Busy Channel 21"]
pub type Busych21R = crate::BitReader;
#[doc = "Field `BUSYCH22` reader - Busy Channel 22"]
pub type Busych22R = crate::BitReader;
#[doc = "Field `BUSYCH23` reader - Busy Channel 23"]
pub type Busych23R = crate::BitReader;
#[doc = "Field `BUSYCH24` reader - Busy Channel 24"]
pub type Busych24R = crate::BitReader;
#[doc = "Field `BUSYCH25` reader - Busy Channel 25"]
pub type Busych25R = crate::BitReader;
#[doc = "Field `BUSYCH26` reader - Busy Channel 26"]
pub type Busych26R = crate::BitReader;
#[doc = "Field `BUSYCH27` reader - Busy Channel 27"]
pub type Busych27R = crate::BitReader;
#[doc = "Field `BUSYCH28` reader - Busy Channel 28"]
pub type Busych28R = crate::BitReader;
#[doc = "Field `BUSYCH29` reader - Busy Channel 29"]
pub type Busych29R = crate::BitReader;
#[doc = "Field `BUSYCH30` reader - Busy Channel 30"]
pub type Busych30R = crate::BitReader;
#[doc = "Field `BUSYCH31` reader - Busy Channel 31"]
pub type Busych31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy Channel 0"]
    #[inline(always)]
    pub fn busych0(&self) -> Busych0R {
        Busych0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy Channel 1"]
    #[inline(always)]
    pub fn busych1(&self) -> Busych1R {
        Busych1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy Channel 2"]
    #[inline(always)]
    pub fn busych2(&self) -> Busych2R {
        Busych2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy Channel 3"]
    #[inline(always)]
    pub fn busych3(&self) -> Busych3R {
        Busych3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy Channel 4"]
    #[inline(always)]
    pub fn busych4(&self) -> Busych4R {
        Busych4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Busy Channel 5"]
    #[inline(always)]
    pub fn busych5(&self) -> Busych5R {
        Busych5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Busy Channel 6"]
    #[inline(always)]
    pub fn busych6(&self) -> Busych6R {
        Busych6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy Channel 7"]
    #[inline(always)]
    pub fn busych7(&self) -> Busych7R {
        Busych7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Busy Channel 8"]
    #[inline(always)]
    pub fn busych8(&self) -> Busych8R {
        Busych8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Busy Channel 9"]
    #[inline(always)]
    pub fn busych9(&self) -> Busych9R {
        Busych9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Busy Channel 10"]
    #[inline(always)]
    pub fn busych10(&self) -> Busych10R {
        Busych10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Busy Channel 11"]
    #[inline(always)]
    pub fn busych11(&self) -> Busych11R {
        Busych11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Busy Channel 12"]
    #[inline(always)]
    pub fn busych12(&self) -> Busych12R {
        Busych12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Busy Channel 13"]
    #[inline(always)]
    pub fn busych13(&self) -> Busych13R {
        Busych13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Busy Channel 14"]
    #[inline(always)]
    pub fn busych14(&self) -> Busych14R {
        Busych14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy Channel 15"]
    #[inline(always)]
    pub fn busych15(&self) -> Busych15R {
        Busych15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy Channel 16"]
    #[inline(always)]
    pub fn busych16(&self) -> Busych16R {
        Busych16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Busy Channel 17"]
    #[inline(always)]
    pub fn busych17(&self) -> Busych17R {
        Busych17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Busy Channel 18"]
    #[inline(always)]
    pub fn busych18(&self) -> Busych18R {
        Busych18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Busy Channel 19"]
    #[inline(always)]
    pub fn busych19(&self) -> Busych19R {
        Busych19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Busy Channel 20"]
    #[inline(always)]
    pub fn busych20(&self) -> Busych20R {
        Busych20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Busy Channel 21"]
    #[inline(always)]
    pub fn busych21(&self) -> Busych21R {
        Busych21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Busy Channel 22"]
    #[inline(always)]
    pub fn busych22(&self) -> Busych22R {
        Busych22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Busy Channel 23"]
    #[inline(always)]
    pub fn busych23(&self) -> Busych23R {
        Busych23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Busy Channel 24"]
    #[inline(always)]
    pub fn busych24(&self) -> Busych24R {
        Busych24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Busy Channel 25"]
    #[inline(always)]
    pub fn busych25(&self) -> Busych25R {
        Busych25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Busy Channel 26"]
    #[inline(always)]
    pub fn busych26(&self) -> Busych26R {
        Busych26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Busy Channel 27"]
    #[inline(always)]
    pub fn busych27(&self) -> Busych27R {
        Busych27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Busy Channel 28"]
    #[inline(always)]
    pub fn busych28(&self) -> Busych28R {
        Busych28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Busy Channel 29"]
    #[inline(always)]
    pub fn busych29(&self) -> Busych29R {
        Busych29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Busy Channel 30"]
    #[inline(always)]
    pub fn busych30(&self) -> Busych30R {
        Busych30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Busy Channel 31"]
    #[inline(always)]
    pub fn busych31(&self) -> Busych31R {
        Busych31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Busy Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`busych::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusychSpec;
impl crate::RegisterSpec for BusychSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busych::R`](R) reader structure"]
impl crate::Readable for BusychSpec {}
#[doc = "`reset()` method sets BUSYCH to value 0"]
impl crate::Resettable for BusychSpec {
    const RESET_VALUE: u32 = 0;
}
