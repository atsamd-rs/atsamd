#[doc = "Register `ITTRIGOUT` writer"]
pub type W = crate::W<IttrigoutSpec>;
impl core::fmt::Debug for crate::generic::Reg<IttrigoutSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "ETM Integration Test Trigger Out\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrigout::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IttrigoutSpec;
impl crate::RegisterSpec for IttrigoutSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ittrigout::W`](W) writer structure"]
impl crate::Writable for IttrigoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITTRIGOUT to value 0"]
impl crate::Resettable for IttrigoutSpec {
    const RESET_VALUE: u32 = 0;
}
