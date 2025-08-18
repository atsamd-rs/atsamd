#[doc = "Register `TXEFC` reader"]
pub struct R(crate::R<TXEFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXEFC` writer"]
pub struct W(crate::W<TXEFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXEFC_SPEC>;
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
impl From<crate::W<TXEFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXEFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFSA` reader - Event FIFO Start Address"]
pub struct EFSA_R(crate::FieldReader<u16, u16>);
impl EFSA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        EFSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFSA` writer - Event FIFO Start Address"]
pub struct EFSA_W<'a> {
    w: &'a mut W,
}
impl<'a> EFSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `EFS` reader - Event FIFO Size"]
pub struct EFS_R(crate::FieldReader<u8, u8>);
impl EFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFS` writer - Event FIFO Size"]
pub struct EFS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `EFWM` reader - Event FIFO Watermark"]
pub struct EFWM_R(crate::FieldReader<u8, u8>);
impl EFWM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EFWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFWM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFWM` writer - Event FIFO Watermark"]
pub struct EFWM_W<'a> {
    w: &'a mut W,
}
impl<'a> EFWM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Event FIFO Start Address"]
    #[inline(always)]
    pub fn efsa(&self) -> EFSA_R {
        EFSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Event FIFO Size"]
    #[inline(always)]
    pub fn efs(&self) -> EFS_R {
        EFS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark"]
    #[inline(always)]
    pub fn efwm(&self) -> EFWM_R {
        EFWM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Event FIFO Start Address"]
    #[inline(always)]
    pub fn efsa(&mut self) -> EFSA_W {
        EFSA_W { w: self }
    }
    #[doc = "Bits 16:21 - Event FIFO Size"]
    #[inline(always)]
    pub fn efs(&mut self) -> EFS_W {
        EFS_W { w: self }
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark"]
    #[inline(always)]
    pub fn efwm(&mut self) -> EFWM_W {
        EFWM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Event FIFO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefc](index.html) module"]
pub struct TXEFC_SPEC;
impl crate::RegisterSpec for TXEFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txefc::R](R) reader structure"]
impl crate::Readable for TXEFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txefc::W](W) writer structure"]
impl crate::Writable for TXEFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXEFC to value 0"]
impl crate::Resettable for TXEFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
