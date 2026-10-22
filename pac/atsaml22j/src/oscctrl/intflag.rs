#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSCRDY` reader - XOSC Ready"]
pub type XOSCRDY_R = crate::BitReader<bool>;
#[doc = "Field `XOSCRDY` writer - XOSC Ready"]
pub type XOSCRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `XOSCFAIL` reader - XOSC Clock Failure Detector"]
pub type XOSCFAIL_R = crate::BitReader<bool>;
#[doc = "Field `XOSCFAIL` writer - XOSC Clock Failure Detector"]
pub type XOSCFAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `OSC16MRDY` reader - OSC16M Ready"]
pub type OSC16MRDY_R = crate::BitReader<bool>;
#[doc = "Field `OSC16MRDY` writer - OSC16M Ready"]
pub type OSC16MRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub type DFLLRDY_R = crate::BitReader<bool>;
#[doc = "Field `DFLLRDY` writer - DFLL Ready"]
pub type DFLLRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds"]
pub type DFLLOOB_R = crate::BitReader<bool>;
#[doc = "Field `DFLLOOB` writer - DFLL Out Of Bounds"]
pub type DFLLOOB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine"]
pub type DFLLLCKF_R = crate::BitReader<bool>;
#[doc = "Field `DFLLLCKF` writer - DFLL Lock Fine"]
pub type DFLLLCKF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse"]
pub type DFLLLCKC_R = crate::BitReader<bool>;
#[doc = "Field `DFLLLCKC` writer - DFLL Lock Coarse"]
pub type DFLLLCKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub type DFLLRCS_R = crate::BitReader<bool>;
#[doc = "Field `DFLLRCS` writer - DFLL Reference Clock Stopped"]
pub type DFLLRCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise"]
pub type DPLLLCKR_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLCKR` writer - DPLL Lock Rise"]
pub type DPLLLCKR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall"]
pub type DPLLLCKF_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLCKF` writer - DPLL Lock Fall"]
pub type DPLLLCKF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DPLLLTO` reader - DPLL Timeout"]
pub type DPLLLTO_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLTO` writer - DPLL Timeout"]
pub type DPLLLTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DPLLLDRTO` reader - DPLL Ratio Ready"]
pub type DPLLLDRTO_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLDRTO` writer - DPLL Ratio Ready"]
pub type DPLLLDRTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
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
    pub fn dplllto(&self) -> DPLLLTO_R {
        DPLLLTO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DPLL Ratio Ready"]
    #[inline(always)]
    pub fn dpllldrto(&self) -> DPLLLDRTO_R {
        DPLLLDRTO_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    #[must_use]
    pub fn xoscrdy(&mut self) -> XOSCRDY_W<0> {
        XOSCRDY_W::new(self)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector"]
    #[inline(always)]
    #[must_use]
    pub fn xoscfail(&mut self) -> XOSCFAIL_W<1> {
        XOSCFAIL_W::new(self)
    }
    #[doc = "Bit 4 - OSC16M Ready"]
    #[inline(always)]
    #[must_use]
    pub fn osc16mrdy(&mut self) -> OSC16MRDY_W<4> {
        OSC16MRDY_W::new(self)
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrdy(&mut self) -> DFLLRDY_W<8> {
        DFLLRDY_W::new(self)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds"]
    #[inline(always)]
    #[must_use]
    pub fn dflloob(&mut self) -> DFLLOOB_W<9> {
        DFLLOOB_W::new(self)
    }
    #[doc = "Bit 10 - DFLL Lock Fine"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckf(&mut self) -> DFLLLCKF_W<10> {
        DFLLLCKF_W::new(self)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckc(&mut self) -> DFLLLCKC_W<11> {
        DFLLLCKC_W::new(self)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrcs(&mut self) -> DFLLRCS_W<12> {
        DFLLRCS_W::new(self)
    }
    #[doc = "Bit 16 - DPLL Lock Rise"]
    #[inline(always)]
    #[must_use]
    pub fn dplllckr(&mut self) -> DPLLLCKR_W<16> {
        DPLLLCKR_W::new(self)
    }
    #[doc = "Bit 17 - DPLL Lock Fall"]
    #[inline(always)]
    #[must_use]
    pub fn dplllckf(&mut self) -> DPLLLCKF_W<17> {
        DPLLLCKF_W::new(self)
    }
    #[doc = "Bit 18 - DPLL Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn dplllto(&mut self) -> DPLLLTO_W<18> {
        DPLLLTO_W::new(self)
    }
    #[doc = "Bit 19 - DPLL Ratio Ready"]
    #[inline(always)]
    #[must_use]
    pub fn dpllldrto(&mut self) -> DPLLLDRTO_W<19> {
        DPLLLDRTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
