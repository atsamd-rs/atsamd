#[doc = "Register `PERB_DITH5` reader"]
pub type R = crate::R<PerbDith5Spec>;
#[doc = "Register `PERB_DITH5` writer"]
pub type W = crate::W<PerbDith5Spec>;
#[doc = "Field `DITHERCYB` reader - Dithering Buffer Cycle Number"]
pub type DithercybR = crate::FieldReader;
#[doc = "Field `DITHERCYB` writer - Dithering Buffer Cycle Number"]
pub type DithercybW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PERB` reader - Period Buffer Value"]
pub type PerbR = crate::FieldReader<u32>;
#[doc = "Field `PERB` writer - Period Buffer Value"]
pub type PerbW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DithercybR {
        DithercybR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&self) -> PerbR {
        PerbR::new((self.bits >> 5) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dithercyb(&mut self) -> DithercybW<PerbDith5Spec> {
        DithercybW::new(self, 0)
    }
    #[doc = "Bits 5:23 - Period Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn perb(&mut self) -> PerbW<PerbDith5Spec> {
        PerbW::new(self, 5)
    }
}
#[doc = "Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perb_dith5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perb_dith5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerbDith5Spec;
impl crate::RegisterSpec for PerbDith5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perb_dith5::R`](R) reader structure"]
impl crate::Readable for PerbDith5Spec {}
#[doc = "`write(|w| ..)` method takes [`perb_dith5::W`](W) writer structure"]
impl crate::Writable for PerbDith5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERB_DITH5 to value 0xffff_ffff"]
impl crate::Resettable for PerbDith5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
