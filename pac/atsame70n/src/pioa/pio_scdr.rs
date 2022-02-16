#[doc = "Register `PIO_SCDR` reader"]
pub struct R(crate::R<PIO_SCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_SCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_SCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_SCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO_SCDR` writer"]
pub struct W(crate::W<PIO_SCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO_SCDR_SPEC>;
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
impl From<crate::W<PIO_SCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO_SCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Slow Clock Divider Selection for Debouncing"]
pub struct DIV_R(crate::FieldReader<u16, u16>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Slow Clock Divider Selection for Debouncing"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Slow Clock Divider Selection for Debouncing"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Slow Clock Divider Selection for Debouncing"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slow Clock Divider Debouncing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_scdr](index.html) module"]
pub struct PIO_SCDR_SPEC;
impl crate::RegisterSpec for PIO_SCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_scdr::R](R) reader structure"]
impl crate::Readable for PIO_SCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio_scdr::W](W) writer structure"]
impl crate::Writable for PIO_SCDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO_SCDR to value 0"]
impl crate::Resettable for PIO_SCDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
