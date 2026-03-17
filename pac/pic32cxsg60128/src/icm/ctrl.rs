#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE` writer - ICM Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE` writer - ICM Disable Register"]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REHASH` writer - Recompute Internal Hash"]
pub type RehashW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RMDIS` writer - Region Monitoring Disable"]
pub type RmdisW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RMEN` writer - Region Monitoring Enable"]
pub type RmenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bit 0 - ICM Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - ICM Disable Register"]
    #[inline(always)]
    #[must_use]
    pub fn disable(&mut self) -> DisableW<CtrlSpec> {
        DisableW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlSpec> {
        SwrstW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Recompute Internal Hash"]
    #[inline(always)]
    #[must_use]
    pub fn rehash(&mut self) -> RehashW<CtrlSpec> {
        RehashW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Region Monitoring Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rmdis(&mut self) -> RmdisW<CtrlSpec> {
        RmdisW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Region Monitoring Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmen(&mut self) -> RmenW<CtrlSpec> {
        RmenW::new(self, 12)
    }
}
#[doc = "Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
