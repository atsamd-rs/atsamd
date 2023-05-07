#[doc = "Register `MCFG` reader"]
pub struct R(crate::R<MCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCFG` writer"]
pub struct W(crate::W<MCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCFG_SPEC>;
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
impl From<crate::W<MCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Cache Controller Monitor Counter Mode"]
pub type MODE_R = crate::FieldReader<u8, MODESELECT_A>;
#[doc = "Cache Controller Monitor Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
    #[doc = "0: Cycle counter"]
    CYCLE_COUNT = 0,
    #[doc = "1: Instruction hit counter"]
    IHIT_COUNT = 1,
    #[doc = "2: Data hit counter"]
    DHIT_COUNT = 2,
}
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::CYCLE_COUNT),
            1 => Some(MODESELECT_A::IHIT_COUNT),
            2 => Some(MODESELECT_A::DHIT_COUNT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE_COUNT`"]
    #[inline(always)]
    pub fn is_cycle_count(&self) -> bool {
        *self == MODESELECT_A::CYCLE_COUNT
    }
    #[doc = "Checks if the value of the field is `IHIT_COUNT`"]
    #[inline(always)]
    pub fn is_ihit_count(&self) -> bool {
        *self == MODESELECT_A::IHIT_COUNT
    }
    #[doc = "Checks if the value of the field is `DHIT_COUNT`"]
    #[inline(always)]
    pub fn is_dhit_count(&self) -> bool {
        *self == MODESELECT_A::DHIT_COUNT
    }
}
#[doc = "Field `MODE` writer - Cache Controller Monitor Counter Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCFG_SPEC, u8, MODESELECT_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Cycle counter"]
    #[inline(always)]
    pub fn cycle_count(self) -> &'a mut W {
        self.variant(MODESELECT_A::CYCLE_COUNT)
    }
    #[doc = "Instruction hit counter"]
    #[inline(always)]
    pub fn ihit_count(self) -> &'a mut W {
        self.variant(MODESELECT_A::IHIT_COUNT)
    }
    #[doc = "Data hit counter"]
    #[inline(always)]
    pub fn dhit_count(self) -> &'a mut W {
        self.variant(MODESELECT_A::DHIT_COUNT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Cache Controller Monitor Counter Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Cache Controller Monitor Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Monitor Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](index.html) module"]
pub struct MCFG_SPEC;
impl crate::RegisterSpec for MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcfg::R](R) reader structure"]
impl crate::Readable for MCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcfg::W](W) writer structure"]
impl crate::Writable for MCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCFG to value 0"]
impl crate::Resettable for MCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
