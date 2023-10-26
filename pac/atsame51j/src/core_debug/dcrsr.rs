#[doc = "Register `DCRSR` writer"]
pub type W = crate::W<DCRSR_SPEC>;
#[doc = "Field `REGSEL` writer - "]
pub type REGSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `REGWnR` writer - "]
pub type REGWN_R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn regsel(&mut self) -> REGSEL_W<DCRSR_SPEC, 0> {
        REGSEL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn regwn_r(&mut self) -> REGWN_R_W<DCRSR_SPEC, 16> {
        REGWN_R_W::new(self)
    }
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
#[doc = "Debug Core Register Selector Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCRSR_SPEC;
impl crate::RegisterSpec for DCRSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dcrsr::W`](W) writer structure"]
impl crate::Writable for DCRSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCRSR to value 0"]
impl crate::Resettable for DCRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
