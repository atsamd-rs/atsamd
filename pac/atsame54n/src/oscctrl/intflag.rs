#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<INTFLAG_SPEC>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<INTFLAG_SPEC>;
#[doc = "Field `XOSCRDY0` reader - XOSC 0 Ready"]
pub type XOSCRDY0_R = crate::BitReader;
#[doc = "Field `XOSCRDY0` writer - XOSC 0 Ready"]
pub type XOSCRDY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XOSCRDY1` reader - XOSC 1 Ready"]
pub type XOSCRDY1_R = crate::BitReader;
#[doc = "Field `XOSCRDY1` writer - XOSC 1 Ready"]
pub type XOSCRDY1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XOSCFAIL0` reader - XOSC 0 Clock Failure Detector"]
pub type XOSCFAIL0_R = crate::BitReader;
#[doc = "Field `XOSCFAIL0` writer - XOSC 0 Clock Failure Detector"]
pub type XOSCFAIL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XOSCFAIL1` reader - XOSC 1 Clock Failure Detector"]
pub type XOSCFAIL1_R = crate::BitReader;
#[doc = "Field `XOSCFAIL1` writer - XOSC 1 Clock Failure Detector"]
pub type XOSCFAIL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub type DFLLRDY_R = crate::BitReader;
#[doc = "Field `DFLLRDY` writer - DFLL Ready"]
pub type DFLLRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds"]
pub type DFLLOOB_R = crate::BitReader;
#[doc = "Field `DFLLOOB` writer - DFLL Out Of Bounds"]
pub type DFLLOOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine"]
pub type DFLLLCKF_R = crate::BitReader;
#[doc = "Field `DFLLLCKF` writer - DFLL Lock Fine"]
pub type DFLLLCKF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse"]
pub type DFLLLCKC_R = crate::BitReader;
#[doc = "Field `DFLLLCKC` writer - DFLL Lock Coarse"]
pub type DFLLLCKC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub type DFLLRCS_R = crate::BitReader;
#[doc = "Field `DFLLRCS` writer - DFLL Reference Clock Stopped"]
pub type DFLLRCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLL0LCKR` reader - DPLL0 Lock Rise"]
pub type DPLL0LCKR_R = crate::BitReader;
#[doc = "Field `DPLL0LCKR` writer - DPLL0 Lock Rise"]
pub type DPLL0LCKR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLL0LCKF` reader - DPLL0 Lock Fall"]
pub type DPLL0LCKF_R = crate::BitReader;
#[doc = "Field `DPLL0LCKF` writer - DPLL0 Lock Fall"]
pub type DPLL0LCKF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLL0LTO` reader - DPLL0 Lock Timeout"]
pub type DPLL0LTO_R = crate::BitReader;
#[doc = "Field `DPLL0LTO` writer - DPLL0 Lock Timeout"]
pub type DPLL0LTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLL0LDRTO` reader - DPLL0 Loop Divider Ratio Update Complete"]
pub type DPLL0LDRTO_R = crate::BitReader;
#[doc = "Field `DPLL0LDRTO` writer - DPLL0 Loop Divider Ratio Update Complete"]
pub type DPLL0LDRTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLL1LCKR` reader - DPLL1 Lock Rise"]
pub type DPLL1LCKR_R = crate::BitReader;
#[doc = "Field `DPLL1LCKR` writer - DPLL1 Lock Rise"]
pub type DPLL1LCKR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLL1LCKF` reader - DPLL1 Lock Fall"]
pub type DPLL1LCKF_R = crate::BitReader;
#[doc = "Field `DPLL1LCKF` writer - DPLL1 Lock Fall"]
pub type DPLL1LCKF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLL1LTO` reader - DPLL1 Lock Timeout"]
pub type DPLL1LTO_R = crate::BitReader;
#[doc = "Field `DPLL1LTO` writer - DPLL1 Lock Timeout"]
pub type DPLL1LTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLL1LDRTO` reader - DPLL1 Loop Divider Ratio Update Complete"]
pub type DPLL1LDRTO_R = crate::BitReader;
#[doc = "Field `DPLL1LDRTO` writer - DPLL1 Loop Divider Ratio Update Complete"]
pub type DPLL1LDRTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 18 - DPLL0 Lock Timeout"]
    #[inline(always)]
    pub fn dpll0lto(&self) -> DPLL0LTO_R {
        DPLL0LTO_R::new(((self.bits >> 18) & 1) != 0)
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
    #[doc = "Bit 26 - DPLL1 Lock Timeout"]
    #[inline(always)]
    pub fn dpll1lto(&self) -> DPLL1LTO_R {
        DPLL1LTO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll1ldrto(&self) -> DPLL1LDRTO_R {
        DPLL1LDRTO_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC 0 Ready"]
    #[inline(always)]
    #[must_use]
    pub fn xoscrdy0(&mut self) -> XOSCRDY0_W<INTFLAG_SPEC, 0> {
        XOSCRDY0_W::new(self)
    }
    #[doc = "Bit 1 - XOSC 1 Ready"]
    #[inline(always)]
    #[must_use]
    pub fn xoscrdy1(&mut self) -> XOSCRDY1_W<INTFLAG_SPEC, 1> {
        XOSCRDY1_W::new(self)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector"]
    #[inline(always)]
    #[must_use]
    pub fn xoscfail0(&mut self) -> XOSCFAIL0_W<INTFLAG_SPEC, 2> {
        XOSCFAIL0_W::new(self)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector"]
    #[inline(always)]
    #[must_use]
    pub fn xoscfail1(&mut self) -> XOSCFAIL1_W<INTFLAG_SPEC, 3> {
        XOSCFAIL1_W::new(self)
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrdy(&mut self) -> DFLLRDY_W<INTFLAG_SPEC, 8> {
        DFLLRDY_W::new(self)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds"]
    #[inline(always)]
    #[must_use]
    pub fn dflloob(&mut self) -> DFLLOOB_W<INTFLAG_SPEC, 9> {
        DFLLOOB_W::new(self)
    }
    #[doc = "Bit 10 - DFLL Lock Fine"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckf(&mut self) -> DFLLLCKF_W<INTFLAG_SPEC, 10> {
        DFLLLCKF_W::new(self)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckc(&mut self) -> DFLLLCKC_W<INTFLAG_SPEC, 11> {
        DFLLLCKC_W::new(self)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrcs(&mut self) -> DFLLRCS_W<INTFLAG_SPEC, 12> {
        DFLLRCS_W::new(self)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0lckr(&mut self) -> DPLL0LCKR_W<INTFLAG_SPEC, 16> {
        DPLL0LCKR_W::new(self)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0lckf(&mut self) -> DPLL0LCKF_W<INTFLAG_SPEC, 17> {
        DPLL0LCKF_W::new(self)
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0lto(&mut self) -> DPLL0LTO_W<INTFLAG_SPEC, 18> {
        DPLL0LTO_W::new(self)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0ldrto(&mut self) -> DPLL0LDRTO_W<INTFLAG_SPEC, 19> {
        DPLL0LDRTO_W::new(self)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1lckr(&mut self) -> DPLL1LCKR_W<INTFLAG_SPEC, 24> {
        DPLL1LCKR_W::new(self)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1lckf(&mut self) -> DPLL1LCKF_W<INTFLAG_SPEC, 25> {
        DPLL1LCKF_W::new(self)
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1lto(&mut self) -> DPLL1LTO_W<INTFLAG_SPEC, 26> {
        DPLL1LTO_W::new(self)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1ldrto(&mut self) -> DPLL1LDRTO_W<INTFLAG_SPEC, 27> {
        DPLL1LDRTO_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
