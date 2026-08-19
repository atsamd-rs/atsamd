#[doc = "Register `CCER` reader"]
pub type R = crate::R<CcerSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Configuration Code Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcerSpec;
impl crate::RegisterSpec for CcerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccer::R`](R) reader structure"]
impl crate::Readable for CcerSpec {}
#[doc = "`reset()` method sets CCER to value 0x1854_1800"]
impl crate::Resettable for CcerSpec {
    const RESET_VALUE: u32 = 0x1854_1800;
}
