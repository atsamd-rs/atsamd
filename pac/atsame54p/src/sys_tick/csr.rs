#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - SysTick Counter Enable"]
pub type ENABLE_R = crate::BitReader<ENABLESELECT_A>;
#[doc = "SysTick Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLESELECT_A {
    #[doc = "0: Counter disabled"]
    VALUE_0 = 0,
    #[doc = "1: Counter enabled"]
    VALUE_1 = 1,
}
impl From<ENABLESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLESELECT_A {
        match self.bits {
            false => ENABLESELECT_A::VALUE_0,
            true => ENABLESELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == ENABLESELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == ENABLESELECT_A::VALUE_1
    }
}
#[doc = "Field `ENABLE` writer - SysTick Counter Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, ENABLESELECT_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(ENABLESELECT_A::VALUE_0)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(ENABLESELECT_A::VALUE_1)
    }
}
#[doc = "Field `TICKINT` reader - SysTick Exception Request Enable"]
pub type TICKINT_R = crate::BitReader<TICKINTSELECT_A>;
#[doc = "SysTick Exception Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TICKINTSELECT_A {
    #[doc = "0: Counting down to 0 does not assert the SysTick exception request"]
    VALUE_0 = 0,
    #[doc = "1: Counting down to 0 asserts the SysTick exception request"]
    VALUE_1 = 1,
}
impl From<TICKINTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TICKINTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TICKINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICKINTSELECT_A {
        match self.bits {
            false => TICKINTSELECT_A::VALUE_0,
            true => TICKINTSELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == TICKINTSELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == TICKINTSELECT_A::VALUE_1
    }
}
#[doc = "Field `TICKINT` writer - SysTick Exception Request Enable"]
pub type TICKINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, TICKINTSELECT_A, O>;
impl<'a, const O: u8> TICKINT_W<'a, O> {
    #[doc = "Counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(TICKINTSELECT_A::VALUE_0)
    }
    #[doc = "Counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(TICKINTSELECT_A::VALUE_1)
    }
}
#[doc = "Field `CLKSOURCE` reader - Clock Source 0=external, 1=processor"]
pub type CLKSOURCE_R = crate::BitReader<CLKSOURCESELECT_A>;
#[doc = "Clock Source 0=external, 1=processor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSOURCESELECT_A {
    #[doc = "0: External clock"]
    VALUE_0 = 0,
    #[doc = "1: Processor clock"]
    VALUE_1 = 1,
}
impl From<CLKSOURCESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSOURCESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSOURCESELECT_A {
        match self.bits {
            false => CLKSOURCESELECT_A::VALUE_0,
            true => CLKSOURCESELECT_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == CLKSOURCESELECT_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == CLKSOURCESELECT_A::VALUE_1
    }
}
#[doc = "Field `CLKSOURCE` writer - Clock Source 0=external, 1=processor"]
pub type CLKSOURCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, CLKSOURCESELECT_A, O>;
impl<'a, const O: u8> CLKSOURCE_W<'a, O> {
    #[doc = "External clock"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(CLKSOURCESELECT_A::VALUE_0)
    }
    #[doc = "Processor clock"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(CLKSOURCESELECT_A::VALUE_1)
    }
}
#[doc = "Field `COUNTFLAG` reader - Timer counted to 0 since last read of register"]
pub type COUNTFLAG_R = crate::BitReader<bool>;
#[doc = "Field `COUNTFLAG` writer - Timer counted to 0 since last read of register"]
pub type COUNTFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SysTick Counter Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SysTick Exception Request Enable"]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Source 0=external, 1=processor"]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer counted to 0 since last read of register"]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SysTick Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - SysTick Exception Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TICKINT_W<1> {
        TICKINT_W::new(self)
    }
    #[doc = "Bit 2 - Clock Source 0=external, 1=processor"]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> CLKSOURCE_W<2> {
        CLKSOURCE_W::new(self)
    }
    #[doc = "Bit 16 - Timer counted to 0 since last read of register"]
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> COUNTFLAG_W<16> {
        COUNTFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0x04"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
