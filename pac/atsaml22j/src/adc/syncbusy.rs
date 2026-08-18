#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWRST` reader - SWRST Synchronization Busy"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - ENABLE Synchronization Busy"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `INPUTCTRL` reader - INPUTCTRL Synchronization Busy"]
pub type INPUTCTRL_R = crate::BitReader<bool>;
#[doc = "Field `CTRLC` reader - CTRLC Synchronization Busy"]
pub type CTRLC_R = crate::BitReader<bool>;
#[doc = "Field `AVGCTRL` reader - AVGCTRL Synchronization Busy"]
pub type AVGCTRL_R = crate::BitReader<bool>;
#[doc = "Field `SAMPCTRL` reader - SAMPCTRL Synchronization Busy"]
pub type SAMPCTRL_R = crate::BitReader<bool>;
#[doc = "Field `WINLT` reader - WINLT Synchronization Busy"]
pub type WINLT_R = crate::BitReader<bool>;
#[doc = "Field `WINUT` reader - WINUT Synchronization Busy"]
pub type WINUT_R = crate::BitReader<bool>;
#[doc = "Field `GAINCORR` reader - GAINCORR Synchronization Busy"]
pub type GAINCORR_R = crate::BitReader<bool>;
#[doc = "Field `OFFSETCORR` reader - OFFSETCTRL Synchronization Busy"]
pub type OFFSETCORR_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG` reader - SWTRG Synchronization Busy"]
pub type SWTRIG_R = crate::BitReader<bool>;
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
    #[doc = "Bit 2 - INPUTCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn inputctrl(&self) -> INPUTCTRL_R {
        INPUTCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTRLC Synchronization Busy"]
    #[inline(always)]
    pub fn ctrlc(&self) -> CTRLC_R {
        CTRLC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AVGCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn avgctrl(&self) -> AVGCTRL_R {
        AVGCTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAMPCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn sampctrl(&self) -> SAMPCTRL_R {
        SAMPCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WINLT Synchronization Busy"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WINUT Synchronization Busy"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GAINCORR Synchronization Busy"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OFFSETCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SWTRG Synchronization Busy"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
