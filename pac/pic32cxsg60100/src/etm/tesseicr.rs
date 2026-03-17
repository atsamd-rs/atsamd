#[doc = "Register `TESSEICR` reader"]
pub type R = crate::R<TesseicrSpec>;
#[doc = "Register `TESSEICR` writer"]
pub type W = crate::W<TesseicrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ETM TraceEnable Start/Stop EmbeddedICE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tesseicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tesseicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TesseicrSpec;
impl crate::RegisterSpec for TesseicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tesseicr::R`](R) reader structure"]
impl crate::Readable for TesseicrSpec {}
#[doc = "`write(|w| ..)` method takes [`tesseicr::W`](W) writer structure"]
impl crate::Writable for TesseicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESSEICR to value 0"]
impl crate::Resettable for TesseicrSpec {
    const RESET_VALUE: u32 = 0;
}
