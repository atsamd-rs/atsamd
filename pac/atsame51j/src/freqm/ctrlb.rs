#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `START` writer - Start Measurement"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start Measurement"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<CtrlbSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "Control B Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
