#[doc = "Register `TSSN` reader"]
pub type R = crate::R<TssnSpec>;
#[doc = "Register `TSSN` writer"]
pub type W = crate::W<TssnSpec>;
#[doc = "Field `VTN` reader - Value Timer Nanoseconds Register Capture"]
pub type VtnR = crate::FieldReader<u32>;
#[doc = "Field `VTN` writer - Value Timer Nanoseconds Register Capture"]
pub type VtnW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Value Timer Nanoseconds Register Capture"]
    #[inline(always)]
    pub fn vtn(&self) -> VtnR {
        VtnR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Value Timer Nanoseconds Register Capture"]
    #[inline(always)]
    #[must_use]
    pub fn vtn(&mut self) -> VtnW<TssnSpec> {
        VtnW::new(self, 0)
    }
}
#[doc = "1588 Timer Sync Strobe Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tssn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tssn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TssnSpec;
impl crate::RegisterSpec for TssnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tssn::R`](R) reader structure"]
impl crate::Readable for TssnSpec {}
#[doc = "`write(|w| ..)` method takes [`tssn::W`](W) writer structure"]
impl crate::Writable for TssnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSSN to value 0"]
impl crate::Resettable for TssnSpec {
    const RESET_VALUE: u32 = 0;
}
