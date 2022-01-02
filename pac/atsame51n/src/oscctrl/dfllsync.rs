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
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - ENABLE Synchronization Busy"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DFLLCTRLB` reader - DFLLCTRLB Synchronization Busy"]
pub struct DFLLCTRLB_R(crate::FieldReader<bool, bool>);
impl DFLLCTRLB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLCTRLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLCTRLB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLCTRLB` writer - DFLLCTRLB Synchronization Busy"]
pub struct DFLLCTRLB_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLCTRLB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DFLLVAL` reader - DFLLVAL Synchronization Busy"]
pub struct DFLLVAL_R(crate::FieldReader<bool, bool>);
impl DFLLVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLVAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLVAL` writer - DFLLVAL Synchronization Busy"]
pub struct DFLLVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLVAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DFLLMUL` reader - DFLLMUL Synchronization Busy"]
pub struct DFLLMUL_R(crate::FieldReader<bool, bool>);
impl DFLLMUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLMUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLMUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLMUL` writer - DFLLMUL Synchronization Busy"]
pub struct DFLLMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLMUL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn dfllctrlb(&self) -> DFLLCTRLB_R {
        DFLLCTRLB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllval(&self) -> DFLLVAL_R {
        DFLLVAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllmul(&self) -> DFLLMUL_R {
        DFLLMUL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn dfllctrlb(&mut self) -> DFLLCTRLB_W {
        DFLLCTRLB_W { w: self }
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllval(&mut self) -> DFLLVAL_W {
        DFLLVAL_W { w: self }
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllmul(&mut self) -> DFLLMUL_W {
        DFLLMUL_W { w: self }
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
}
#[doc = "`reset()` method sets DFLLSYNC to value 0"]
impl crate::Resettable for DFLLSYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
