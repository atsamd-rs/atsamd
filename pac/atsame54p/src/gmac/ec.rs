#[doc = "Register `EC` reader"]
pub type R = crate::R<EC_SPEC>;
#[doc = "Field `XCOL` reader - Excessive Collisions"]
pub type XCOL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Excessive Collisions"]
    #[inline(always)]
    pub fn xcol(&self) -> XCOL_R {
        XCOL_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Excessive Collisions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EC_SPEC;
impl crate::RegisterSpec for EC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ec::R`](R) reader structure"]
impl crate::Readable for EC_SPEC {}
#[doc = "`reset()` method sets EC to value 0"]
impl crate::Resettable for EC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
