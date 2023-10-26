#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready"]
pub type XOSC32KRDY_R = crate::BitReader;
#[doc = "Field `XOSC32KFAIL` reader - XOSC32K Clock Failure Detector"]
pub type XOSC32KFAIL_R = crate::BitReader;
#[doc = "Field `XOSC32KSW` reader - XOSC32K Clock switch"]
pub type XOSC32KSW_R = crate::BitReader;
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
#[doc = "Power and Clocks Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
