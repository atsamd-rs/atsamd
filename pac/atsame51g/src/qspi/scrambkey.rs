#[doc = "Register `SCRAMBKEY` writer"]
pub type W = crate::W<SCRAMBKEY_SPEC>;
#[doc = "Field `KEY` writer - Scrambling User Key"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Scrambling User Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<SCRAMBKEY_SPEC, 0> {
        KEY_W::new(self)
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
#[doc = "Scrambling Key\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scrambkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRAMBKEY_SPEC;
impl crate::RegisterSpec for SCRAMBKEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scrambkey::W`](W) writer structure"]
impl crate::Writable for SCRAMBKEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCRAMBKEY to value 0"]
impl crate::Resettable for SCRAMBKEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
