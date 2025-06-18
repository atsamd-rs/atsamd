#[doc = "Register `DFLLCTRLB` reader"]
pub type R = crate::R<DfllctrlbSpec>;
#[doc = "Register `DFLLCTRLB` writer"]
pub type W = crate::W<DfllctrlbSpec>;
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
    #[doc = "Bit 0 - Operating Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&self) -> StableR {
        StableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&self) -> LlawR {
        LlawR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB Clock Recovery Mode"]
    #[inline(always)]
    pub fn usbcrm(&self) -> UsbcrmR {
        UsbcrmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&self) -> CcdisR {
        CcdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&self) -> QldisR {
        QldisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bypass Coarse Lock"]
    #[inline(always)]
    pub fn bplckc(&self) -> BplckcR {
        BplckcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wait Lock"]
    #[inline(always)]
    pub fn waitlock(&self) -> WaitlockR {
        WaitlockR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operating Mode Selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<DfllctrlbSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&mut self) -> StableW<DfllctrlbSpec> {
        StableW::new(self, 1)
    }
    #[doc = "Bit 2 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&mut self) -> LlawW<DfllctrlbSpec> {
        LlawW::new(self, 2)
    }
    #[doc = "Bit 3 - USB Clock Recovery Mode"]
    #[inline(always)]
    pub fn usbcrm(&mut self) -> UsbcrmW<DfllctrlbSpec> {
        UsbcrmW::new(self, 3)
    }
    #[doc = "Bit 4 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&mut self) -> CcdisW<DfllctrlbSpec> {
        CcdisW::new(self, 4)
    }
    #[doc = "Bit 5 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&mut self) -> QldisW<DfllctrlbSpec> {
        QldisW::new(self, 5)
    }
    #[doc = "Bit 6 - Bypass Coarse Lock"]
    #[inline(always)]
    pub fn bplckc(&mut self) -> BplckcW<DfllctrlbSpec> {
        BplckcW::new(self, 6)
    }
    #[doc = "Bit 7 - Wait Lock"]
    #[inline(always)]
    pub fn waitlock(&mut self) -> WaitlockW<DfllctrlbSpec> {
        WaitlockW::new(self, 7)
    }
}
#[doc = "DFLL48M Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfllctrlbSpec;
impl crate::RegisterSpec for DfllctrlbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dfllctrlb::R`](R) reader structure"]
impl crate::Readable for DfllctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`dfllctrlb::W`](W) writer structure"]
impl crate::Writable for DfllctrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFLLCTRLB to value 0"]
impl crate::Resettable for DfllctrlbSpec {}
