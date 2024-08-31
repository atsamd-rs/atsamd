#[doc = "Register `ITMISCIN` reader"]
pub type R = crate::R<ItmiscinSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Integration Test Miscellaneous Inputs\n\nYou can [`read`](crate::Reg::read) this register and get [`itmiscin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ItmiscinSpec;
impl crate::RegisterSpec for ItmiscinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itmiscin::R`](R) reader structure"]
impl crate::Readable for ItmiscinSpec {}
#[doc = "`reset()` method sets ITMISCIN to value 0"]
impl crate::Resettable for ItmiscinSpec {
    const RESET_VALUE: u32 = 0;
}
