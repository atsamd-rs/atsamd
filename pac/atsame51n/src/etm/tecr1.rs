#[doc = "Register `TECR1` reader"]
pub type R = crate::R<Tecr1Spec>;
#[doc = "Register `TECR1` writer"]
pub type W = crate::W<Tecr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ETM TraceEnable Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tecr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tecr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tecr1Spec;
impl crate::RegisterSpec for Tecr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tecr1::R`](R) reader structure"]
impl crate::Readable for Tecr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tecr1::W`](W) writer structure"]
impl crate::Writable for Tecr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TECR1 to value 0"]
impl crate::Resettable for Tecr1Spec {}
