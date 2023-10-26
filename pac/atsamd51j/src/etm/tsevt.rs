#[doc = "Register `TSEVT` reader"]
pub type R = crate::R<TSEVT_SPEC>;
#[doc = "Register `TSEVT` writer"]
pub type W = crate::W<TSEVT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TSEVT_SPEC> {
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
#[doc = "ETM TimeStamp Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsevt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsevt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSEVT_SPEC;
impl crate::RegisterSpec for TSEVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsevt::R`](R) reader structure"]
impl crate::Readable for TSEVT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsevt::W`](W) writer structure"]
impl crate::Writable for TSEVT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSEVT to value 0"]
impl crate::Resettable for TSEVT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
