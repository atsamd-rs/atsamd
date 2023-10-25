#[doc = "Register `PBLDATA[%s]` reader"]
pub type R = crate::R<PBLDATA_SPEC>;
#[doc = "Field `DATA` reader - Page Buffer Data"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Page Buffer Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Page Buffer Load Data x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbldata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PBLDATA_SPEC;
impl crate::RegisterSpec for PBLDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbldata::R`](R) reader structure"]
impl crate::Readable for PBLDATA_SPEC {}
#[doc = "`reset()` method sets PBLDATA[%s]
to value 0xffff_ffff"]
impl crate::Resettable for PBLDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
