#[doc = "Register `DFSR` reader"]
pub struct R(crate::R<DFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSR` writer"]
pub struct W(crate::W<DFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSR_SPEC>;
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
impl From<crate::W<DFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALTED` reader - "]
pub struct HALTED_R(crate::FieldReader<bool, bool>);
impl HALTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALTED` writer - "]
pub struct HALTED_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTED_W<'a> {
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
#[doc = "Field `BKPT` reader - "]
pub struct BKPT_R(crate::FieldReader<bool, bool>);
impl BKPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BKPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKPT` writer - "]
pub struct BKPT_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPT_W<'a> {
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
#[doc = "Field `DWTTRAP` reader - "]
pub struct DWTTRAP_R(crate::FieldReader<bool, bool>);
impl DWTTRAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DWTTRAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DWTTRAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DWTTRAP` writer - "]
pub struct DWTTRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DWTTRAP_W<'a> {
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
#[doc = "Field `VCATCH` reader - "]
pub struct VCATCH_R(crate::FieldReader<bool, bool>);
impl VCATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCATCH` writer - "]
pub struct VCATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VCATCH_W<'a> {
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
#[doc = "Field `EXTERNAL` reader - "]
pub struct EXTERNAL_R(crate::FieldReader<bool, bool>);
impl EXTERNAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTERNAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTERNAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTERNAL` writer - "]
pub struct EXTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bkpt(&self) -> BKPT_R {
        BKPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwttrap(&self) -> DWTTRAP_R {
        DWTTRAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vcatch(&self) -> VCATCH_R {
        VCATCH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn halted(&mut self) -> HALTED_W {
        HALTED_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bkpt(&mut self) -> BKPT_W {
        BKPT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwttrap(&mut self) -> DWTTRAP_W {
        DWTTRAP_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vcatch(&mut self) -> VCATCH_W {
        VCATCH_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn external(&mut self) -> EXTERNAL_W {
        EXTERNAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsr](index.html) module"]
pub struct DFSR_SPEC;
impl crate::RegisterSpec for DFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsr::R](R) reader structure"]
impl crate::Readable for DFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsr::W](W) writer structure"]
impl crate::Writable for DFSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFSR to value 0"]
impl crate::Resettable for DFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
