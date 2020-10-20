#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `INPUTCTRL`"]
pub type INPUTCTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTRLB`"]
pub type CTRLB_R = crate::R<bool, bool>;
#[doc = "Reader of field `REFCTRL`"]
pub type REFCTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `AVGCTRL`"]
pub type AVGCTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SAMPCTRL`"]
pub type SAMPCTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `WINLT`"]
pub type WINLT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WINUT`"]
pub type WINUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `GAINCORR`"]
pub type GAINCORR_R = crate::R<bool, bool>;
#[doc = "Reader of field `OFFSETCORR`"]
pub type OFFSETCORR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWTRIG`"]
pub type SWTRIG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SWRST Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input Control Synchronization Busy"]
    #[inline(always)]
    pub fn inputctrl(&self) -> INPUTCTRL_R {
        INPUTCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control B Synchronization Busy"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reference Control Synchronization Busy"]
    #[inline(always)]
    pub fn refctrl(&self) -> REFCTRL_R {
        REFCTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Average Control Synchronization Busy"]
    #[inline(always)]
    pub fn avgctrl(&self) -> AVGCTRL_R {
        AVGCTRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sampling Time Control Synchronization Busy"]
    #[inline(always)]
    pub fn sampctrl(&self) -> SAMPCTRL_R {
        SAMPCTRL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Window Monitor Lower Threshold Synchronization Busy"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Window Monitor Upper Threshold Synchronization Busy"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Gain Correction Synchronization Busy"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Offset Correction Synchronization Busy"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software Trigger Synchronization Busy"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
