#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STOP` reader - Stop"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE` reader - Slave"]
pub type SLAVE_R = crate::BitReader<bool>;
#[doc = "Field `SYNCBUSY` reader - Synchronization Busy"]
pub type SYNCBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SYNCBUSY_R {
        SYNCBUSY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x08"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
