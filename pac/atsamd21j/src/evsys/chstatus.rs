#[doc = "Register `CHSTATUS` reader"]
pub type R = crate::R<ChstatusSpec>;
#[doc = "Field `USRRDY0` reader - Channel 0 User Ready"]
pub type Usrrdy0R = crate::BitReader;
#[doc = "Field `USRRDY1` reader - Channel 1 User Ready"]
pub type Usrrdy1R = crate::BitReader;
#[doc = "Field `USRRDY2` reader - Channel 2 User Ready"]
pub type Usrrdy2R = crate::BitReader;
#[doc = "Field `USRRDY3` reader - Channel 3 User Ready"]
pub type Usrrdy3R = crate::BitReader;
#[doc = "Field `USRRDY4` reader - Channel 4 User Ready"]
pub type Usrrdy4R = crate::BitReader;
#[doc = "Field `USRRDY5` reader - Channel 5 User Ready"]
pub type Usrrdy5R = crate::BitReader;
#[doc = "Field `USRRDY6` reader - Channel 6 User Ready"]
pub type Usrrdy6R = crate::BitReader;
#[doc = "Field `USRRDY7` reader - Channel 7 User Ready"]
pub type Usrrdy7R = crate::BitReader;
#[doc = "Field `CHBUSY0` reader - Channel 0 Busy"]
pub type Chbusy0R = crate::BitReader;
#[doc = "Field `CHBUSY1` reader - Channel 1 Busy"]
pub type Chbusy1R = crate::BitReader;
#[doc = "Field `CHBUSY2` reader - Channel 2 Busy"]
pub type Chbusy2R = crate::BitReader;
#[doc = "Field `CHBUSY3` reader - Channel 3 Busy"]
pub type Chbusy3R = crate::BitReader;
#[doc = "Field `CHBUSY4` reader - Channel 4 Busy"]
pub type Chbusy4R = crate::BitReader;
#[doc = "Field `CHBUSY5` reader - Channel 5 Busy"]
pub type Chbusy5R = crate::BitReader;
#[doc = "Field `CHBUSY6` reader - Channel 6 Busy"]
pub type Chbusy6R = crate::BitReader;
#[doc = "Field `CHBUSY7` reader - Channel 7 Busy"]
pub type Chbusy7R = crate::BitReader;
#[doc = "Field `USRRDY8` reader - Channel 8 User Ready"]
pub type Usrrdy8R = crate::BitReader;
#[doc = "Field `USRRDY9` reader - Channel 9 User Ready"]
pub type Usrrdy9R = crate::BitReader;
#[doc = "Field `USRRDY10` reader - Channel 10 User Ready"]
pub type Usrrdy10R = crate::BitReader;
#[doc = "Field `USRRDY11` reader - Channel 11 User Ready"]
pub type Usrrdy11R = crate::BitReader;
#[doc = "Field `CHBUSY8` reader - Channel 8 Busy"]
pub type Chbusy8R = crate::BitReader;
#[doc = "Field `CHBUSY9` reader - Channel 9 Busy"]
pub type Chbusy9R = crate::BitReader;
#[doc = "Field `CHBUSY10` reader - Channel 10 Busy"]
pub type Chbusy10R = crate::BitReader;
#[doc = "Field `CHBUSY11` reader - Channel 11 Busy"]
pub type Chbusy11R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 User Ready"]
    #[inline(always)]
    pub fn usrrdy0(&self) -> Usrrdy0R {
        Usrrdy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 User Ready"]
    #[inline(always)]
    pub fn usrrdy1(&self) -> Usrrdy1R {
        Usrrdy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 User Ready"]
    #[inline(always)]
    pub fn usrrdy2(&self) -> Usrrdy2R {
        Usrrdy2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 User Ready"]
    #[inline(always)]
    pub fn usrrdy3(&self) -> Usrrdy3R {
        Usrrdy3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 User Ready"]
    #[inline(always)]
    pub fn usrrdy4(&self) -> Usrrdy4R {
        Usrrdy4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 User Ready"]
    #[inline(always)]
    pub fn usrrdy5(&self) -> Usrrdy5R {
        Usrrdy5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 User Ready"]
    #[inline(always)]
    pub fn usrrdy6(&self) -> Usrrdy6R {
        Usrrdy6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 User Ready"]
    #[inline(always)]
    pub fn usrrdy7(&self) -> Usrrdy7R {
        Usrrdy7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Busy"]
    #[inline(always)]
    pub fn chbusy0(&self) -> Chbusy0R {
        Chbusy0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Busy"]
    #[inline(always)]
    pub fn chbusy1(&self) -> Chbusy1R {
        Chbusy1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Busy"]
    #[inline(always)]
    pub fn chbusy2(&self) -> Chbusy2R {
        Chbusy2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Busy"]
    #[inline(always)]
    pub fn chbusy3(&self) -> Chbusy3R {
        Chbusy3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Busy"]
    #[inline(always)]
    pub fn chbusy4(&self) -> Chbusy4R {
        Chbusy4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Busy"]
    #[inline(always)]
    pub fn chbusy5(&self) -> Chbusy5R {
        Chbusy5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Busy"]
    #[inline(always)]
    pub fn chbusy6(&self) -> Chbusy6R {
        Chbusy6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Busy"]
    #[inline(always)]
    pub fn chbusy7(&self) -> Chbusy7R {
        Chbusy7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 8 User Ready"]
    #[inline(always)]
    pub fn usrrdy8(&self) -> Usrrdy8R {
        Usrrdy8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 9 User Ready"]
    #[inline(always)]
    pub fn usrrdy9(&self) -> Usrrdy9R {
        Usrrdy9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 10 User Ready"]
    #[inline(always)]
    pub fn usrrdy10(&self) -> Usrrdy10R {
        Usrrdy10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 11 User Ready"]
    #[inline(always)]
    pub fn usrrdy11(&self) -> Usrrdy11R {
        Usrrdy11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 8 Busy"]
    #[inline(always)]
    pub fn chbusy8(&self) -> Chbusy8R {
        Chbusy8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 9 Busy"]
    #[inline(always)]
    pub fn chbusy9(&self) -> Chbusy9R {
        Chbusy9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 10 Busy"]
    #[inline(always)]
    pub fn chbusy10(&self) -> Chbusy10R {
        Chbusy10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 11 Busy"]
    #[inline(always)]
    pub fn chbusy11(&self) -> Chbusy11R {
        Chbusy11R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Channel Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatusSpec;
impl crate::RegisterSpec for ChstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for ChstatusSpec {}
#[doc = "`reset()` method sets CHSTATUS to value 0x000f_00ff"]
impl crate::Resettable for ChstatusSpec {
    const RESET_VALUE: u32 = 0x000f_00ff;
}
