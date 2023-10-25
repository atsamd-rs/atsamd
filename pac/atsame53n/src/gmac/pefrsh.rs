#[doc = "Register `PEFRSH` reader"]
pub type R = crate::R<PEFRSH_SPEC>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Peer Event Frame Received Seconds High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pefrsh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEFRSH_SPEC;
impl crate::RegisterSpec for PEFRSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pefrsh::R`](R) reader structure"]
impl crate::Readable for PEFRSH_SPEC {}
#[doc = "`reset()` method sets PEFRSH to value 0"]
impl crate::Resettable for PEFRSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
