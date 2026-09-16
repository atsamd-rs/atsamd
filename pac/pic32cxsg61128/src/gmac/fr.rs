#[doc = "Register `FR` reader"]
pub type R = crate::R<FrSpec>;
#[doc = "Field `FRX` reader - Frames Received without Error"]
pub type FrxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Frames Received without Error"]
    #[inline(always)]
    pub fn frx(&self) -> FrxR {
        FrxR::new(self.bits)
    }
}
#[doc = "Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrSpec;
impl crate::RegisterSpec for FrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FrSpec {}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FrSpec {
    const RESET_VALUE: u32 = 0;
}
