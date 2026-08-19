#[doc = "Register `PER_DITH5_MODE` reader"]
pub type R = crate::R<PerDith5ModeSpec>;
#[doc = "Register `PER_DITH5_MODE` writer"]
pub type W = crate::W<PerDith5ModeSpec>;
#[doc = "Field `DITHER` reader - Dithering Cycle Number"]
pub type DitherR = crate::FieldReader;
#[doc = "Field `DITHER` writer - Dithering Cycle Number"]
pub type DitherW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PER` reader - Period Value"]
pub type PerR = crate::FieldReader<u32>;
#[doc = "Field `PER` writer - Period Value"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:4 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dither(&self) -> DitherR {
        DitherR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:23 - Period Value"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new((self.bits >> 5) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dithering Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DitherW<PerDith5ModeSpec> {
        DitherW::new(self, 0)
    }
    #[doc = "Bits 5:23 - Period Value"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<PerDith5ModeSpec> {
        PerW::new(self, 5)
    }
}
#[doc = "Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith5_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith5_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerDith5ModeSpec;
impl crate::RegisterSpec for PerDith5ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_dith5_mode::R`](R) reader structure"]
impl crate::Readable for PerDith5ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`per_dith5_mode::W`](W) writer structure"]
impl crate::Writable for PerDith5ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PER_DITH5_MODE to value 0xffff_ffff"]
impl crate::Resettable for PerDith5ModeSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
