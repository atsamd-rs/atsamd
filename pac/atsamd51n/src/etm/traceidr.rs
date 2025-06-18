#[doc = "Register `TRACEIDR` reader"]
pub type R = crate::R<TraceidrSpec>;
#[doc = "Register `TRACEIDR` writer"]
pub type W = crate::W<TraceidrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ETM CoreSight Trace ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`traceidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TraceidrSpec;
impl crate::RegisterSpec for TraceidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`traceidr::R`](R) reader structure"]
impl crate::Readable for TraceidrSpec {}
#[doc = "`write(|w| ..)` method takes [`traceidr::W`](W) writer structure"]
impl crate::Writable for TraceidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRACEIDR to value 0"]
impl crate::Resettable for TraceidrSpec {}
