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
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready"]
pub type XOSC32KRDY_R = crate::BitReader<bool>;
#[doc = "Field `XOSC32KFAIL` reader - XOSC32K Clock Failure Detector"]
pub type XOSC32KFAIL_R = crate::BitReader<bool>;
#[doc = "Field `XOSC32KSW` reader - XOSC32K Clock switch"]
pub type XOSC32KSW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector"]
    #[inline(always)]
    pub fn xosc32kfail(&self) -> XOSC32KFAIL_R {
        XOSC32KFAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XOSC32K Clock switch"]
    #[inline(always)]
    pub fn xosc32ksw(&self) -> XOSC32KSW_R {
        XOSC32KSW_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Power and Clocks Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
