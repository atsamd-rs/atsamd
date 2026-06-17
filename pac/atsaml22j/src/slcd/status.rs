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
#[doc = "Field `VLCDR` reader - VLCD Ready"]
pub type VLCDR_R = crate::BitReader<bool>;
#[doc = "Field `PRUN` reader - LCD Charge Pump is Running"]
pub type PRUN_R = crate::BitReader<bool>;
#[doc = "Field `VLCDS` reader - VLCD Status"]
pub type VLCDS_R = crate::BitReader<bool>;
#[doc = "Field `CMWRBUSY` reader - Character mapping write busy"]
pub type CMWRBUSY_R = crate::BitReader<bool>;
#[doc = "Field `ACMBUSY` reader - ACM state machine busy"]
pub type ACMBUSY_R = crate::BitReader<bool>;
#[doc = "Field `ABMBUSY` reader - ABM state machine busy"]
pub type ABMBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - VLCD Ready"]
    #[inline(always)]
    pub fn vlcdr(&self) -> VLCDR_R {
        VLCDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Charge Pump is Running"]
    #[inline(always)]
    pub fn prun(&self) -> PRUN_R {
        PRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VLCD Status"]
    #[inline(always)]
    pub fn vlcds(&self) -> VLCDS_R {
        VLCDS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Character mapping write busy"]
    #[inline(always)]
    pub fn cmwrbusy(&self) -> CMWRBUSY_R {
        CMWRBUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ACM state machine busy"]
    #[inline(always)]
    pub fn acmbusy(&self) -> ACMBUSY_R {
        ACMBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ABM state machine busy"]
    #[inline(always)]
    pub fn abmbusy(&self) -> ABMBUSY_R {
        ABMBUSY_R::new(((self.bits >> 5) & 1) != 0)
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
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
