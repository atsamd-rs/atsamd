#[doc = "Register `CCBUF_DITH6_MODE[%s]` reader"]
pub type R = crate::R<CcbufDith6ModeSpec>;
#[doc = "Register `CCBUF_DITH6_MODE[%s]` writer"]
pub type W = crate::W<CcbufDith6ModeSpec>;
#[doc = "Field `DITHERBUF` reader - Dithering Buffer Cycle Number"]
pub type DitherbufR = crate::FieldReader;
#[doc = "Field `DITHERBUF` writer - Dithering Buffer Cycle Number"]
pub type DitherbufW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CCBUF` reader - Channel Compare/Capture Buffer Value"]
pub type CcbufR = crate::FieldReader<u32>;
#[doc = "Field `CCBUF` writer - Channel Compare/Capture Buffer Value"]
pub type CcbufW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DitherbufR {
        DitherbufR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CcbufR {
        CcbufR::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn ditherbuf(&mut self) -> DitherbufW<CcbufDith6ModeSpec> {
        DitherbufW::new(self, 0)
    }
    #[doc = "Bits 6:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccbuf(&mut self) -> CcbufW<CcbufDith6ModeSpec> {
        CcbufW::new(self, 6)
    }
}
#[doc = "Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf_dith6_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf_dith6_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcbufDith6ModeSpec;
impl crate::RegisterSpec for CcbufDith6ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccbuf_dith6_mode::R`](R) reader structure"]
impl crate::Readable for CcbufDith6ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`ccbuf_dith6_mode::W`](W) writer structure"]
impl crate::Writable for CcbufDith6ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCBUF_DITH6_MODE[%s]
to value 0"]
impl crate::Resettable for CcbufDith6ModeSpec {
    const RESET_VALUE: u32 = 0;
}
