#[doc = "Register `TXBRP` reader"]
pub type R = crate::R<TxbrpSpec>;
#[doc = "Field `TRP0` reader - Transmission Request Pending 0"]
pub type Trp0R = crate::BitReader;
#[doc = "Field `TRP1` reader - Transmission Request Pending 1"]
pub type Trp1R = crate::BitReader;
#[doc = "Field `TRP2` reader - Transmission Request Pending 2"]
pub type Trp2R = crate::BitReader;
#[doc = "Field `TRP3` reader - Transmission Request Pending 3"]
pub type Trp3R = crate::BitReader;
#[doc = "Field `TRP4` reader - Transmission Request Pending 4"]
pub type Trp4R = crate::BitReader;
#[doc = "Field `TRP5` reader - Transmission Request Pending 5"]
pub type Trp5R = crate::BitReader;
#[doc = "Field `TRP6` reader - Transmission Request Pending 6"]
pub type Trp6R = crate::BitReader;
#[doc = "Field `TRP7` reader - Transmission Request Pending 7"]
pub type Trp7R = crate::BitReader;
#[doc = "Field `TRP8` reader - Transmission Request Pending 8"]
pub type Trp8R = crate::BitReader;
#[doc = "Field `TRP9` reader - Transmission Request Pending 9"]
pub type Trp9R = crate::BitReader;
#[doc = "Field `TRP10` reader - Transmission Request Pending 10"]
pub type Trp10R = crate::BitReader;
#[doc = "Field `TRP11` reader - Transmission Request Pending 11"]
pub type Trp11R = crate::BitReader;
#[doc = "Field `TRP12` reader - Transmission Request Pending 12"]
pub type Trp12R = crate::BitReader;
#[doc = "Field `TRP13` reader - Transmission Request Pending 13"]
pub type Trp13R = crate::BitReader;
#[doc = "Field `TRP14` reader - Transmission Request Pending 14"]
pub type Trp14R = crate::BitReader;
#[doc = "Field `TRP15` reader - Transmission Request Pending 15"]
pub type Trp15R = crate::BitReader;
#[doc = "Field `TRP16` reader - Transmission Request Pending 16"]
pub type Trp16R = crate::BitReader;
#[doc = "Field `TRP17` reader - Transmission Request Pending 17"]
pub type Trp17R = crate::BitReader;
#[doc = "Field `TRP18` reader - Transmission Request Pending 18"]
pub type Trp18R = crate::BitReader;
#[doc = "Field `TRP19` reader - Transmission Request Pending 19"]
pub type Trp19R = crate::BitReader;
#[doc = "Field `TRP20` reader - Transmission Request Pending 20"]
pub type Trp20R = crate::BitReader;
#[doc = "Field `TRP21` reader - Transmission Request Pending 21"]
pub type Trp21R = crate::BitReader;
#[doc = "Field `TRP22` reader - Transmission Request Pending 22"]
pub type Trp22R = crate::BitReader;
#[doc = "Field `TRP23` reader - Transmission Request Pending 23"]
pub type Trp23R = crate::BitReader;
#[doc = "Field `TRP24` reader - Transmission Request Pending 24"]
pub type Trp24R = crate::BitReader;
#[doc = "Field `TRP25` reader - Transmission Request Pending 25"]
pub type Trp25R = crate::BitReader;
#[doc = "Field `TRP26` reader - Transmission Request Pending 26"]
pub type Trp26R = crate::BitReader;
#[doc = "Field `TRP27` reader - Transmission Request Pending 27"]
pub type Trp27R = crate::BitReader;
#[doc = "Field `TRP28` reader - Transmission Request Pending 28"]
pub type Trp28R = crate::BitReader;
#[doc = "Field `TRP29` reader - Transmission Request Pending 29"]
pub type Trp29R = crate::BitReader;
#[doc = "Field `TRP30` reader - Transmission Request Pending 30"]
pub type Trp30R = crate::BitReader;
#[doc = "Field `TRP31` reader - Transmission Request Pending 31"]
pub type Trp31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Request Pending 0"]
    #[inline(always)]
    pub fn trp0(&self) -> Trp0R {
        Trp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission Request Pending 1"]
    #[inline(always)]
    pub fn trp1(&self) -> Trp1R {
        Trp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Request Pending 2"]
    #[inline(always)]
    pub fn trp2(&self) -> Trp2R {
        Trp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Request Pending 3"]
    #[inline(always)]
    pub fn trp3(&self) -> Trp3R {
        Trp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Request Pending 4"]
    #[inline(always)]
    pub fn trp4(&self) -> Trp4R {
        Trp4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission Request Pending 5"]
    #[inline(always)]
    pub fn trp5(&self) -> Trp5R {
        Trp5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Request Pending 6"]
    #[inline(always)]
    pub fn trp6(&self) -> Trp6R {
        Trp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission Request Pending 7"]
    #[inline(always)]
    pub fn trp7(&self) -> Trp7R {
        Trp7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission Request Pending 8"]
    #[inline(always)]
    pub fn trp8(&self) -> Trp8R {
        Trp8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Request Pending 9"]
    #[inline(always)]
    pub fn trp9(&self) -> Trp9R {
        Trp9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Request Pending 10"]
    #[inline(always)]
    pub fn trp10(&self) -> Trp10R {
        Trp10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Request Pending 11"]
    #[inline(always)]
    pub fn trp11(&self) -> Trp11R {
        Trp11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmission Request Pending 12"]
    #[inline(always)]
    pub fn trp12(&self) -> Trp12R {
        Trp12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmission Request Pending 13"]
    #[inline(always)]
    pub fn trp13(&self) -> Trp13R {
        Trp13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmission Request Pending 14"]
    #[inline(always)]
    pub fn trp14(&self) -> Trp14R {
        Trp14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmission Request Pending 15"]
    #[inline(always)]
    pub fn trp15(&self) -> Trp15R {
        Trp15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmission Request Pending 16"]
    #[inline(always)]
    pub fn trp16(&self) -> Trp16R {
        Trp16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmission Request Pending 17"]
    #[inline(always)]
    pub fn trp17(&self) -> Trp17R {
        Trp17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmission Request Pending 18"]
    #[inline(always)]
    pub fn trp18(&self) -> Trp18R {
        Trp18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmission Request Pending 19"]
    #[inline(always)]
    pub fn trp19(&self) -> Trp19R {
        Trp19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmission Request Pending 20"]
    #[inline(always)]
    pub fn trp20(&self) -> Trp20R {
        Trp20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmission Request Pending 21"]
    #[inline(always)]
    pub fn trp21(&self) -> Trp21R {
        Trp21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Request Pending 22"]
    #[inline(always)]
    pub fn trp22(&self) -> Trp22R {
        Trp22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmission Request Pending 23"]
    #[inline(always)]
    pub fn trp23(&self) -> Trp23R {
        Trp23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmission Request Pending 24"]
    #[inline(always)]
    pub fn trp24(&self) -> Trp24R {
        Trp24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmission Request Pending 25"]
    #[inline(always)]
    pub fn trp25(&self) -> Trp25R {
        Trp25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Request Pending 26"]
    #[inline(always)]
    pub fn trp26(&self) -> Trp26R {
        Trp26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmission Request Pending 27"]
    #[inline(always)]
    pub fn trp27(&self) -> Trp27R {
        Trp27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmission Request Pending 28"]
    #[inline(always)]
    pub fn trp28(&self) -> Trp28R {
        Trp28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmission Request Pending 29"]
    #[inline(always)]
    pub fn trp29(&self) -> Trp29R {
        Trp29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmission Request Pending 30"]
    #[inline(always)]
    pub fn trp30(&self) -> Trp30R {
        Trp30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmission Request Pending 31"]
    #[inline(always)]
    pub fn trp31(&self) -> Trp31R {
        Trp31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Tx Buffer Request Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbrpSpec;
impl crate::RegisterSpec for TxbrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbrp::R`](R) reader structure"]
impl crate::Readable for TxbrpSpec {}
#[doc = "`reset()` method sets TXBRP to value 0"]
impl crate::Resettable for TxbrpSpec {
    const RESET_VALUE: u32 = 0;
}
