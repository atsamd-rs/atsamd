#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESRDY` reader - Result Ready Interrupt Enable"]
pub struct RESRDY_R(crate::FieldReader<bool, bool>);
impl RESRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESRDY` writer - Result Ready Interrupt Enable"]
pub struct RESRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `OVERRUN` reader - Overrun Interrupt Enable"]
pub struct OVERRUN_R(crate::FieldReader<bool, bool>);
impl OVERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN` writer - Overrun Interrupt Enable"]
pub struct OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN_W<'a> {
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
#[doc = "Field `WINMON` reader - Window Monitor Interrupt Enable"]
pub struct WINMON_R(crate::FieldReader<bool, bool>);
impl WINMON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINMON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINMON` writer - Window Monitor Interrupt Enable"]
pub struct WINMON_W<'a> {
    w: &'a mut W,
}
impl<'a> WINMON_W<'a> {
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
#[doc = "Field `SYNCRDY` reader - Synchronization Ready Interrupt Enable"]
pub struct SYNCRDY_R(crate::FieldReader<bool, bool>);
impl SYNCRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCRDY` writer - Synchronization Ready Interrupt Enable"]
pub struct SYNCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCRDY_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Result Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy(&self) -> RESRDY_R {
        RESRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Window Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn winmon(&self) -> WINMON_R {
        WINMON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SYNCRDY_R {
        SYNCRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy(&mut self) -> RESRDY_W {
        RESRDY_W { w: self }
    }
    #[doc = "Bit 1 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W {
        OVERRUN_W { w: self }
    }
    #[doc = "Bit 2 - Window Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn winmon(&mut self) -> WINMON_W {
        WINMON_W { w: self }
    }
    #[doc = "Bit 3 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&mut self) -> SYNCRDY_W {
        SYNCRDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
