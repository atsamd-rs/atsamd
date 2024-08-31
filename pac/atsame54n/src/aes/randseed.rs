#[doc = "Register `RANDSEED` reader"]
pub type R = crate::R<RandseedSpec>;
#[doc = "Register `RANDSEED` writer"]
pub type W = crate::W<RandseedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANDSEED to value 0"]
impl crate::Resettable for RandseedSpec {
    const RESET_VALUE: u32 = 0;
}
