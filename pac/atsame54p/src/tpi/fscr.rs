#[doc = "Register `FSCR` reader"]
pub type R = crate::R<FscrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Formatter Synchronization Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FscrSpec;
impl crate::RegisterSpec for FscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscr::R`](R) reader structure"]
impl crate::Readable for FscrSpec {}
#[doc = "`reset()` method sets FSCR to value 0"]
impl crate::Resettable for FscrSpec {
    const RESET_VALUE: u32 = 0;
}
