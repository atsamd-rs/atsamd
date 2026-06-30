#[doc = "Register `CCBUF_DITH4_MODE[%s]` reader"]
pub type R = crate::R<CcbufDith4ModeSpec>;
#[doc = "Register `CCBUF_DITH4_MODE[%s]` writer"]
pub type W = crate::W<CcbufDith4ModeSpec>;
#[doc = "Field `CCBUF` reader - Channel Compare/Capture Buffer Value"]
pub type CcbufR = crate::FieldReader;
#[doc = "Field `CCBUF` writer - Channel Compare/Capture Buffer Value"]
pub type CcbufW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DITHERBUF` reader - Dithering Buffer Cycle Number"]
pub type DitherbufR = crate::FieldReader<u32>;
#[doc = "Field `DITHERBUF` writer - Dithering Buffer Cycle Number"]
pub type DitherbufW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:3 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CcbufR {
        CcbufR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DitherbufR {
        DitherbufR::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccbuf(&mut self) -> CcbufW<CcbufDith4ModeSpec> {
        CcbufW::new(self, 0)
    }
    #[doc = "Bits 4:23 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn ditherbuf(&mut self) -> DitherbufW<CcbufDith4ModeSpec> {
        DitherbufW::new(self, 4)
    }
}
#[doc = "Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf_dith4_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf_dith4_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcbufDith4ModeSpec;
impl crate::RegisterSpec for CcbufDith4ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccbuf_dith4_mode::R`](R) reader structure"]
impl crate::Readable for CcbufDith4ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`ccbuf_dith4_mode::W`](W) writer structure"]
impl crate::Writable for CcbufDith4ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCBUF_DITH4_MODE[%s]
to value 0"]
impl crate::Resettable for CcbufDith4ModeSpec {
    const RESET_VALUE: u32 = 0;
}
