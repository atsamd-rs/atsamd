#[doc = "Register `KEYWORD[%s]` writer"]
pub type W = crate::W<KeywordSpec>;
#[doc = "Field `KEYWORD` writer - Key Word Value"]
pub type KeywordW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Key Word Value"]
    #[inline(always)]
    #[must_use]
    pub fn keyword(&mut self) -> KeywordW<KeywordSpec> {
        KeywordW::new(self, 0)
    }
}
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
