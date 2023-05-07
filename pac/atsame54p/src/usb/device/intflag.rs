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
#[doc = "Field `SUSPEND` reader - Suspend"]
pub type SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `SUSPEND` writer - Suspend"]
pub type SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `MSOF` reader - Micro Start of Frame in High Speed Mode"]
pub type MSOF_R = crate::BitReader<bool>;
#[doc = "Field `MSOF` writer - Micro Start of Frame in High Speed Mode"]
pub type MSOF_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `SOF` reader - Start Of Frame"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - Start Of Frame"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EORST` reader - End of Reset"]
pub type EORST_R = crate::BitReader<bool>;
#[doc = "Field `EORST` writer - End of Reset"]
pub type EORST_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `WAKEUP` reader - Wake Up"]
pub type WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP` writer - Wake Up"]
pub type WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EORSM` reader - End Of Resume"]
pub type EORSM_R = crate::BitReader<bool>;
#[doc = "Field `EORSM` writer - End Of Resume"]
pub type EORSM_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `UPRSM` reader - Upstream Resume"]
pub type UPRSM_R = crate::BitReader<bool>;
#[doc = "Field `UPRSM` writer - Upstream Resume"]
pub type UPRSM_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `RAMACER` reader - Ram Access"]
pub type RAMACER_R = crate::BitReader<bool>;
#[doc = "Field `RAMACER` writer - Ram Access"]
pub type RAMACER_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `LPMNYET` reader - Link Power Management Not Yet"]
pub type LPMNYET_R = crate::BitReader<bool>;
#[doc = "Field `LPMNYET` writer - Link Power Management Not Yet"]
pub type LPMNYET_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `LPMSUSP` reader - Link Power Management Suspend"]
pub type LPMSUSP_R = crate::BitReader<bool>;
#[doc = "Field `LPMSUSP` writer - Link Power Management Suspend"]
pub type LPMSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Suspend"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame in High Speed Mode"]
    #[inline(always)]
    pub fn msof(&self) -> MSOF_R {
        MSOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Reset"]
    #[inline(always)]
    pub fn eorst(&self) -> EORST_R {
        EORST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake Up"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Of Resume"]
    #[inline(always)]
    pub fn eorsm(&self) -> EORSM_R {
        EORSM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ram Access"]
    #[inline(always)]
    pub fn ramacer(&self) -> RAMACER_R {
        RAMACER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Link Power Management Not Yet"]
    #[inline(always)]
    pub fn lpmnyet(&self) -> LPMNYET_R {
        LPMNYET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Link Power Management Suspend"]
    #[inline(always)]
    pub fn lpmsusp(&self) -> LPMSUSP_R {
        LPMSUSP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SUSPEND_W<0> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 1 - Micro Start of Frame in High Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn msof(&mut self) -> MSOF_W<1> {
        MSOF_W::new(self)
    }
    #[doc = "Bit 2 - Start Of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<2> {
        SOF_W::new(self)
    }
    #[doc = "Bit 3 - End of Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorst(&mut self) -> EORST_W<3> {
        EORST_W::new(self)
    }
    #[doc = "Bit 4 - Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<4> {
        WAKEUP_W::new(self)
    }
    #[doc = "Bit 5 - End Of Resume"]
    #[inline(always)]
    #[must_use]
    pub fn eorsm(&mut self) -> EORSM_W<5> {
        EORSM_W::new(self)
    }
    #[doc = "Bit 6 - Upstream Resume"]
    #[inline(always)]
    #[must_use]
    pub fn uprsm(&mut self) -> UPRSM_W<6> {
        UPRSM_W::new(self)
    }
    #[doc = "Bit 7 - Ram Access"]
    #[inline(always)]
    #[must_use]
    pub fn ramacer(&mut self) -> RAMACER_W<7> {
        RAMACER_W::new(self)
    }
    #[doc = "Bit 8 - Link Power Management Not Yet"]
    #[inline(always)]
    #[must_use]
    pub fn lpmnyet(&mut self) -> LPMNYET_W<8> {
        LPMNYET_W::new(self)
    }
    #[doc = "Bit 9 - Link Power Management Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn lpmsusp(&mut self) -> LPMSUSP_W<9> {
        LPMSUSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE Device Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u16;
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
