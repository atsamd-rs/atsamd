#[doc = "Register `LC` reader"]
pub type R = crate::R<LC_SPEC>;
#[doc = "Field `LCOL` reader - Late Collisions"]
pub type LCOL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Late Collisions"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Late Collisions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_SPEC;
impl crate::RegisterSpec for LC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc::R`](R) reader structure"]
impl crate::Readable for LC_SPEC {}
#[doc = "`reset()` method sets LC to value 0"]
impl crate::Resettable for LC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
