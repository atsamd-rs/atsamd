#[doc = "Register `RANDSEED` reader"]
pub type R = crate::R<RandseedSpec>;
#[doc = "Register `RANDSEED` writer"]
pub type W = crate::W<RandseedSpec>;
#[doc = "Field `RANDSEED` reader - Random Seed"]
pub type RandseedR = crate::FieldReader<u32>;
#[doc = "Field `RANDSEED` writer - Random Seed"]
pub type RandseedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Random Seed"]
    #[inline(always)]
    pub fn randseed(&self) -> RandseedR {
        RandseedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Random Seed"]
    #[inline(always)]
    pub fn randseed(&mut self) -> RandseedW<RandseedSpec> {
        RandseedW::new(self, 0)
    }
}
#[doc = "Random Seed\n\nYou can [`read`](crate::Reg::read) this register and get [`randseed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randseed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RandseedSpec;
impl crate::RegisterSpec for RandseedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randseed::R`](R) reader structure"]
impl crate::Readable for RandseedSpec {}
#[doc = "`write(|w| ..)` method takes [`randseed::W`](W) writer structure"]
impl crate::Writable for RandseedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RANDSEED to value 0"]
impl crate::Resettable for RandseedSpec {}
