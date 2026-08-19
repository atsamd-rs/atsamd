#[doc = "Register `PFR[%s]` reader"]
pub type R = crate::R<PfrSpec>;
#[doc = "Register `PFR[%s]` writer"]
pub type W = crate::W<PfrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Processor Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfrSpec;
impl crate::RegisterSpec for PfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr::R`](R) reader structure"]
impl crate::Readable for PfrSpec {}
#[doc = "`write(|w| ..)` method takes [`pfr::W`](W) writer structure"]
impl crate::Writable for PfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR[%s]
to value 0"]
impl crate::Resettable for PfrSpec {
    const RESET_VALUE: u32 = 0;
}
