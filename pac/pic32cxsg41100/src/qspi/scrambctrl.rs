#[doc = "Register `SCRAMBCTRL` reader"]
pub type R = crate::R<ScrambctrlSpec>;
#[doc = "Register `SCRAMBCTRL` writer"]
pub type W = crate::W<ScrambctrlSpec>;
#[doc = "Field `ENABLE` reader - Scrambling/Unscrambling Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Scrambling/Unscrambling Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANDOMDIS` reader - Scrambling/Unscrambling Random Value Disable"]
pub type RandomdisR = crate::BitReader;
#[doc = "Field `RANDOMDIS` writer - Scrambling/Unscrambling Random Value Disable"]
pub type RandomdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn randomdis(&self) -> RandomdisR {
        RandomdisR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ScrambctrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    #[must_use]
    pub fn randomdis(&mut self) -> RandomdisW<ScrambctrlSpec> {
        RandomdisW::new(self, 1)
    }
}
#[doc = "Scrambling Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`scrambctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scrambctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrambctrlSpec;
impl crate::RegisterSpec for ScrambctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scrambctrl::R`](R) reader structure"]
impl crate::Readable for ScrambctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`scrambctrl::W`](W) writer structure"]
impl crate::Writable for ScrambctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRAMBCTRL to value 0"]
impl crate::Resettable for ScrambctrlSpec {
    const RESET_VALUE: u32 = 0;
}
