#[doc = "Register `KEYWORD[%s]` writer"]
pub type W = crate::W<KeywordSpec>;
impl core::fmt::Debug for crate::generic::Reg<KeywordSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Keyword n\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyword::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeywordSpec;
impl crate::RegisterSpec for KeywordSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyword::W`](W) writer structure"]
impl crate::Writable for KeywordSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYWORD[%s]
to value 0"]
impl crate::Resettable for KeywordSpec {
    const RESET_VALUE: u32 = 0;
}
