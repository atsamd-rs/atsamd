#[doc = "Register `MEMTYPE` reader"]
pub type R = crate::R<MEMTYPE_SPEC>;
#[doc = "Field `SMEMP` reader - System Memory Present"]
pub type SMEMP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - System Memory Present"]
    #[inline(always)]
    pub fn smemp(&self) -> SMEMP_R {
        SMEMP_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CoreSight ROM Table Memory Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memtype::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEMTYPE_SPEC;
impl crate::RegisterSpec for MEMTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memtype::R`](R) reader structure"]
impl crate::Readable for MEMTYPE_SPEC {}
#[doc = "`reset()` method sets MEMTYPE to value 0"]
impl crate::Resettable for MEMTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
