#[doc = "Register `DFLLCTRL` reader"]
pub type R = crate::R<DfllctrlSpec>;
#[doc = "Register `DFLLCTRL` writer"]
pub type W = crate::W<DfllctrlSpec>;
#[doc = "Field `ENABLE` reader - DFLL Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - DFLL Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Operating Mode Selection"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - Operating Mode Selection"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STABLE` reader - Stable DFLL Frequency"]
pub type StableR = crate::BitReader;
#[doc = "Field `STABLE` writer - Stable DFLL Frequency"]
pub type StableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLAW` reader - Lose Lock After Wake"]
pub type LlawR = crate::BitReader;
#[doc = "Field `LLAW` writer - Lose Lock After Wake"]
pub type LlawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCRM` reader - USB Clock Recovery Mode"]
pub type UsbcrmR = crate::BitReader;
#[doc = "Field `USBCRM` writer - USB Clock Recovery Mode"]
pub type UsbcrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDIS` reader - Chill Cycle Disable"]
pub type CcdisR = crate::BitReader;
#[doc = "Field `CCDIS` writer - Chill Cycle Disable"]
pub type CcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QLDIS` reader - Quick Lock Disable"]
pub type QldisR = crate::BitReader;
#[doc = "Field `QLDIS` writer - Quick Lock Disable"]
pub type QldisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPLCKC` reader - Bypass Coarse Lock"]
pub type BplckcR = crate::BitReader;
#[doc = "Field `BPLCKC` writer - Bypass Coarse Lock"]
pub type BplckcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITLOCK` reader - Wait Lock"]
pub type WaitlockR = crate::BitReader;
#[doc = "Field `WAITLOCK` writer - Wait Lock"]
pub type WaitlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - DFLL Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operating Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&self) -> StableR {
        StableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&self) -> LlawR {
        LlawR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB Clock Recovery Mode"]
    #[inline(always)]
    pub fn usbcrm(&self) -> UsbcrmR {
        UsbcrmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&self) -> CcdisR {
        CcdisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&self) -> QldisR {
        QldisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bypass Coarse Lock"]
    #[inline(always)]
    pub fn bplckc(&self) -> BplckcR {
        BplckcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wait Lock"]
    #[inline(always)]
    pub fn waitlock(&self) -> WaitlockR {
        WaitlockR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DFLL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<DfllctrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Operating Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<DfllctrlSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 3 - Stable DFLL Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn stable(&mut self) -> StableW<DfllctrlSpec> {
        StableW::new(self, 3)
    }
    #[doc = "Bit 4 - Lose Lock After Wake"]
    #[inline(always)]
    #[must_use]
    pub fn llaw(&mut self) -> LlawW<DfllctrlSpec> {
        LlawW::new(self, 4)
    }
    #[doc = "Bit 5 - USB Clock Recovery Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbcrm(&mut self) -> UsbcrmW<DfllctrlSpec> {
        UsbcrmW::new(self, 5)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<DfllctrlSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> OndemandW<DfllctrlSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bit 8 - Chill Cycle Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ccdis(&mut self) -> CcdisW<DfllctrlSpec> {
        CcdisW::new(self, 8)
    }
    #[doc = "Bit 9 - Quick Lock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn qldis(&mut self) -> QldisW<DfllctrlSpec> {
        QldisW::new(self, 9)
    }
    #[doc = "Bit 10 - Bypass Coarse Lock"]
    #[inline(always)]
    #[must_use]
    pub fn bplckc(&mut self) -> BplckcW<DfllctrlSpec> {
        BplckcW::new(self, 10)
    }
    #[doc = "Bit 11 - Wait Lock"]
    #[inline(always)]
    #[must_use]
    pub fn waitlock(&mut self) -> WaitlockW<DfllctrlSpec> {
        WaitlockW::new(self, 11)
    }
}
#[doc = "DFLL48M Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfllctrlSpec;
impl crate::RegisterSpec for DfllctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dfllctrl::R`](R) reader structure"]
impl crate::Readable for DfllctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dfllctrl::W`](W) writer structure"]
impl crate::Writable for DfllctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DFLLCTRL to value 0x80"]
impl crate::Resettable for DfllctrlSpec {
    const RESET_VALUE: u16 = 0x80;
}
