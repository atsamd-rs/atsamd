#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `XOSCRDY0` reader - XOSC 0 Ready"]
pub type Xoscrdy0R = crate::BitReader;
#[doc = "Field `XOSCRDY1` reader - XOSC 1 Ready"]
pub type Xoscrdy1R = crate::BitReader;
#[doc = "Field `XOSCFAIL0` reader - XOSC 0 Clock Failure Detector"]
pub type Xoscfail0R = crate::BitReader;
#[doc = "Field `XOSCFAIL1` reader - XOSC 1 Clock Failure Detector"]
pub type Xoscfail1R = crate::BitReader;
#[doc = "Field `XOSCCKSW0` reader - XOSC 0 Clock Switch"]
pub type Xosccksw0R = crate::BitReader;
#[doc = "Field `XOSCCKSW1` reader - XOSC 1 Clock Switch"]
pub type Xosccksw1R = crate::BitReader;
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub type DfllrdyR = crate::BitReader;
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds"]
pub type DflloobR = crate::BitReader;
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine"]
pub type DflllckfR = crate::BitReader;
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse"]
pub type DflllckcR = crate::BitReader;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub type DfllrcsR = crate::BitReader;
#[doc = "Field `DPLL0LCKR` reader - DPLL0 Lock Rise"]
pub type Dpll0lckrR = crate::BitReader;
#[doc = "Field `DPLL0LCKF` reader - DPLL0 Lock Fall"]
pub type Dpll0lckfR = crate::BitReader;
#[doc = "Field `DPLL0TO` reader - DPLL0 Timeout"]
pub type Dpll0toR = crate::BitReader;
#[doc = "Field `DPLL0LDRTO` reader - DPLL0 Loop Divider Ratio Update Complete"]
pub type Dpll0ldrtoR = crate::BitReader;
#[doc = "Field `DPLL1LCKR` reader - DPLL1 Lock Rise"]
pub type Dpll1lckrR = crate::BitReader;
#[doc = "Field `DPLL1LCKF` reader - DPLL1 Lock Fall"]
pub type Dpll1lckfR = crate::BitReader;
#[doc = "Field `DPLL1TO` reader - DPLL1 Timeout"]
pub type Dpll1toR = crate::BitReader;
#[doc = "Field `DPLL1LDRTO` reader - DPLL1 Loop Divider Ratio Update Complete"]
pub type Dpll1ldrtoR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XOSC 0 Ready"]
    #[inline(always)]
    pub fn xoscrdy0(&self) -> Xoscrdy0R {
        Xoscrdy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC 1 Ready"]
    #[inline(always)]
    pub fn xoscrdy1(&self) -> Xoscrdy1R {
        Xoscrdy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail0(&self) -> Xoscfail0R {
        Xoscfail0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail1(&self) -> Xoscfail1R {
        Xoscfail1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XOSC 0 Clock Switch"]
    #[inline(always)]
    pub fn xosccksw0(&self) -> Xosccksw0R {
        Xosccksw0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XOSC 1 Clock Switch"]
    #[inline(always)]
    pub fn xosccksw1(&self) -> Xosccksw1R {
        Xosccksw1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DfllrdyR {
        DfllrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&self) -> DflloobR {
        DflloobR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DflllckfR {
        DflllckfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DflllckcR {
        DflllckcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DfllrcsR {
        DfllrcsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise"]
    #[inline(always)]
    pub fn dpll0lckr(&self) -> Dpll0lckrR {
        Dpll0lckrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall"]
    #[inline(always)]
    pub fn dpll0lckf(&self) -> Dpll0lckfR {
        Dpll0lckfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DPLL0 Timeout"]
    #[inline(always)]
    pub fn dpll0to(&self) -> Dpll0toR {
        Dpll0toR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll0ldrto(&self) -> Dpll0ldrtoR {
        Dpll0ldrtoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise"]
    #[inline(always)]
    pub fn dpll1lckr(&self) -> Dpll1lckrR {
        Dpll1lckrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall"]
    #[inline(always)]
    pub fn dpll1lckf(&self) -> Dpll1lckfR {
        Dpll1lckfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DPLL1 Timeout"]
    #[inline(always)]
    pub fn dpll1to(&self) -> Dpll1toR {
        Dpll1toR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll1ldrto(&self) -> Dpll1ldrtoR {
        Dpll1ldrtoR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
