#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `RXRDY0` reader - Receive Ready 0 Interrupt Enable"]
pub type Rxrdy0R = crate::BitReader;
#[doc = "Field `RXRDY0` writer - Receive Ready 0 Interrupt Enable"]
pub type Rxrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY1` reader - Receive Ready 1 Interrupt Enable"]
pub type Rxrdy1R = crate::BitReader;
#[doc = "Field `RXRDY1` writer - Receive Ready 1 Interrupt Enable"]
pub type Rxrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOR0` reader - Receive Overrun 0 Interrupt Enable"]
pub type Rxor0R = crate::BitReader;
#[doc = "Field `RXOR0` writer - Receive Overrun 0 Interrupt Enable"]
pub type Rxor0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOR1` reader - Receive Overrun 1 Interrupt Enable"]
pub type Rxor1R = crate::BitReader;
#[doc = "Field `RXOR1` writer - Receive Overrun 1 Interrupt Enable"]
pub type Rxor1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY0` reader - Transmit Ready 0 Interrupt Enable"]
pub type Txrdy0R = crate::BitReader;
#[doc = "Field `TXRDY0` writer - Transmit Ready 0 Interrupt Enable"]
pub type Txrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY1` reader - Transmit Ready 1 Interrupt Enable"]
pub type Txrdy1R = crate::BitReader;
#[doc = "Field `TXRDY1` writer - Transmit Ready 1 Interrupt Enable"]
pub type Txrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUR0` reader - Transmit Underrun 0 Interrupt Enable"]
pub type Txur0R = crate::BitReader;
#[doc = "Field `TXUR0` writer - Transmit Underrun 0 Interrupt Enable"]
pub type Txur0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUR1` reader - Transmit Underrun 1 Interrupt Enable"]
pub type Txur1R = crate::BitReader;
#[doc = "Field `TXUR1` writer - Transmit Underrun 1 Interrupt Enable"]
pub type Txur1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy0(&self) -> Rxrdy0R {
        Rxrdy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy1(&self) -> Rxrdy1R {
        Rxrdy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor0(&self) -> Rxor0R {
        Rxor0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor1(&self) -> Rxor1R {
        Rxor1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy0(&self) -> Txrdy0R {
        Txrdy0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy1(&self) -> Txrdy1R {
        Txrdy1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txur0(&self) -> Txur0R {
        Txur0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txur1(&self) -> Txur1R {
        Txur1R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy0(&mut self) -> Rxrdy0W<IntenclrSpec> {
        Rxrdy0W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy1(&mut self) -> Rxrdy1W<IntenclrSpec> {
        Rxrdy1W::new(self, 1)
    }
    #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxor0(&mut self) -> Rxor0W<IntenclrSpec> {
        Rxor0W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxor1(&mut self) -> Rxor1W<IntenclrSpec> {
        Rxor1W::new(self, 5)
    }
    #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy0(&mut self) -> Txrdy0W<IntenclrSpec> {
        Txrdy0W::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy1(&mut self) -> Txrdy1W<IntenclrSpec> {
        Txrdy1W::new(self, 9)
    }
    #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txur0(&mut self) -> Txur0W<IntenclrSpec> {
        Txur0W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txur1(&mut self) -> Txur1W<IntenclrSpec> {
        Txur1W::new(self, 13)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u16 = 0;
}
