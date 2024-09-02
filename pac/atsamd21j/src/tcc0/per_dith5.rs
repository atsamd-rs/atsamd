#[doc = "Register `PER_DITH5` reader"]
pub type R = crate::R<PerDith5Spec>;
#[doc = "Register `PER_DITH5` writer"]
pub type W = crate::W<PerDith5Spec>;
#[doc = "Field `DITHERCY` reader - Dithering Cycle Number"]
pub type DithercyR = crate::FieldReader;
#[doc = "Field `DITHERCY` writer - Dithering Cycle Number"]
pub type DithercyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PER` reader - Period Value"]
pub type PerR = crate::FieldReader<u32>;
#[doc = "Field `PER` writer - Period Value"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:4 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&self) -> DithercyR {
        DithercyR::new((self.bits & 0x1f) as u8)
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
    pub fn dithercy(&mut self) -> DithercyW<PerDith5Spec> {
        DithercyW::new(self, 0)
    }
    #[doc = "Bits 5:23 - Period Value"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<PerDith5Spec> {
        PerW::new(self, 5)
    }
}
#[doc = "Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerDith5Spec;
impl crate::RegisterSpec for PerDith5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_dith5::R`](R) reader structure"]
impl crate::Readable for PerDith5Spec {}
#[doc = "`write(|w| ..)` method takes [`per_dith5::W`](W) writer structure"]
impl crate::Writable for PerDith5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PER_DITH5 to value 0xffff_ffff"]
impl crate::Resettable for PerDith5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
