#[doc = "Register `UIHVAL[%s]` writer"]
pub type W = crate::W<UihvalSpec>;
#[doc = "Field `VAL` writer - Initial Hash Value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initial Hash Value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<UihvalSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "User Initial Hash Value n\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UihvalSpec;
impl crate::RegisterSpec for UihvalSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uihval::W`](W) writer structure"]
impl crate::Writable for UihvalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UIHVAL[%s] to value 0"]
impl crate::Resettable for UihvalSpec {}
