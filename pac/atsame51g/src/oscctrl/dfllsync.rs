#[doc = "Register `DFLLSYNC` reader"]
pub struct R(crate::R<DFLLSYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLSYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLSYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLSYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLSYNC` writer"]
pub struct W(crate::W<DFLLSYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLSYNC_SPEC>;
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
impl From<crate::W<DFLLSYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLSYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - ENABLE Synchronization Busy"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - ENABLE Synchronization Busy"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLSYNC_SPEC, bool, O>;
#[doc = "Field `DFLLCTRLB` reader - DFLLCTRLB Synchronization Busy"]
pub type DFLLCTRLB_R = crate::BitReader<bool>;
#[doc = "Field `DFLLCTRLB` writer - DFLLCTRLB Synchronization Busy"]
pub type DFLLCTRLB_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLSYNC_SPEC, bool, O>;
#[doc = "Field `DFLLVAL` reader - DFLLVAL Synchronization Busy"]
pub type DFLLVAL_R = crate::BitReader<bool>;
#[doc = "Field `DFLLVAL` writer - DFLLVAL Synchronization Busy"]
pub type DFLLVAL_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLSYNC_SPEC, bool, O>;
#[doc = "Field `DFLLMUL` reader - DFLLMUL Synchronization Busy"]
pub type DFLLMUL_R = crate::BitReader<bool>;
#[doc = "Field `DFLLMUL` writer - DFLLMUL Synchronization Busy"]
pub type DFLLMUL_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLSYNC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn dfllctrlb(&self) -> DFLLCTRLB_R {
        DFLLCTRLB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllval(&self) -> DFLLVAL_R {
        DFLLVAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllmul(&self) -> DFLLMUL_R {
        DFLLMUL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn dfllctrlb(&mut self) -> DFLLCTRLB_W<2> {
        DFLLCTRLB_W::new(self)
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn dfllval(&mut self) -> DFLLVAL_W<3> {
        DFLLVAL_W::new(self)
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn dfllmul(&mut self) -> DFLLMUL_W<4> {
        DFLLMUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL48M Synchronization\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllsync](index.html) module"]
pub struct DFLLSYNC_SPEC;
impl crate::RegisterSpec for DFLLSYNC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dfllsync::R](R) reader structure"]
impl crate::Readable for DFLLSYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllsync::W](W) writer structure"]
impl crate::Writable for DFLLSYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLSYNC to value 0"]
impl crate::Resettable for DFLLSYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
