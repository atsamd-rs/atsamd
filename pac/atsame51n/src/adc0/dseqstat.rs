#[doc = "Register `DSEQSTAT` reader"]
pub type R = crate::R<DSEQSTAT_SPEC>;
#[doc = "Field `INPUTCTRL` reader - Input Control"]
pub type INPUTCTRL_R = crate::BitReader;
#[doc = "Field `CTRLB` reader - Control B"]
pub type CTRLB_R = crate::BitReader;
#[doc = "Field `REFCTRL` reader - Reference Control"]
pub type REFCTRL_R = crate::BitReader;
#[doc = "Field `AVGCTRL` reader - Average Control"]
pub type AVGCTRL_R = crate::BitReader;
#[doc = "Field `SAMPCTRL` reader - Sampling Time Control"]
pub type SAMPCTRL_R = crate::BitReader;
#[doc = "Field `WINLT` reader - Window Monitor Lower Threshold"]
pub type WINLT_R = crate::BitReader;
#[doc = "Field `WINUT` reader - Window Monitor Upper Threshold"]
pub type WINUT_R = crate::BitReader;
#[doc = "Field `GAINCORR` reader - Gain Correction"]
pub type GAINCORR_R = crate::BitReader;
#[doc = "Field `OFFSETCORR` reader - Offset Correction"]
pub type OFFSETCORR_R = crate::BitReader;
#[doc = "Field `BUSY` reader - DMA Sequencing Busy"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input Control"]
    #[inline(always)]
    pub fn inputctrl(&self) -> INPUTCTRL_R {
        INPUTCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control B"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reference Control"]
    #[inline(always)]
    pub fn refctrl(&self) -> REFCTRL_R {
        REFCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Average Control"]
    #[inline(always)]
    pub fn avgctrl(&self) -> AVGCTRL_R {
        AVGCTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sampling Time Control"]
    #[inline(always)]
    pub fn sampctrl(&self) -> SAMPCTRL_R {
        SAMPCTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Window Monitor Upper Threshold"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Sequencing Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DMA Sequencial Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dseqstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSEQSTAT_SPEC;
impl crate::RegisterSpec for DSEQSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dseqstat::R`](R) reader structure"]
impl crate::Readable for DSEQSTAT_SPEC {}
#[doc = "`reset()` method sets DSEQSTAT to value 0"]
impl crate::Resettable for DSEQSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
