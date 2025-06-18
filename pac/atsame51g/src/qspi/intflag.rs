#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `RXC` reader - Receive Data Register Full"]
pub type RxcR = crate::BitReader;
#[doc = "Field `RXC` writer - Receive Data Register Full"]
pub type RxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRE` reader - Transmit Data Register Empty"]
pub type DreR = crate::BitReader;
#[doc = "Field `DRE` writer - Transmit Data Register Empty"]
pub type DreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` reader - Transmission Complete"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - Transmission Complete"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Overrun Error"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Overrun Error"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRISE` reader - Chip Select Rise"]
pub type CsriseR = crate::BitReader;
#[doc = "Field `CSRISE` writer - Chip Select Rise"]
pub type CsriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTREND` reader - Instruction End"]
pub type InstrendR = crate::BitReader;
#[doc = "Field `INSTREND` writer - Instruction End"]
pub type InstrendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full"]
    #[inline(always)]
    pub fn rxc(&self) -> RxcR {
        RxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    pub fn dre(&self) -> DreR {
        DreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Chip Select Rise"]
    #[inline(always)]
    pub fn csrise(&self) -> CsriseR {
        CsriseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Instruction End"]
    #[inline(always)]
    pub fn instrend(&self) -> InstrendR {
        InstrendR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Register Full"]
    #[inline(always)]
    pub fn rxc(&mut self) -> RxcW<IntflagSpec> {
        RxcW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    pub fn dre(&mut self) -> DreW<IntflagSpec> {
        DreW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmission Complete"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<IntflagSpec> {
        TxcW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<IntflagSpec> {
        ErrorW::new(self, 3)
    }
    #[doc = "Bit 8 - Chip Select Rise"]
    #[inline(always)]
    pub fn csrise(&mut self) -> CsriseW<IntflagSpec> {
        CsriseW::new(self, 8)
    }
    #[doc = "Bit 10 - Instruction End"]
    #[inline(always)]
    pub fn instrend(&mut self) -> InstrendW<IntflagSpec> {
        InstrendW::new(self, 10)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
