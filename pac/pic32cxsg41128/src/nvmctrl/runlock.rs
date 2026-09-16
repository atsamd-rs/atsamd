#[doc = "Register `RUNLOCK` reader"]
pub type R = crate::R<RunlockSpec>;
#[doc = "Field `RUNLOCK` reader - Region Un-Lock Bits"]
pub type RunlockR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Region Un-Lock Bits"]
    #[inline(always)]
    pub fn runlock(&self) -> RunlockR {
        RunlockR::new(self.bits)
    }
}
#[doc = "Lock Section\n\nYou can [`read`](crate::Reg::read) this register and get [`runlock::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RunlockSpec;
impl crate::RegisterSpec for RunlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`runlock::R`](R) reader structure"]
impl crate::Readable for RunlockSpec {}
#[doc = "`reset()` method sets RUNLOCK to value 0"]
impl crate::Resettable for RunlockSpec {
    const RESET_VALUE: u32 = 0;
}
