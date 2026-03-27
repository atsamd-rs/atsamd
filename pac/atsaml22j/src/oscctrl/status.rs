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
#[doc = "Field `XOSCRDY` reader - XOSC Ready"]
pub type XOSCRDY_R = crate::BitReader<bool>;
#[doc = "Field `XOSCFAIL` reader - XOSC Clock Failure Detector"]
pub type XOSCFAIL_R = crate::BitReader<bool>;
#[doc = "Field `XOSCCKSW` reader - XOSC Clock Switch"]
pub type XOSCCKSW_R = crate::BitReader<bool>;
#[doc = "Field `OSC16MRDY` reader - OSC16M Ready"]
pub type OSC16MRDY_R = crate::BitReader<bool>;
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub type DFLLRDY_R = crate::BitReader<bool>;
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds"]
pub type DFLLOOB_R = crate::BitReader<bool>;
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine"]
pub type DFLLLCKF_R = crate::BitReader<bool>;
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse"]
pub type DFLLLCKC_R = crate::BitReader<bool>;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub type DFLLRCS_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise"]
pub type DPLLLCKR_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall"]
pub type DPLLLCKF_R = crate::BitReader<bool>;
#[doc = "Field `DPLLTO` reader - DPLL Timeout"]
pub type DPLLTO_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLDRTO` reader - DPLL Ratio Ready"]
pub type DPLLLDRTO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XOSCFAIL_R {
        XOSCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC Clock Switch"]
    #[inline(always)]
    pub fn xosccksw(&self) -> XOSCCKSW_R {
        XOSCCKSW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OSC16M Ready"]
    #[inline(always)]
    pub fn osc16mrdy(&self) -> OSC16MRDY_R {
        OSC16MRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DPLL Timeout"]
    #[inline(always)]
    pub fn dpllto(&self) -> DPLLTO_R {
        DPLLTO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DPLL Ratio Ready"]
    #[inline(always)]
    pub fn dpllldrto(&self) -> DPLLLDRTO_R {
        DPLLLDRTO_R::new(((self.bits >> 19) & 1) != 0)
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
