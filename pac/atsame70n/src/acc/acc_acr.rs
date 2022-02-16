#[doc = "Register `ACC_ACR` reader"]
pub struct R(crate::R<ACC_ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACC_ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACC_ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACC_ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACC_ACR` writer"]
pub struct W(crate::W<ACC_ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACC_ACR_SPEC>;
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
impl From<crate::W<ACC_ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACC_ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Current Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISEL_A {
    #[doc = "0: Low-power option."]
    LOPW = 0,
    #[doc = "1: High-speed option."]
    HISP = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISEL` reader - Current Selection"]
pub struct ISEL_R(crate::FieldReader<bool, ISEL_A>);
impl ISEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::LOPW,
            true => ISEL_A::HISP,
        }
    }
    #[doc = "Checks if the value of the field is `LOPW`"]
    #[inline(always)]
    pub fn is_lopw(&self) -> bool {
        **self == ISEL_A::LOPW
    }
    #[doc = "Checks if the value of the field is `HISP`"]
    #[inline(always)]
    pub fn is_hisp(&self) -> bool {
        **self == ISEL_A::HISP
    }
}
impl core::ops::Deref for ISEL_R {
    type Target = crate::FieldReader<bool, ISEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEL` writer - Current Selection"]
pub struct ISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low-power option."]
    #[inline(always)]
    pub fn lopw(self) -> &'a mut W {
        self.variant(ISEL_A::LOPW)
    }
    #[doc = "High-speed option."]
    #[inline(always)]
    pub fn hisp(self) -> &'a mut W {
        self.variant(ISEL_A::HISP)
    }
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
#[doc = "Field `HYST` reader - Hysteresis Selection"]
pub struct HYST_R(crate::FieldReader<u8, u8>);
impl HYST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST` writer - Hysteresis Selection"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Current Selection"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current Selection"]
    #[inline(always)]
    pub fn isel(&mut self) -> ISEL_W {
        ISEL_W { w: self }
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_acr](index.html) module"]
pub struct ACC_ACR_SPEC;
impl crate::RegisterSpec for ACC_ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acc_acr::R](R) reader structure"]
impl crate::Readable for ACC_ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acc_acr::W](W) writer structure"]
impl crate::Writable for ACC_ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACC_ACR to value 0"]
impl crate::Resettable for ACC_ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
