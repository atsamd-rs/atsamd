#[doc = "Register `MCTRL` writer"]
pub type W = crate::W<MctrlSpec>;
#[doc = "Field `SWRST` writer - Cache Controller Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Cache Controller Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<MctrlSpec> {
        SwrstW::new(self, 0)
    }
}
#[doc = "Cache Monitor Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctrlSpec;
impl crate::RegisterSpec for MctrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mctrl::W`](W) writer structure"]
impl crate::Writable for MctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MctrlSpec {
    const RESET_VALUE: u32 = 0;
}
