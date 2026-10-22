#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSCRDY` reader - XOSC Ready"]
pub struct XOSCRDY_R(crate::FieldReader<bool, bool>);
impl XOSCRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCRDY` writer - XOSC Ready"]
pub struct XOSCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCRDY_W<'a> {
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
#[doc = "Field `XOSCFAIL` reader - XOSC Clock Failure Detector"]
pub struct XOSCFAIL_R(crate::FieldReader<bool, bool>);
impl XOSCFAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCFAIL` writer - XOSC Clock Failure Detector"]
pub struct XOSCFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCFAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `OSC48MRDY` reader - OSC48M Ready"]
pub struct OSC48MRDY_R(crate::FieldReader<bool, bool>);
impl OSC48MRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC48MRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC48MRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC48MRDY` writer - OSC48M Ready"]
pub struct OSC48MRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC48MRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise"]
pub struct DPLLLCKR_R(crate::FieldReader<bool, bool>);
impl DPLLLCKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLCKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLCKR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLCKR` writer - DPLL Lock Rise"]
pub struct DPLLLCKR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLCKR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall"]
pub struct DPLLLCKF_R(crate::FieldReader<bool, bool>);
impl DPLLLCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLCKF` writer - DPLL Lock Fall"]
pub struct DPLLLCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLCKF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `DPLLLTO` reader - DPLL Timeout"]
pub struct DPLLLTO_R(crate::FieldReader<bool, bool>);
impl DPLLLTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLTO` writer - DPLL Timeout"]
pub struct DPLLLTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `DPLLLDRTO` reader - DPLL Ratio Ready"]
pub struct DPLLLDRTO_R(crate::FieldReader<bool, bool>);
impl DPLLLDRTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLDRTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLDRTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLDRTO` writer - DPLL Ratio Ready"]
pub struct DPLLLDRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLDRTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XOSCFAIL_R {
        XOSCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSC48M Ready"]
    #[inline(always)]
    pub fn osc48mrdy(&self) -> OSC48MRDY_R {
        OSC48MRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DPLL Timeout"]
    #[inline(always)]
    pub fn dplllto(&self) -> DPLLLTO_R {
        DPLLLTO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DPLL Ratio Ready"]
    #[inline(always)]
    pub fn dpllldrto(&self) -> DPLLLDRTO_R {
        DPLLLDRTO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&mut self) -> XOSCRDY_W {
        XOSCRDY_W { w: self }
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail(&mut self) -> XOSCFAIL_W {
        XOSCFAIL_W { w: self }
    }
    #[doc = "Bit 4 - OSC48M Ready"]
    #[inline(always)]
    pub fn osc48mrdy(&mut self) -> OSC48MRDY_W {
        OSC48MRDY_W { w: self }
    }
    #[doc = "Bit 8 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&mut self) -> DPLLLCKR_W {
        DPLLLCKR_W { w: self }
    }
    #[doc = "Bit 9 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&mut self) -> DPLLLCKF_W {
        DPLLLCKF_W { w: self }
    }
    #[doc = "Bit 10 - DPLL Timeout"]
    #[inline(always)]
    pub fn dplllto(&mut self) -> DPLLLTO_W {
        DPLLLTO_W { w: self }
    }
    #[doc = "Bit 11 - DPLL Ratio Ready"]
    #[inline(always)]
    pub fn dpllldrto(&mut self) -> DPLLLDRTO_W {
        DPLLLDRTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
