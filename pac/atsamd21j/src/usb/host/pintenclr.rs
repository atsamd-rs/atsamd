#[doc = "Register `PINTENCLR%s` reader"]
pub struct R(crate::R<PINTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINTENCLR%s` writer"]
pub struct W(crate::W<PINTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINTENCLR_SPEC>;
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
impl From<crate::W<PINTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Disable"]
pub type TRCPT0_R = crate::BitReader<bool>;
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Disable"]
pub type TRCPT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINTENCLR_SPEC, bool, O>;
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Disable"]
pub type TRCPT1_R = crate::BitReader<bool>;
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Disable"]
pub type TRCPT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINTENCLR_SPEC, bool, O>;
#[doc = "Field `TRFAIL` reader - Error Flow Interrupt Disable"]
pub type TRFAIL_R = crate::BitReader<bool>;
#[doc = "Field `TRFAIL` writer - Error Flow Interrupt Disable"]
pub type TRFAIL_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINTENCLR_SPEC, bool, O>;
#[doc = "Field `PERR` reader - Pipe Error Interrupt Disable"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `PERR` writer - Pipe Error Interrupt Disable"]
pub type PERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINTENCLR_SPEC, bool, O>;
#[doc = "Field `TXSTP` reader - Transmit Setup Interrupt Disable"]
pub type TXSTP_R = crate::BitReader<bool>;
#[doc = "Field `TXSTP` writer - Transmit Setup Interrupt Disable"]
pub type TXSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINTENCLR_SPEC, bool, O>;
#[doc = "Field `STALL` reader - Stall Inetrrupt Disable"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - Stall Inetrrupt Disable"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer Complete 0 Disable"]
    #[inline(always)]
    pub fn trcpt0(&self) -> TRCPT0_R {
        TRCPT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Disable"]
    #[inline(always)]
    pub fn trcpt1(&self) -> TRCPT1_R {
        TRCPT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Flow Interrupt Disable"]
    #[inline(always)]
    pub fn trfail(&self) -> TRFAIL_R {
        TRFAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Disable"]
    #[inline(always)]
    pub fn txstp(&self) -> TXSTP_R {
        TXSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall Inetrrupt Disable"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt0(&mut self) -> TRCPT0_W<0> {
        TRCPT0_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt1(&mut self) -> TRCPT1_W<1> {
        TRCPT1_W::new(self)
    }
    #[doc = "Bit 2 - Error Flow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trfail(&mut self) -> TRFAIL_W<2> {
        TRFAIL_W::new(self)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<3> {
        PERR_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txstp(&mut self) -> TXSTP_W<4> {
        TXSTP_W::new(self)
    }
    #[doc = "Bit 5 - Stall Inetrrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<5> {
        STALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST Pipe Interrupt Flag Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintenclr](index.html) module"]
pub struct PINTENCLR_SPEC;
impl crate::RegisterSpec for PINTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pintenclr::R](R) reader structure"]
impl crate::Readable for PINTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pintenclr::W](W) writer structure"]
impl crate::Writable for PINTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINTENCLR%s to value 0"]
impl crate::Resettable for PINTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
