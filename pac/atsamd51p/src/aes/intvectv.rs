#[doc = "Register `INTVECTV[%s]` writer"]
pub type W = crate::W<INTVECTV_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<INTVECTV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Initialisation Vector n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intvectv::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTVECTV_SPEC;
impl crate::RegisterSpec for INTVECTV_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intvectv::W`](W) writer structure"]
impl crate::Writable for INTVECTV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTVECTV[%s]
to value 0"]
impl crate::Resettable for INTVECTV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
