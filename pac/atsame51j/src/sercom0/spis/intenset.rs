#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<INTENSET_SPEC>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<INTENSET_SPEC>;
#[doc = "Field `DRE` reader - Data Register Empty Interrupt Enable"]
pub type DRE_R = crate::BitReader;
#[doc = "Field `DRE` writer - Data Register Empty Interrupt Enable"]
pub type DRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXC` reader - Transmit Complete Interrupt Enable"]
pub type TXC_R = crate::BitReader;
#[doc = "Field `TXC` writer - Transmit Complete Interrupt Enable"]
pub type TXC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXC` reader - Receive Complete Interrupt Enable"]
pub type RXC_R = crate::BitReader;
#[doc = "Field `RXC` writer - Receive Complete Interrupt Enable"]
pub type RXC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSL` reader - Slave Select Low Interrupt Enable"]
pub type SSL_R = crate::BitReader;
#[doc = "Field `SSL` writer - Slave Select Low Interrupt Enable"]
pub type SSL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt Enable"]
pub type ERROR_R = crate::BitReader;
#[doc = "Field `ERROR` writer - Combined Error Interrupt Enable"]
pub type ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn dre(&self) -> DRE_R {
        DRE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxc(&self) -> RXC_R {
        RXC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Select Low Interrupt Enable"]
    #[inline(always)]
    pub fn ssl(&self) -> SSL_R {
        SSL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dre(&mut self) -> DRE_W<INTENSET_SPEC, 0> {
        DRE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<INTENSET_SPEC, 1> {
        TXC_W::new(self)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxc(&mut self) -> RXC_W<INTENSET_SPEC, 2> {
        RXC_W::new(self)
    }
    #[doc = "Bit 3 - Slave Select Low Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssl(&mut self) -> SSL_W<INTENSET_SPEC, 3> {
        SSL_W::new(self)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<INTENSET_SPEC, 7> {
        ERROR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPIS Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
