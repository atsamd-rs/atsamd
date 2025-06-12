#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `RXRDY0` reader - Receive Ready 0"]
pub type Rxrdy0R = crate::BitReader;
#[doc = "Field `RXRDY0` writer - Receive Ready 0"]
pub type Rxrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY1` reader - Receive Ready 1"]
pub type Rxrdy1R = crate::BitReader;
#[doc = "Field `RXRDY1` writer - Receive Ready 1"]
pub type Rxrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOR0` reader - Receive Overrun 0"]
pub type Rxor0R = crate::BitReader;
#[doc = "Field `RXOR0` writer - Receive Overrun 0"]
pub type Rxor0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOR1` reader - Receive Overrun 1"]
pub type Rxor1R = crate::BitReader;
#[doc = "Field `RXOR1` writer - Receive Overrun 1"]
pub type Rxor1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY0` reader - Transmit Ready 0"]
pub type Txrdy0R = crate::BitReader;
#[doc = "Field `TXRDY0` writer - Transmit Ready 0"]
pub type Txrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY1` reader - Transmit Ready 1"]
pub type Txrdy1R = crate::BitReader;
#[doc = "Field `TXRDY1` writer - Transmit Ready 1"]
pub type Txrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUR0` reader - Transmit Underrun 0"]
pub type Txur0R = crate::BitReader;
#[doc = "Field `TXUR0` writer - Transmit Underrun 0"]
pub type Txur0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUR1` reader - Transmit Underrun 1"]
pub type Txur1R = crate::BitReader;
#[doc = "Field `TXUR1` writer - Transmit Underrun 1"]
pub type Txur1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive Ready 0"]
    #[inline(always)]
    pub fn rxrdy0(&self) -> Rxrdy0R {
        Rxrdy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Ready 1"]
    #[inline(always)]
    pub fn rxrdy1(&self) -> Rxrdy1R {
        Rxrdy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overrun 0"]
    #[inline(always)]
    pub fn rxor0(&self) -> Rxor0R {
        Rxor0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun 1"]
    #[inline(always)]
    pub fn rxor1(&self) -> Rxor1R {
        Rxor1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Ready 0"]
    #[inline(always)]
    pub fn txrdy0(&self) -> Txrdy0R {
        Txrdy0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Ready 1"]
    #[inline(always)]
    pub fn txrdy1(&self) -> Txrdy1R {
        Txrdy1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Underrun 0"]
    #[inline(always)]
    pub fn txur0(&self) -> Txur0R {
        Txur0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Underrun 1"]
    #[inline(always)]
    pub fn txur1(&self) -> Txur1R {
        Txur1R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Ready 0"]
    #[inline(always)]
    pub fn rxrdy0(&mut self) -> Rxrdy0W<IntflagSpec> {
        Rxrdy0W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Ready 1"]
    #[inline(always)]
    pub fn rxrdy1(&mut self) -> Rxrdy1W<IntflagSpec> {
        Rxrdy1W::new(self, 1)
    }
    #[doc = "Bit 4 - Receive Overrun 0"]
    #[inline(always)]
    pub fn rxor0(&mut self) -> Rxor0W<IntflagSpec> {
        Rxor0W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Overrun 1"]
    #[inline(always)]
    pub fn rxor1(&mut self) -> Rxor1W<IntflagSpec> {
        Rxor1W::new(self, 5)
    }
    #[doc = "Bit 8 - Transmit Ready 0"]
    #[inline(always)]
    pub fn txrdy0(&mut self) -> Txrdy0W<IntflagSpec> {
        Txrdy0W::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit Ready 1"]
    #[inline(always)]
    pub fn txrdy1(&mut self) -> Txrdy1W<IntflagSpec> {
        Txrdy1W::new(self, 9)
    }
    #[doc = "Bit 12 - Transmit Underrun 0"]
    #[inline(always)]
    pub fn txur0(&mut self) -> Txur0W<IntflagSpec> {
        Txur0W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Underrun 1"]
    #[inline(always)]
    pub fn txur1(&mut self) -> Txur1W<IntflagSpec> {
        Txur1W::new(self, 13)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
