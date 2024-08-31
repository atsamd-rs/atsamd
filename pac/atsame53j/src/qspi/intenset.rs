#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `RXC` reader - Receive Data Register Full Interrupt Enable"]
pub type RxcR = crate::BitReader;
#[doc = "Field `RXC` writer - Receive Data Register Full Interrupt Enable"]
pub type RxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRE` reader - Transmit Data Register Empty Interrupt Enable"]
pub type DreR = crate::BitReader;
#[doc = "Field `DRE` writer - Transmit Data Register Empty Interrupt Enable"]
pub type DreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` reader - Transmission Complete Interrupt Enable"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - Transmission Complete Interrupt Enable"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Overrun Error Interrupt Enable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Overrun Error Interrupt Enable"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRISE` reader - Chip Select Rise Interrupt Enable"]
pub type CsriseR = crate::BitReader;
#[doc = "Field `CSRISE` writer - Chip Select Rise Interrupt Enable"]
pub type CsriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTREND` reader - Instruction End Interrupt Enable"]
pub type InstrendR = crate::BitReader;
#[doc = "Field `INSTREND` writer - Instruction End Interrupt Enable"]
pub type InstrendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Enable"]
    #[inline(always)]
    pub fn rxc(&self) -> RxcR {
        RxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn dre(&self) -> DreR {
        DreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Chip Select Rise Interrupt Enable"]
    #[inline(always)]
    pub fn csrise(&self) -> CsriseR {
        CsriseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Instruction End Interrupt Enable"]
    #[inline(always)]
    pub fn instrend(&self) -> InstrendR {
        InstrendR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxc(&mut self) -> RxcW<IntensetSpec> {
        RxcW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dre(&mut self) -> DreW<IntensetSpec> {
        DreW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmission Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IntensetSpec> {
        TxcW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntensetSpec> {
        ErrorW::new(self, 3)
    }
    #[doc = "Bit 8 - Chip Select Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csrise(&mut self) -> CsriseW<IntensetSpec> {
        CsriseW::new(self, 8)
    }
    #[doc = "Bit 10 - Instruction End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn instrend(&mut self) -> InstrendW<IntensetSpec> {
        InstrendW::new(self, 10)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
