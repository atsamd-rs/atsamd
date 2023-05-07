#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QERR` reader - Quadrature Error Flag"]
pub type QERR_R = crate::BitReader<bool>;
#[doc = "Field `QERR` writer - Quadrature Error Flag"]
pub type QERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `IDXERR` reader - Index Error Flag"]
pub type IDXERR_R = crate::BitReader<bool>;
#[doc = "Field `IDXERR` writer - Index Error Flag"]
pub type IDXERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `MPERR` reader - Missing Pulse Error flag"]
pub type MPERR_R = crate::BitReader<bool>;
#[doc = "Field `MPERR` writer - Missing Pulse Error flag"]
pub type MPERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `WINERR` reader - Window Error Flag"]
pub type WINERR_R = crate::BitReader<bool>;
#[doc = "Field `WINERR` writer - Window Error Flag"]
pub type WINERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `HERR` reader - Hall Error Flag"]
pub type HERR_R = crate::BitReader<bool>;
#[doc = "Field `HERR` writer - Hall Error Flag"]
pub type HERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Stop"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Stop"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `DIR` reader - Direction Status Flag"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Direction Status Flag"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `PRESCBUFV` reader - Prescaler Buffer Valid"]
pub type PRESCBUFV_R = crate::BitReader<bool>;
#[doc = "Field `PRESCBUFV` writer - Prescaler Buffer Valid"]
pub type PRESCBUFV_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `FILTERBUFV` reader - Filter Buffer Valid"]
pub type FILTERBUFV_R = crate::BitReader<bool>;
#[doc = "Field `FILTERBUFV` writer - Filter Buffer Valid"]
pub type FILTERBUFV_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `CCBUFV0` reader - Compare Channel 0 Buffer Valid"]
pub type CCBUFV0_R = crate::BitReader<bool>;
#[doc = "Field `CCBUFV0` writer - Compare Channel 0 Buffer Valid"]
pub type CCBUFV0_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
#[doc = "Field `CCBUFV1` reader - Compare Channel 1 Buffer Valid"]
pub type CCBUFV1_R = crate::BitReader<bool>;
#[doc = "Field `CCBUFV1` writer - Compare Channel 1 Buffer Valid"]
pub type CCBUFV1_W<'a, const O: u8> = crate::BitWriter<'a, u16, STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Quadrature Error Flag"]
    #[inline(always)]
    pub fn qerr(&self) -> QERR_R {
        QERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Index Error Flag"]
    #[inline(always)]
    pub fn idxerr(&self) -> IDXERR_R {
        IDXERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Missing Pulse Error flag"]
    #[inline(always)]
    pub fn mperr(&self) -> MPERR_R {
        MPERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Window Error Flag"]
    #[inline(always)]
    pub fn winerr(&self) -> WINERR_R {
        WINERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hall Error Flag"]
    #[inline(always)]
    pub fn herr(&self) -> HERR_R {
        HERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Direction Status Flag"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Prescaler Buffer Valid"]
    #[inline(always)]
    pub fn prescbufv(&self) -> PRESCBUFV_R {
        PRESCBUFV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter Buffer Valid"]
    #[inline(always)]
    pub fn filterbufv(&self) -> FILTERBUFV_R {
        FILTERBUFV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv0(&self) -> CCBUFV0_R {
        CCBUFV0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv1(&self) -> CCBUFV1_R {
        CCBUFV1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quadrature Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn qerr(&mut self) -> QERR_W<0> {
        QERR_W::new(self)
    }
    #[doc = "Bit 1 - Index Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn idxerr(&mut self) -> IDXERR_W<1> {
        IDXERR_W::new(self)
    }
    #[doc = "Bit 2 - Missing Pulse Error flag"]
    #[inline(always)]
    #[must_use]
    pub fn mperr(&mut self) -> MPERR_W<2> {
        MPERR_W::new(self)
    }
    #[doc = "Bit 4 - Window Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn winerr(&mut self) -> WINERR_W<4> {
        WINERR_W::new(self)
    }
    #[doc = "Bit 5 - Hall Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn herr(&mut self) -> HERR_W<5> {
        HERR_W::new(self)
    }
    #[doc = "Bit 6 - Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<6> {
        STOP_W::new(self)
    }
    #[doc = "Bit 7 - Direction Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<7> {
        DIR_W::new(self)
    }
    #[doc = "Bit 8 - Prescaler Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn prescbufv(&mut self) -> PRESCBUFV_W<8> {
        PRESCBUFV_W::new(self)
    }
    #[doc = "Bit 9 - Filter Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn filterbufv(&mut self) -> FILTERBUFV_W<9> {
        FILTERBUFV_W::new(self)
    }
    #[doc = "Bit 12 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv0(&mut self) -> CCBUFV0_W<12> {
        CCBUFV0_W::new(self)
    }
    #[doc = "Bit 13 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv1(&mut self) -> CCBUFV1_W<13> {
        CCBUFV1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x40"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
