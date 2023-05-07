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
#[doc = "Field `RXRDY0` reader - Receive Ready 0 Interrupt Enable"]
pub type RXRDY0_R = crate::BitReader<bool>;
#[doc = "Field `RXRDY0` writer - Receive Ready 0 Interrupt Enable"]
pub type RXRDY0_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENSET_SPEC, bool, O>;
#[doc = "Field `RXRDY1` reader - Receive Ready 1 Interrupt Enable"]
pub type RXRDY1_R = crate::BitReader<bool>;
#[doc = "Field `RXRDY1` writer - Receive Ready 1 Interrupt Enable"]
pub type RXRDY1_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENSET_SPEC, bool, O>;
#[doc = "Field `RXOR0` reader - Receive Overrun 0 Interrupt Enable"]
pub type RXOR0_R = crate::BitReader<bool>;
#[doc = "Field `RXOR0` writer - Receive Overrun 0 Interrupt Enable"]
pub type RXOR0_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENSET_SPEC, bool, O>;
#[doc = "Field `RXOR1` reader - Receive Overrun 1 Interrupt Enable"]
pub type RXOR1_R = crate::BitReader<bool>;
#[doc = "Field `RXOR1` writer - Receive Overrun 1 Interrupt Enable"]
pub type RXOR1_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENSET_SPEC, bool, O>;
#[doc = "Field `TXRDY0` reader - Transmit Ready 0 Interrupt Enable"]
pub type TXRDY0_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY0` writer - Transmit Ready 0 Interrupt Enable"]
pub type TXRDY0_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENSET_SPEC, bool, O>;
#[doc = "Field `TXRDY1` reader - Transmit Ready 1 Interrupt Enable"]
pub type TXRDY1_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY1` writer - Transmit Ready 1 Interrupt Enable"]
pub type TXRDY1_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENSET_SPEC, bool, O>;
#[doc = "Field `TXUR0` reader - Transmit Underrun 0 Interrupt Enable"]
pub type TXUR0_R = crate::BitReader<bool>;
#[doc = "Field `TXUR0` writer - Transmit Underrun 0 Interrupt Enable"]
pub type TXUR0_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENSET_SPEC, bool, O>;
#[doc = "Field `TXUR1` reader - Transmit Underrun 1 Interrupt Enable"]
pub type TXUR1_R = crate::BitReader<bool>;
#[doc = "Field `TXUR1` writer - Transmit Underrun 1 Interrupt Enable"]
pub type TXUR1_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy0(&self) -> RXRDY0_R {
        RXRDY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy1(&self) -> RXRDY1_R {
        RXRDY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor0(&self) -> RXOR0_R {
        RXOR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor1(&self) -> RXOR1_R {
        RXOR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy0(&self) -> TXRDY0_R {
        TXRDY0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy1(&self) -> TXRDY1_R {
        TXRDY1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txur0(&self) -> TXUR0_R {
        TXUR0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txur1(&self) -> TXUR1_R {
        TXUR1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy0(&mut self) -> RXRDY0_W<0> {
        RXRDY0_W::new(self)
    }
    #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy1(&mut self) -> RXRDY1_W<1> {
        RXRDY1_W::new(self)
    }
    #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxor0(&mut self) -> RXOR0_W<4> {
        RXOR0_W::new(self)
    }
    #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxor1(&mut self) -> RXOR1_W<5> {
        RXOR1_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy0(&mut self) -> TXRDY0_W<8> {
        TXRDY0_W::new(self)
    }
    #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy1(&mut self) -> TXRDY1_W<9> {
        TXRDY1_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txur0(&mut self) -> TXUR0_W<12> {
        TXUR0_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txur1(&mut self) -> TXUR1_W<13> {
        TXUR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u16;
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
