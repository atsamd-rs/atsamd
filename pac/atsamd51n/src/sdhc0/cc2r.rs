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
#[doc = "Field `FSDCLKD` reader - Force SDCK Disabled"]
pub type FSDCLKD_R = crate::BitReader<FSDCLKDSELECT_A>;
#[doc = "Force SDCK Disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSDCLKDSELECT_A {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    DISABLE = 1,
}
impl From<FSDCLKDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FSDCLKDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FSDCLKD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSDCLKDSELECT_A {
        match self.bits {
            false => FSDCLKDSELECT_A::NOEFFECT,
            true => FSDCLKDSELECT_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_noeffect(&self) -> bool {
        *self == FSDCLKDSELECT_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FSDCLKDSELECT_A::DISABLE
    }
}
#[doc = "Field `FSDCLKD` writer - Force SDCK Disabled"]
pub type FSDCLKD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC2R_SPEC, FSDCLKDSELECT_A, O>;
impl<'a, const O: u8> FSDCLKD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(FSDCLKDSELECT_A::NOEFFECT)
    }
    #[doc = "SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FSDCLKDSELECT_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Force SDCK Disabled"]
    #[inline(always)]
    pub fn fsdclkd(&self) -> FSDCLKD_R {
        FSDCLKD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force SDCK Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn fsdclkd(&mut self) -> FSDCLKD_W<0> {
        FSDCLKD_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC2R to value 0"]
impl crate::Resettable for CC2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
