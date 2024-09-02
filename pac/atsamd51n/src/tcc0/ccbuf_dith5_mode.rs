#[doc = "Register `CCBUF_DITH5_MODE[%s]` reader"]
pub type R = crate::R<CcbufDith5ModeSpec>;
#[doc = "Register `CCBUF_DITH5_MODE[%s]` writer"]
pub type W = crate::W<CcbufDith5ModeSpec>;
#[doc = "Field `DITHERBUF` reader - Dithering Buffer Cycle Number"]
pub type DitherbufR = crate::FieldReader;
#[doc = "Field `DITHERBUF` writer - Dithering Buffer Cycle Number"]
pub type DitherbufW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCBUF` reader - Channel Compare/Capture Buffer Value"]
pub type CcbufR = crate::FieldReader<u32>;
#[doc = "Field `CCBUF` writer - Channel Compare/Capture Buffer Value"]
pub type CcbufW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DitherbufR {
        DitherbufR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CcbufR {
        CcbufR::new((self.bits >> 5) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn ditherbuf(&mut self) -> DitherbufW<CcbufDith5ModeSpec> {
        DitherbufW::new(self, 0)
    }
    #[doc = "Bits 5:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccbuf(&mut self) -> CcbufW<CcbufDith5ModeSpec> {
        CcbufW::new(self, 5)
    }
}
#[doc = "Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf_dith5_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf_dith5_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcbufDith5ModeSpec;
impl crate::RegisterSpec for CcbufDith5ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccbuf_dith5_mode::R`](R) reader structure"]
impl crate::Readable for CcbufDith5ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`ccbuf_dith5_mode::W`](W) writer structure"]
impl crate::Writable for CcbufDith5ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCBUF_DITH5_MODE[%s]
to value 0"]
impl crate::Resettable for CcbufDith5ModeSpec {
    const RESET_VALUE: u32 = 0;
}
