#[doc = "Register `ERRADDR` reader"]
pub type R = crate::R<ERRADDR_SPEC>;
#[doc = "Field `ERRADDR` reader - Error Address"]
pub type ERRADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Error Address"]
    #[inline(always)]
    pub fn erraddr(&self) -> ERRADDR_R {
        ERRADDR_R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "Error Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`erraddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERRADDR_SPEC;
impl crate::RegisterSpec for ERRADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erraddr::R`](R) reader structure"]
impl crate::Readable for ERRADDR_SPEC {}
#[doc = "`reset()` method sets ERRADDR to value 0"]
impl crate::Resettable for ERRADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
