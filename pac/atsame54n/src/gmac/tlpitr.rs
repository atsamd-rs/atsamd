#[doc = "Register `TLPITR` reader"]
pub type R = crate::R<TLPITR_SPEC>;
#[doc = "Field `TLPITR` reader - Count number of times enable LPI tx bit 20 goes from low to high"]
pub type TLPITR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count number of times enable LPI tx bit 20 goes from low to high"]
    #[inline(always)]
    pub fn tlpitr(&self) -> TLPITR_R {
        TLPITR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive LPI transition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tlpitr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TLPITR_SPEC;
impl crate::RegisterSpec for TLPITR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tlpitr::R`](R) reader structure"]
impl crate::Readable for TLPITR_SPEC {}
#[doc = "`reset()` method sets TLPITR to value 0"]
impl crate::Resettable for TLPITR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
