#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSCRDY0` reader - XOSC 0 Ready Interrupt Enable"]
pub type XOSCRDY0_R = crate::BitReader<bool>;
#[doc = "Field `XOSCRDY0` writer - XOSC 0 Ready Interrupt Enable"]
pub type XOSCRDY0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `XOSCRDY1` reader - XOSC 1 Ready Interrupt Enable"]
pub type XOSCRDY1_R = crate::BitReader<bool>;
#[doc = "Field `XOSCRDY1` writer - XOSC 1 Ready Interrupt Enable"]
pub type XOSCRDY1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `XOSCFAIL0` reader - XOSC 0 Clock Failure Detector Interrupt Enable"]
pub type XOSCFAIL0_R = crate::BitReader<bool>;
#[doc = "Field `XOSCFAIL0` writer - XOSC 0 Clock Failure Detector Interrupt Enable"]
pub type XOSCFAIL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `XOSCFAIL1` reader - XOSC 1 Clock Failure Detector Interrupt Enable"]
pub type XOSCFAIL1_R = crate::BitReader<bool>;
#[doc = "Field `XOSCFAIL1` writer - XOSC 1 Clock Failure Detector Interrupt Enable"]
pub type XOSCFAIL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DFLLRDY` reader - DFLL Ready Interrupt Enable"]
pub type DFLLRDY_R = crate::BitReader<bool>;
#[doc = "Field `DFLLRDY` writer - DFLL Ready Interrupt Enable"]
pub type DFLLRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds Interrupt Enable"]
pub type DFLLOOB_R = crate::BitReader<bool>;
#[doc = "Field `DFLLOOB` writer - DFLL Out Of Bounds Interrupt Enable"]
pub type DFLLOOB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine Interrupt Enable"]
pub type DFLLLCKF_R = crate::BitReader<bool>;
#[doc = "Field `DFLLLCKF` writer - DFLL Lock Fine Interrupt Enable"]
pub type DFLLLCKF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse Interrupt Enable"]
pub type DFLLLCKC_R = crate::BitReader<bool>;
#[doc = "Field `DFLLLCKC` writer - DFLL Lock Coarse Interrupt Enable"]
pub type DFLLLCKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped Interrupt Enable"]
pub type DFLLRCS_R = crate::BitReader<bool>;
#[doc = "Field `DFLLRCS` writer - DFLL Reference Clock Stopped Interrupt Enable"]
pub type DFLLRCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLL0LCKR` reader - DPLL0 Lock Rise Interrupt Enable"]
pub type DPLL0LCKR_R = crate::BitReader<bool>;
#[doc = "Field `DPLL0LCKR` writer - DPLL0 Lock Rise Interrupt Enable"]
pub type DPLL0LCKR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLL0LCKF` reader - DPLL0 Lock Fall Interrupt Enable"]
pub type DPLL0LCKF_R = crate::BitReader<bool>;
#[doc = "Field `DPLL0LCKF` writer - DPLL0 Lock Fall Interrupt Enable"]
pub type DPLL0LCKF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLL0LTO` reader - DPLL0 Lock Timeout Interrupt Enable"]
pub type DPLL0LTO_R = crate::BitReader<bool>;
#[doc = "Field `DPLL0LTO` writer - DPLL0 Lock Timeout Interrupt Enable"]
pub type DPLL0LTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLL0LDRTO` reader - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
pub type DPLL0LDRTO_R = crate::BitReader<bool>;
#[doc = "Field `DPLL0LDRTO` writer - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
pub type DPLL0LDRTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLL1LCKR` reader - DPLL1 Lock Rise Interrupt Enable"]
pub type DPLL1LCKR_R = crate::BitReader<bool>;
#[doc = "Field `DPLL1LCKR` writer - DPLL1 Lock Rise Interrupt Enable"]
pub type DPLL1LCKR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLL1LCKF` reader - DPLL1 Lock Fall Interrupt Enable"]
pub type DPLL1LCKF_R = crate::BitReader<bool>;
#[doc = "Field `DPLL1LCKF` writer - DPLL1 Lock Fall Interrupt Enable"]
pub type DPLL1LCKF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLL1LTO` reader - DPLL1 Lock Timeout Interrupt Enable"]
pub type DPLL1LTO_R = crate::BitReader<bool>;
#[doc = "Field `DPLL1LTO` writer - DPLL1 Lock Timeout Interrupt Enable"]
pub type DPLL1LTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLL1LDRTO` reader - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
pub type DPLL1LDRTO_R = crate::BitReader<bool>;
#[doc = "Field `DPLL1LDRTO` writer - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
pub type DPLL1LDRTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - XOSC 0 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy0(&self) -> XOSCRDY0_R {
        XOSCRDY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC 1 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy1(&self) -> XOSCRDY1_R {
        XOSCRDY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail0(&self) -> XOSCFAIL0_R {
        XOSCFAIL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail1(&self) -> XOSCFAIL1_R {
        XOSCFAIL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckr(&self) -> DPLL0LCKR_R {
        DPLL0LCKR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckf(&self) -> DPLL0LCKF_R {
        DPLL0LCKF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lto(&self) -> DPLL0LTO_R {
        DPLL0LTO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0ldrto(&self) -> DPLL0LDRTO_R {
        DPLL0LDRTO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckr(&self) -> DPLL1LCKR_R {
        DPLL1LCKR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckf(&self) -> DPLL1LCKF_R {
        DPLL1LCKF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lto(&self) -> DPLL1LTO_R {
        DPLL1LTO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1ldrto(&self) -> DPLL1LDRTO_R {
        DPLL1LDRTO_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC 0 Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscrdy0(&mut self) -> XOSCRDY0_W<0> {
        XOSCRDY0_W::new(self)
    }
    #[doc = "Bit 1 - XOSC 1 Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscrdy1(&mut self) -> XOSCRDY1_W<1> {
        XOSCRDY1_W::new(self)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscfail0(&mut self) -> XOSCFAIL0_W<2> {
        XOSCFAIL0_W::new(self)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscfail1(&mut self) -> XOSCFAIL1_W<3> {
        XOSCFAIL1_W::new(self)
    }
    #[doc = "Bit 8 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrdy(&mut self) -> DFLLRDY_W<8> {
        DFLLRDY_W::new(self)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflloob(&mut self) -> DFLLOOB_W<9> {
        DFLLOOB_W::new(self)
    }
    #[doc = "Bit 10 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckf(&mut self) -> DFLLLCKF_W<10> {
        DFLLLCKF_W::new(self)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckc(&mut self) -> DFLLLCKC_W<11> {
        DFLLLCKC_W::new(self)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrcs(&mut self) -> DFLLRCS_W<12> {
        DFLLRCS_W::new(self)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0lckr(&mut self) -> DPLL0LCKR_W<16> {
        DPLL0LCKR_W::new(self)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0lckf(&mut self) -> DPLL0LCKF_W<17> {
        DPLL0LCKF_W::new(self)
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0lto(&mut self) -> DPLL0LTO_W<18> {
        DPLL0LTO_W::new(self)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0ldrto(&mut self) -> DPLL0LDRTO_W<19> {
        DPLL0LDRTO_W::new(self)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1lckr(&mut self) -> DPLL1LCKR_W<24> {
        DPLL1LCKR_W::new(self)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1lckf(&mut self) -> DPLL1LCKF_W<25> {
        DPLL1LCKF_W::new(self)
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1lto(&mut self) -> DPLL1LTO_W<26> {
        DPLL1LTO_W::new(self)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1ldrto(&mut self) -> DPLL1LDRTO_W<27> {
        DPLL1LDRTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
