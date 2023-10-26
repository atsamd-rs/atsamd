#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `SWRST` reader - Software Reset Synchronization Busy"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `ENABLE` reader - SERCOM Enable Synchronization Busy"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `SYSOP` reader - System Operation Synchronization Busy"]
pub type SYSOP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System Operation Synchronization Busy"]
    #[inline(always)]
    pub fn sysop(&self) -> SYSOP_R {
        SYSOP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "I2CM Syncbusy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
