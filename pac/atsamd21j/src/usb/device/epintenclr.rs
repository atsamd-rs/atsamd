#[doc = "Register `EPINTENCLR%s` reader"]
pub type R = crate::R<EPINTENCLR_SPEC>;
#[doc = "Register `EPINTENCLR%s` writer"]
pub type W = crate::W<EPINTENCLR_SPEC>;
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Interrupt Disable"]
pub type TRCPT0_R = crate::BitReader;
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Interrupt Disable"]
pub type TRCPT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Interrupt Disable"]
pub type TRCPT1_R = crate::BitReader;
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Interrupt Disable"]
pub type TRCPT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRFAIL0` reader - Error Flow 0 Interrupt Disable"]
pub type TRFAIL0_R = crate::BitReader;
#[doc = "Field `TRFAIL0` writer - Error Flow 0 Interrupt Disable"]
pub type TRFAIL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRFAIL1` reader - Error Flow 1 Interrupt Disable"]
pub type TRFAIL1_R = crate::BitReader;
#[doc = "Field `TRFAIL1` writer - Error Flow 1 Interrupt Disable"]
pub type TRFAIL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTP` reader - Received Setup Interrupt Disable"]
pub type RXSTP_R = crate::BitReader;
#[doc = "Field `RXSTP` writer - Received Setup Interrupt Disable"]
pub type RXSTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALL0` reader - Stall 0 In/Out Interrupt Disable"]
pub type STALL0_R = crate::BitReader;
#[doc = "Field `STALL0` writer - Stall 0 In/Out Interrupt Disable"]
pub type STALL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALL1` reader - Stall 1 In/Out Interrupt Disable"]
pub type STALL1_R = crate::BitReader;
#[doc = "Field `STALL1` writer - Stall 1 In/Out Interrupt Disable"]
pub type STALL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn trcpt0(&mut self) -> TRCPT0_W<EPINTENCLR_SPEC, 0> {
        TRCPT0_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt1(&mut self) -> TRCPT1_W<EPINTENCLR_SPEC, 1> {
        TRCPT1_W::new(self)
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trfail0(&mut self) -> TRFAIL0_W<EPINTENCLR_SPEC, 2> {
        TRFAIL0_W::new(self)
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trfail1(&mut self) -> TRFAIL1_W<EPINTENCLR_SPEC, 3> {
        TRFAIL1_W::new(self)
    }
    #[doc = "Bit 4 - Received Setup Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstp(&mut self) -> RXSTP_W<EPINTENCLR_SPEC, 4> {
        RXSTP_W::new(self)
    }
    #[doc = "Bit 5 - Stall 0 In/Out Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn stall0(&mut self) -> STALL0_W<EPINTENCLR_SPEC, 5> {
        STALL0_W::new(self)
    }
    #[doc = "Bit 6 - Stall 1 In/Out Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn stall1(&mut self) -> STALL1_W<EPINTENCLR_SPEC, 6> {
        STALL1_W::new(self)
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
#[doc = "DEVICE End Point Interrupt Clear Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epintenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epintenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPINTENCLR_SPEC;
impl crate::RegisterSpec for EPINTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`epintenclr::R`](R) reader structure"]
impl crate::Readable for EPINTENCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epintenclr::W`](W) writer structure"]
impl crate::Writable for EPINTENCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPINTENCLR%s to value 0"]
impl crate::Resettable for EPINTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
