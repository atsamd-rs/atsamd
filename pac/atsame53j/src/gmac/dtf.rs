#[doc = "Register `DTF` reader"]
pub type R = crate::R<DtfSpec>;
#[doc = "Field `DEFT` reader - Deferred Transmission"]
pub type DeftR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Deferred Transmission"]
    #[inline(always)]
    pub fn deft(&self) -> DeftR {
        DeftR::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Deferred Transmission Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtfSpec;
impl crate::RegisterSpec for DtfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtf::R`](R) reader structure"]
impl crate::Readable for DtfSpec {}
#[doc = "`reset()` method sets DTF to value 0"]
impl crate::Resettable for DtfSpec {}
