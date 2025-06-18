#[doc = "Register `DATABUF[%s]` writer"]
pub type W = crate::W<DatabufSpec>;
#[doc = "Field `DATABUF` writer - DAC0 Data Buffer"]
pub type DatabufW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - DAC0 Data Buffer"]
    #[inline(always)]
    pub fn databuf(&mut self) -> DatabufW<DatabufSpec> {
        DatabufW::new(self, 0)
    }
}
#[doc = "DAC n Data Buffer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatabufSpec;
impl crate::RegisterSpec for DatabufSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`databuf::W`](W) writer structure"]
impl crate::Writable for DatabufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATABUF[%s] to value 0"]
impl crate::Resettable for DatabufSpec {}
