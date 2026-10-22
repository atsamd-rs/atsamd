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
#[doc = "Field `SWRST` reader - swrst"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CTRLB` reader - CTRLB"]
pub type CTRLB_R = crate::BitReader<bool>;
#[doc = "Field `STATUS` reader - STATUS"]
pub type STATUS_R = crate::BitReader<bool>;
#[doc = "Field `COUNT` reader - Counter"]
pub type COUNT_R = crate::BitReader<bool>;
#[doc = "Field `PER` reader - Period"]
pub type PER_R = crate::BitReader<bool>;
#[doc = "Field `CC0` reader - Compare Channel 0"]
pub type CC0_R = crate::BitReader<bool>;
#[doc = "Field `CC1` reader - Compare Channel 1"]
pub type CC1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - swrst"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTRLB"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STATUS"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare Channel 0"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Channel 1"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Synchronization Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
