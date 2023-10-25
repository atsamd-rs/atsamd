#[doc = "Register `SCF` reader"]
pub type R = crate::R<SCF_SPEC>;
#[doc = "Field `SCOL` reader - Single Collision"]
pub type SCOL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Single Collision"]
    #[inline(always)]
    pub fn scol(&self) -> SCOL_R {
        SCOL_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Single Collision Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCF_SPEC;
impl crate::RegisterSpec for SCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scf::R`](R) reader structure"]
impl crate::Readable for SCF_SPEC {}
#[doc = "`reset()` method sets SCF to value 0"]
impl crate::Resettable for SCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
