#[doc = "Register `USBHS_HSTADDR3` reader"]
pub struct R(crate::R<USBHS_HSTADDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTADDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTADDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTADDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_HSTADDR3` writer"]
pub struct W(crate::W<USBHS_HSTADDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTADDR3_SPEC>;
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
impl From<crate::W<USBHS_HSTADDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTADDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTADDRP8` reader - USB Host Address"]
pub struct HSTADDRP8_R(crate::FieldReader<u8, u8>);
impl HSTADDRP8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTADDRP8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTADDRP8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTADDRP8` writer - USB Host Address"]
pub struct HSTADDRP8_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `HSTADDRP9` reader - USB Host Address"]
pub struct HSTADDRP9_R(crate::FieldReader<u8, u8>);
impl HSTADDRP9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTADDRP9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTADDRP9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTADDRP9` writer - USB Host Address"]
pub struct HSTADDRP9_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp8(&self) -> HSTADDRP8_R {
        HSTADDRP8_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp9(&self) -> HSTADDRP9_R {
        HSTADDRP9_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp8(&mut self) -> HSTADDRP8_W {
        HSTADDRP8_W { w: self }
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp9(&mut self) -> HSTADDRP9_W {
        HSTADDRP9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstaddr3](index.html) module"]
pub struct USBHS_HSTADDR3_SPEC;
impl crate::RegisterSpec for USBHS_HSTADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstaddr3::R](R) reader structure"]
impl crate::Readable for USBHS_HSTADDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstaddr3::W](W) writer structure"]
impl crate::Writable for USBHS_HSTADDR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTADDR3 to value 0"]
impl crate::Resettable for USBHS_HSTADDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
