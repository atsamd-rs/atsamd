#[doc = "Register `DCRSR` writer"]
pub type W = crate::W<DcrsrSpec>;
#[doc = "Field `REGSEL` writer - "]
pub type RegselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REGWnR` writer - "]
pub type RegwnRW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn regsel(&mut self) -> RegselW<DcrsrSpec> {
        RegselW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn regwn_r(&mut self) -> RegwnRW<DcrsrSpec> {
        RegwnRW::new(self, 16)
    }
}
#[doc = "Debug Core Register Selector Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrsrSpec;
impl crate::RegisterSpec for DcrsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dcrsr::W`](W) writer structure"]
impl crate::Writable for DcrsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCRSR to value 0"]
impl crate::Resettable for DcrsrSpec {
    const RESET_VALUE: u32 = 0;
}
