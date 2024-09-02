#[doc = "Register `INTSTATUS` reader"]
pub type R = crate::R<IntstatusSpec>;
#[doc = "Field `CHINT0` reader - Channel 0 Pending Interrupt"]
pub type Chint0R = crate::BitReader;
#[doc = "Field `CHINT1` reader - Channel 1 Pending Interrupt"]
pub type Chint1R = crate::BitReader;
#[doc = "Field `CHINT2` reader - Channel 2 Pending Interrupt"]
pub type Chint2R = crate::BitReader;
#[doc = "Field `CHINT3` reader - Channel 3 Pending Interrupt"]
pub type Chint3R = crate::BitReader;
#[doc = "Field `CHINT4` reader - Channel 4 Pending Interrupt"]
pub type Chint4R = crate::BitReader;
#[doc = "Field `CHINT5` reader - Channel 5 Pending Interrupt"]
pub type Chint5R = crate::BitReader;
#[doc = "Field `CHINT6` reader - Channel 6 Pending Interrupt"]
pub type Chint6R = crate::BitReader;
#[doc = "Field `CHINT7` reader - Channel 7 Pending Interrupt"]
pub type Chint7R = crate::BitReader;
#[doc = "Field `CHINT8` reader - Channel 8 Pending Interrupt"]
pub type Chint8R = crate::BitReader;
#[doc = "Field `CHINT9` reader - Channel 9 Pending Interrupt"]
pub type Chint9R = crate::BitReader;
#[doc = "Field `CHINT10` reader - Channel 10 Pending Interrupt"]
pub type Chint10R = crate::BitReader;
#[doc = "Field `CHINT11` reader - Channel 11 Pending Interrupt"]
pub type Chint11R = crate::BitReader;
#[doc = "Field `CHINT12` reader - Channel 12 Pending Interrupt"]
pub type Chint12R = crate::BitReader;
#[doc = "Field `CHINT13` reader - Channel 13 Pending Interrupt"]
pub type Chint13R = crate::BitReader;
#[doc = "Field `CHINT14` reader - Channel 14 Pending Interrupt"]
pub type Chint14R = crate::BitReader;
#[doc = "Field `CHINT15` reader - Channel 15 Pending Interrupt"]
pub type Chint15R = crate::BitReader;
#[doc = "Field `CHINT16` reader - Channel 16 Pending Interrupt"]
pub type Chint16R = crate::BitReader;
#[doc = "Field `CHINT17` reader - Channel 17 Pending Interrupt"]
pub type Chint17R = crate::BitReader;
#[doc = "Field `CHINT18` reader - Channel 18 Pending Interrupt"]
pub type Chint18R = crate::BitReader;
#[doc = "Field `CHINT19` reader - Channel 19 Pending Interrupt"]
pub type Chint19R = crate::BitReader;
#[doc = "Field `CHINT20` reader - Channel 20 Pending Interrupt"]
pub type Chint20R = crate::BitReader;
#[doc = "Field `CHINT21` reader - Channel 21 Pending Interrupt"]
pub type Chint21R = crate::BitReader;
#[doc = "Field `CHINT22` reader - Channel 22 Pending Interrupt"]
pub type Chint22R = crate::BitReader;
#[doc = "Field `CHINT23` reader - Channel 23 Pending Interrupt"]
pub type Chint23R = crate::BitReader;
#[doc = "Field `CHINT24` reader - Channel 24 Pending Interrupt"]
pub type Chint24R = crate::BitReader;
#[doc = "Field `CHINT25` reader - Channel 25 Pending Interrupt"]
pub type Chint25R = crate::BitReader;
#[doc = "Field `CHINT26` reader - Channel 26 Pending Interrupt"]
pub type Chint26R = crate::BitReader;
#[doc = "Field `CHINT27` reader - Channel 27 Pending Interrupt"]
pub type Chint27R = crate::BitReader;
#[doc = "Field `CHINT28` reader - Channel 28 Pending Interrupt"]
pub type Chint28R = crate::BitReader;
#[doc = "Field `CHINT29` reader - Channel 29 Pending Interrupt"]
pub type Chint29R = crate::BitReader;
#[doc = "Field `CHINT30` reader - Channel 30 Pending Interrupt"]
pub type Chint30R = crate::BitReader;
#[doc = "Field `CHINT31` reader - Channel 31 Pending Interrupt"]
pub type Chint31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Pending Interrupt"]
    #[inline(always)]
    pub fn chint0(&self) -> Chint0R {
        Chint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Pending Interrupt"]
    #[inline(always)]
    pub fn chint1(&self) -> Chint1R {
        Chint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Pending Interrupt"]
    #[inline(always)]
    pub fn chint2(&self) -> Chint2R {
        Chint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Pending Interrupt"]
    #[inline(always)]
    pub fn chint3(&self) -> Chint3R {
        Chint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Pending Interrupt"]
    #[inline(always)]
    pub fn chint4(&self) -> Chint4R {
        Chint4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Pending Interrupt"]
    #[inline(always)]
    pub fn chint5(&self) -> Chint5R {
        Chint5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Pending Interrupt"]
    #[inline(always)]
    pub fn chint6(&self) -> Chint6R {
        Chint6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Pending Interrupt"]
    #[inline(always)]
    pub fn chint7(&self) -> Chint7R {
        Chint7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Pending Interrupt"]
    #[inline(always)]
    pub fn chint8(&self) -> Chint8R {
        Chint8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Pending Interrupt"]
    #[inline(always)]
    pub fn chint9(&self) -> Chint9R {
        Chint9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Pending Interrupt"]
    #[inline(always)]
    pub fn chint10(&self) -> Chint10R {
        Chint10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Pending Interrupt"]
    #[inline(always)]
    pub fn chint11(&self) -> Chint11R {
        Chint11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Pending Interrupt"]
    #[inline(always)]
    pub fn chint12(&self) -> Chint12R {
        Chint12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Pending Interrupt"]
    #[inline(always)]
    pub fn chint13(&self) -> Chint13R {
        Chint13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Pending Interrupt"]
    #[inline(always)]
    pub fn chint14(&self) -> Chint14R {
        Chint14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Pending Interrupt"]
    #[inline(always)]
    pub fn chint15(&self) -> Chint15R {
        Chint15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 16 Pending Interrupt"]
    #[inline(always)]
    pub fn chint16(&self) -> Chint16R {
        Chint16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 17 Pending Interrupt"]
    #[inline(always)]
    pub fn chint17(&self) -> Chint17R {
        Chint17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 18 Pending Interrupt"]
    #[inline(always)]
    pub fn chint18(&self) -> Chint18R {
        Chint18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 19 Pending Interrupt"]
    #[inline(always)]
    pub fn chint19(&self) -> Chint19R {
        Chint19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 20 Pending Interrupt"]
    #[inline(always)]
    pub fn chint20(&self) -> Chint20R {
        Chint20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 21 Pending Interrupt"]
    #[inline(always)]
    pub fn chint21(&self) -> Chint21R {
        Chint21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 22 Pending Interrupt"]
    #[inline(always)]
    pub fn chint22(&self) -> Chint22R {
        Chint22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 23 Pending Interrupt"]
    #[inline(always)]
    pub fn chint23(&self) -> Chint23R {
        Chint23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 24 Pending Interrupt"]
    #[inline(always)]
    pub fn chint24(&self) -> Chint24R {
        Chint24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 25 Pending Interrupt"]
    #[inline(always)]
    pub fn chint25(&self) -> Chint25R {
        Chint25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 26 Pending Interrupt"]
    #[inline(always)]
    pub fn chint26(&self) -> Chint26R {
        Chint26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 27 Pending Interrupt"]
    #[inline(always)]
    pub fn chint27(&self) -> Chint27R {
        Chint27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Channel 28 Pending Interrupt"]
    #[inline(always)]
    pub fn chint28(&self) -> Chint28R {
        Chint28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel 29 Pending Interrupt"]
    #[inline(always)]
    pub fn chint29(&self) -> Chint29R {
        Chint29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel 30 Pending Interrupt"]
    #[inline(always)]
    pub fn chint30(&self) -> Chint30R {
        Chint30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel 31 Pending Interrupt"]
    #[inline(always)]
    pub fn chint31(&self) -> Chint31R {
        Chint31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatusSpec;
impl crate::RegisterSpec for IntstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatus::R`](R) reader structure"]
impl crate::Readable for IntstatusSpec {}
#[doc = "`reset()` method sets INTSTATUS to value 0"]
impl crate::Resettable for IntstatusSpec {
    const RESET_VALUE: u32 = 0;
}
