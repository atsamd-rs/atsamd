#[doc = "Register `CC2R` reader"]
pub struct R(crate::R<CC2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC2R` writer"]
pub struct W(crate::W<CC2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC2R_SPEC>;
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
impl From<crate::W<CC2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Force SDCK Disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSDCLKD_A {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    DISABLE = 1,
}
impl From<FSDCLKD_A> for bool {
    #[inline(always)]
    fn from(variant: FSDCLKD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSDCLKD` reader - Force SDCK Disabled"]
pub struct FSDCLKD_R(crate::FieldReader<bool, FSDCLKD_A>);
impl FSDCLKD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSDCLKD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSDCLKD_A {
        match self.bits {
            false => FSDCLKD_A::NOEFFECT,
            true => FSDCLKD_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_noeffect(&self) -> bool {
        **self == FSDCLKD_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FSDCLKD_A::DISABLE
    }
}
impl core::ops::Deref for FSDCLKD_R {
    type Target = crate::FieldReader<bool, FSDCLKD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSDCLKD` writer - Force SDCK Disabled"]
pub struct FSDCLKD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDCLKD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSDCLKD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(FSDCLKD_A::NOEFFECT)
    }
    #[doc = "SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FSDCLKD_A::DISABLE)
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
impl R {
    #[doc = "Bit 0 - Force SDCK Disabled"]
    #[inline(always)]
    pub fn fsdclkd(&self) -> FSDCLKD_R {
        FSDCLKD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force SDCK Disabled"]
    #[inline(always)]
    pub fn fsdclkd(&mut self) -> FSDCLKD_W {
        FSDCLKD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc2r](index.html) module"]
pub struct CC2R_SPEC;
impl crate::RegisterSpec for CC2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc2r::R](R) reader structure"]
impl crate::Readable for CC2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc2r::W](W) writer structure"]
impl crate::Writable for CC2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC2R to value 0"]
impl crate::Resettable for CC2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
