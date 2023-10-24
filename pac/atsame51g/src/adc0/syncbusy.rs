#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `SWRST` reader - SWRST Synchronization Busy"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `ENABLE` reader - ENABLE Synchronization Busy"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `INPUTCTRL` reader - Input Control Synchronization Busy"]
pub type INPUTCTRL_R = crate::BitReader;
#[doc = "Field `CTRLB` reader - Control B Synchronization Busy"]
pub type CTRLB_R = crate::BitReader;
#[doc = "Field `REFCTRL` reader - Reference Control Synchronization Busy"]
pub type REFCTRL_R = crate::BitReader;
#[doc = "Field `AVGCTRL` reader - Average Control Synchronization Busy"]
pub type AVGCTRL_R = crate::BitReader;
#[doc = "Field `SAMPCTRL` reader - Sampling Time Control Synchronization Busy"]
pub type SAMPCTRL_R = crate::BitReader;
#[doc = "Field `WINLT` reader - Window Monitor Lower Threshold Synchronization Busy"]
pub type WINLT_R = crate::BitReader;
#[doc = "Field `WINUT` reader - Window Monitor Upper Threshold Synchronization Busy"]
pub type WINUT_R = crate::BitReader;
#[doc = "Field `GAINCORR` reader - Gain Correction Synchronization Busy"]
pub type GAINCORR_R = crate::BitReader;
#[doc = "Field `OFFSETCORR` reader - Offset Correction Synchronization Busy"]
pub type OFFSETCORR_R = crate::BitReader;
#[doc = "Field `SWTRIG` reader - Software Trigger Synchronization Busy"]
pub type SWTRIG_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SWRST Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Control Synchronization Busy"]
    #[inline(always)]
    pub fn inputctrl(&self) -> INPUTCTRL_R {
        INPUTCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control B Synchronization Busy"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reference Control Synchronization Busy"]
    #[inline(always)]
    pub fn refctrl(&self) -> REFCTRL_R {
        REFCTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Average Control Synchronization Busy"]
    #[inline(always)]
    pub fn avgctrl(&self) -> AVGCTRL_R {
        AVGCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sampling Time Control Synchronization Busy"]
    #[inline(always)]
    pub fn sampctrl(&self) -> SAMPCTRL_R {
        SAMPCTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Window Monitor Lower Threshold Synchronization Busy"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Window Monitor Upper Threshold Synchronization Busy"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Gain Correction Synchronization Busy"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Offset Correction Synchronization Busy"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software Trigger Synchronization Busy"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
