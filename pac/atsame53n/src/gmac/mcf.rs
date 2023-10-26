#[doc = "Register `MCF` reader"]
pub type R = crate::R<MCF_SPEC>;
#[doc = "Field `MCOL` reader - Multiple Collision"]
pub type MCOL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Multiple Collision"]
    #[inline(always)]
    pub fn mcol(&self) -> MCOL_R {
        MCOL_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Multiple Collision Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCF_SPEC;
impl crate::RegisterSpec for MCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcf::R`](R) reader structure"]
impl crate::Readable for MCF_SPEC {}
#[doc = "`reset()` method sets MCF to value 0"]
impl crate::Resettable for MCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
