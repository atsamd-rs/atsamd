#[doc = "Register `PINSTATE` reader"]
pub type R = crate::R<PinstateSpec>;
#[doc = "Field `PINSTATE` reader - Pin State"]
pub type PinstateR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Pin State"]
    #[inline(always)]
    pub fn pinstate(&self) -> PinstateR {
        PinstateR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pin State\n\nYou can [`read`](crate::Reg::read) this register and get [`pinstate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinstateSpec;
impl crate::RegisterSpec for PinstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinstate::R`](R) reader structure"]
impl crate::Readable for PinstateSpec {}
#[doc = "`reset()` method sets PINSTATE to value 0"]
impl crate::Resettable for PinstateSpec {
    const RESET_VALUE: u32 = 0;
}
