#[doc = "Register `RXPL` reader"]
pub type R = crate::R<RxplSpec>;
#[doc = "Register `RXPL` writer"]
pub type W = crate::W<RxplSpec>;
#[doc = "Field `RXPL` reader - Receive Pulse Length"]
pub type RxplR = crate::FieldReader;
#[doc = "Field `RXPL` writer - Receive Pulse Length"]
pub type RxplW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Pulse Length"]
    #[inline(always)]
    pub fn rxpl(&self) -> RxplR {
        RxplR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn rxpl(&mut self) -> RxplW<RxplSpec> {
        RxplW::new(self, 0)
    }
}
#[doc = "USART_INT Receive Pulse Length\n\nYou can [`read`](crate::Reg::read) this register and get [`rxpl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxpl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxplSpec;
impl crate::RegisterSpec for RxplSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxpl::R`](R) reader structure"]
impl crate::Readable for RxplSpec {}
#[doc = "`write(|w| ..)` method takes [`rxpl::W`](W) writer structure"]
impl crate::Writable for RxplSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RXPL to value 0"]
impl crate::Resettable for RxplSpec {
    const RESET_VALUE: u8 = 0;
}
