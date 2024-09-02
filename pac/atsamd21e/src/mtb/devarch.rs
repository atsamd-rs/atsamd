#[doc = "Register `DEVARCH` reader"]
pub type R = crate::R<DevarchSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "MTB Device Architecture\n\nYou can [`read`](crate::Reg::read) this register and get [`devarch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevarchSpec;
impl crate::RegisterSpec for DevarchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devarch::R`](R) reader structure"]
impl crate::Readable for DevarchSpec {}
#[doc = "`reset()` method sets DEVARCH to value 0"]
impl crate::Resettable for DevarchSpec {
    const RESET_VALUE: u32 = 0;
}
