#[doc = "Register `BKIN` reader"]
pub struct R(crate::R<BKIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BKIN` reader - Backup Input Value"]
pub type BKIN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Backup Input Value"]
    #[inline(always)]
    pub fn bkin(&self) -> BKIN_R {
        BKIN_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Backup Input Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkin](index.html) module"]
pub struct BKIN_SPEC;
impl crate::RegisterSpec for BKIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkin::R](R) reader structure"]
impl crate::Readable for BKIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BKIN to value 0"]
impl crate::Resettable for BKIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
