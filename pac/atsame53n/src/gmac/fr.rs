#[doc = "Register `FR` reader"]
pub struct R(crate::R<FR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRX` reader - Frames Received without Error"]
pub type FRX_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Frames Received without Error"]
    #[inline(always)]
    pub fn frx(&self) -> FRX_R {
        FRX_R::new(self.bits)
    }
}
#[doc = "Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](index.html) module"]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr::R](R) reader structure"]
impl crate::Readable for FR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
