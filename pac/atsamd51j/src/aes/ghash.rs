#[doc = "Register `GHASH[%s]` reader"]
pub type R = crate::R<GhashSpec>;
#[doc = "Register `GHASH[%s]` writer"]
pub type W = crate::W<GhashSpec>;
#[doc = "Field `GHASH` reader - Galois Hash Value"]
pub type GhashR = crate::FieldReader<u32>;
#[doc = "Field `GHASH` writer - Galois Hash Value"]
pub type GhashW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Galois Hash Value"]
    #[inline(always)]
    pub fn ghash(&self) -> GhashR {
        GhashR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Galois Hash Value"]
    #[inline(always)]
    pub fn ghash(&mut self) -> GhashW<GhashSpec> {
        GhashW::new(self, 0)
    }
}
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
}
#[doc = "`reset()` method sets GHASH[%s] to value 0"]
impl crate::Resettable for GhashSpec {}
