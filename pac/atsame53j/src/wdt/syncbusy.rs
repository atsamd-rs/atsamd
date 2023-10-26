#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `ENABLE` reader - Enable Synchronization Busy"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `WEN` reader - Window Enable Synchronization Busy"]
pub type WEN_R = crate::BitReader;
#[doc = "Field `ALWAYSON` reader - Always-On Synchronization Busy"]
pub type ALWAYSON_R = crate::BitReader;
#[doc = "Field `CLEAR` reader - Clear Synchronization Busy"]
pub type CLEAR_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Enable Synchronization Busy"]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Always-On Synchronization Busy"]
    #[inline(always)]
    pub fn alwayson(&self) -> ALWAYSON_R {
        ALWAYSON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear Synchronization Busy"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 4) & 1) != 0)
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
