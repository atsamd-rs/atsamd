#[doc = "Register `MFT` reader"]
pub type R = crate::R<MftSpec>;
#[doc = "Field `MFTX` reader - Multicast Frames Transmitted without Error"]
pub type MftxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast Frames Transmitted without Error"]
    #[inline(always)]
    pub fn mftx(&self) -> MftxR {
        MftxR::new(self.bits)
    }
}
#[doc = "Multicast Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mft::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MftSpec;
impl crate::RegisterSpec for MftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mft::R`](R) reader structure"]
impl crate::Readable for MftSpec {}
#[doc = "`reset()` method sets MFT to value 0"]
impl crate::Resettable for MftSpec {}
