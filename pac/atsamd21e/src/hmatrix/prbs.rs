#[doc = "Register `PRBS%s` reader"]
pub type R = crate::R<PRBS_SPEC>;
#[doc = "Register `PRBS%s` writer"]
pub type W = crate::W<PRBS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PRBS_SPEC> {
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
#[doc = "Priority B for Slave\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prbs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prbs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRBS_SPEC;
impl crate::RegisterSpec for PRBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prbs::R`](R) reader structure"]
impl crate::Readable for PRBS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prbs::W`](W) writer structure"]
impl crate::Writable for PRBS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRBS%s to value 0"]
impl crate::Resettable for PRBS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
