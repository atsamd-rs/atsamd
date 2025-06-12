#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Field `DATA` reader - Output Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Output Data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {}
