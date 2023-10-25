#[doc = "Register `PEFTSH` reader"]
pub type R = crate::R<PEFTSH_SPEC>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peftsh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEFTSH_SPEC;
impl crate::RegisterSpec for PEFTSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peftsh::R`](R) reader structure"]
impl crate::Readable for PEFTSH_SPEC {}
#[doc = "`reset()` method sets PEFTSH to value 0"]
impl crate::Resettable for PEFTSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
