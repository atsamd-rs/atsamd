#[doc = "Register `CHSTATUS` reader"]
pub type R = crate::R<CHSTATUS_SPEC>;
#[doc = "Field `PEND` reader - Channel Pending"]
pub type PEND_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Channel Busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `FERR` reader - Channel Fetch Error"]
pub type FERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel Pending"]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Fetch Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Channel Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSTATUS_SPEC;
impl crate::RegisterSpec for CHSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for CHSTATUS_SPEC {}
#[doc = "`reset()` method sets CHSTATUS to value 0"]
impl crate::Resettable for CHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
