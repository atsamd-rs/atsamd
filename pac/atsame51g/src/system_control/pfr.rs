#[doc = "Register `PFR[%s]` reader"]
pub type R = crate::R<PFR_SPEC>;
#[doc = "Register `PFR[%s]` writer"]
pub type W = crate::W<PFR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PFR_SPEC> {
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
#[doc = "Processor Feature Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR_SPEC;
impl crate::RegisterSpec for PFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr::R`](R) reader structure"]
impl crate::Readable for PFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr::W`](W) writer structure"]
impl crate::Writable for PFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFR[%s]
to value 0"]
impl crate::Resettable for PFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
