#[doc = "Register `GHASH[%s]` reader"]
pub type R = crate::R<GhashSpec>;
#[doc = "Register `GHASH[%s]` writer"]
pub type W = crate::W<GhashSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Galois Hash n\n\nYou can [`read`](crate::Reg::read) this register and get [`ghash::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghash::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GhashSpec;
impl crate::RegisterSpec for GhashSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghash::R`](R) reader structure"]
impl crate::Readable for GhashSpec {}
#[doc = "`write(|w| ..)` method takes [`ghash::W`](W) writer structure"]
impl crate::Writable for GhashSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASH[%s]
to value 0"]
impl crate::Resettable for GhashSpec {
    const RESET_VALUE: u32 = 0;
}
