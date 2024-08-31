#[doc = "Register `INTVECTV[%s]` writer"]
pub type W = crate::W<IntvectvSpec>;
impl core::fmt::Debug for crate::generic::Reg<IntvectvSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Initialisation Vector n\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvectv::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntvectvSpec;
impl crate::RegisterSpec for IntvectvSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intvectv::W`](W) writer structure"]
impl crate::Writable for IntvectvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTVECTV[%s]
to value 0"]
impl crate::Resettable for IntvectvSpec {
    const RESET_VALUE: u32 = 0;
}
