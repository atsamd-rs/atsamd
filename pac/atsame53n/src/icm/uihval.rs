#[doc = "Register `UIHVAL[%s]` writer"]
pub type W = crate::W<UIHVAL_SPEC>;
#[doc = "Field `VAL` writer - Initial Hash Value"]
pub type VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Initial Hash Value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<UIHVAL_SPEC, 0> {
        VAL_W::new(self)
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
#[doc = "User Initial Hash Value n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uihval::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UIHVAL_SPEC;
impl crate::RegisterSpec for UIHVAL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uihval::W`](W) writer structure"]
impl crate::Writable for UIHVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UIHVAL[%s]
to value 0"]
impl crate::Resettable for UIHVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
