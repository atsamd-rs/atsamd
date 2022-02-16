#[doc = "Register `UART_BRGR` reader"]
pub struct R(crate::R<UART_BRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_BRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_BRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_BRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_BRGR` writer"]
pub struct W(crate::W<UART_BRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_BRGR_SPEC>;
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
impl From<crate::W<UART_BRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_BRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD` reader - Clock Divisor"]
pub struct CD_R(crate::FieldReader<u16, u16>);
impl CD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD` writer - Clock Divisor"]
pub struct CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W {
        CD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_brgr](index.html) module"]
pub struct UART_BRGR_SPEC;
impl crate::RegisterSpec for UART_BRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_brgr::R](R) reader structure"]
impl crate::Readable for UART_BRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_brgr::W](W) writer structure"]
impl crate::Writable for UART_BRGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_BRGR to value 0"]
impl crate::Resettable for UART_BRGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
