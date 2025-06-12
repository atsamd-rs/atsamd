#[doc = "Register `DATA[%s]` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `DATA` writer - DAC0 Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - DAC0 Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<DataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "DAC n Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA[%s] to value 0"]
impl crate::Resettable for DataSpec {}
