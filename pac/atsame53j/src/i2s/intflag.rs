#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<INTFLAG_SPEC>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<INTFLAG_SPEC>;
#[doc = "Field `RXRDY0` reader - Receive Ready 0"]
pub type RXRDY0_R = crate::BitReader;
#[doc = "Field `RXRDY0` writer - Receive Ready 0"]
pub type RXRDY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXRDY1` reader - Receive Ready 1"]
pub type RXRDY1_R = crate::BitReader;
#[doc = "Field `RXRDY1` writer - Receive Ready 1"]
pub type RXRDY1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOR0` reader - Receive Overrun 0"]
pub type RXOR0_R = crate::BitReader;
#[doc = "Field `RXOR0` writer - Receive Overrun 0"]
pub type RXOR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOR1` reader - Receive Overrun 1"]
pub type RXOR1_R = crate::BitReader;
#[doc = "Field `RXOR1` writer - Receive Overrun 1"]
pub type RXOR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRDY0` reader - Transmit Ready 0"]
pub type TXRDY0_R = crate::BitReader;
#[doc = "Field `TXRDY0` writer - Transmit Ready 0"]
pub type TXRDY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRDY1` reader - Transmit Ready 1"]
pub type TXRDY1_R = crate::BitReader;
#[doc = "Field `TXRDY1` writer - Transmit Ready 1"]
pub type TXRDY1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUR0` reader - Transmit Underrun 0"]
pub type TXUR0_R = crate::BitReader;
#[doc = "Field `TXUR0` writer - Transmit Underrun 0"]
pub type TXUR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUR1` reader - Transmit Underrun 1"]
pub type TXUR1_R = crate::BitReader;
#[doc = "Field `TXUR1` writer - Transmit Underrun 1"]
pub type TXUR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive Ready 0"]
    #[inline(always)]
    pub fn rxrdy0(&self) -> RXRDY0_R {
        RXRDY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Ready 1"]
    #[inline(always)]
    pub fn rxrdy1(&self) -> RXRDY1_R {
        RXRDY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overrun 0"]
    #[inline(always)]
    pub fn rxor0(&self) -> RXOR0_R {
        RXOR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun 1"]
    #[inline(always)]
    pub fn rxor1(&self) -> RXOR1_R {
        RXOR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Ready 0"]
    #[inline(always)]
    pub fn txrdy0(&self) -> TXRDY0_R {
        TXRDY0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Ready 1"]
    #[inline(always)]
    pub fn txrdy1(&self) -> TXRDY1_R {
        TXRDY1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Underrun 0"]
    #[inline(always)]
    pub fn txur0(&self) -> TXUR0_R {
        TXUR0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Underrun 1"]
    #[inline(always)]
    pub fn txur1(&self) -> TXUR1_R {
        TXUR1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Ready 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy0(&mut self) -> RXRDY0_W<INTFLAG_SPEC, 0> {
        RXRDY0_W::new(self)
    }
    #[doc = "Bit 1 - Receive Ready 1"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy1(&mut self) -> RXRDY1_W<INTFLAG_SPEC, 1> {
        RXRDY1_W::new(self)
    }
    #[doc = "Bit 4 - Receive Overrun 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxor0(&mut self) -> RXOR0_W<INTFLAG_SPEC, 4> {
        RXOR0_W::new(self)
    }
    #[doc = "Bit 5 - Receive Overrun 1"]
    #[inline(always)]
    #[must_use]
    pub fn rxor1(&mut self) -> RXOR1_W<INTFLAG_SPEC, 5> {
        RXOR1_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Ready 0"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy0(&mut self) -> TXRDY0_W<INTFLAG_SPEC, 8> {
        TXRDY0_W::new(self)
    }
    #[doc = "Bit 9 - Transmit Ready 1"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy1(&mut self) -> TXRDY1_W<INTFLAG_SPEC, 9> {
        TXRDY1_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Underrun 0"]
    #[inline(always)]
    #[must_use]
    pub fn txur0(&mut self) -> TXUR0_W<INTFLAG_SPEC, 12> {
        TXUR0_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Underrun 1"]
    #[inline(always)]
    #[must_use]
    pub fn txur1(&mut self) -> TXUR1_W<INTFLAG_SPEC, 13> {
        TXUR1_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u16;
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
