#[doc = "Register `DTF` reader"]
pub struct R(crate::R<DTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEFT` reader - Deferred Transmission"]
pub type DEFT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Deferred Transmission"]
    #[inline(always)]
    pub fn deft(&self) -> DEFT_R {
        DEFT_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Deferred Transmission Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtf](index.html) module"]
pub struct DTF_SPEC;
impl crate::RegisterSpec for DTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtf::R](R) reader structure"]
impl crate::Readable for DTF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTF to value 0"]
impl crate::Resettable for DTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
