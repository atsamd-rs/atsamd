#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<INTFLAG_SPEC>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<INTFLAG_SPEC>;
#[doc = "Field `PREC` reader - Stop Received Interrupt"]
pub type PREC_R = crate::BitReader;
#[doc = "Field `PREC` writer - Stop Received Interrupt"]
pub type PREC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMATCH` reader - Address Match Interrupt"]
pub type AMATCH_R = crate::BitReader;
#[doc = "Field `AMATCH` writer - Address Match Interrupt"]
pub type AMATCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRDY` reader - Data Interrupt"]
pub type DRDY_R = crate::BitReader;
#[doc = "Field `DRDY` writer - Data Interrupt"]
pub type DRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt"]
pub type ERROR_R = crate::BitReader;
#[doc = "Field `ERROR` writer - Combined Error Interrupt"]
pub type ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Stop Received Interrupt"]
    #[inline(always)]
    pub fn prec(&self) -> PREC_R {
        PREC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Match Interrupt"]
    #[inline(always)]
    pub fn amatch(&self) -> AMATCH_R {
        AMATCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Interrupt"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn prec(&mut self) -> PREC_W<INTFLAG_SPEC, 0> {
        PREC_W::new(self)
    }
    #[doc = "Bit 1 - Address Match Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn amatch(&mut self) -> AMATCH_W<INTFLAG_SPEC, 1> {
        AMATCH_W::new(self)
    }
    #[doc = "Bit 2 - Data Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DRDY_W<INTFLAG_SPEC, 2> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<INTFLAG_SPEC, 7> {
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
#[doc = "I2CS Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u8;
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
