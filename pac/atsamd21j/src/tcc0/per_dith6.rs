#[doc = "Register `PER_DITH6` reader"]
pub type R = crate::R<PerDith6Spec>;
#[doc = "Register `PER_DITH6` writer"]
pub type W = crate::W<PerDith6Spec>;
#[doc = "Field `DITHERCY` reader - Dithering Cycle Number"]
pub type DithercyR = crate::FieldReader;
#[doc = "Field `DITHERCY` writer - Dithering Cycle Number"]
pub type DithercyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PER` reader - Period Value"]
pub type PerR = crate::FieldReader<u32>;
#[doc = "Field `PER` writer - Period Value"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:5 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&self) -> DithercyR {
        DithercyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Period Value"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dithering Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dithercy(&mut self) -> DithercyW<PerDith6Spec> {
        DithercyW::new(self, 0)
    }
    #[doc = "Bits 6:23 - Period Value"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<PerDith6Spec> {
        PerW::new(self, 6)
    }
}
#[doc = "Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerDith6Spec;
impl crate::RegisterSpec for PerDith6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_dith6::R`](R) reader structure"]
impl crate::Readable for PerDith6Spec {}
#[doc = "`write(|w| ..)` method takes [`per_dith6::W`](W) writer structure"]
impl crate::Writable for PerDith6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PER_DITH6 to value 0xffff_ffff"]
impl crate::Resettable for PerDith6Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
