#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `XOSCRDY0`"]
pub type XOSCRDY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCRDY1`"]
pub type XOSCRDY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCFAIL0`"]
pub type XOSCFAIL0_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCFAIL1`"]
pub type XOSCFAIL1_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCCKSW0`"]
pub type XOSCCKSW0_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCCKSW1`"]
pub type XOSCCKSW1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLRDY`"]
pub type DFLLRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLOOB`"]
pub type DFLLOOB_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLLCKF`"]
pub type DFLLLCKF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLLCKC`"]
pub type DFLLLCKC_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLRCS`"]
pub type DFLLRCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLL0LCKR`"]
pub type DPLL0LCKR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLL0LCKF`"]
pub type DPLL0LCKF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLL0TO`"]
pub type DPLL0TO_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLL0LDRTO`"]
pub type DPLL0LDRTO_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLL1LCKR`"]
pub type DPLL1LCKR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLL1LCKF`"]
pub type DPLL1LCKF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLL1TO`"]
pub type DPLL1TO_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLL1LDRTO`"]
pub type DPLL1LDRTO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - XOSC 0 Ready"]
    #[inline(always)]
    pub fn xoscrdy0(&self) -> XOSCRDY0_R {
        XOSCRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC 1 Ready"]
    #[inline(always)]
    pub fn xoscrdy1(&self) -> XOSCRDY1_R {
        XOSCRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail0(&self) -> XOSCFAIL0_R {
        XOSCFAIL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail1(&self) -> XOSCFAIL1_R {
        XOSCFAIL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XOSC 0 Clock Switch"]
    #[inline(always)]
    pub fn xosccksw0(&self) -> XOSCCKSW0_R {
        XOSCCKSW0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XOSC 1 Clock Switch"]
    #[inline(always)]
    pub fn xosccksw1(&self) -> XOSCCKSW1_R {
        XOSCCKSW1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise"]
    #[inline(always)]
    pub fn dpll0lckr(&self) -> DPLL0LCKR_R {
        DPLL0LCKR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall"]
    #[inline(always)]
    pub fn dpll0lckf(&self) -> DPLL0LCKF_R {
        DPLL0LCKF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DPLL0 Timeout"]
    #[inline(always)]
    pub fn dpll0to(&self) -> DPLL0TO_R {
        DPLL0TO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll0ldrto(&self) -> DPLL0LDRTO_R {
        DPLL0LDRTO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise"]
    #[inline(always)]
    pub fn dpll1lckr(&self) -> DPLL1LCKR_R {
        DPLL1LCKR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall"]
    #[inline(always)]
    pub fn dpll1lckf(&self) -> DPLL1LCKF_R {
        DPLL1LCKF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DPLL1 Timeout"]
    #[inline(always)]
    pub fn dpll1to(&self) -> DPLL1TO_R {
        DPLL1TO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll1ldrto(&self) -> DPLL1LDRTO_R {
        DPLL1LDRTO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
