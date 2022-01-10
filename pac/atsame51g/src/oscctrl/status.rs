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
pub struct XOSCRDY0_R(crate::FieldReader<bool, bool>);
impl XOSCRDY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCRDY1` reader - XOSC 1 Ready"]
pub struct XOSCRDY1_R(crate::FieldReader<bool, bool>);
impl XOSCRDY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCFAIL0` reader - XOSC 0 Clock Failure Detector"]
pub struct XOSCFAIL0_R(crate::FieldReader<bool, bool>);
impl XOSCFAIL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCFAIL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCFAIL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCFAIL1` reader - XOSC 1 Clock Failure Detector"]
pub struct XOSCFAIL1_R(crate::FieldReader<bool, bool>);
impl XOSCFAIL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCFAIL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCFAIL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCCKSW0` reader - XOSC 0 Clock Switch"]
pub struct XOSCCKSW0_R(crate::FieldReader<bool, bool>);
impl XOSCCKSW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCCKSW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCCKSW0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCCKSW1` reader - XOSC 1 Clock Switch"]
pub struct XOSCCKSW1_R(crate::FieldReader<bool, bool>);
impl XOSCCKSW1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCCKSW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCCKSW1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub struct DFLLRDY_R(crate::FieldReader<bool, bool>);
impl DFLLRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds"]
pub struct DFLLOOB_R(crate::FieldReader<bool, bool>);
impl DFLLOOB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLOOB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLOOB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine"]
pub struct DFLLLCKF_R(crate::FieldReader<bool, bool>);
impl DFLLLCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLLCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLLCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse"]
pub struct DFLLLCKC_R(crate::FieldReader<bool, bool>);
impl DFLLLCKC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLLCKC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLLCKC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub struct DFLLRCS_R(crate::FieldReader<bool, bool>);
impl DFLLRCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLRCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLRCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL0LCKR` reader - DPLL0 Lock Rise"]
pub struct DPLL0LCKR_R(crate::FieldReader<bool, bool>);
impl DPLL0LCKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL0LCKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL0LCKR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL0LCKF` reader - DPLL0 Lock Fall"]
pub struct DPLL0LCKF_R(crate::FieldReader<bool, bool>);
impl DPLL0LCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL0LCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL0LCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL0TO` reader - DPLL0 Timeout"]
pub struct DPLL0TO_R(crate::FieldReader<bool, bool>);
impl DPLL0TO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL0TO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL0TO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL0LDRTO` reader - DPLL0 Loop Divider Ratio Update Complete"]
pub struct DPLL0LDRTO_R(crate::FieldReader<bool, bool>);
impl DPLL0LDRTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL0LDRTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL0LDRTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL1LCKR` reader - DPLL1 Lock Rise"]
pub struct DPLL1LCKR_R(crate::FieldReader<bool, bool>);
impl DPLL1LCKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL1LCKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL1LCKR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL1LCKF` reader - DPLL1 Lock Fall"]
pub struct DPLL1LCKF_R(crate::FieldReader<bool, bool>);
impl DPLL1LCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL1LCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL1LCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL1TO` reader - DPLL1 Timeout"]
pub struct DPLL1TO_R(crate::FieldReader<bool, bool>);
impl DPLL1TO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL1TO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL1TO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLL1LDRTO` reader - DPLL1 Loop Divider Ratio Update Complete"]
pub struct DPLL1LDRTO_R(crate::FieldReader<bool, bool>);
impl DPLL1LDRTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLL1LDRTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLL1LDRTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
