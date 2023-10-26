#[doc = "Register `RXERRCNT` reader"]
pub type R = crate::R<RXERRCNT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXERRCNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "USART_EXT Receive Error Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxerrcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXERRCNT_SPEC;
impl crate::RegisterSpec for RXERRCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxerrcnt::R`](R) reader structure"]
impl crate::Readable for RXERRCNT_SPEC {}
#[doc = "`reset()` method sets RXERRCNT to value 0"]
impl crate::Resettable for RXERRCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
