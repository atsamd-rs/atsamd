#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<INTFLAG_SPEC>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<INTFLAG_SPEC>;
#[doc = "Field `RXC` reader - Receive Data Register Full"]
pub type RXC_R = crate::BitReader;
#[doc = "Field `RXC` writer - Receive Data Register Full"]
pub type RXC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRE` reader - Transmit Data Register Empty"]
pub type DRE_R = crate::BitReader;
#[doc = "Field `DRE` writer - Transmit Data Register Empty"]
pub type DRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXC` reader - Transmission Complete"]
pub type TXC_R = crate::BitReader;
#[doc = "Field `TXC` writer - Transmission Complete"]
pub type TXC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERROR` reader - Overrun Error"]
pub type ERROR_R = crate::BitReader;
#[doc = "Field `ERROR` writer - Overrun Error"]
pub type ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSRISE` reader - Chip Select Rise"]
pub type CSRISE_R = crate::BitReader;
#[doc = "Field `CSRISE` writer - Chip Select Rise"]
pub type CSRISE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INSTREND` reader - Instruction End"]
pub type INSTREND_R = crate::BitReader;
#[doc = "Field `INSTREND` writer - Instruction End"]
pub type INSTREND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full"]
    #[inline(always)]
    pub fn rxc(&self) -> RXC_R {
        RXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    pub fn dre(&self) -> DRE_R {
        DRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Chip Select Rise"]
    #[inline(always)]
    pub fn csrise(&self) -> CSRISE_R {
        CSRISE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Instruction End"]
    #[inline(always)]
    pub fn instrend(&self) -> INSTREND_R {
        INSTREND_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Register Full"]
    #[inline(always)]
    #[must_use]
    pub fn rxc(&mut self) -> RXC_W<INTFLAG_SPEC, 0> {
        RXC_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    #[must_use]
    pub fn dre(&mut self) -> DRE_W<INTFLAG_SPEC, 1> {
        DRE_W::new(self)
    }
    #[doc = "Bit 2 - Transmission Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<INTFLAG_SPEC, 2> {
        TXC_W::new(self)
    }
    #[doc = "Bit 3 - Overrun Error"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<INTFLAG_SPEC, 3> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 8 - Chip Select Rise"]
    #[inline(always)]
    #[must_use]
    pub fn csrise(&mut self) -> CSRISE_W<INTFLAG_SPEC, 8> {
        CSRISE_W::new(self)
    }
    #[doc = "Bit 10 - Instruction End"]
    #[inline(always)]
    #[must_use]
    pub fn instrend(&mut self) -> INSTREND_W<INTFLAG_SPEC, 10> {
        INSTREND_W::new(self)
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
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
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
