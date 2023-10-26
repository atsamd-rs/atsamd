#[doc = "Register `JR` reader"]
pub type R = crate::R<JR_SPEC>;
#[doc = "Field `JRX` reader - Jabbers Received"]
pub type JRX_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Jabbers Received"]
    #[inline(always)]
    pub fn jrx(&self) -> JRX_R {
        JRX_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Jabbers Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JR_SPEC;
impl crate::RegisterSpec for JR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jr::R`](R) reader structure"]
impl crate::Readable for JR_SPEC {}
#[doc = "`reset()` method sets JR to value 0"]
impl crate::Resettable for JR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
