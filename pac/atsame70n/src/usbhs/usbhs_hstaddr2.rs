#[doc = "Register `USBHS_HSTADDR2` reader"]
pub struct R(crate::R<USBHS_HSTADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTADDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_HSTADDR2` writer"]
pub struct W(crate::W<USBHS_HSTADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTADDR2_SPEC>;
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
impl From<crate::W<USBHS_HSTADDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTADDRP4` reader - USB Host Address"]
pub struct HSTADDRP4_R(crate::FieldReader<u8, u8>);
impl HSTADDRP4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTADDRP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTADDRP4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTADDRP4` writer - USB Host Address"]
pub struct HSTADDRP4_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `HSTADDRP5` reader - USB Host Address"]
pub struct HSTADDRP5_R(crate::FieldReader<u8, u8>);
impl HSTADDRP5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTADDRP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTADDRP5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTADDRP5` writer - USB Host Address"]
pub struct HSTADDRP5_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `HSTADDRP6` reader - USB Host Address"]
pub struct HSTADDRP6_R(crate::FieldReader<u8, u8>);
impl HSTADDRP6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTADDRP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTADDRP6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTADDRP6` writer - USB Host Address"]
pub struct HSTADDRP6_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `HSTADDRP7` reader - USB Host Address"]
pub struct HSTADDRP7_R(crate::FieldReader<u8, u8>);
impl HSTADDRP7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTADDRP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTADDRP7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTADDRP7` writer - USB Host Address"]
pub struct HSTADDRP7_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp4(&self) -> HSTADDRP4_R {
        HSTADDRP4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp5(&self) -> HSTADDRP5_R {
        HSTADDRP5_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp6(&self) -> HSTADDRP6_R {
        HSTADDRP6_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp7(&self) -> HSTADDRP7_R {
        HSTADDRP7_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp4(&mut self) -> HSTADDRP4_W {
        HSTADDRP4_W { w: self }
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp5(&mut self) -> HSTADDRP5_W {
        HSTADDRP5_W { w: self }
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp6(&mut self) -> HSTADDRP6_W {
        HSTADDRP6_W { w: self }
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp7(&mut self) -> HSTADDRP7_W {
        HSTADDRP7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstaddr2](index.html) module"]
pub struct USBHS_HSTADDR2_SPEC;
impl crate::RegisterSpec for USBHS_HSTADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstaddr2::R](R) reader structure"]
impl crate::Readable for USBHS_HSTADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstaddr2::W](W) writer structure"]
impl crate::Writable for USBHS_HSTADDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTADDR2 to value 0"]
impl crate::Resettable for USBHS_HSTADDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
