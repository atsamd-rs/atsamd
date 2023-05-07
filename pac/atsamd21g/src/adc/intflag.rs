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
#[doc = "Field `RESRDY` reader - Result Ready"]
pub type RESRDY_R = crate::BitReader<bool>;
#[doc = "Field `RESRDY` writer - Result Ready"]
pub type RESRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
#[doc = "Field `OVERRUN` reader - Overrun"]
pub type OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN` writer - Overrun"]
pub type OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
#[doc = "Field `WINMON` reader - Window Monitor"]
pub type WINMON_R = crate::BitReader<bool>;
#[doc = "Field `WINMON` writer - Window Monitor"]
pub type WINMON_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready"]
pub type SYNCRDY_R = crate::BitReader<bool>;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready"]
pub type SYNCRDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Result Ready"]
    #[inline(always)]
    pub fn resrdy(&self) -> RESRDY_R {
        RESRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Monitor"]
    #[inline(always)]
    pub fn winmon(&self) -> WINMON_R {
        WINMON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Ready"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SYNCRDY_R {
        SYNCRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Ready"]
    #[inline(always)]
    #[must_use]
    pub fn resrdy(&mut self) -> RESRDY_W<0> {
        RESRDY_W::new(self)
    }
    #[doc = "Bit 1 - Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<1> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 2 - Window Monitor"]
    #[inline(always)]
    #[must_use]
    pub fn winmon(&mut self) -> WINMON_W<2> {
        WINMON_W::new(self)
    }
    #[doc = "Bit 3 - Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn syncrdy(&mut self) -> SYNCRDY_W<3> {
        SYNCRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u8;
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
