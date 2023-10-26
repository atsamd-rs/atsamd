#[doc = "Register `EFTSH` reader"]
pub type R = crate::R<EFTSH_SPEC>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Event Frame Transmitted Seconds High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eftsh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EFTSH_SPEC;
impl crate::RegisterSpec for EFTSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eftsh::R`](R) reader structure"]
impl crate::Readable for EFTSH_SPEC {}
#[doc = "`reset()` method sets EFTSH to value 0"]
impl crate::Resettable for EFTSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
