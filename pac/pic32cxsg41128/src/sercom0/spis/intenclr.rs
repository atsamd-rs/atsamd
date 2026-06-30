#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `DRE` reader - Data Register Empty Interrupt Disable"]
pub type DreR = crate::BitReader;
#[doc = "Field `DRE` writer - Data Register Empty Interrupt Disable"]
pub type DreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` reader - Transmit Complete Interrupt Disable"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - Transmit Complete Interrupt Disable"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC` reader - Receive Complete Interrupt Disable"]
pub type RxcR = crate::BitReader;
#[doc = "Field `RXC` writer - Receive Complete Interrupt Disable"]
pub type RxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSL` reader - Slave Select Low Interrupt Disable"]
pub type SslR = crate::BitReader;
#[doc = "Field `SSL` writer - Slave Select Low Interrupt Disable"]
pub type SslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt Disable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Combined Error Interrupt Disable"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Register Empty Interrupt Disable"]
    #[inline(always)]
    pub fn dre(&self) -> DreR {
        DreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt Disable"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt Disable"]
    #[inline(always)]
    pub fn rxc(&self) -> RxcR {
        RxcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Select Low Interrupt Disable"]
    #[inline(always)]
    pub fn ssl(&self) -> SslR {
        SslR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Register Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dre(&mut self) -> DreW<IntenclrSpec> {
        DreW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IntenclrSpec> {
        TxcW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxc(&mut self) -> RxcW<IntenclrSpec> {
        RxcW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Select Low Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ssl(&mut self) -> SslW<IntenclrSpec> {
        SslW::new(self, 3)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntenclrSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "SPIS Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u8 = 0;
}
