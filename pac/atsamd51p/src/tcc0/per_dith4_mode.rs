#[doc = "Register `PER_DITH4_MODE` reader"]
pub type R = crate::R<PerDith4ModeSpec>;
#[doc = "Register `PER_DITH4_MODE` writer"]
pub type W = crate::W<PerDith4ModeSpec>;
#[doc = "Field `DITHER` reader - Dithering Cycle Number"]
pub type DitherR = crate::FieldReader;
#[doc = "Field `DITHER` writer - Dithering Cycle Number"]
pub type DitherW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PER` reader - Period Value"]
pub type PerR = crate::FieldReader<u32>;
#[doc = "Field `PER` writer - Period Value"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dither(&self) -> DitherR {
        DitherR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Period Value"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DitherW<PerDith4ModeSpec> {
        DitherW::new(self, 0)
    }
    #[doc = "Bits 4:23 - Period Value"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<PerDith4ModeSpec> {
        PerW::new(self, 4)
    }
}
#[doc = "Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith4_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith4_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerDith4ModeSpec;
impl crate::RegisterSpec for PerDith4ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_dith4_mode::R`](R) reader structure"]
impl crate::Readable for PerDith4ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`per_dith4_mode::W`](W) writer structure"]
impl crate::Writable for PerDith4ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PER_DITH4_MODE to value 0xffff_ffff"]
impl crate::Resettable for PerDith4ModeSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
