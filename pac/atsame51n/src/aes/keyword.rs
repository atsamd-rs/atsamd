#[doc = "Register `KEYWORD[%s]` writer"]
pub type W = crate::W<KEYWORD_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<KEYWORD_SPEC> {
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
#[doc = "Keyword n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyword::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYWORD_SPEC;
impl crate::RegisterSpec for KEYWORD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyword::W`](W) writer structure"]
impl crate::Writable for KEYWORD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYWORD[%s]
to value 0"]
impl crate::Resettable for KEYWORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
