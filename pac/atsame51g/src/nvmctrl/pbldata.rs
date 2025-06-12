#[doc = "Register `PBLDATA[%s]` reader"]
pub type R = crate::R<PbldataSpec>;
#[doc = "Field `DATA` reader - Page Buffer Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Page Buffer Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Page Buffer Load Data x\n\nYou can [`read`](crate::Reg::read) this register and get [`pbldata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbldataSpec;
impl crate::RegisterSpec for PbldataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbldata::R`](R) reader structure"]
impl crate::Readable for PbldataSpec {}
#[doc = "`reset()` method sets PBLDATA[%s] to value 0xffff_ffff"]
impl crate::Resettable for PbldataSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
