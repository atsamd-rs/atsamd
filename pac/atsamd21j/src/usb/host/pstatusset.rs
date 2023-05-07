#[doc = "Register `PSTATUSSET%s` writer"]
pub struct W(crate::W<PSTATUSSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSTATUSSET_SPEC>;
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
impl From<crate::W<PSTATUSSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSTATUSSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTGL` writer - Data Toggle Set"]
pub type DTGL_W<'a, const O: u8> = crate::BitWriter<'a, u8, PSTATUSSET_SPEC, bool, O>;
#[doc = "Field `CURBK` writer - Current Bank Set"]
pub type CURBK_W<'a, const O: u8> = crate::BitWriter<'a, u8, PSTATUSSET_SPEC, bool, O>;
#[doc = "Field `PFREEZE` writer - Pipe Freeze Set"]
pub type PFREEZE_W<'a, const O: u8> = crate::BitWriter<'a, u8, PSTATUSSET_SPEC, bool, O>;
#[doc = "Field `BK0RDY` writer - Bank 0 Ready Set"]
pub type BK0RDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, PSTATUSSET_SPEC, bool, O>;
#[doc = "Field `BK1RDY` writer - Bank 1 Ready Set"]
pub type BK1RDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, PSTATUSSET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Data Toggle Set"]
    #[inline(always)]
    #[must_use]
    pub fn dtgl(&mut self) -> DTGL_W<0> {
        DTGL_W::new(self)
    }
    #[doc = "Bit 2 - Current Bank Set"]
    #[inline(always)]
    #[must_use]
    pub fn curbk(&mut self) -> CURBK_W<2> {
        CURBK_W::new(self)
    }
    #[doc = "Bit 4 - Pipe Freeze Set"]
    #[inline(always)]
    #[must_use]
    pub fn pfreeze(&mut self) -> PFREEZE_W<4> {
        PFREEZE_W::new(self)
    }
    #[doc = "Bit 6 - Bank 0 Ready Set"]
    #[inline(always)]
    #[must_use]
    pub fn bk0rdy(&mut self) -> BK0RDY_W<6> {
        BK0RDY_W::new(self)
    }
    #[doc = "Bit 7 - Bank 1 Ready Set"]
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
#[doc = "HOST End Point Pipe Status Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstatusset](index.html) module"]
pub struct PSTATUSSET_SPEC;
impl crate::RegisterSpec for PSTATUSSET_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [pstatusset::W](W) writer structure"]
impl crate::Writable for PSTATUSSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSTATUSSET%s to value 0"]
impl crate::Resettable for PSTATUSSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
