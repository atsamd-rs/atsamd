#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `START0` writer - Comparator 0 Start Comparison"]
pub type Start0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START1` writer - Comparator 1 Start Comparison"]
pub type Start1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Comparator 0 Start Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn start0(&mut self) -> Start0W<CtrlbSpec> {
        Start0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 Start Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn start1(&mut self) -> Start1W<CtrlbSpec> {
        Start1W::new(self, 1)
    }
}
#[doc = "Control B\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u8 = 0;
}
