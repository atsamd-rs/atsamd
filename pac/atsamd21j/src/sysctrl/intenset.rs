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
#[doc = "Field `XOSCRDY` reader - XOSC Ready Interrupt Enable"]
pub type XOSCRDY_R = crate::BitReader<bool>;
#[doc = "Field `XOSCRDY` writer - XOSC Ready Interrupt Enable"]
pub type XOSCRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready Interrupt Enable"]
pub type XOSC32KRDY_R = crate::BitReader<bool>;
#[doc = "Field `XOSC32KRDY` writer - XOSC32K Ready Interrupt Enable"]
pub type XOSC32KRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OSC32KRDY` reader - OSC32K Ready Interrupt Enable"]
pub type OSC32KRDY_R = crate::BitReader<bool>;
#[doc = "Field `OSC32KRDY` writer - OSC32K Ready Interrupt Enable"]
pub type OSC32KRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OSC8MRDY` reader - OSC8M Ready Interrupt Enable"]
pub type OSC8MRDY_R = crate::BitReader<bool>;
#[doc = "Field `OSC8MRDY` writer - OSC8M Ready Interrupt Enable"]
pub type OSC8MRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
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
#[doc = "Field `BOD33RDY` reader - BOD33 Ready Interrupt Enable"]
pub type BOD33RDY_R = crate::BitReader<bool>;
#[doc = "Field `BOD33RDY` writer - BOD33 Ready Interrupt Enable"]
pub type BOD33RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `BOD33DET` reader - BOD33 Detection Interrupt Enable"]
pub type BOD33DET_R = crate::BitReader<bool>;
#[doc = "Field `BOD33DET` writer - BOD33 Detection Interrupt Enable"]
pub type BOD33DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready Interrupt Enable"]
pub type B33SRDY_R = crate::BitReader<bool>;
#[doc = "Field `B33SRDY` writer - BOD33 Synchronization Ready Interrupt Enable"]
pub type B33SRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise Interrupt Enable"]
pub type DPLLLCKR_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLCKR` writer - DPLL Lock Rise Interrupt Enable"]
pub type DPLLLCKR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall Interrupt Enable"]
pub type DPLLLCKF_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLCKF` writer - DPLL Lock Fall Interrupt Enable"]
pub type DPLLLCKF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DPLLLTO` reader - DPLL Lock Timeout Interrupt Enable"]
pub type DPLLLTO_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLTO` writer - DPLL Lock Timeout Interrupt Enable"]
pub type DPLLLTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> OSC32KRDY_R {
        OSC32KRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc8mrdy(&self) -> OSC8MRDY_R {
        OSC8MRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dplllto(&self) -> DPLLLTO_R {
        DPLLLTO_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscrdy(&mut self) -> XOSCRDY_W<0> {
        XOSCRDY_W::new(self)
    }
    #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xosc32krdy(&mut self) -> XOSC32KRDY_W<1> {
        XOSC32KRDY_W::new(self)
    }
    #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc32krdy(&mut self) -> OSC32KRDY_W<2> {
        OSC32KRDY_W::new(self)
    }
    #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc8mrdy(&mut self) -> OSC8MRDY_W<3> {
        OSC8MRDY_W::new(self)
    }
    #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrdy(&mut self) -> DFLLRDY_W<4> {
        DFLLRDY_W::new(self)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflloob(&mut self) -> DFLLOOB_W<5> {
        DFLLOOB_W::new(self)
    }
    #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckf(&mut self) -> DFLLLCKF_W<6> {
        DFLLLCKF_W::new(self)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckc(&mut self) -> DFLLLCKC_W<7> {
        DFLLLCKC_W::new(self)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrcs(&mut self) -> DFLLRCS_W<8> {
        DFLLRCS_W::new(self)
    }
    #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bod33rdy(&mut self) -> BOD33RDY_W<9> {
        BOD33RDY_W::new(self)
    }
    #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bod33det(&mut self) -> BOD33DET_W<10> {
        BOD33DET_W::new(self)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b33srdy(&mut self) -> B33SRDY_W<11> {
        B33SRDY_W::new(self)
    }
    #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllckr(&mut self) -> DPLLLCKR_W<15> {
        DPLLLCKR_W::new(self)
    }
    #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllckf(&mut self) -> DPLLLCKF_W<16> {
        DPLLLCKF_W::new(self)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllto(&mut self) -> DPLLLTO_W<17> {
        DPLLLTO_W::new(self)
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
