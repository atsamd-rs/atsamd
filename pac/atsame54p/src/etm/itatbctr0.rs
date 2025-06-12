#[doc = "Register `ITATBCTR0` writer"]
pub type W = crate::W<Itatbctr0Spec>;
impl core::fmt::Debug for crate::generic::Reg<Itatbctr0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "ETM Integration Test ATB Control 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbctr0Spec;
impl crate::RegisterSpec for Itatbctr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`itatbctr0::W`](W) writer structure"]
impl crate::Writable for Itatbctr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITATBCTR0 to value 0"]
impl crate::Resettable for Itatbctr0Spec {}
