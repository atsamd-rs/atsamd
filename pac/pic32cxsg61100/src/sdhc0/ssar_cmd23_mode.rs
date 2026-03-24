#[doc = "Register `SSAR_CMD23_MODE` reader"]
pub type R = crate::R<SsarCmd23ModeSpec>;
#[doc = "Register `SSAR_CMD23_MODE` writer"]
pub type W = crate::W<SsarCmd23ModeSpec>;
#[doc = "Field `ARG2` reader - Argument 2"]
pub type Arg2R = crate::FieldReader<u32>;
#[doc = "Field `ARG2` writer - Argument 2"]
pub type Arg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Argument 2"]
    #[inline(always)]
    pub fn arg2(&self) -> Arg2R {
        Arg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Argument 2"]
    #[inline(always)]
    #[must_use]
    pub fn arg2(&mut self) -> Arg2W<SsarCmd23ModeSpec> {
        Arg2W::new(self, 0)
    }
}
#[doc = "SDMA System Address / Argument 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ssar_cmd23_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssar_cmd23_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsarCmd23ModeSpec;
impl crate::RegisterSpec for SsarCmd23ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssar_cmd23_mode::R`](R) reader structure"]
impl crate::Readable for SsarCmd23ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`ssar_cmd23_mode::W`](W) writer structure"]
impl crate::Writable for SsarCmd23ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSAR_CMD23_MODE to value 0"]
impl crate::Resettable for SsarCmd23ModeSpec {
    const RESET_VALUE: u32 = 0;
}
