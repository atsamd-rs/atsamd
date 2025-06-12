#[doc = "Register `CNTRLDVR1` reader"]
pub type R = crate::R<Cntrldvr1Spec>;
#[doc = "Register `CNTRLDVR1` writer"]
pub type W = crate::W<Cntrldvr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ETM Free-running Counter Reload Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cntrldvr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntrldvr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cntrldvr1Spec;
impl crate::RegisterSpec for Cntrldvr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntrldvr1::R`](R) reader structure"]
impl crate::Readable for Cntrldvr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cntrldvr1::W`](W) writer structure"]
impl crate::Writable for Cntrldvr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTRLDVR1 to value 0"]
impl crate::Resettable for Cntrldvr1Spec {}
