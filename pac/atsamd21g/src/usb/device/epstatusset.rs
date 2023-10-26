#[doc = "Register `EPSTATUSSET%s` writer"]
pub type W = crate::W<EPSTATUSSET_SPEC>;
#[doc = "Field `DTGLOUT` writer - Data Toggle OUT Set"]
pub type DTGLOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTGLIN` writer - Data Toggle IN Set"]
pub type DTGLIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CURBK` writer - Current Bank Set"]
pub type CURBK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLRQ0` writer - Stall 0 Request Set"]
pub type STALLRQ0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLRQ1` writer - Stall 1 Request Set"]
pub type STALLRQ1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BK0RDY` writer - Bank 0 Ready Set"]
pub type BK0RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BK1RDY` writer - Bank 1 Ready Set"]
pub type BK1RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Data Toggle OUT Set"]
    #[inline(always)]
    #[must_use]
    pub fn dtglout(&mut self) -> DTGLOUT_W<EPSTATUSSET_SPEC, 0> {
        DTGLOUT_W::new(self)
    }
    #[doc = "Bit 1 - Data Toggle IN Set"]
    #[inline(always)]
    #[must_use]
    pub fn dtglin(&mut self) -> DTGLIN_W<EPSTATUSSET_SPEC, 1> {
        DTGLIN_W::new(self)
    }
    #[doc = "Bit 2 - Current Bank Set"]
    #[inline(always)]
    #[must_use]
    pub fn curbk(&mut self) -> CURBK_W<EPSTATUSSET_SPEC, 2> {
        CURBK_W::new(self)
    }
    #[doc = "Bit 4 - Stall 0 Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn stallrq0(&mut self) -> STALLRQ0_W<EPSTATUSSET_SPEC, 4> {
        STALLRQ0_W::new(self)
    }
    #[doc = "Bit 5 - Stall 1 Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn stallrq1(&mut self) -> STALLRQ1_W<EPSTATUSSET_SPEC, 5> {
        STALLRQ1_W::new(self)
    }
    #[doc = "Bit 6 - Bank 0 Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn bk0rdy(&mut self) -> BK0RDY_W<EPSTATUSSET_SPEC, 6> {
        BK0RDY_W::new(self)
    }
    #[doc = "Bit 7 - Bank 1 Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn bk1rdy(&mut self) -> BK1RDY_W<EPSTATUSSET_SPEC, 7> {
        BK1RDY_W::new(self)
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
#[doc = "DEVICE End Point Pipe Status Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epstatusset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPSTATUSSET_SPEC;
impl crate::RegisterSpec for EPSTATUSSET_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`epstatusset::W`](W) writer structure"]
impl crate::Writable for EPSTATUSSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPSTATUSSET%s to value 0"]
impl crate::Resettable for EPSTATUSSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
