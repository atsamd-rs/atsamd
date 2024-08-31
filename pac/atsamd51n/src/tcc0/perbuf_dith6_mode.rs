#[doc = "Register `PERBUF_DITH6_MODE` reader"]
pub type R = crate::R<PerbufDith6ModeSpec>;
#[doc = "Register `PERBUF_DITH6_MODE` writer"]
pub type W = crate::W<PerbufDith6ModeSpec>;
#[doc = "Field `DITHERBUF` reader - Dithering Buffer Cycle Number"]
pub type DitherbufR = crate::FieldReader;
#[doc = "Field `DITHERBUF` writer - Dithering Buffer Cycle Number"]
pub type DitherbufW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PERBUF` reader - Period Buffer Value"]
pub type PerbufR = crate::FieldReader<u32>;
#[doc = "Field `PERBUF` writer - Period Buffer Value"]
pub type PerbufW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DitherbufR {
        DitherbufR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&self) -> PerbufR {
        PerbufR::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn ditherbuf(&mut self) -> DitherbufW<PerbufDith6ModeSpec> {
        DitherbufW::new(self, 0)
    }
    #[doc = "Bits 6:23 - Period Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn perbuf(&mut self) -> PerbufW<PerbufDith6ModeSpec> {
        PerbufW::new(self, 6)
    }
}
#[doc = "Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perbuf_dith6_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbuf_dith6_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerbufDith6ModeSpec;
impl crate::RegisterSpec for PerbufDith6ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perbuf_dith6_mode::R`](R) reader structure"]
impl crate::Readable for PerbufDith6ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`perbuf_dith6_mode::W`](W) writer structure"]
impl crate::Writable for PerbufDith6ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERBUF_DITH6_MODE to value 0xffff_ffff"]
impl crate::Resettable for PerbufDith6ModeSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
