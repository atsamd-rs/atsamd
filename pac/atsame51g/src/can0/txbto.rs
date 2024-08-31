#[doc = "Register `TXBTO` reader"]
pub type R = crate::R<TxbtoSpec>;
#[doc = "Field `TO0` reader - Transmission Occurred 0"]
pub type To0R = crate::BitReader;
#[doc = "Field `TO1` reader - Transmission Occurred 1"]
pub type To1R = crate::BitReader;
#[doc = "Field `TO2` reader - Transmission Occurred 2"]
pub type To2R = crate::BitReader;
#[doc = "Field `TO3` reader - Transmission Occurred 3"]
pub type To3R = crate::BitReader;
#[doc = "Field `TO4` reader - Transmission Occurred 4"]
pub type To4R = crate::BitReader;
#[doc = "Field `TO5` reader - Transmission Occurred 5"]
pub type To5R = crate::BitReader;
#[doc = "Field `TO6` reader - Transmission Occurred 6"]
pub type To6R = crate::BitReader;
#[doc = "Field `TO7` reader - Transmission Occurred 7"]
pub type To7R = crate::BitReader;
#[doc = "Field `TO8` reader - Transmission Occurred 8"]
pub type To8R = crate::BitReader;
#[doc = "Field `TO9` reader - Transmission Occurred 9"]
pub type To9R = crate::BitReader;
#[doc = "Field `TO10` reader - Transmission Occurred 10"]
pub type To10R = crate::BitReader;
#[doc = "Field `TO11` reader - Transmission Occurred 11"]
pub type To11R = crate::BitReader;
#[doc = "Field `TO12` reader - Transmission Occurred 12"]
pub type To12R = crate::BitReader;
#[doc = "Field `TO13` reader - Transmission Occurred 13"]
pub type To13R = crate::BitReader;
#[doc = "Field `TO14` reader - Transmission Occurred 14"]
pub type To14R = crate::BitReader;
#[doc = "Field `TO15` reader - Transmission Occurred 15"]
pub type To15R = crate::BitReader;
#[doc = "Field `TO16` reader - Transmission Occurred 16"]
pub type To16R = crate::BitReader;
#[doc = "Field `TO17` reader - Transmission Occurred 17"]
pub type To17R = crate::BitReader;
#[doc = "Field `TO18` reader - Transmission Occurred 18"]
pub type To18R = crate::BitReader;
#[doc = "Field `TO19` reader - Transmission Occurred 19"]
pub type To19R = crate::BitReader;
#[doc = "Field `TO20` reader - Transmission Occurred 20"]
pub type To20R = crate::BitReader;
#[doc = "Field `TO21` reader - Transmission Occurred 21"]
pub type To21R = crate::BitReader;
#[doc = "Field `TO22` reader - Transmission Occurred 22"]
pub type To22R = crate::BitReader;
#[doc = "Field `TO23` reader - Transmission Occurred 23"]
pub type To23R = crate::BitReader;
#[doc = "Field `TO24` reader - Transmission Occurred 24"]
pub type To24R = crate::BitReader;
#[doc = "Field `TO25` reader - Transmission Occurred 25"]
pub type To25R = crate::BitReader;
#[doc = "Field `TO26` reader - Transmission Occurred 26"]
pub type To26R = crate::BitReader;
#[doc = "Field `TO27` reader - Transmission Occurred 27"]
pub type To27R = crate::BitReader;
#[doc = "Field `TO28` reader - Transmission Occurred 28"]
pub type To28R = crate::BitReader;
#[doc = "Field `TO29` reader - Transmission Occurred 29"]
pub type To29R = crate::BitReader;
#[doc = "Field `TO30` reader - Transmission Occurred 30"]
pub type To30R = crate::BitReader;
#[doc = "Field `TO31` reader - Transmission Occurred 31"]
pub type To31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Occurred 0"]
    #[inline(always)]
    pub fn to0(&self) -> To0R {
        To0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission Occurred 1"]
    #[inline(always)]
    pub fn to1(&self) -> To1R {
        To1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Occurred 2"]
    #[inline(always)]
    pub fn to2(&self) -> To2R {
        To2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Occurred 3"]
    #[inline(always)]
    pub fn to3(&self) -> To3R {
        To3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Occurred 4"]
    #[inline(always)]
    pub fn to4(&self) -> To4R {
        To4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission Occurred 5"]
    #[inline(always)]
    pub fn to5(&self) -> To5R {
        To5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Occurred 6"]
    #[inline(always)]
    pub fn to6(&self) -> To6R {
        To6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission Occurred 7"]
    #[inline(always)]
    pub fn to7(&self) -> To7R {
        To7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission Occurred 8"]
    #[inline(always)]
    pub fn to8(&self) -> To8R {
        To8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Occurred 9"]
    #[inline(always)]
    pub fn to9(&self) -> To9R {
        To9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Occurred 10"]
    #[inline(always)]
    pub fn to10(&self) -> To10R {
        To10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Occurred 11"]
    #[inline(always)]
    pub fn to11(&self) -> To11R {
        To11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmission Occurred 12"]
    #[inline(always)]
    pub fn to12(&self) -> To12R {
        To12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmission Occurred 13"]
    #[inline(always)]
    pub fn to13(&self) -> To13R {
        To13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmission Occurred 14"]
    #[inline(always)]
    pub fn to14(&self) -> To14R {
        To14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmission Occurred 15"]
    #[inline(always)]
    pub fn to15(&self) -> To15R {
        To15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmission Occurred 16"]
    #[inline(always)]
    pub fn to16(&self) -> To16R {
        To16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmission Occurred 17"]
    #[inline(always)]
    pub fn to17(&self) -> To17R {
        To17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmission Occurred 18"]
    #[inline(always)]
    pub fn to18(&self) -> To18R {
        To18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmission Occurred 19"]
    #[inline(always)]
    pub fn to19(&self) -> To19R {
        To19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmission Occurred 20"]
    #[inline(always)]
    pub fn to20(&self) -> To20R {
        To20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmission Occurred 21"]
    #[inline(always)]
    pub fn to21(&self) -> To21R {
        To21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Occurred 22"]
    #[inline(always)]
    pub fn to22(&self) -> To22R {
        To22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmission Occurred 23"]
    #[inline(always)]
    pub fn to23(&self) -> To23R {
        To23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmission Occurred 24"]
    #[inline(always)]
    pub fn to24(&self) -> To24R {
        To24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmission Occurred 25"]
    #[inline(always)]
    pub fn to25(&self) -> To25R {
        To25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Occurred 26"]
    #[inline(always)]
    pub fn to26(&self) -> To26R {
        To26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmission Occurred 27"]
    #[inline(always)]
    pub fn to27(&self) -> To27R {
        To27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmission Occurred 28"]
    #[inline(always)]
    pub fn to28(&self) -> To28R {
        To28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmission Occurred 29"]
    #[inline(always)]
    pub fn to29(&self) -> To29R {
        To29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmission Occurred 30"]
    #[inline(always)]
    pub fn to30(&self) -> To30R {
        To30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmission Occurred 31"]
    #[inline(always)]
    pub fn to31(&self) -> To31R {
        To31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Tx Buffer Transmission Occurred\n\nYou can [`read`](crate::Reg::read) this register and get [`txbto::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbtoSpec;
impl crate::RegisterSpec for TxbtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbto::R`](R) reader structure"]
impl crate::Readable for TxbtoSpec {}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TxbtoSpec {
    const RESET_VALUE: u32 = 0;
}
