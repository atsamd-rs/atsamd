#[doc = "Register `TUR` reader"]
pub type R = crate::R<TurSpec>;
#[doc = "Field `TXUNR` reader - Transmit Underruns"]
pub type TxunrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Transmit Underruns"]
    #[inline(always)]
    pub fn txunr(&self) -> TxunrR {
        TxunrR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Transmit Underruns Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TurSpec;
impl crate::RegisterSpec for TurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tur::R`](R) reader structure"]
impl crate::Readable for TurSpec {}
#[doc = "`reset()` method sets TUR to value 0"]
impl crate::Resettable for TurSpec {
    const RESET_VALUE: u32 = 0;
}
