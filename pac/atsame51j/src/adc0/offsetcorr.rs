#[doc = "Register `OFFSETCORR` reader"]
pub type R = crate::R<OffsetcorrSpec>;
#[doc = "Register `OFFSETCORR` writer"]
pub type W = crate::W<OffsetcorrSpec>;
#[doc = "Field `OFFSETCORR` reader - Offset Correction Value"]
pub type OffsetcorrR = crate::FieldReader<u16>;
#[doc = "Field `OFFSETCORR` writer - Offset Correction Value"]
pub type OffsetcorrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Offset Correction Value"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OffsetcorrR {
        OffsetcorrR::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - Offset Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn offsetcorr(&mut self) -> OffsetcorrW<OffsetcorrSpec> {
        OffsetcorrW::new(self, 0)
    }
}
#[doc = "Offset Correction\n\nYou can [`read`](crate::Reg::read) this register and get [`offsetcorr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offsetcorr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OffsetcorrSpec;
impl crate::RegisterSpec for OffsetcorrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`offsetcorr::R`](R) reader structure"]
impl crate::Readable for OffsetcorrSpec {}
#[doc = "`write(|w| ..)` method takes [`offsetcorr::W`](W) writer structure"]
impl crate::Writable for OffsetcorrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets OFFSETCORR to value 0"]
impl crate::Resettable for OffsetcorrSpec {
    const RESET_VALUE: u16 = 0;
}
