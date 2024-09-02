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
}
#[doc = "Channel Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatusSpec;
impl crate::RegisterSpec for ChstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for ChstatusSpec {}
#[doc = "`reset()` method sets CHSTATUS to value 0x3f"]
impl crate::Resettable for ChstatusSpec {
    const RESET_VALUE: u32 = 0x3f;
}
