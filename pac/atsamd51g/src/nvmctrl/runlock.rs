#[doc = "Register `RUNLOCK` reader"]
pub type R = crate::R<RUNLOCK_SPEC>;
#[doc = "Field `RUNLOCK` reader - Region Un-Lock Bits"]
pub type RUNLOCK_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Region Un-Lock Bits"]
    #[inline(always)]
    pub fn runlock(&self) -> RUNLOCK_R {
        RUNLOCK_R::new(self.bits)
    }
}
#[doc = "Lock Section\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`runlock::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RUNLOCK_SPEC;
impl crate::RegisterSpec for RUNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`runlock::R`](R) reader structure"]
impl crate::Readable for RUNLOCK_SPEC {}
#[doc = "`reset()` method sets RUNLOCK to value 0"]
impl crate::Resettable for RUNLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
