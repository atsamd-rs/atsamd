#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `DRE` reader - Data Register Empty Interrupt"]
pub type DreR = crate::BitReader;
#[doc = "Field `DRE` writer - Data Register Empty Interrupt"]
pub type DreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` reader - Transmit Complete Interrupt"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - Transmit Complete Interrupt"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC` reader - Receive Complete Interrupt"]
pub type RxcR = crate::BitReader;
#[doc = "Field `RXC` writer - Receive Complete Interrupt"]
pub type RxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSL` reader - Slave Select Low Interrupt Flag"]
pub type SslR = crate::BitReader;
#[doc = "Field `SSL` writer - Slave Select Low Interrupt Flag"]
pub type SslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Combined Error Interrupt"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Register Empty Interrupt"]
    #[inline(always)]
    pub fn dre(&self) -> DreR {
        DreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt"]
    #[inline(always)]
    pub fn rxc(&self) -> RxcR {
        RxcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Select Low Interrupt Flag"]
    #[inline(always)]
    pub fn ssl(&self) -> SslR {
        SslR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Register Empty Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dre(&mut self) -> DreW<IntflagSpec> {
        DreW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IntflagSpec> {
        TxcW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxc(&mut self) -> RxcW<IntflagSpec> {
        RxcW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Select Low Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssl(&mut self) -> SslW<IntflagSpec> {
        SslW::new(self, 3)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntflagSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "SPIS Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u8 = 0;
}
