#[doc = "Register `ERRADDR` reader"]
pub type R = crate::R<ErraddrSpec>;
#[doc = "Field `ERRADDR` reader - Error Address"]
pub type ErraddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Error Address"]
    #[inline(always)]
    pub fn erraddr(&self) -> ErraddrR {
        ErraddrR::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "Error Address\n\nYou can [`read`](crate::Reg::read) this register and get [`erraddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErraddrSpec;
impl crate::RegisterSpec for ErraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erraddr::R`](R) reader structure"]
impl crate::Readable for ErraddrSpec {}
#[doc = "`reset()` method sets ERRADDR to value 0"]
impl crate::Resettable for ErraddrSpec {}
