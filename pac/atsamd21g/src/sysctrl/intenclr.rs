#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<INTENCLR_SPEC>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<INTENCLR_SPEC>;
#[doc = "Field `XOSCRDY` reader - XOSC Ready Interrupt Enable"]
pub type XOSCRDY_R = crate::BitReader;
#[doc = "Field `XOSCRDY` writer - XOSC Ready Interrupt Enable"]
pub type XOSCRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready Interrupt Enable"]
pub type XOSC32KRDY_R = crate::BitReader;
#[doc = "Field `XOSC32KRDY` writer - XOSC32K Ready Interrupt Enable"]
pub type XOSC32KRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSC32KRDY` reader - OSC32K Ready Interrupt Enable"]
pub type OSC32KRDY_R = crate::BitReader;
#[doc = "Field `OSC32KRDY` writer - OSC32K Ready Interrupt Enable"]
pub type OSC32KRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSC8MRDY` reader - OSC8M Ready Interrupt Enable"]
pub type OSC8MRDY_R = crate::BitReader;
#[doc = "Field `OSC8MRDY` writer - OSC8M Ready Interrupt Enable"]
pub type OSC8MRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLRDY` reader - DFLL Ready Interrupt Enable"]
pub type DFLLRDY_R = crate::BitReader;
#[doc = "Field `DFLLRDY` writer - DFLL Ready Interrupt Enable"]
pub type DFLLRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds Interrupt Enable"]
pub type DFLLOOB_R = crate::BitReader;
#[doc = "Field `DFLLOOB` writer - DFLL Out Of Bounds Interrupt Enable"]
pub type DFLLOOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine Interrupt Enable"]
pub type DFLLLCKF_R = crate::BitReader;
#[doc = "Field `DFLLLCKF` writer - DFLL Lock Fine Interrupt Enable"]
pub type DFLLLCKF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse Interrupt Enable"]
pub type DFLLLCKC_R = crate::BitReader;
#[doc = "Field `DFLLLCKC` writer - DFLL Lock Coarse Interrupt Enable"]
pub type DFLLLCKC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped Interrupt Enable"]
pub type DFLLRCS_R = crate::BitReader;
#[doc = "Field `DFLLRCS` writer - DFLL Reference Clock Stopped Interrupt Enable"]
pub type DFLLRCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOD33RDY` reader - BOD33 Ready Interrupt Enable"]
pub type BOD33RDY_R = crate::BitReader;
#[doc = "Field `BOD33RDY` writer - BOD33 Ready Interrupt Enable"]
pub type BOD33RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOD33DET` reader - BOD33 Detection Interrupt Enable"]
pub type BOD33DET_R = crate::BitReader;
#[doc = "Field `BOD33DET` writer - BOD33 Detection Interrupt Enable"]
pub type BOD33DET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready Interrupt Enable"]
pub type B33SRDY_R = crate::BitReader;
#[doc = "Field `B33SRDY` writer - BOD33 Synchronization Ready Interrupt Enable"]
pub type B33SRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise Interrupt Enable"]
pub type DPLLLCKR_R = crate::BitReader;
#[doc = "Field `DPLLLCKR` writer - DPLL Lock Rise Interrupt Enable"]
pub type DPLLLCKR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall Interrupt Enable"]
pub type DPLLLCKF_R = crate::BitReader;
#[doc = "Field `DPLLLCKF` writer - DPLL Lock Fall Interrupt Enable"]
pub type DPLLLCKF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPLLLTO` reader - DPLL Lock Timeout Interrupt Enable"]
pub type DPLLLTO_R = crate::BitReader;
#[doc = "Field `DPLLLTO` writer - DPLL Lock Timeout Interrupt Enable"]
pub type DPLLLTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn xoscrdy(&mut self) -> XOSCRDY_W<INTENCLR_SPEC, 0> {
        XOSCRDY_W::new(self)
    }
    #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xosc32krdy(&mut self) -> XOSC32KRDY_W<INTENCLR_SPEC, 1> {
        XOSC32KRDY_W::new(self)
    }
    #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc32krdy(&mut self) -> OSC32KRDY_W<INTENCLR_SPEC, 2> {
        OSC32KRDY_W::new(self)
    }
    #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc8mrdy(&mut self) -> OSC8MRDY_W<INTENCLR_SPEC, 3> {
        OSC8MRDY_W::new(self)
    }
    #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrdy(&mut self) -> DFLLRDY_W<INTENCLR_SPEC, 4> {
        DFLLRDY_W::new(self)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflloob(&mut self) -> DFLLOOB_W<INTENCLR_SPEC, 5> {
        DFLLOOB_W::new(self)
    }
    #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckf(&mut self) -> DFLLLCKF_W<INTENCLR_SPEC, 6> {
        DFLLLCKF_W::new(self)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckc(&mut self) -> DFLLLCKC_W<INTENCLR_SPEC, 7> {
        DFLLLCKC_W::new(self)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrcs(&mut self) -> DFLLRCS_W<INTENCLR_SPEC, 8> {
        DFLLRCS_W::new(self)
    }
    #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bod33rdy(&mut self) -> BOD33RDY_W<INTENCLR_SPEC, 9> {
        BOD33RDY_W::new(self)
    }
    #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bod33det(&mut self) -> BOD33DET_W<INTENCLR_SPEC, 10> {
        BOD33DET_W::new(self)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b33srdy(&mut self) -> B33SRDY_W<INTENCLR_SPEC, 11> {
        B33SRDY_W::new(self)
    }
    #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllckr(&mut self) -> DPLLLCKR_W<INTENCLR_SPEC, 15> {
        DPLLLCKR_W::new(self)
    }
    #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllckf(&mut self) -> DPLLLCKF_W<INTENCLR_SPEC, 16> {
        DPLLLCKF_W::new(self)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllto(&mut self) -> DPLLLTO_W<INTENCLR_SPEC, 17> {
        DPLLLTO_W::new(self)
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
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
