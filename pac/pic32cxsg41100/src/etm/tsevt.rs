#[doc = "Register `TSEVT` reader"]
pub type R = crate::R<TsevtSpec>;
#[doc = "Register `TSEVT` writer"]
pub type W = crate::W<TsevtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ETM TimeStamp Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsevt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsevt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsevtSpec;
impl crate::RegisterSpec for TsevtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsevt::R`](R) reader structure"]
impl crate::Readable for TsevtSpec {}
#[doc = "`write(|w| ..)` method takes [`tsevt::W`](W) writer structure"]
impl crate::Writable for TsevtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSEVT to value 0"]
impl crate::Resettable for TsevtSpec {
    const RESET_VALUE: u32 = 0;
}
