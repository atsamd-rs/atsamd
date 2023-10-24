#[doc = "Register `RLPITR` reader"]
pub type R = crate::R<RLPITR_SPEC>;
#[doc = "Field `RLPITR` reader - Count number of times transition from rx normal idle to low power idle"]
pub type RLPITR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count number of times transition from rx normal idle to low power idle"]
    #[inline(always)]
    pub fn rlpitr(&self) -> RLPITR_R {
        RLPITR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive LPI transition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlpitr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLPITR_SPEC;
impl crate::RegisterSpec for RLPITR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlpitr::R`](R) reader structure"]
impl crate::Readable for RLPITR_SPEC {}
#[doc = "`reset()` method sets RLPITR to value 0"]
impl crate::Resettable for RLPITR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
