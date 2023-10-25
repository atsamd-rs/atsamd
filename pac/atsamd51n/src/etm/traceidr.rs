#[doc = "Register `TRACEIDR` reader"]
pub type R = crate::R<TRACEIDR_SPEC>;
#[doc = "Register `TRACEIDR` writer"]
pub type W = crate::W<TRACEIDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TRACEIDR_SPEC> {
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
#[doc = "ETM CoreSight Trace ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`traceidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`traceidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACEIDR_SPEC;
impl crate::RegisterSpec for TRACEIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`traceidr::R`](R) reader structure"]
impl crate::Readable for TRACEIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`traceidr::W`](W) writer structure"]
impl crate::Writable for TRACEIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACEIDR to value 0"]
impl crate::Resettable for TRACEIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
