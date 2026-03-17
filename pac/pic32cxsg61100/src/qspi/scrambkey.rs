#[doc = "Register `SCRAMBKEY` writer"]
pub type W = crate::W<ScrambkeySpec>;
#[doc = "Field `KEY` writer - Scrambling User Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Scrambling User Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<ScrambkeySpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Scrambling Key\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scrambkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrambkeySpec;
impl crate::RegisterSpec for ScrambkeySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scrambkey::W`](W) writer structure"]
impl crate::Writable for ScrambkeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRAMBKEY to value 0"]
impl crate::Resettable for ScrambkeySpec {
    const RESET_VALUE: u32 = 0;
}
