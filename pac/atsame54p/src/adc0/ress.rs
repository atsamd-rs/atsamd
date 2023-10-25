#[doc = "Register `RESS` reader"]
pub type R = crate::R<RESS_SPEC>;
#[doc = "Field `RESS` reader - Last ADC conversion result"]
pub type RESS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Last ADC conversion result"]
    #[inline(always)]
    pub fn ress(&self) -> RESS_R {
        RESS_R::new(self.bits)
    }
}
#[doc = "Last Sample Result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ress::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESS_SPEC;
impl crate::RegisterSpec for RESS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ress::R`](R) reader structure"]
impl crate::Readable for RESS_SPEC {}
#[doc = "`reset()` method sets RESS to value 0"]
impl crate::Resettable for RESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
