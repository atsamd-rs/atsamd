#[doc = "Register `PBLDATA[%s]` reader"]
pub struct R(crate::R<PBLDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBLDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBLDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBLDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Page Buffer Data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Page Buffer Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Page Buffer Load Data x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbldata](index.html) module"]
pub struct PBLDATA_SPEC;
impl crate::RegisterSpec for PBLDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbldata::R](R) reader structure"]
impl crate::Readable for PBLDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PBLDATA[%s]
to value 0xffff_ffff"]
impl crate::Resettable for PBLDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
