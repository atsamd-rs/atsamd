#[doc = "Register `ITTRIGOUT` writer"]
pub type W = crate::W<ITTRIGOUT_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<ITTRIGOUT_SPEC> {
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
#[doc = "ETM Integration Test Trigger Out\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ittrigout::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITTRIGOUT_SPEC;
impl crate::RegisterSpec for ITTRIGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ittrigout::W`](W) writer structure"]
impl crate::Writable for ITTRIGOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITTRIGOUT to value 0"]
impl crate::Resettable for ITTRIGOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
