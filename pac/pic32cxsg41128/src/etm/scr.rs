#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM System Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`reset()` method sets SCR to value 0x0002_0d09"]
impl crate::Resettable for ScrSpec {
    const RESET_VALUE: u32 = 0x0002_0d09;
}
