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
#[doc = "Field `READREQ` writer - Read Request"]
pub type READREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, DFLLSYNC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 7 - Read Request"]
    #[inline(always)]
    #[must_use]
    pub fn readreq(&mut self) -> READREQ_W<7> {
        READREQ_W::new(self)
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
