#[doc = "Register `BAUD_USARTFP_MODE` reader"]
pub type R = crate::R<BaudUsartfpModeSpec>;
#[doc = "Register `BAUD_USARTFP_MODE` writer"]
pub type W = crate::W<BaudUsartfpModeSpec>;
#[doc = "Field `BAUD` reader - Baud Rate Value"]
pub type BaudR = crate::FieldReader<u16>;
#[doc = "Field `BAUD` writer - Baud Rate Value"]
pub type BaudW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BaudR {
        BaudR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate Value"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BaudW<BaudUsartfpModeSpec> {
        BaudW::new(self, 0)
    }
}
#[doc = "USART_EXT Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud_usartfp_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud_usartfp_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudUsartfpModeSpec;
impl crate::RegisterSpec for BaudUsartfpModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`baud_usartfp_mode::R`](R) reader structure"]
impl crate::Readable for BaudUsartfpModeSpec {}
#[doc = "`write(|w| ..)` method takes [`baud_usartfp_mode::W`](W) writer structure"]
impl crate::Writable for BaudUsartfpModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BAUD_USARTFP_MODE to value 0"]
impl crate::Resettable for BaudUsartfpModeSpec {
    const RESET_VALUE: u16 = 0;
}
