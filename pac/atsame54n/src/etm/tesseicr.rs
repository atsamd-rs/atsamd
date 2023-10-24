#[doc = "Register `TESSEICR` reader"]
pub type R = crate::R<TESSEICR_SPEC>;
#[doc = "Register `TESSEICR` writer"]
pub type W = crate::W<TESSEICR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TESSEICR_SPEC> {
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
#[doc = "ETM TraceEnable Start/Stop EmbeddedICE Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tesseicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tesseicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TESSEICR_SPEC;
impl crate::RegisterSpec for TESSEICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tesseicr::R`](R) reader structure"]
impl crate::Readable for TESSEICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tesseicr::W`](W) writer structure"]
impl crate::Writable for TESSEICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TESSEICR to value 0"]
impl crate::Resettable for TESSEICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
