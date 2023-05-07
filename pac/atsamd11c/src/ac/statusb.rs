#[doc = "Register `STATUSB` reader"]
pub struct R(crate::R<STATUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READY0` reader - Comparator 0 Ready"]
pub type READY0_R = crate::BitReader<bool>;
#[doc = "Field `READY1` reader - Comparator 1 Ready"]
pub type READY1_R = crate::BitReader<bool>;
#[doc = "Field `SYNCBUSY` reader - Synchronization Busy"]
pub type SYNCBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Ready"]
    #[inline(always)]
    pub fn ready0(&self) -> READY0_R {
        READY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Ready"]
    #[inline(always)]
    pub fn ready1(&self) -> READY1_R {
        READY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SYNCBUSY_R {
        SYNCBUSY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusb](index.html) module"]
pub struct STATUSB_SPEC;
impl crate::RegisterSpec for STATUSB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [statusb::R](R) reader structure"]
impl crate::Readable for STATUSB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSB to value 0"]
impl crate::Resettable for STATUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
