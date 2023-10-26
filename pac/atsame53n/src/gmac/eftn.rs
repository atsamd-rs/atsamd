#[doc = "Register `EFTN` reader"]
pub type R = crate::R<EFTN_SPEC>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Event Frame Transmitted Nanoseconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eftn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EFTN_SPEC;
impl crate::RegisterSpec for EFTN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eftn::R`](R) reader structure"]
impl crate::Readable for EFTN_SPEC {}
#[doc = "`reset()` method sets EFTN to value 0"]
impl crate::Resettable for EFTN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
