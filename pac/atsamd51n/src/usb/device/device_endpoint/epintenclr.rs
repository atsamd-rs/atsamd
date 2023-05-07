#[doc = "Register `EPINTENCLR` reader"]
pub struct R(crate::R<EPINTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPINTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPINTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPINTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPINTENCLR` writer"]
pub struct W(crate::W<EPINTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPINTENCLR_SPEC>;
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
impl From<crate::W<EPINTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPINTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Interrupt Disable"]
pub type TRCPT0_R = crate::BitReader<bool>;
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Interrupt Disable"]
pub type TRCPT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPINTENCLR_SPEC, bool, O>;
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Interrupt Disable"]
pub type TRCPT1_R = crate::BitReader<bool>;
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Interrupt Disable"]
pub type TRCPT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPINTENCLR_SPEC, bool, O>;
#[doc = "Field `TRFAIL0` reader - Error Flow 0 Interrupt Disable"]
pub type TRFAIL0_R = crate::BitReader<bool>;
#[doc = "Field `TRFAIL0` writer - Error Flow 0 Interrupt Disable"]
pub type TRFAIL0_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPINTENCLR_SPEC, bool, O>;
#[doc = "Field `TRFAIL1` reader - Error Flow 1 Interrupt Disable"]
pub type TRFAIL1_R = crate::BitReader<bool>;
#[doc = "Field `TRFAIL1` writer - Error Flow 1 Interrupt Disable"]
pub type TRFAIL1_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPINTENCLR_SPEC, bool, O>;
#[doc = "Field `RXSTP` reader - Received Setup Interrupt Disable"]
pub type RXSTP_R = crate::BitReader<bool>;
#[doc = "Field `RXSTP` writer - Received Setup Interrupt Disable"]
pub type RXSTP_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPINTENCLR_SPEC, bool, O>;
#[doc = "Field `STALL0` reader - Stall 0 In/Out Interrupt Disable"]
pub type STALL0_R = crate::BitReader<bool>;
#[doc = "Field `STALL0` writer - Stall 0 In/Out Interrupt Disable"]
pub type STALL0_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPINTENCLR_SPEC, bool, O>;
#[doc = "Field `STALL1` reader - Stall 1 In/Out Interrupt Disable"]
pub type STALL1_R = crate::BitReader<bool>;
#[doc = "Field `STALL1` writer - Stall 1 In/Out Interrupt Disable"]
pub type STALL1_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPINTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt0(&self) -> TRCPT0_R {
        TRCPT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt1(&self) -> TRCPT1_R {
        TRCPT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail0(&self) -> TRFAIL0_R {
        TRFAIL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail1(&self) -> TRFAIL1_R {
        TRFAIL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Received Setup Interrupt Disable"]
    #[inline(always)]
    pub fn rxstp(&self) -> RXSTP_R {
        RXSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall 0 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall0(&self) -> STALL0_R {
        STALL0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stall 1 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall1(&self) -> STALL1_R {
        STALL1_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt0(&mut self) -> TRCPT0_W<0> {
        TRCPT0_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt1(&mut self) -> TRCPT1_W<1> {
        TRCPT1_W::new(self)
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trfail0(&mut self) -> TRFAIL0_W<2> {
        TRFAIL0_W::new(self)
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trfail1(&mut self) -> TRFAIL1_W<3> {
        TRFAIL1_W::new(self)
    }
    #[doc = "Bit 4 - Received Setup Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstp(&mut self) -> RXSTP_W<4> {
        RXSTP_W::new(self)
    }
    #[doc = "Bit 5 - Stall 0 In/Out Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn stall0(&mut self) -> STALL0_W<5> {
        STALL0_W::new(self)
    }
    #[doc = "Bit 6 - Stall 1 In/Out Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn stall1(&mut self) -> STALL1_W<6> {
        STALL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE_ENDPOINT End Point Interrupt Clear Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintenclr](index.html) module"]
pub struct EPINTENCLR_SPEC;
impl crate::RegisterSpec for EPINTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [epintenclr::R](R) reader structure"]
impl crate::Readable for EPINTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epintenclr::W](W) writer structure"]
impl crate::Writable for EPINTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPINTENCLR to value 0"]
impl crate::Resettable for EPINTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
