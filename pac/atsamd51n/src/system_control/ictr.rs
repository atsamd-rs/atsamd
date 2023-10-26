#[doc = "Register `ICTR` reader"]
pub type R = crate::R<ICTR_SPEC>;
#[doc = "Field `INTLINESNUM` reader - "]
pub type INTLINESNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn intlinesnum(&self) -> INTLINESNUM_R {
        INTLINESNUM_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Interrupt Controller Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ictr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICTR_SPEC;
impl crate::RegisterSpec for ICTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ictr::R`](R) reader structure"]
impl crate::Readable for ICTR_SPEC {}
#[doc = "`reset()` method sets ICTR to value 0"]
impl crate::Resettable for ICTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
