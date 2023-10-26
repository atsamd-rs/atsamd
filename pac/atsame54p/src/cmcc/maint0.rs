#[doc = "Register `MAINT0` writer"]
pub type W = crate::W<MAINT0_SPEC>;
#[doc = "Field `INVALL` writer - Cache Controller invalidate All"]
pub type INVALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Cache Controller invalidate All"]
    #[inline(always)]
    #[must_use]
    pub fn invall(&mut self) -> INVALL_W<MAINT0_SPEC, 0> {
        INVALL_W::new(self)
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
#[doc = "Cache Maintenance Register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maint0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAINT0_SPEC;
impl crate::RegisterSpec for MAINT0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`maint0::W`](W) writer structure"]
impl crate::Writable for MAINT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAINT0 to value 0"]
impl crate::Resettable for MAINT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
