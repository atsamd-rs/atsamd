#[doc = "Register `INTFLAGD` reader"]
pub struct R(crate::R<INTFLAGD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGD` writer"]
pub struct W(crate::W<INTFLAGD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGD_SPEC>;
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
impl From<crate::W<INTFLAGD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERCOM6_` reader - SERCOM6"]
pub struct SERCOM6__R(crate::FieldReader<bool, bool>);
impl SERCOM6__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM6__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM6__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM6_` writer - SERCOM6"]
pub struct SERCOM6__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM6__W<'a> {
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
#[doc = "Field `SERCOM7_` reader - SERCOM7"]
pub struct SERCOM7__R(crate::FieldReader<bool, bool>);
impl SERCOM7__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM7__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM7__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM7_` writer - SERCOM7"]
pub struct SERCOM7__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM7__W<'a> {
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
#[doc = "Field `TC5_` reader - TC5"]
pub struct TC5__R(crate::FieldReader<bool, bool>);
impl TC5__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC5__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC5__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC5_` writer - TC5"]
pub struct TC5__W<'a> {
    w: &'a mut W,
}
impl<'a> TC5__W<'a> {
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
#[doc = "Field `TC6_` reader - TC6"]
pub struct TC6__R(crate::FieldReader<bool, bool>);
impl TC6__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC6__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC6__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC6_` writer - TC6"]
pub struct TC6__W<'a> {
    w: &'a mut W,
}
impl<'a> TC6__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TC7_` reader - TC7"]
pub struct TC7__R(crate::FieldReader<bool, bool>);
impl TC7__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC7__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC7__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC7_` writer - TC7"]
pub struct TC7__W<'a> {
    w: &'a mut W,
}
impl<'a> TC7__W<'a> {
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
impl R {
    #[doc = "Bit 0 - SERCOM6"]
    #[inline(always)]
    pub fn sercom6_(&self) -> SERCOM6__R {
        SERCOM6__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM7"]
    #[inline(always)]
    pub fn sercom7_(&self) -> SERCOM7__R {
        SERCOM7__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TC5"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TC6"]
    #[inline(always)]
    pub fn tc6_(&self) -> TC6__R {
        TC6__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TC7"]
    #[inline(always)]
    pub fn tc7_(&self) -> TC7__R {
        TC7__R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM6"]
    #[inline(always)]
    pub fn sercom6_(&mut self) -> SERCOM6__W {
        SERCOM6__W { w: self }
    }
    #[doc = "Bit 1 - SERCOM7"]
    #[inline(always)]
    pub fn sercom7_(&mut self) -> SERCOM7__W {
        SERCOM7__W { w: self }
    }
    #[doc = "Bit 2 - TC5"]
    #[inline(always)]
    pub fn tc5_(&mut self) -> TC5__W {
        TC5__W { w: self }
    }
    #[doc = "Bit 3 - TC6"]
    #[inline(always)]
    pub fn tc6_(&mut self) -> TC6__W {
        TC6__W { w: self }
    }
    #[doc = "Bit 4 - TC7"]
    #[inline(always)]
    pub fn tc7_(&mut self) -> TC7__W {
        TC7__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagd](index.html) module"]
pub struct INTFLAGD_SPEC;
impl crate::RegisterSpec for INTFLAGD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagd::R](R) reader structure"]
impl crate::Readable for INTFLAGD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagd::W](W) writer structure"]
impl crate::Writable for INTFLAGD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGD to value 0"]
impl crate::Resettable for INTFLAGD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
