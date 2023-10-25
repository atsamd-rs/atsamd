#[doc = "Register `CLAIMSET` reader"]
pub type R = crate::R<CLAIMSET_SPEC>;
#[doc = "Register `CLAIMSET` writer"]
pub type W = crate::W<CLAIMSET_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLAIMSET_SPEC> {
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
#[doc = "MTB Claim Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLAIMSET_SPEC;
impl crate::RegisterSpec for CLAIMSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimset::R`](R) reader structure"]
impl crate::Readable for CLAIMSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`claimset::W`](W) writer structure"]
impl crate::Writable for CLAIMSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLAIMSET to value 0"]
impl crate::Resettable for CLAIMSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
