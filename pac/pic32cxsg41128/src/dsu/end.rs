#[doc = "Register `END` reader"]
pub type R = crate::R<EndSpec>;
#[doc = "Field `END` reader - End Marker"]
pub type EndR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - End Marker"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(self.bits)
    }
}
#[doc = "CoreSight ROM Table End\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndSpec;
impl crate::RegisterSpec for EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`end::R`](R) reader structure"]
impl crate::Readable for EndSpec {}
#[doc = "`reset()` method sets END to value 0"]
impl crate::Resettable for EndSpec {
    const RESET_VALUE: u32 = 0;
}
