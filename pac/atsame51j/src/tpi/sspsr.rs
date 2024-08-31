#[doc = "Register `SSPSR` reader"]
pub type R = crate::R<SspsrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Supported Parallel Port Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspsrSpec;
impl crate::RegisterSpec for SspsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspsr::R`](R) reader structure"]
impl crate::Readable for SspsrSpec {}
#[doc = "`reset()` method sets SSPSR to value 0"]
impl crate::Resettable for SspsrSpec {
    const RESET_VALUE: u32 = 0;
}
