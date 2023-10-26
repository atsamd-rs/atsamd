#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<INTENSET_SPEC>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<INTENSET_SPEC>;
#[doc = "Field `SUSPEND` reader - Suspend Interrupt Enable"]
pub type SUSPEND_R = crate::BitReader;
#[doc = "Field `SUSPEND` writer - Suspend Interrupt Enable"]
pub type SUSPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSOF` reader - Micro Start of Frame Interrupt Enable in High Speed Mode"]
pub type MSOF_R = crate::BitReader;
#[doc = "Field `MSOF` writer - Micro Start of Frame Interrupt Enable in High Speed Mode"]
pub type MSOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOF` reader - Start Of Frame Interrupt Enable"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - Start Of Frame Interrupt Enable"]
pub type SOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EORST` reader - End of Reset Interrupt Enable"]
pub type EORST_R = crate::BitReader;
#[doc = "Field `EORST` writer - End of Reset Interrupt Enable"]
pub type EORST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUP` reader - Wake Up Interrupt Enable"]
pub type WAKEUP_R = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Wake Up Interrupt Enable"]
pub type WAKEUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EORSM` reader - End Of Resume Interrupt Enable"]
pub type EORSM_R = crate::BitReader;
#[doc = "Field `EORSM` writer - End Of Resume Interrupt Enable"]
pub type EORSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPRSM` reader - Upstream Resume Interrupt Enable"]
pub type UPRSM_R = crate::BitReader;
#[doc = "Field `UPRSM` writer - Upstream Resume Interrupt Enable"]
pub type UPRSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAMACER` reader - Ram Access Interrupt Enable"]
pub type RAMACER_R = crate::BitReader;
#[doc = "Field `RAMACER` writer - Ram Access Interrupt Enable"]
pub type RAMACER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPMNYET` reader - Link Power Management Not Yet Interrupt Enable"]
pub type LPMNYET_R = crate::BitReader;
#[doc = "Field `LPMNYET` writer - Link Power Management Not Yet Interrupt Enable"]
pub type LPMNYET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPMSUSP` reader - Link Power Management Suspend Interrupt Enable"]
pub type LPMSUSP_R = crate::BitReader;
#[doc = "Field `LPMSUSP` writer - Link Power Management Suspend Interrupt Enable"]
pub type LPMSUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Enable in High Speed Mode"]
    #[inline(always)]
    pub fn msof(&self) -> MSOF_R {
        MSOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn eorst(&self) -> EORST_R {
        EORST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake Up Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn eorsm(&self) -> EORSM_R {
        EORSM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ram Access Interrupt Enable"]
    #[inline(always)]
    pub fn ramacer(&self) -> RAMACER_R {
        RAMACER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Link Power Management Not Yet Interrupt Enable"]
    #[inline(always)]
    pub fn lpmnyet(&self) -> LPMNYET_R {
        LPMNYET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Link Power Management Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn lpmsusp(&self) -> LPMSUSP_R {
        LPMSUSP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SUSPEND_W<INTENSET_SPEC, 0> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Enable in High Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn msof(&mut self) -> MSOF_W<INTENSET_SPEC, 1> {
        MSOF_W::new(self)
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<INTENSET_SPEC, 2> {
        SOF_W::new(self)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eorst(&mut self) -> EORST_W<INTENSET_SPEC, 3> {
        EORST_W::new(self)
    }
    #[doc = "Bit 4 - Wake Up Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<INTENSET_SPEC, 4> {
        WAKEUP_W::new(self)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eorsm(&mut self) -> EORSM_W<INTENSET_SPEC, 5> {
        EORSM_W::new(self)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uprsm(&mut self) -> UPRSM_W<INTENSET_SPEC, 6> {
        UPRSM_W::new(self)
    }
    #[doc = "Bit 7 - Ram Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramacer(&mut self) -> RAMACER_W<INTENSET_SPEC, 7> {
        RAMACER_W::new(self)
    }
    #[doc = "Bit 8 - Link Power Management Not Yet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmnyet(&mut self) -> LPMNYET_W<INTENSET_SPEC, 8> {
        LPMNYET_W::new(self)
    }
    #[doc = "Bit 9 - Link Power Management Suspend Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmsusp(&mut self) -> LPMSUSP_W<INTENSET_SPEC, 9> {
        LPMSUSP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DEVICE Device Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u16;
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
