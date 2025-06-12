#[doc = "Register `TEEVR` reader"]
pub type R = crate::R<TeevrSpec>;
#[doc = "Register `TEEVR` writer"]
pub type W = crate::W<TeevrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ETM TraceEnable Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`teevr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`teevr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TeevrSpec;
impl crate::RegisterSpec for TeevrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`teevr::R`](R) reader structure"]
impl crate::Readable for TeevrSpec {}
#[doc = "`write(|w| ..)` method takes [`teevr::W`](W) writer structure"]
impl crate::Writable for TeevrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEEVR to value 0"]
impl crate::Resettable for TeevrSpec {}
