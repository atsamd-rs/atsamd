#[doc = "Register `RXERRCNT` reader"]
pub type R = crate::R<RxerrcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "USART_INT Receive Error Count\n\nYou can [`read`](crate::Reg::read) this register and get [`rxerrcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxerrcntSpec;
impl crate::RegisterSpec for RxerrcntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxerrcnt::R`](R) reader structure"]
impl crate::Readable for RxerrcntSpec {}
#[doc = "`reset()` method sets RXERRCNT to value 0"]
impl crate::Resettable for RxerrcntSpec {
    const RESET_VALUE: u8 = 0;
}
