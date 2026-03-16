#[doc = "Register `PERBUF_DITH4_MODE` reader"]
pub type R = crate::R<PerbufDith4ModeSpec>;
#[doc = "Register `PERBUF_DITH4_MODE` writer"]
pub type W = crate::W<PerbufDith4ModeSpec>;
#[doc = "Field `DITHERBUF` reader - Dithering Buffer Cycle Number"]
pub type DitherbufR = crate::FieldReader;
#[doc = "Field `DITHERBUF` writer - Dithering Buffer Cycle Number"]
pub type DitherbufW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PERBUF` reader - Period Buffer Value"]
pub type PerbufR = crate::FieldReader<u32>;
#[doc = "Field `PERBUF` writer - Period Buffer Value"]
pub type PerbufW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DitherbufR {
        DitherbufR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&self) -> PerbufR {
        PerbufR::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn ditherbuf(&mut self) -> DitherbufW<PerbufDith4ModeSpec> {
        DitherbufW::new(self, 0)
    }
    #[doc = "Bits 4:23 - Period Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn perbuf(&mut self) -> PerbufW<PerbufDith4ModeSpec> {
        PerbufW::new(self, 4)
    }
}
#[doc = "Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perbuf_dith4_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbuf_dith4_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerbufDith4ModeSpec;
impl crate::RegisterSpec for PerbufDith4ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perbuf_dith4_mode::R`](R) reader structure"]
impl crate::Readable for PerbufDith4ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`perbuf_dith4_mode::W`](W) writer structure"]
impl crate::Writable for PerbufDith4ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERBUF_DITH4_MODE to value 0xffff_ffff"]
impl crate::Resettable for PerbufDith4ModeSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
