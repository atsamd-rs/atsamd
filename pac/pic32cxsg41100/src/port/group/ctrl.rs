#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SAMPLING` reader - Input Sampling Mode"]
pub type SamplingR = crate::FieldReader<u32>;
#[doc = "Field `SAMPLING` writer - Input Sampling Mode"]
pub type SamplingW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Input Sampling Mode"]
    #[inline(always)]
    pub fn sampling(&self) -> SamplingR {
        SamplingR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Sampling Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sampling(&mut self) -> SamplingW<CtrlSpec> {
        SamplingW::new(self, 0)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
