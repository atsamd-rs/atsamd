#[doc = "Register `USBHS_HSTADDR1` reader"]
pub struct R(crate::R<USBHS_HSTADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_HSTADDR1` writer"]
pub struct W(crate::W<USBHS_HSTADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTADDR1_SPEC>;
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
impl From<crate::W<USBHS_HSTADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTADDRP0` reader - USB Host Address"]
pub struct HSTADDRP0_R(crate::FieldReader<u8, u8>);
impl HSTADDRP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTADDRP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTADDRP0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTADDRP0` writer - USB Host Address"]
pub struct HSTADDRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `HSTADDRP1` reader - USB Host Address"]
pub struct HSTADDRP1_R(crate::FieldReader<u8, u8>);
impl HSTADDRP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTADDRP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTADDRP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTADDRP1` writer - USB Host Address"]
pub struct HSTADDRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `HSTADDRP2` reader - USB Host Address"]
pub struct HSTADDRP2_R(crate::FieldReader<u8, u8>);
impl HSTADDRP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTADDRP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTADDRP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTADDRP2` writer - USB Host Address"]
pub struct HSTADDRP2_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `HSTADDRP3` reader - USB Host Address"]
pub struct HSTADDRP3_R(crate::FieldReader<u8, u8>);
impl HSTADDRP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTADDRP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTADDRP3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTADDRP3` writer - USB Host Address"]
pub struct HSTADDRP3_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP3_W<'a> {
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
    pub fn hstaddrp0(&self) -> HSTADDRP0_R {
        HSTADDRP0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp1(&self) -> HSTADDRP1_R {
        HSTADDRP1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp2(&self) -> HSTADDRP2_R {
        HSTADDRP2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp3(&self) -> HSTADDRP3_R {
        HSTADDRP3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp0(&mut self) -> HSTADDRP0_W {
        HSTADDRP0_W { w: self }
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp1(&mut self) -> HSTADDRP1_W {
        HSTADDRP1_W { w: self }
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp2(&mut self) -> HSTADDRP2_W {
        HSTADDRP2_W { w: self }
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp3(&mut self) -> HSTADDRP3_W {
        HSTADDRP3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstaddr1](index.html) module"]
pub struct USBHS_HSTADDR1_SPEC;
impl crate::RegisterSpec for USBHS_HSTADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstaddr1::R](R) reader structure"]
impl crate::Readable for USBHS_HSTADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstaddr1::W](W) writer structure"]
impl crate::Writable for USBHS_HSTADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTADDR1 to value 0"]
impl crate::Resettable for USBHS_HSTADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
