#[doc = "Register `PDSR` reader"]
pub type R = crate::R<PdsrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Device Power-Down Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsrSpec;
impl crate::RegisterSpec for PdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdsr::R`](R) reader structure"]
impl crate::Readable for PdsrSpec {}
#[doc = "`reset()` method sets PDSR to value 0x01"]
impl crate::Resettable for PdsrSpec {
    const RESET_VALUE: u32 = 0x01;
}
