#[doc = "Register `HASHKEY[%s]` reader"]
pub type R = crate::R<HashkeySpec>;
#[doc = "Register `HASHKEY[%s]` writer"]
pub type W = crate::W<HashkeySpec>;
#[doc = "Field `HASHKEY` reader - Hash Key Value"]
pub type HashkeyR = crate::FieldReader<u32>;
#[doc = "Field `HASHKEY` writer - Hash Key Value"]
pub type HashkeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Key Value"]
    #[inline(always)]
    pub fn hashkey(&self) -> HashkeyR {
        HashkeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Key Value"]
    #[inline(always)]
    #[must_use]
    pub fn hashkey(&mut self) -> HashkeyW<HashkeySpec> {
        HashkeyW::new(self, 0)
    }
}
#[doc = "Hash key n\n\nYou can [`read`](crate::Reg::read) this register and get [`hashkey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashkey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashkeySpec;
impl crate::RegisterSpec for HashkeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashkey::R`](R) reader structure"]
impl crate::Readable for HashkeySpec {}
#[doc = "`write(|w| ..)` method takes [`hashkey::W`](W) writer structure"]
impl crate::Writable for HashkeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHKEY[%s]
to value 0"]
impl crate::Resettable for HashkeySpec {
    const RESET_VALUE: u32 = 0;
}
