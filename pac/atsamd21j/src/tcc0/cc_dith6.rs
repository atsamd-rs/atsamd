#[doc = "Register `CC%s_DITH6` reader"]
pub type R = crate::R<CcDith6Spec>;
#[doc = "Register `CC%s_DITH6` writer"]
pub type W = crate::W<CcDith6Spec>;
#[doc = "Field `DITHERCY` reader - Dithering Cycle Number"]
pub type DithercyR = crate::FieldReader;
#[doc = "Field `DITHERCY` writer - Dithering Cycle Number"]
pub type DithercyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CC` reader - Channel Compare/Capture Value"]
pub type CcR = crate::FieldReader<u32>;
#[doc = "Field `CC` writer - Channel Compare/Capture Value"]
pub type CcW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:5 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&self) -> DithercyR {
        DithercyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Channel Compare/Capture Value"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&mut self) -> DithercyW<CcDith6Spec> {
        DithercyW::new(self, 0)
    }
    #[doc = "Bits 6:23 - Channel Compare/Capture Value"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<CcDith6Spec> {
        CcW::new(self, 6)
    }
}
#[doc = "Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_dith6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_dith6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcDith6Spec;
impl crate::RegisterSpec for CcDith6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_dith6::R`](R) reader structure"]
impl crate::Readable for CcDith6Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_dith6::W`](W) writer structure"]
impl crate::Writable for CcDith6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC%s_DITH6 to value 0"]
impl crate::Resettable for CcDith6Spec {}
