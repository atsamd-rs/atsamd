#[doc = "Register `MATRIX_SCFG[%s]` reader"]
pub struct R(crate::R<MATRIX_SCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_SCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_SCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_SCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_SCFG[%s]` writer"]
pub struct W(crate::W<MATRIX_SCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_SCFG_SPEC>;
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
impl From<crate::W<MATRIX_SCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_SCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOT_CYCLE` reader - Maximum Bus Grant Duration for Masters"]
pub struct SLOT_CYCLE_R(crate::FieldReader<u16, u16>);
impl SLOT_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SLOT_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOT_CYCLE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOT_CYCLE` writer - Maximum Bus Grant Duration for Masters"]
pub struct SLOT_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOT_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Default Master Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEFMSTR_TYPE_A {
    #[doc = "0: No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    NONE = 0,
    #[doc = "1: Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    LAST = 1,
    #[doc = "2: Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    FIXED = 2,
}
impl From<DEFMSTR_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEFMSTR_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub struct DEFMSTR_TYPE_R(crate::FieldReader<u8, DEFMSTR_TYPE_A>);
impl DEFMSTR_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEFMSTR_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEFMSTR_TYPE_A> {
        match self.bits {
            0 => Some(DEFMSTR_TYPE_A::NONE),
            1 => Some(DEFMSTR_TYPE_A::LAST),
            2 => Some(DEFMSTR_TYPE_A::FIXED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == DEFMSTR_TYPE_A::NONE
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        **self == DEFMSTR_TYPE_A::LAST
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        **self == DEFMSTR_TYPE_A::FIXED
    }
}
impl core::ops::Deref for DEFMSTR_TYPE_R {
    type Target = crate::FieldReader<u8, DEFMSTR_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub struct DEFMSTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFMSTR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEFMSTR_TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPE_A::NONE)
    }
    #[doc = "Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    #[inline(always)]
    pub fn last(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPE_A::LAST)
    }
    #[doc = "Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPE_A::FIXED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Default Master"]
pub struct FIXED_DEFMSTR_R(crate::FieldReader<u8, u8>);
impl FIXED_DEFMSTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIXED_DEFMSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIXED_DEFMSTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Default Master"]
pub struct FIXED_DEFMSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXED_DEFMSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SLOT_CYCLE_R {
        SLOT_CYCLE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPE_R {
        DEFMSTR_TYPE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTR_R {
        FIXED_DEFMSTR_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline(always)]
    pub fn slot_cycle(&mut self) -> SLOT_CYCLE_W {
        SLOT_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&mut self) -> DEFMSTR_TYPE_W {
        DEFMSTR_TYPE_W { w: self }
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&mut self) -> FIXED_DEFMSTR_W {
        FIXED_DEFMSTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_scfg](index.html) module"]
pub struct MATRIX_SCFG_SPEC;
impl crate::RegisterSpec for MATRIX_SCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_scfg::R](R) reader structure"]
impl crate::Readable for MATRIX_SCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_scfg::W](W) writer structure"]
impl crate::Writable for MATRIX_SCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATRIX_SCFG[%s]
to value 0"]
impl crate::Resettable for MATRIX_SCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
