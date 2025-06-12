#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CEN` writer - Cache Controller Enable"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Cache Controller Enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<CtrlSpec> {
        CenW::new(self, 0)
    }
}
#[doc = "Cache Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
