#[doc = "Register `BAUD_FRACFP_MODE` reader"]
pub type R = crate::R<BaudFracfpModeSpec>;
#[doc = "Register `BAUD_FRACFP_MODE` writer"]
pub type W = crate::W<BaudFracfpModeSpec>;
#[doc = "Field `BAUD` reader - Baud Rate Value"]
pub type BaudR = crate::FieldReader<u16>;
#[doc = "Field `BAUD` writer - Baud Rate Value"]
pub type BaudW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `FP` reader - Fractional Part"]
pub type FpR = crate::FieldReader;
#[doc = "Field `FP` writer - Fractional Part"]
pub type FpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BaudR {
        BaudR::new(self.bits & 0x1fff)
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FpR {
        FpR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BaudW<BaudFracfpModeSpec> {
        BaudW::new(self, 0)
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    #[must_use]
    pub fn fp(&mut self) -> FpW<BaudFracfpModeSpec> {
        FpW::new(self, 13)
    }
}
#[doc = "USART_INT Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud_fracfp_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud_fracfp_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudFracfpModeSpec;
impl crate::RegisterSpec for BaudFracfpModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`baud_fracfp_mode::R`](R) reader structure"]
impl crate::Readable for BaudFracfpModeSpec {}
#[doc = "`write(|w| ..)` method takes [`baud_fracfp_mode::W`](W) writer structure"]
impl crate::Writable for BaudFracfpModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BAUD_FRACFP_MODE to value 0"]
impl crate::Resettable for BaudFracfpModeSpec {
    const RESET_VALUE: u16 = 0;
}
