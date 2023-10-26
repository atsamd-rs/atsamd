#[doc = "Register `PSTATUSSET` writer"]
pub type W = crate::W<PSTATUSSET_SPEC>;
#[doc = "Field `DTGL` writer - Data Toggle Set"]
pub type DTGL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CURBK` writer - Current Bank Set"]
pub type CURBK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFREEZE` writer - Pipe Freeze Set"]
pub type PFREEZE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BK0RDY` writer - Bank 0 Ready Set"]
pub type BK0RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BK1RDY` writer - Bank 1 Ready Set"]
pub type BK1RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Data Toggle Set"]
    #[inline(always)]
    #[must_use]
    pub fn dtgl(&mut self) -> DTGL_W<PSTATUSSET_SPEC, 0> {
        DTGL_W::new(self)
    }
    #[doc = "Bit 2 - Current Bank Set"]
    #[inline(always)]
    #[must_use]
    pub fn curbk(&mut self) -> CURBK_W<PSTATUSSET_SPEC, 2> {
        CURBK_W::new(self)
    }
    #[doc = "Bit 4 - Pipe Freeze Set"]
    #[inline(always)]
    #[must_use]
    pub fn pfreeze(&mut self) -> PFREEZE_W<PSTATUSSET_SPEC, 4> {
        PFREEZE_W::new(self)
    }
    #[doc = "Bit 6 - Bank 0 Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn bk0rdy(&mut self) -> BK0RDY_W<PSTATUSSET_SPEC, 6> {
        BK0RDY_W::new(self)
    }
    #[doc = "Bit 7 - Bank 1 Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn bk1rdy(&mut self) -> BK1RDY_W<PSTATUSSET_SPEC, 7> {
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
#[doc = "HOST_PIPE End Point Pipe Status Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pstatusset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSTATUSSET_SPEC;
impl crate::RegisterSpec for PSTATUSSET_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`pstatusset::W`](W) writer structure"]
impl crate::Writable for PSTATUSSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSTATUSSET to value 0"]
impl crate::Resettable for PSTATUSSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
