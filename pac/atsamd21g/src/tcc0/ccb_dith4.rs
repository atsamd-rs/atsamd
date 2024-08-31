#[doc = "Register `CCB%s_DITH4` reader"]
pub type R = crate::R<CcbDith4Spec>;
#[doc = "Register `CCB%s_DITH4` writer"]
pub type W = crate::W<CcbDith4Spec>;
#[doc = "Field `DITHERCYB` reader - Dithering Buffer Cycle Number"]
pub type DithercybR = crate::FieldReader;
#[doc = "Field `DITHERCYB` writer - Dithering Buffer Cycle Number"]
pub type DithercybW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCB` reader - Channel Compare/Capture Buffer Value"]
pub type CcbR = crate::FieldReader<u32>;
#[doc = "Field `CCB` writer - Channel Compare/Capture Buffer Value"]
pub type CcbW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DithercybR {
        DithercybR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&self) -> CcbR {
        CcbR::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dithercyb(&mut self) -> DithercybW<CcbDith4Spec> {
        DithercybW::new(self, 0)
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccb(&mut self) -> CcbW<CcbDith4Spec> {
        CcbW::new(self, 4)
    }
}
#[doc = "Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccb_dith4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccb_dith4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcbDith4Spec;
impl crate::RegisterSpec for CcbDith4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccb_dith4::R`](R) reader structure"]
impl crate::Readable for CcbDith4Spec {}
#[doc = "`write(|w| ..)` method takes [`ccb_dith4::W`](W) writer structure"]
impl crate::Writable for CcbDith4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCB%s_DITH4 to value 0"]
impl crate::Resettable for CcbDith4Spec {
    const RESET_VALUE: u32 = 0;
}
