#[doc = "Register `DSEQDATA` writer"]
pub type W = crate::W<DseqdataSpec>;
#[doc = "Field `DATA` writer - DMA Sequential Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - DMA Sequential Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<DseqdataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "DMA Sequencial Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dseqdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DseqdataSpec;
impl crate::RegisterSpec for DseqdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dseqdata::W`](W) writer structure"]
impl crate::Writable for DseqdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSEQDATA to value 0"]
impl crate::Resettable for DseqdataSpec {}
