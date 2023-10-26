#[doc = "Register `EFRSH` reader"]
pub type R = crate::R<EFRSH_SPEC>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Event Frame Received Seconds High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efrsh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EFRSH_SPEC;
impl crate::RegisterSpec for EFRSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efrsh::R`](R) reader structure"]
impl crate::Readable for EFRSH_SPEC {}
#[doc = "`reset()` method sets EFRSH to value 0"]
impl crate::Resettable for EFRSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
