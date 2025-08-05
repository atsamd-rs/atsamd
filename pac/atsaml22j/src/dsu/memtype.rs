#[doc = "Register `MEMTYPE` reader"]
pub struct R(crate::R<MEMTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SMEMP` reader - System Memory Present"]
pub type SMEMP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - System Memory Present"]
    #[inline(always)]
    pub fn smemp(&self) -> SMEMP_R {
        SMEMP_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CoreSight ROM Table Memory Type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memtype](index.html) module"]
pub struct MEMTYPE_SPEC;
impl crate::RegisterSpec for MEMTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memtype::R](R) reader structure"]
impl crate::Readable for MEMTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEMTYPE to value 0"]
impl crate::Resettable for MEMTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
