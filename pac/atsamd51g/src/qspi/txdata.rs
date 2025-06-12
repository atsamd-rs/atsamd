#[doc = "Register `TXDATA` writer"]
pub type W = crate::W<TxdataSpec>;
#[doc = "Field `DATA` writer - Transmit Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<TxdataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Transmit Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdataSpec;
impl crate::RegisterSpec for TxdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TxdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TxdataSpec {}
