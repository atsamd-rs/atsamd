#[doc = "Register `CCB%s_DITH6` reader"]
pub type R = crate::R<CcbDith6Spec>;
#[doc = "Register `CCB%s_DITH6` writer"]
pub type W = crate::W<CcbDith6Spec>;
#[doc = "Field `DITHERCYB` reader - Dithering Buffer Cycle Number"]
pub type DithercybR = crate::FieldReader;
#[doc = "Field `DITHERCYB` writer - Dithering Buffer Cycle Number"]
pub type DithercybW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CCB` reader - Channel Compare/Capture Buffer Value"]
pub type CcbR = crate::FieldReader<u32>;
#[doc = "Field `CCB` writer - Channel Compare/Capture Buffer Value"]
pub type CcbW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DithercybR {
        DithercybR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&self) -> CcbR {
        CcbR::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&mut self) -> DithercybW<CcbDith6Spec> {
        DithercybW::new(self, 0)
    }
    #[doc = "Bits 6:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&mut self) -> CcbW<CcbDith6Spec> {
        CcbW::new(self, 6)
    }
}
#[doc = "Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccb_dith6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccb_dith6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcbDith6Spec;
impl crate::RegisterSpec for CcbDith6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccb_dith6::R`](R) reader structure"]
impl crate::Readable for CcbDith6Spec {}
#[doc = "`write(|w| ..)` method takes [`ccb_dith6::W`](W) writer structure"]
impl crate::Writable for CcbDith6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCB%s_DITH6 to value 0"]
impl crate::Resettable for CcbDith6Spec {}
