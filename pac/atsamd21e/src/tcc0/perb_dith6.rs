#[doc = "Register `PERB_DITH6` reader"]
pub type R = crate::R<PerbDith6Spec>;
#[doc = "Register `PERB_DITH6` writer"]
pub type W = crate::W<PerbDith6Spec>;
#[doc = "Field `DITHERCYB` reader - Dithering Buffer Cycle Number"]
pub type DithercybR = crate::FieldReader;
#[doc = "Field `DITHERCYB` writer - Dithering Buffer Cycle Number"]
pub type DithercybW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PERB` reader - Period Buffer Value"]
pub type PerbR = crate::FieldReader<u32>;
#[doc = "Field `PERB` writer - Period Buffer Value"]
pub type PerbW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DithercybR {
        DithercybR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&self) -> PerbR {
        PerbR::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&mut self) -> DithercybW<PerbDith6Spec> {
        DithercybW::new(self, 0)
    }
    #[doc = "Bits 6:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&mut self) -> PerbW<PerbDith6Spec> {
        PerbW::new(self, 6)
    }
}
#[doc = "Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perb_dith6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perb_dith6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerbDith6Spec;
impl crate::RegisterSpec for PerbDith6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perb_dith6::R`](R) reader structure"]
impl crate::Readable for PerbDith6Spec {}
#[doc = "`write(|w| ..)` method takes [`perb_dith6::W`](W) writer structure"]
impl crate::Writable for PerbDith6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERB_DITH6 to value 0xffff_ffff"]
impl crate::Resettable for PerbDith6Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
