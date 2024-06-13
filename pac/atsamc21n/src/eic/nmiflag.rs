#[doc = "Register `NMIFLAG` reader"]
pub struct R(crate::R<NMIFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMIFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMIFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMIFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMIFLAG` writer"]
pub struct W(crate::W<NMIFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMIFLAG_SPEC>;
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
impl From<crate::W<NMIFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMIFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMI` reader - Non-Maskable Interrupt"]
pub struct NMI_R(crate::FieldReader<bool, bool>);
impl NMI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMI` writer - Non-Maskable Interrupt"]
pub struct NMI_W<'a> {
    w: &'a mut W,
}
impl<'a> NMI_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Non-Maskable Interrupt"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Maskable Interrupt"]
    #[inline(always)]
    pub fn nmi(&mut self) -> NMI_W {
        NMI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Maskable Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmiflag](index.html) module"]
pub struct NMIFLAG_SPEC;
impl crate::RegisterSpec for NMIFLAG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nmiflag::R](R) reader structure"]
impl crate::Readable for NMIFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmiflag::W](W) writer structure"]
impl crate::Writable for NMIFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMIFLAG to value 0"]
impl crate::Resettable for NMIFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
