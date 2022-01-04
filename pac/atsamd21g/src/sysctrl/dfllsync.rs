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
pub struct READREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> READREQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 7 - Read Request"]
    #[inline(always)]
    pub fn readreq(&mut self) -> READREQ_W {
        READREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL48M Synchronization\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllsync](index.html) module"]
pub struct DFLLSYNC_SPEC;
impl crate::RegisterSpec for DFLLSYNC_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [dfllsync::W](W) writer structure"]
impl crate::Writable for DFLLSYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLLSYNC to value 0"]
impl crate::Resettable for DFLLSYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
