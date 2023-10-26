#[doc = "Register `CHSTATUS` reader"]
pub type R = crate::R<CHSTATUS_SPEC>;
#[doc = "Field `USRRDY0` reader - Channel 0 User Ready"]
pub type USRRDY0_R = crate::BitReader;
#[doc = "Field `USRRDY1` reader - Channel 1 User Ready"]
pub type USRRDY1_R = crate::BitReader;
#[doc = "Field `USRRDY2` reader - Channel 2 User Ready"]
pub type USRRDY2_R = crate::BitReader;
#[doc = "Field `USRRDY3` reader - Channel 3 User Ready"]
pub type USRRDY3_R = crate::BitReader;
#[doc = "Field `USRRDY4` reader - Channel 4 User Ready"]
pub type USRRDY4_R = crate::BitReader;
#[doc = "Field `USRRDY5` reader - Channel 5 User Ready"]
pub type USRRDY5_R = crate::BitReader;
#[doc = "Field `CHBUSY0` reader - Channel 0 Busy"]
pub type CHBUSY0_R = crate::BitReader;
#[doc = "Field `CHBUSY1` reader - Channel 1 Busy"]
pub type CHBUSY1_R = crate::BitReader;
#[doc = "Field `CHBUSY2` reader - Channel 2 Busy"]
pub type CHBUSY2_R = crate::BitReader;
#[doc = "Field `CHBUSY3` reader - Channel 3 Busy"]
pub type CHBUSY3_R = crate::BitReader;
#[doc = "Field `CHBUSY4` reader - Channel 4 Busy"]
pub type CHBUSY4_R = crate::BitReader;
#[doc = "Field `CHBUSY5` reader - Channel 5 Busy"]
pub type CHBUSY5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 User Ready"]
    #[inline(always)]
    pub fn usrrdy0(&self) -> USRRDY0_R {
        USRRDY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 User Ready"]
    #[inline(always)]
    pub fn usrrdy1(&self) -> USRRDY1_R {
        USRRDY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 User Ready"]
    #[inline(always)]
    pub fn usrrdy2(&self) -> USRRDY2_R {
        USRRDY2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 User Ready"]
    #[inline(always)]
    pub fn usrrdy3(&self) -> USRRDY3_R {
        USRRDY3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 User Ready"]
    #[inline(always)]
    pub fn usrrdy4(&self) -> USRRDY4_R {
        USRRDY4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 User Ready"]
    #[inline(always)]
    pub fn usrrdy5(&self) -> USRRDY5_R {
        USRRDY5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Busy"]
    #[inline(always)]
    pub fn chbusy0(&self) -> CHBUSY0_R {
        CHBUSY0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Busy"]
    #[inline(always)]
    pub fn chbusy1(&self) -> CHBUSY1_R {
        CHBUSY1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Busy"]
    #[inline(always)]
    pub fn chbusy2(&self) -> CHBUSY2_R {
        CHBUSY2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Busy"]
    #[inline(always)]
    pub fn chbusy3(&self) -> CHBUSY3_R {
        CHBUSY3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Busy"]
    #[inline(always)]
    pub fn chbusy4(&self) -> CHBUSY4_R {
        CHBUSY4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Busy"]
    #[inline(always)]
    pub fn chbusy5(&self) -> CHBUSY5_R {
        CHBUSY5_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Channel Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSTATUS_SPEC;
impl crate::RegisterSpec for CHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for CHSTATUS_SPEC {}
#[doc = "`reset()` method sets CHSTATUS to value 0x3f"]
impl crate::Resettable for CHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
