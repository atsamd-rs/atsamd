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
#[doc = "Field `SWRST` reader - Software Reset Synchronization Busy"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - Enable Synchronization Busy"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `WINCTRL` reader - WINCTRL Synchronization Busy"]
pub type WINCTRL_R = crate::BitReader<bool>;
#[doc = "Field `COMPCTRL0` reader - COMPCTRL 0 Synchronization Busy"]
pub type COMPCTRL0_R = crate::BitReader<bool>;
#[doc = "Field `COMPCTRL1` reader - COMPCTRL 1 Synchronization Busy"]
pub type COMPCTRL1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WINCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn winctrl(&self) -> WINCTRL_R {
        WINCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COMPCTRL 0 Synchronization Busy"]
    #[inline(always)]
    pub fn compctrl0(&self) -> COMPCTRL0_R {
        COMPCTRL0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - COMPCTRL 1 Synchronization Busy"]
    #[inline(always)]
    pub fn compctrl1(&self) -> COMPCTRL1_R {
        COMPCTRL1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
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
