#[doc = "Register `CHSTATUS` reader"]
pub type R = crate::R<ChstatusSpec>;
#[doc = "Field `PEND` reader - Channel Pending"]
pub type PendR = crate::BitReader;
#[doc = "Field `BUSY` reader - Channel Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `FERR` reader - Channel Fetch Error"]
pub type FerrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel Pending"]
    #[inline(always)]
    pub fn pend(&self) -> PendR {
        PendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Fetch Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Channel Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatusSpec;
impl crate::RegisterSpec for ChstatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for ChstatusSpec {}
#[doc = "`reset()` method sets CHSTATUS to value 0"]
impl crate::Resettable for ChstatusSpec {
    const RESET_VALUE: u8 = 0;
}
