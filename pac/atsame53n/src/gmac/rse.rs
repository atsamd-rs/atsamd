#[doc = "Register `RSE` reader"]
pub type R = crate::R<RseSpec>;
#[doc = "Field `RXSE` reader - Receive Symbol Errors"]
pub type RxseR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rxse(&self) -> RxseR {
        RxseR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receive Symbol Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rse::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RseSpec;
impl crate::RegisterSpec for RseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rse::R`](R) reader structure"]
impl crate::Readable for RseSpec {}
#[doc = "`reset()` method sets RSE to value 0"]
impl crate::Resettable for RseSpec {}
