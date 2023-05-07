#[doc = "Register `BKUPEXIT` reader"]
pub struct R(crate::R<BKUPEXIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKUPEXIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKUPEXIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKUPEXIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC` reader - Real Timer Counter Interrupt"]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `BBPS` reader - Battery Backup Power Switch"]
pub type BBPS_R = crate::BitReader<bool>;
#[doc = "Field `HIB` reader - Hibernate"]
pub type HIB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Real Timer Counter Interrupt"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Battery Backup Power Switch"]
    #[inline(always)]
    pub fn bbps(&self) -> BBPS_R {
        BBPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Hibernate"]
    #[inline(always)]
    pub fn hib(&self) -> HIB_R {
        HIB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Backup Exit Source\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkupexit](index.html) module"]
pub struct BKUPEXIT_SPEC;
impl crate::RegisterSpec for BKUPEXIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bkupexit::R](R) reader structure"]
impl crate::Readable for BKUPEXIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BKUPEXIT to value 0"]
impl crate::Resettable for BKUPEXIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
