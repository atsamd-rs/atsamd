#[doc = "Register `DSEQSTAT` reader"]
pub type R = crate::R<DseqstatSpec>;
#[doc = "Field `INPUTCTRL` reader - Input Control"]
pub type InputctrlR = crate::BitReader;
#[doc = "Field `CTRLB` reader - Control B"]
pub type CtrlbR = crate::BitReader;
#[doc = "Field `REFCTRL` reader - Reference Control"]
pub type RefctrlR = crate::BitReader;
#[doc = "Field `AVGCTRL` reader - Average Control"]
pub type AvgctrlR = crate::BitReader;
#[doc = "Field `SAMPCTRL` reader - Sampling Time Control"]
pub type SampctrlR = crate::BitReader;
#[doc = "Field `WINLT` reader - Window Monitor Lower Threshold"]
pub type WinltR = crate::BitReader;
#[doc = "Field `WINUT` reader - Window Monitor Upper Threshold"]
pub type WinutR = crate::BitReader;
#[doc = "Field `GAINCORR` reader - Gain Correction"]
pub type GaincorrR = crate::BitReader;
#[doc = "Field `OFFSETCORR` reader - Offset Correction"]
pub type OffsetcorrR = crate::BitReader;
#[doc = "Field `BUSY` reader - DMA Sequencing Busy"]
pub type BusyR = crate::BitReader;
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
    #[doc = "Bit 31 - DMA Sequencing Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DMA Sequencial Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dseqstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DseqstatSpec;
impl crate::RegisterSpec for DseqstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dseqstat::R`](R) reader structure"]
impl crate::Readable for DseqstatSpec {}
#[doc = "`reset()` method sets DSEQSTAT to value 0"]
impl crate::Resettable for DseqstatSpec {}
