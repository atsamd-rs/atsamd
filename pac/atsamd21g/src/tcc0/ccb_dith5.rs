#[doc = "Register `CCB%s_DITH5` reader"]
pub type R = crate::R<CcbDith5Spec>;
#[doc = "Register `CCB%s_DITH5` writer"]
pub type W = crate::W<CcbDith5Spec>;
#[doc = "Field `DITHERCYB` reader - Dithering Buffer Cycle Number"]
pub type DithercybR = crate::FieldReader;
#[doc = "Field `DITHERCYB` writer - Dithering Buffer Cycle Number"]
pub type DithercybW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCB` reader - Channel Compare/Capture Buffer Value"]
pub type CcbR = crate::FieldReader<u32>;
#[doc = "Field `CCB` writer - Channel Compare/Capture Buffer Value"]
pub type CcbW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DithercybR {
        DithercybR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&self) -> CcbR {
        CcbR::new((self.bits >> 5) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&mut self) -> DithercybW<CcbDith5Spec> {
        DithercybW::new(self, 0)
    }
    #[doc = "Bits 5:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&mut self) -> CcbW<CcbDith5Spec> {
        CcbW::new(self, 5)
    }
}
#[doc = "Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccb_dith5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccb_dith5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcbDith5Spec;
impl crate::RegisterSpec for CcbDith5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccb_dith5::R`](R) reader structure"]
impl crate::Readable for CcbDith5Spec {}
#[doc = "`write(|w| ..)` method takes [`ccb_dith5::W`](W) writer structure"]
impl crate::Writable for CcbDith5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCB%s_DITH5 to value 0"]
impl crate::Resettable for CcbDith5Spec {}
