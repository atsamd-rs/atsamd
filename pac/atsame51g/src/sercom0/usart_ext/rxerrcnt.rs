#[doc = "Register `RXERRCNT` reader"]
pub type R = crate::R<RxerrcntSpec>;
#[doc = "Field `RXERRCNT` reader - Receive Error Count"]
pub type RxerrcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive Error Count"]
    #[inline(always)]
    pub fn rxerrcnt(&self) -> RxerrcntR {
        RxerrcntR::new(self.bits)
    }
}
#[doc = "USART_EXT Receive Error Count\n\nYou can [`read`](crate::Reg::read) this register and get [`rxerrcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxerrcntSpec;
impl crate::RegisterSpec for RxerrcntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxerrcnt::R`](R) reader structure"]
impl crate::Readable for RxerrcntSpec {}
#[doc = "`reset()` method sets RXERRCNT to value 0"]
impl crate::Resettable for RxerrcntSpec {}
