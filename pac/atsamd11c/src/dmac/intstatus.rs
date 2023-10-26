#[doc = "Register `INTSTATUS` reader"]
pub type R = crate::R<INTSTATUS_SPEC>;
#[doc = "Field `CHINT0` reader - Channel 0 Pending Interrupt"]
pub type CHINT0_R = crate::BitReader;
#[doc = "Field `CHINT1` reader - Channel 1 Pending Interrupt"]
pub type CHINT1_R = crate::BitReader;
#[doc = "Field `CHINT2` reader - Channel 2 Pending Interrupt"]
pub type CHINT2_R = crate::BitReader;
#[doc = "Field `CHINT3` reader - Channel 3 Pending Interrupt"]
pub type CHINT3_R = crate::BitReader;
#[doc = "Field `CHINT4` reader - Channel 4 Pending Interrupt"]
pub type CHINT4_R = crate::BitReader;
#[doc = "Field `CHINT5` reader - Channel 5 Pending Interrupt"]
pub type CHINT5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Pending Interrupt"]
    #[inline(always)]
    pub fn chint0(&self) -> CHINT0_R {
        CHINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Pending Interrupt"]
    #[inline(always)]
    pub fn chint1(&self) -> CHINT1_R {
        CHINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Pending Interrupt"]
    #[inline(always)]
    pub fn chint2(&self) -> CHINT2_R {
        CHINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Pending Interrupt"]
    #[inline(always)]
    pub fn chint3(&self) -> CHINT3_R {
        CHINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Pending Interrupt"]
    #[inline(always)]
    pub fn chint4(&self) -> CHINT4_R {
        CHINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Pending Interrupt"]
    #[inline(always)]
    pub fn chint5(&self) -> CHINT5_R {
        CHINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTATUS_SPEC;
impl crate::RegisterSpec for INTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatus::R`](R) reader structure"]
impl crate::Readable for INTSTATUS_SPEC {}
#[doc = "`reset()` method sets INTSTATUS to value 0"]
impl crate::Resettable for INTSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
