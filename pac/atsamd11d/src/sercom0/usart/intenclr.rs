#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRE` reader - Data Register Empty Interrupt Disable"]
pub type DRE_R = crate::BitReader<bool>;
#[doc = "Field `DRE` writer - Data Register Empty Interrupt Disable"]
pub type DRE_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `TXC` reader - Transmit Complete Interrupt Disable"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - Transmit Complete Interrupt Disable"]
pub type TXC_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `RXC` reader - Receive Complete Interrupt Disable"]
pub type RXC_R = crate::BitReader<bool>;
#[doc = "Field `RXC` writer - Receive Complete Interrupt Disable"]
pub type RXC_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `RXS` reader - Receive Start Interrupt Disable"]
pub type RXS_R = crate::BitReader<bool>;
#[doc = "Field `RXS` writer - Receive Start Interrupt Disable"]
pub type RXS_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `CTSIC` reader - Clear To Send Input Change Interrupt Disable"]
pub type CTSIC_R = crate::BitReader<bool>;
#[doc = "Field `CTSIC` writer - Clear To Send Input Change Interrupt Disable"]
pub type CTSIC_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `RXBRK` reader - Break Received Interrupt Disable"]
pub type RXBRK_R = crate::BitReader<bool>;
#[doc = "Field `RXBRK` writer - Break Received Interrupt Disable"]
pub type RXBRK_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt Disable"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Combined Error Interrupt Disable"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Data Register Empty Interrupt Disable"]
    #[inline(always)]
    pub fn dre(&self) -> DRE_R {
        DRE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt Disable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt Disable"]
    #[inline(always)]
    pub fn rxc(&self) -> RXC_R {
        RXC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Start Interrupt Disable"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear To Send Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn ctsic(&self) -> CTSIC_R {
        CTSIC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Break Received Interrupt Disable"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Register Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dre(&mut self) -> DRE_W<0> {
        DRE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<1> {
        TXC_W::new(self)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxc(&mut self) -> RXC_W<2> {
        RXC_W::new(self)
    }
    #[doc = "Bit 3 - Receive Start Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxs(&mut self) -> RXS_W<3> {
        RXS_W::new(self)
    }
    #[doc = "Bit 4 - Clear To Send Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsic(&mut self) -> CTSIC_W<4> {
        CTSIC_W::new(self)
    }
    #[doc = "Bit 5 - Break Received Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk(&mut self) -> RXBRK_W<5> {
        RXBRK_W::new(self)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
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
#[doc = "USART Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
