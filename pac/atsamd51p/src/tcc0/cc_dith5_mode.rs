#[doc = "Register `CC_DITH5_MODE[%s]` reader"]
pub type R = crate::R<CcDith5ModeSpec>;
#[doc = "Register `CC_DITH5_MODE[%s]` writer"]
pub type W = crate::W<CcDith5ModeSpec>;
#[doc = "Field `DITHER` reader - Dithering Cycle Number"]
pub type DitherR = crate::FieldReader;
#[doc = "Field `DITHER` writer - Dithering Cycle Number"]
pub type DitherW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CC` reader - Channel Compare/Capture Value"]
pub type CcR = crate::FieldReader<u32>;
#[doc = "Field `CC` writer - Channel Compare/Capture Value"]
pub type CcW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:4 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dither(&self) -> DitherR {
        DitherR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:23 - Channel Compare/Capture Value"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new((self.bits >> 5) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dithering Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DitherW<CcDith5ModeSpec> {
        DitherW::new(self, 0)
    }
    #[doc = "Bits 5:23 - Channel Compare/Capture Value"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CcW<CcDith5ModeSpec> {
        CcW::new(self, 5)
    }
}
#[doc = "Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_dith5_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_dith5_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcDith5ModeSpec;
impl crate::RegisterSpec for CcDith5ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_dith5_mode::R`](R) reader structure"]
impl crate::Readable for CcDith5ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`cc_dith5_mode::W`](W) writer structure"]
impl crate::Writable for CcDith5ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_DITH5_MODE[%s]
to value 0"]
impl crate::Resettable for CcDith5ModeSpec {
    const RESET_VALUE: u32 = 0;
}
