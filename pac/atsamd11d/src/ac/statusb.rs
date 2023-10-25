#[doc = "Register `STATUSB` reader"]
pub type R = crate::R<STATUSB_SPEC>;
#[doc = "Field `READY0` reader - Comparator 0 Ready"]
pub type READY0_R = crate::BitReader;
#[doc = "Field `READY1` reader - Comparator 1 Ready"]
pub type READY1_R = crate::BitReader;
#[doc = "Field `SYNCBUSY` reader - Synchronization Busy"]
pub type SYNCBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Comparator 0 Ready"]
    #[inline(always)]
    pub fn ready0(&self) -> READY0_R {
        READY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Ready"]
    #[inline(always)]
    pub fn ready1(&self) -> READY1_R {
        READY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SYNCBUSY_R {
        SYNCBUSY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSB_SPEC;
impl crate::RegisterSpec for STATUSB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`statusb::R`](R) reader structure"]
impl crate::Readable for STATUSB_SPEC {}
#[doc = "`reset()` method sets STATUSB to value 0"]
impl crate::Resettable for STATUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
