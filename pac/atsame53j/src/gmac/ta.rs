#[doc = "Register `TA` writer"]
pub type W = crate::W<TA_SPEC>;
#[doc = "Field `ITDT` writer - Increment/Decrement"]
pub type ITDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
#[doc = "Field `ADJ` writer - Adjust 1588 Timer"]
pub type ADJ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:29 - Increment/Decrement"]
    #[inline(always)]
    #[must_use]
    pub fn itdt(&mut self) -> ITDT_W<TA_SPEC, 0> {
        ITDT_W::new(self)
    }
    #[doc = "Bit 31 - Adjust 1588 Timer"]
    #[inline(always)]
    #[must_use]
    pub fn adj(&mut self) -> ADJ_W<TA_SPEC, 31> {
        ADJ_W::new(self)
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
#[doc = "1588 Timer Adjust Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TA_SPEC;
impl crate::RegisterSpec for TA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ta::W`](W) writer structure"]
impl crate::Writable for TA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TA to value 0"]
impl crate::Resettable for TA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
