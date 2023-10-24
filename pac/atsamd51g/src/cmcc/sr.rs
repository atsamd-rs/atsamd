#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `CSTS` reader - Cache Controller Status"]
pub type CSTS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Cache Controller Status"]
    #[inline(always)]
    pub fn csts(&self) -> CSTS_R {
        CSTS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Cache Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
