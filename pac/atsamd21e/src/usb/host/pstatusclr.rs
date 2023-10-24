#[doc = "Register `PSTATUSCLR%s` writer"]
pub type W = crate::W<PSTATUSCLR_SPEC>;
#[doc = "Field `CURBK` writer - Curren Bank clear"]
pub type CURBK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFREEZE` writer - Pipe Freeze Clear"]
pub type PFREEZE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BK0RDY` writer - Bank 0 Ready Clear"]
pub type BK0RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BK1RDY` writer - Bank 1 Ready Clear"]
pub type BK1RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 2 - Curren Bank clear"]
    #[inline(always)]
    #[must_use]
    pub fn curbk(&mut self) -> CURBK_W<PSTATUSCLR_SPEC, 2> {
        CURBK_W::new(self)
    }
    #[doc = "Bit 4 - Pipe Freeze Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pfreeze(&mut self) -> PFREEZE_W<PSTATUSCLR_SPEC, 4> {
        PFREEZE_W::new(self)
    }
    #[doc = "Bit 6 - Bank 0 Ready Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bk0rdy(&mut self) -> BK0RDY_W<PSTATUSCLR_SPEC, 6> {
        BK0RDY_W::new(self)
    }
    #[doc = "Bit 7 - Bank 1 Ready Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bk1rdy(&mut self) -> BK1RDY_W<PSTATUSCLR_SPEC, 7> {
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
#[doc = "HOST End Point Pipe Status Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pstatusclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSTATUSCLR_SPEC;
impl crate::RegisterSpec for PSTATUSCLR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`pstatusclr::W`](W) writer structure"]
impl crate::Writable for PSTATUSCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSTATUSCLR%s to value 0"]
impl crate::Resettable for PSTATUSCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
