#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXC` reader - Receive Data Register Full"]
pub type RXC_R = crate::BitReader<bool>;
#[doc = "Field `RXC` writer - Receive Data Register Full"]
pub type RXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DRE` reader - Transmit Data Register Empty"]
pub type DRE_R = crate::BitReader<bool>;
#[doc = "Field `DRE` writer - Transmit Data Register Empty"]
pub type DRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `TXC` reader - Transmission Complete"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - Transmission Complete"]
pub type TXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - Overrun Error"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Overrun Error"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `CSRISE` reader - Chip Select Rise"]
pub type CSRISE_R = crate::BitReader<bool>;
#[doc = "Field `CSRISE` writer - Chip Select Rise"]
pub type CSRISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `INSTREND` reader - Instruction End"]
pub type INSTREND_R = crate::BitReader<bool>;
#[doc = "Field `INSTREND` writer - Instruction End"]
pub type INSTREND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
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
    pub fn rxc(&mut self) -> RXC_W<0> {
        RXC_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    #[must_use]
    pub fn dre(&mut self) -> DRE_W<1> {
        DRE_W::new(self)
    }
    #[doc = "Bit 2 - Transmission Complete"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<2> {
        TXC_W::new(self)
    }
    #[doc = "Bit 3 - Overrun Error"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<3> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 8 - Chip Select Rise"]
    #[inline(always)]
    #[must_use]
    pub fn csrise(&mut self) -> CSRISE_W<8> {
        CSRISE_W::new(self)
    }
    #[doc = "Bit 10 - Instruction End"]
    #[inline(always)]
    #[must_use]
    pub fn instrend(&mut self) -> INSTREND_W<10> {
        INSTREND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
