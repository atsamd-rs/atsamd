#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Configuration Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`reset()` method sets CCR to value 0x8c80_2000"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0x8c80_2000;
}
