#[doc = "Register `POSITION` reader"]
pub struct R(crate::R<POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POSITION` writer"]
pub struct W(crate::W<POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POSITION_SPEC>;
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
impl From<crate::W<POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRAP` reader - Pointer Value Wraps"]
pub struct WRAP_R(crate::FieldReader<bool, bool>);
impl WRAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRAP` writer - Pointer Value Wraps"]
pub struct WRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAP_W<'a> {
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
#[doc = "Field `POINTER` reader - Trace Packet Location Pointer"]
pub struct POINTER_R(crate::FieldReader<u32, u32>);
impl POINTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        POINTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POINTER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POINTER` writer - Trace Packet Location Pointer"]
pub struct POINTER_W<'a> {
    w: &'a mut W,
}
impl<'a> POINTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Pointer Value Wraps"]
    #[inline(always)]
    pub fn wrap(&self) -> WRAP_R {
        WRAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:31 - Trace Packet Location Pointer"]
    #[inline(always)]
    pub fn pointer(&self) -> POINTER_R {
        POINTER_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 2 - Pointer Value Wraps"]
    #[inline(always)]
    pub fn wrap(&mut self) -> WRAP_W {
        WRAP_W { w: self }
    }
    #[doc = "Bits 3:31 - Trace Packet Location Pointer"]
    #[inline(always)]
    pub fn pointer(&mut self) -> POINTER_W {
        POINTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB Position\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [position](index.html) module"]
pub struct POSITION_SPEC;
impl crate::RegisterSpec for POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [position::R](R) reader structure"]
impl crate::Readable for POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [position::W](W) writer structure"]
impl crate::Writable for POSITION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POSITION to value 0"]
impl crate::Resettable for POSITION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
