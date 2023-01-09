#[doc = "Register `STDBYCFG` reader"]
pub struct R(crate::R<STDBYCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STDBYCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STDBYCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STDBYCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STDBYCFG` writer"]
pub struct W(crate::W<STDBYCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STDBYCFG_SPEC>;
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
impl From<crate::W<STDBYCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STDBYCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREGSMOD` reader - Voltage Regulator Standby mode"]
pub type VREGSMOD_R = crate::FieldReader<u8, VREGSMODSELECT_A>;
#[doc = "Voltage Regulator Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VREGSMODSELECT_A {
    #[doc = "0: Automatic mode"]
    AUTO = 0,
    #[doc = "1: Performance oriented"]
    PERFORMANCE = 1,
    #[doc = "2: Low Power oriented"]
    LP = 2,
}
impl From<VREGSMODSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: VREGSMODSELECT_A) -> Self {
        variant as _
    }
}
impl VREGSMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VREGSMODSELECT_A> {
        match self.bits {
            0 => Some(VREGSMODSELECT_A::AUTO),
            1 => Some(VREGSMODSELECT_A::PERFORMANCE),
            2 => Some(VREGSMODSELECT_A::LP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == VREGSMODSELECT_A::AUTO
    }
    #[doc = "Checks if the value of the field is `PERFORMANCE`"]
    #[inline(always)]
    pub fn is_performance(&self) -> bool {
        *self == VREGSMODSELECT_A::PERFORMANCE
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == VREGSMODSELECT_A::LP
    }
}
#[doc = "Field `VREGSMOD` writer - Voltage Regulator Standby mode"]
pub type VREGSMOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, STDBYCFG_SPEC, u8, VREGSMODSELECT_A, 2, O>;
impl<'a, const O: u8> VREGSMOD_W<'a, O> {
    #[doc = "Automatic mode"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(VREGSMODSELECT_A::AUTO)
    }
    #[doc = "Performance oriented"]
    #[inline(always)]
    pub fn performance(self) -> &'a mut W {
        self.variant(VREGSMODSELECT_A::PERFORMANCE)
    }
    #[doc = "Low Power oriented"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(VREGSMODSELECT_A::LP)
    }
}
#[doc = "Field `BBIASHS` reader - Back Bias for HMCRAMCHS"]
pub type BBIASHS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BBIASHS` writer - Back Bias for HMCRAMCHS"]
pub type BBIASHS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, STDBYCFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    pub fn vregsmod(&self) -> VREGSMOD_R {
        VREGSMOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Back Bias for HMCRAMCHS"]
    #[inline(always)]
    pub fn bbiashs(&self) -> BBIASHS_R {
        BBIASHS_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn vregsmod(&mut self) -> VREGSMOD_W<6> {
        VREGSMOD_W::new(self)
    }
    #[doc = "Bits 10:11 - Back Bias for HMCRAMCHS"]
    #[inline(always)]
    #[must_use]
    pub fn bbiashs(&mut self) -> BBIASHS_W<10> {
        BBIASHS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standby Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stdbycfg](index.html) module"]
pub struct STDBYCFG_SPEC;
impl crate::RegisterSpec for STDBYCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [stdbycfg::R](R) reader structure"]
impl crate::Readable for STDBYCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stdbycfg::W](W) writer structure"]
impl crate::Writable for STDBYCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STDBYCFG to value 0x0400"]
impl crate::Resettable for STDBYCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
