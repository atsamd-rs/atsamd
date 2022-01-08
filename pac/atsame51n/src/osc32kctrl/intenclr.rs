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
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready Interrupt Enable"]
pub struct XOSC32KRDY_R(crate::FieldReader<bool, bool>);
impl XOSC32KRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSC32KRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC32KRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC32KRDY` writer - XOSC32K Ready Interrupt Enable"]
pub struct XOSC32KRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC32KRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `XOSC32KFAIL` reader - XOSC32K Clock Failure Detector Interrupt Enable"]
pub struct XOSC32KFAIL_R(crate::FieldReader<bool, bool>);
impl XOSC32KFAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSC32KFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC32KFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC32KFAIL` writer - XOSC32K Clock Failure Detector Interrupt Enable"]
pub struct XOSC32KFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC32KFAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32kfail(&self) -> XOSC32KFAIL_R {
        XOSC32KFAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32krdy(&mut self) -> XOSC32KRDY_W {
        XOSC32KRDY_W { w: self }
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32kfail(&mut self) -> XOSC32KFAIL_W {
        XOSC32KFAIL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
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
