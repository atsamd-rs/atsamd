#[doc = "Register `CIPLEN` reader"]
pub type R = crate::R<CIPLEN_SPEC>;
#[doc = "Register `CIPLEN` writer"]
pub type W = crate::W<CIPLEN_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CIPLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
#[doc = "Cipher Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ciplen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ciplen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIPLEN_SPEC;
impl crate::RegisterSpec for CIPLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ciplen::R`](R) reader structure"]
impl crate::Readable for CIPLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ciplen::W`](W) writer structure"]
impl crate::Writable for CIPLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIPLEN to value 0"]
impl crate::Resettable for CIPLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
