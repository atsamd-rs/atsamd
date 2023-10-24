#[doc = "Register `ITATBCTR0` writer"]
pub type W = crate::W<ITATBCTR0_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<ITATBCTR0_SPEC> {
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
#[doc = "ETM Integration Test ATB Control 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itatbctr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITATBCTR0_SPEC;
impl crate::RegisterSpec for ITATBCTR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`itatbctr0::W`](W) writer structure"]
impl crate::Writable for ITATBCTR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITATBCTR0 to value 0"]
impl crate::Resettable for ITATBCTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
