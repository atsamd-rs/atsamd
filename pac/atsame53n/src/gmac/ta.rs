#[doc = "Register `TA` writer"]
pub type W = crate::W<TaSpec>;
#[doc = "Field `ITDT` writer - Increment/Decrement"]
pub type ItdtW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `ADJ` writer - Adjust 1588 Timer"]
pub type AdjW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:29 - Increment/Decrement"]
    #[inline(always)]
    pub fn itdt(&mut self) -> ItdtW<TaSpec> {
        ItdtW::new(self, 0)
    }
    #[doc = "Bit 31 - Adjust 1588 Timer"]
    #[inline(always)]
    pub fn adj(&mut self) -> AdjW<TaSpec> {
        AdjW::new(self, 31)
    }
}
#[doc = "1588 Timer Adjust Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaSpec;
impl crate::RegisterSpec for TaSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ta::W`](W) writer structure"]
impl crate::Writable for TaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA to value 0"]
impl crate::Resettable for TaSpec {}
