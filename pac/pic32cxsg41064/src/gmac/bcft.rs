#[doc = "Register `BCFT` reader"]
pub type R = crate::R<BcftSpec>;
#[doc = "Field `BFTX` reader - Broadcast Frames Transmitted without Error"]
pub type BftxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast Frames Transmitted without Error"]
    #[inline(always)]
    pub fn bftx(&self) -> BftxR {
        BftxR::new(self.bits)
    }
}
#[doc = "Broadcast Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcft::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcftSpec;
impl crate::RegisterSpec for BcftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcft::R`](R) reader structure"]
impl crate::Readable for BcftSpec {}
#[doc = "`reset()` method sets BCFT to value 0"]
impl crate::Resettable for BcftSpec {
    const RESET_VALUE: u32 = 0;
}
