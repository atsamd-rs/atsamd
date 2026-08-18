#[doc = "Register `VREG33` reader"]
pub struct R(crate::R<VREG33_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREG33_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREG33_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREG33_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREG33` writer"]
pub struct W(crate::W<VREG33_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREG33_SPEC>;
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
impl From<crate::W<VREG33_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREG33_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - VREG33 Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - VREG33 Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Field `ENRDY` reader - VREG33 Ready Enable"]
pub struct ENRDY_R(crate::FieldReader<bool, bool>);
impl ENRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENRDY` writer - VREG33 Ready Enable"]
pub struct ENRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRDY_W<'a> {
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
#[doc = "Field `BYPASS` reader - VREG33 Bypass"]
pub struct BYPASS_R(crate::FieldReader<bool, bool>);
impl BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - VREG33 Bypass"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Field `ISOEN` reader - Isolation Enable"]
pub struct ISOEN_R(crate::FieldReader<bool, bool>);
impl ISOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOEN` writer - Isolation Enable"]
pub struct ISOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOEN_W<'a> {
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
    #[doc = "Bit 1 - VREG33 Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VREG33 Ready Enable"]
    #[inline(always)]
    pub fn enrdy(&self) -> ENRDY_R {
        ENRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VREG33 Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Isolation Enable"]
    #[inline(always)]
    pub fn isoen(&self) -> ISOEN_R {
        ISOEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - VREG33 Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - VREG33 Ready Enable"]
    #[inline(always)]
    pub fn enrdy(&mut self) -> ENRDY_W {
        ENRDY_W { w: self }
    }
    #[doc = "Bit 3 - VREG33 Bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 4 - Isolation Enable"]
    #[inline(always)]
    pub fn isoen(&mut self) -> ISOEN_W {
        ISOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREG33 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreg33](index.html) module"]
pub struct VREG33_SPEC;
impl crate::RegisterSpec for VREG33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vreg33::R](R) reader structure"]
impl crate::Readable for VREG33_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vreg33::W](W) writer structure"]
impl crate::Writable for VREG33_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREG33 to value 0x10"]
impl crate::Resettable for VREG33_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
