#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONBASETHRDENA` reader - Indicates how processor enters Thread mode"]
pub type NONBASETHRDENA_R = crate::BitReader<bool>;
#[doc = "Field `NONBASETHRDENA` writer - Indicates how processor enters Thread mode"]
pub type NONBASETHRDENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `USERSETMPEND` reader - Enables unprivileged software access to STIR register"]
pub type USERSETMPEND_R = crate::BitReader<bool>;
#[doc = "Field `USERSETMPEND` writer - Enables unprivileged software access to STIR register"]
pub type USERSETMPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `UNALIGN_TRP` reader - Enables unaligned access traps"]
pub type UNALIGN_TRP_R = crate::BitReader<UNALIGN_TRPSELECT_A>;
#[doc = "Enables unaligned access traps\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNALIGN_TRPSELECT_A {
    #[doc = "0: Do not trap unaligned halfword and word accesses"]
    VALUE_0 = 0,
    #[doc = "1: Trap unaligned halfword and word accesses"]
    VALUE_1 = 1,
}
impl From<UNALIGN_TRPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGN_TRPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl UNALIGN_TRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNALIGN_TRPSELECT_A {
        match self.bits {
            false => UNALIGN_TRPSELECT_A::VALUE_0,
            true => UNALIGN_TRPSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == UNALIGN_TRPSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == UNALIGN_TRPSELECT_A::VALUE_1
    }
}
#[doc = "Field `UNALIGN_TRP` writer - Enables unaligned access traps"]
pub type UNALIGN_TRP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCR_SPEC, UNALIGN_TRPSELECT_A, O>;
impl<'a, const O: u8> UNALIGN_TRP_W<'a, O> {
    #[doc = "Do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(UNALIGN_TRPSELECT_A::VALUE_0)
    }
    #[doc = "Trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(UNALIGN_TRPSELECT_A::VALUE_1)
    }
}
#[doc = "Field `DIV_0_TRP` reader - Enables divide by 0 trap"]
pub type DIV_0_TRP_R = crate::BitReader<bool>;
#[doc = "Field `DIV_0_TRP` writer - Enables divide by 0 trap"]
pub type DIV_0_TRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `BFHFNMIGN` reader - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
pub type BFHFNMIGN_R = crate::BitReader<bool>;
#[doc = "Field `BFHFNMIGN` writer - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
pub type BFHFNMIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `STKALIGN` reader - Indicates stack alignment on exception entry"]
pub type STKALIGN_R = crate::BitReader<STKALIGNSELECT_A>;
#[doc = "Indicates stack alignment on exception entry\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STKALIGNSELECT_A {
    #[doc = "0: 4-byte aligned"]
    VALUE_0 = 0,
    #[doc = "1: 8-byte aligned"]
    VALUE_1 = 1,
}
impl From<STKALIGNSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: STKALIGNSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl STKALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STKALIGNSELECT_A {
        match self.bits {
            false => STKALIGNSELECT_A::VALUE_0,
            true => STKALIGNSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == STKALIGNSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == STKALIGNSELECT_A::VALUE_1
    }
}
#[doc = "Field `STKALIGN` writer - Indicates stack alignment on exception entry"]
pub type STKALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, STKALIGNSELECT_A, O>;
impl<'a, const O: u8> STKALIGN_W<'a, O> {
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(STKALIGNSELECT_A::VALUE_0)
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(STKALIGNSELECT_A::VALUE_1)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates how processor enters Thread mode"]
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENA_R {
        NONBASETHRDENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to STIR register"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables divide by 0 trap"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates how processor enters Thread mode"]
    #[inline(always)]
    #[must_use]
    pub fn nonbasethrdena(&mut self) -> NONBASETHRDENA_W<0> {
        NONBASETHRDENA_W::new(self)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to STIR register"]
    #[inline(always)]
    #[must_use]
    pub fn usersetmpend(&mut self) -> USERSETMPEND_W<1> {
        USERSETMPEND_W::new(self)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    #[must_use]
    pub fn unalign_trp(&mut self) -> UNALIGN_TRP_W<3> {
        UNALIGN_TRP_W::new(self)
    }
    #[doc = "Bit 4 - Enables divide by 0 trap"]
    #[inline(always)]
    #[must_use]
    pub fn div_0_trp(&mut self) -> DIV_0_TRP_W<4> {
        DIV_0_TRP_W::new(self)
    }
    #[doc = "Bit 8 - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmign(&mut self) -> BFHFNMIGN_W<8> {
        BFHFNMIGN_W::new(self)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    #[must_use]
    pub fn stkalign(&mut self) -> STKALIGN_W<9> {
        STKALIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0x0200"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
