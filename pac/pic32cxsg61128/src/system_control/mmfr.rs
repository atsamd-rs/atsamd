#[doc = "Register `MMFR[%s]` reader"]
pub type R = crate::R<MmfrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Memory Model Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmfrSpec;
impl crate::RegisterSpec for MmfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmfr::R`](R) reader structure"]
impl crate::Readable for MmfrSpec {}
#[doc = "`reset()` method sets MMFR[%s]
to value 0"]
impl crate::Resettable for MmfrSpec {
    const RESET_VALUE: u32 = 0;
}
