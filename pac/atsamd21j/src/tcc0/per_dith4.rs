#[doc = "Register `PER_DITH4` reader"]
pub type R = crate::R<PerDith4Spec>;
#[doc = "Register `PER_DITH4` writer"]
pub type W = crate::W<PerDith4Spec>;
#[doc = "Field `DITHERCY` reader - Dithering Cycle Number"]
pub type DithercyR = crate::FieldReader;
#[doc = "Field `DITHERCY` writer - Dithering Cycle Number"]
pub type DithercyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PER` reader - Period Value"]
pub type PerR = crate::FieldReader<u32>;
#[doc = "Field `PER` writer - Period Value"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&self) -> DithercyR {
        DithercyR::new((self.bits & 0x0f) as u8)
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
    pub fn dithercy(&mut self) -> DithercyW<PerDith4Spec> {
        DithercyW::new(self, 0)
    }
    #[doc = "Bits 4:23 - Period Value"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<PerDith4Spec> {
        PerW::new(self, 4)
    }
}
#[doc = "Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerDith4Spec;
impl crate::RegisterSpec for PerDith4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_dith4::R`](R) reader structure"]
impl crate::Readable for PerDith4Spec {}
#[doc = "`write(|w| ..)` method takes [`per_dith4::W`](W) writer structure"]
impl crate::Writable for PerDith4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PER_DITH4 to value 0xffff_ffff"]
impl crate::Resettable for PerDith4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
