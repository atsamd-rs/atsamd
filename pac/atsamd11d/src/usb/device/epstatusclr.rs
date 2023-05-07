#[doc = "Register `EPSTATUSCLR%s` writer"]
pub struct W(crate::W<EPSTATUSCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPSTATUSCLR_SPEC>;
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
impl From<crate::W<EPSTATUSCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPSTATUSCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTGLOUT` writer - Data Toggle OUT Clear"]
pub type DTGLOUT_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPSTATUSCLR_SPEC, bool, O>;
#[doc = "Field `DTGLIN` writer - Data Toggle IN Clear"]
pub type DTGLIN_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPSTATUSCLR_SPEC, bool, O>;
#[doc = "Field `CURBK` writer - Current Bank Clear"]
pub type CURBK_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPSTATUSCLR_SPEC, bool, O>;
#[doc = "Field `STALLRQ0` writer - Stall 0 Request Clear"]
pub type STALLRQ0_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPSTATUSCLR_SPEC, bool, O>;
#[doc = "Field `STALLRQ1` writer - Stall 1 Request Clear"]
pub type STALLRQ1_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPSTATUSCLR_SPEC, bool, O>;
#[doc = "Field `BK0RDY` writer - Bank 0 Ready Clear"]
pub type BK0RDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPSTATUSCLR_SPEC, bool, O>;
#[doc = "Field `BK1RDY` writer - Bank 1 Ready Clear"]
pub type BK1RDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, EPSTATUSCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Data Toggle OUT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtglout(&mut self) -> DTGLOUT_W<0> {
        DTGLOUT_W::new(self)
    }
    #[doc = "Bit 1 - Data Toggle IN Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtglin(&mut self) -> DTGLIN_W<1> {
        DTGLIN_W::new(self)
    }
    #[doc = "Bit 2 - Current Bank Clear"]
    #[inline(always)]
    #[must_use]
    pub fn curbk(&mut self) -> CURBK_W<2> {
        CURBK_W::new(self)
    }
    #[doc = "Bit 4 - Stall 0 Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stallrq0(&mut self) -> STALLRQ0_W<4> {
        STALLRQ0_W::new(self)
    }
    #[doc = "Bit 5 - Stall 1 Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stallrq1(&mut self) -> STALLRQ1_W<5> {
        STALLRQ1_W::new(self)
    }
    #[doc = "Bit 6 - Bank 0 Ready Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bk0rdy(&mut self) -> BK0RDY_W<6> {
        BK0RDY_W::new(self)
    }
    #[doc = "Bit 7 - Bank 1 Ready Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bk1rdy(&mut self) -> BK1RDY_W<7> {
        BK1RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE End Point Pipe Status Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatusclr](index.html) module"]
pub struct EPSTATUSCLR_SPEC;
impl crate::RegisterSpec for EPSTATUSCLR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [epstatusclr::W](W) writer structure"]
impl crate::Writable for EPSTATUSCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPSTATUSCLR%s to value 0"]
impl crate::Resettable for EPSTATUSCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
