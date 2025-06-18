#[doc = "Register `LAR` writer"]
pub type W = crate::W<LarSpec>;
impl core::fmt::Debug for crate::generic::Reg<LarSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "ETM Lock Access Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LarSpec;
impl crate::RegisterSpec for LarSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lar::W`](W) writer structure"]
impl crate::Writable for LarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LAR to value 0"]
impl crate::Resettable for LarSpec {}
