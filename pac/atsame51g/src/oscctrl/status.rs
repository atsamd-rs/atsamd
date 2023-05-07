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
#[doc = "Field `XOSCRDY0` reader - XOSC 0 Ready"]
pub type XOSCRDY0_R = crate::BitReader<bool>;
#[doc = "Field `XOSCRDY1` reader - XOSC 1 Ready"]
pub type XOSCRDY1_R = crate::BitReader<bool>;
#[doc = "Field `XOSCFAIL0` reader - XOSC 0 Clock Failure Detector"]
pub type XOSCFAIL0_R = crate::BitReader<bool>;
#[doc = "Field `XOSCFAIL1` reader - XOSC 1 Clock Failure Detector"]
pub type XOSCFAIL1_R = crate::BitReader<bool>;
#[doc = "Field `XOSCCKSW0` reader - XOSC 0 Clock Switch"]
pub type XOSCCKSW0_R = crate::BitReader<bool>;
#[doc = "Field `XOSCCKSW1` reader - XOSC 1 Clock Switch"]
pub type XOSCCKSW1_R = crate::BitReader<bool>;
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
#[doc = "Field `DPLL0LCKR` reader - DPLL0 Lock Rise"]
pub type DPLL0LCKR_R = crate::BitReader<bool>;
#[doc = "Field `DPLL0LCKF` reader - DPLL0 Lock Fall"]
pub type DPLL0LCKF_R = crate::BitReader<bool>;
#[doc = "Field `DPLL0TO` reader - DPLL0 Timeout"]
pub type DPLL0TO_R = crate::BitReader<bool>;
#[doc = "Field `DPLL0LDRTO` reader - DPLL0 Loop Divider Ratio Update Complete"]
pub type DPLL0LDRTO_R = crate::BitReader<bool>;
#[doc = "Field `DPLL1LCKR` reader - DPLL1 Lock Rise"]
pub type DPLL1LCKR_R = crate::BitReader<bool>;
#[doc = "Field `DPLL1LCKF` reader - DPLL1 Lock Fall"]
pub type DPLL1LCKF_R = crate::BitReader<bool>;
#[doc = "Field `DPLL1TO` reader - DPLL1 Timeout"]
pub type DPLL1TO_R = crate::BitReader<bool>;
#[doc = "Field `DPLL1LDRTO` reader - DPLL1 Loop Divider Ratio Update Complete"]
pub type DPLL1LDRTO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - XOSC 0 Ready"]
    #[inline(always)]
    pub fn xoscrdy0(&self) -> XOSCRDY0_R {
        XOSCRDY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC 1 Ready"]
    #[inline(always)]
    pub fn xoscrdy1(&self) -> XOSCRDY1_R {
        XOSCRDY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail0(&self) -> XOSCFAIL0_R {
        XOSCFAIL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail1(&self) -> XOSCFAIL1_R {
        XOSCFAIL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XOSC 0 Clock Switch"]
    #[inline(always)]
    pub fn xosccksw0(&self) -> XOSCCKSW0_R {
        XOSCCKSW0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XOSC 1 Clock Switch"]
    #[inline(always)]
    pub fn xosccksw1(&self) -> XOSCCKSW1_R {
        XOSCCKSW1_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 16 - DPLL0 Lock Rise"]
    #[inline(always)]
    pub fn dpll0lckr(&self) -> DPLL0LCKR_R {
        DPLL0LCKR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall"]
    #[inline(always)]
    pub fn dpll0lckf(&self) -> DPLL0LCKF_R {
        DPLL0LCKF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DPLL0 Timeout"]
    #[inline(always)]
    pub fn dpll0to(&self) -> DPLL0TO_R {
        DPLL0TO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll0ldrto(&self) -> DPLL0LDRTO_R {
        DPLL0LDRTO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise"]
    #[inline(always)]
    pub fn dpll1lckr(&self) -> DPLL1LCKR_R {
        DPLL1LCKR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall"]
    #[inline(always)]
    pub fn dpll1lckf(&self) -> DPLL1LCKF_R {
        DPLL1LCKF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DPLL1 Timeout"]
    #[inline(always)]
    pub fn dpll1to(&self) -> DPLL1TO_R {
        DPLL1TO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll1ldrto(&self) -> DPLL1LDRTO_R {
        DPLL1LDRTO_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
