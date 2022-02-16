#[doc = "Register `USBHS_HSTFNUM` reader"]
pub struct R(crate::R<USBHS_HSTFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTFNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTFNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_HSTFNUM` writer"]
pub struct W(crate::W<USBHS_HSTFNUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTFNUM_SPEC>;
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
impl From<crate::W<USBHS_HSTFNUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTFNUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub struct MFNUM_R(crate::FieldReader<u8, u8>);
impl MFNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MFNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFNUM` writer - Micro Frame Number"]
pub struct MFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> MFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `FNUM` reader - Frame Number"]
pub struct FNUM_R(crate::FieldReader<u16, u16>);
impl FNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNUM` writer - Frame Number"]
pub struct FNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> FNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 3)) | ((value as u32 & 0x07ff) << 3);
        self.w
    }
}
#[doc = "Field `FLENHIGH` reader - Frame Length"]
pub struct FLENHIGH_R(crate::FieldReader<u8, u8>);
impl FLENHIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLENHIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLENHIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLENHIGH` writer - Frame Length"]
pub struct FLENHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLENHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&self) -> FLENHIGH_R {
        FLENHIGH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&mut self) -> MFNUM_W {
        MFNUM_W { w: self }
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&mut self) -> FNUM_W {
        FNUM_W { w: self }
    }
    #[doc = "Bits 16:23 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&mut self) -> FLENHIGH_W {
        FLENHIGH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstfnum](index.html) module"]
pub struct USBHS_HSTFNUM_SPEC;
impl crate::RegisterSpec for USBHS_HSTFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstfnum::R](R) reader structure"]
impl crate::Readable for USBHS_HSTFNUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstfnum::W](W) writer structure"]
impl crate::Writable for USBHS_HSTFNUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTFNUM to value 0"]
impl crate::Resettable for USBHS_HSTFNUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
