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
#[doc = "Field `PREC` reader - Stop Received Interrupt Enable"]
pub type PREC_R = crate::BitReader<bool>;
#[doc = "Field `PREC` writer - Stop Received Interrupt Enable"]
pub type PREC_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
#[doc = "Field `AMATCH` reader - Address Match Interrupt Enable"]
pub type AMATCH_R = crate::BitReader<bool>;
#[doc = "Field `AMATCH` writer - Address Match Interrupt Enable"]
pub type AMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
#[doc = "Field `DRDY` reader - Data Interrupt Enable"]
pub type DRDY_R = crate::BitReader<bool>;
#[doc = "Field `DRDY` writer - Data Interrupt Enable"]
pub type DRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt Enable"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Combined Error Interrupt Enable"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Stop Received Interrupt Enable"]
    #[inline(always)]
    pub fn prec(&self) -> PREC_R {
        PREC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Match Interrupt Enable"]
    #[inline(always)]
    pub fn amatch(&self) -> AMATCH_R {
        AMATCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Interrupt Enable"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prec(&mut self) -> PREC_W<0> {
        PREC_W::new(self)
    }
    #[doc = "Bit 1 - Address Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn amatch(&mut self) -> AMATCH_W<1> {
        AMATCH_W::new(self)
    }
    #[doc = "Bit 2 - Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DRDY_W<2> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<7> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u8;
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
