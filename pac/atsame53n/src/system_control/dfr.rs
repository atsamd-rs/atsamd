#[doc = "Register `DFR` reader"]
pub type R = crate::R<DfrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Debug Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfrSpec;
impl crate::RegisterSpec for DfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfr::R`](R) reader structure"]
impl crate::Readable for DfrSpec {}
#[doc = "`reset()` method sets DFR to value 0"]
impl crate::Resettable for DfrSpec {}
