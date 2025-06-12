#[doc = "Register `DSEQCTRL` reader"]
pub type R = crate::R<DseqctrlSpec>;
#[doc = "Register `DSEQCTRL` writer"]
pub type W = crate::W<DseqctrlSpec>;
#[doc = "Field `INPUTCTRL` reader - Input Control"]
pub type InputctrlR = crate::BitReader;
#[doc = "Field `INPUTCTRL` writer - Input Control"]
pub type InputctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLB` reader - Control B"]
pub type CtrlbR = crate::BitReader;
#[doc = "Field `CTRLB` writer - Control B"]
pub type CtrlbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFCTRL` reader - Reference Control"]
pub type RefctrlR = crate::BitReader;
#[doc = "Field `REFCTRL` writer - Reference Control"]
pub type RefctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVGCTRL` reader - Average Control"]
pub type AvgctrlR = crate::BitReader;
#[doc = "Field `AVGCTRL` writer - Average Control"]
pub type AvgctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPCTRL` reader - Sampling Time Control"]
pub type SampctrlR = crate::BitReader;
#[doc = "Field `SAMPCTRL` writer - Sampling Time Control"]
pub type SampctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINLT` reader - Window Monitor Lower Threshold"]
pub type WinltR = crate::BitReader;
#[doc = "Field `WINLT` writer - Window Monitor Lower Threshold"]
pub type WinltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINUT` reader - Window Monitor Upper Threshold"]
pub type WinutR = crate::BitReader;
#[doc = "Field `WINUT` writer - Window Monitor Upper Threshold"]
pub type WinutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAINCORR` reader - Gain Correction"]
pub type GaincorrR = crate::BitReader;
#[doc = "Field `GAINCORR` writer - Gain Correction"]
pub type GaincorrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSETCORR` reader - Offset Correction"]
pub type OffsetcorrR = crate::BitReader;
#[doc = "Field `OFFSETCORR` writer - Offset Correction"]
pub type OffsetcorrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSTART` reader - ADC Auto-Start Conversion"]
pub type AutostartR = crate::BitReader;
#[doc = "Field `AUTOSTART` writer - ADC Auto-Start Conversion"]
pub type AutostartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Input Control"]
    #[inline(always)]
    pub fn inputctrl(&self) -> InputctrlR {
        InputctrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control B"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CtrlbR {
        CtrlbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reference Control"]
    #[inline(always)]
    pub fn refctrl(&self) -> RefctrlR {
        RefctrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Average Control"]
    #[inline(always)]
    pub fn avgctrl(&self) -> AvgctrlR {
        AvgctrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sampling Time Control"]
    #[inline(always)]
    pub fn sampctrl(&self) -> SampctrlR {
        SampctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WinltR {
        WinltR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Window Monitor Upper Threshold"]
    #[inline(always)]
    pub fn winut(&self) -> WinutR {
        WinutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GaincorrR {
        GaincorrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OffsetcorrR {
        OffsetcorrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC Auto-Start Conversion"]
    #[inline(always)]
    pub fn autostart(&self) -> AutostartR {
        AutostartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Control"]
    #[inline(always)]
    pub fn inputctrl(&mut self) -> InputctrlW<DseqctrlSpec> {
        InputctrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Control B"]
    #[inline(always)]
    pub fn ctrlb(&mut self) -> CtrlbW<DseqctrlSpec> {
        CtrlbW::new(self, 1)
    }
    #[doc = "Bit 2 - Reference Control"]
    #[inline(always)]
    pub fn refctrl(&mut self) -> RefctrlW<DseqctrlSpec> {
        RefctrlW::new(self, 2)
    }
    #[doc = "Bit 3 - Average Control"]
    #[inline(always)]
    pub fn avgctrl(&mut self) -> AvgctrlW<DseqctrlSpec> {
        AvgctrlW::new(self, 3)
    }
    #[doc = "Bit 4 - Sampling Time Control"]
    #[inline(always)]
    pub fn sampctrl(&mut self) -> SampctrlW<DseqctrlSpec> {
        SampctrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Window Monitor Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&mut self) -> WinltW<DseqctrlSpec> {
        WinltW::new(self, 5)
    }
    #[doc = "Bit 6 - Window Monitor Upper Threshold"]
    #[inline(always)]
    pub fn winut(&mut self) -> WinutW<DseqctrlSpec> {
        WinutW::new(self, 6)
    }
    #[doc = "Bit 7 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&mut self) -> GaincorrW<DseqctrlSpec> {
        GaincorrW::new(self, 7)
    }
    #[doc = "Bit 8 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&mut self) -> OffsetcorrW<DseqctrlSpec> {
        OffsetcorrW::new(self, 8)
    }
    #[doc = "Bit 31 - ADC Auto-Start Conversion"]
    #[inline(always)]
    pub fn autostart(&mut self) -> AutostartW<DseqctrlSpec> {
        AutostartW::new(self, 31)
    }
}
#[doc = "DMA Sequential Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dseqctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dseqctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DseqctrlSpec;
impl crate::RegisterSpec for DseqctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dseqctrl::R`](R) reader structure"]
impl crate::Readable for DseqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dseqctrl::W`](W) writer structure"]
impl crate::Writable for DseqctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSEQCTRL to value 0"]
impl crate::Resettable for DseqctrlSpec {}
