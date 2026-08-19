#[doc = "Register `FT` reader"]
pub type R = crate::R<FtSpec>;
#[doc = "Field `FTX` reader - Frames Transmitted without Error"]
pub type FtxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Frames Transmitted without Error"]
    #[inline(always)]
    pub fn ftx(&self) -> FtxR {
        FtxR::new(self.bits)
    }
}
#[doc = "Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ft::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtSpec;
impl crate::RegisterSpec for FtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ft::R`](R) reader structure"]
impl crate::Readable for FtSpec {}
#[doc = "`reset()` method sets FT to value 0"]
impl crate::Resettable for FtSpec {
    const RESET_VALUE: u32 = 0;
}
