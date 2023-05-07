#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<SWRSTSELECT_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTSELECT_A {
    #[doc = "0: The peripheral is not reset"]
    DISABLE = 0,
    #[doc = "1: The peripheral is reset"]
    ENABLE = 1,
}
impl From<SWRSTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTSELECT_A {
        match self.bits {
            false => SWRSTSELECT_A::DISABLE,
            true => SWRSTSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SWRSTSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SWRSTSELECT_A::ENABLE
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, SWRSTSELECT_A, O>;
impl<'a, const O: u8> SWRST_W<'a, O> {
    #[doc = "The peripheral is not reset"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWRSTSELECT_A::DISABLE)
    }
    #[doc = "The peripheral is reset"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWRSTSELECT_A::ENABLE)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<ENABLESELECT_A>;
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLESELECT_A {
    #[doc = "0: The peripheral is disabled"]
    DISABLE = 0,
    #[doc = "1: The peripheral is enabled"]
    ENABLE = 1,
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
            false => ENABLESELECT_A::DISABLE,
            true => ENABLESELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLESELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLESELECT_A::ENABLE
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, ENABLESELECT_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "The peripheral is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLESELECT_A::DISABLE)
    }
    #[doc = "The peripheral is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLESELECT_A::ENABLE)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<RUNSTDBYSELECT_A>;
#[doc = "Run in Standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUNSTDBYSELECT_A {
    #[doc = "0: Generic clock is not required in standby sleep mode"]
    DISABLE = 0,
    #[doc = "1: Generic clock is required in standby sleep mode"]
    ENABLE = 1,
}
impl From<RUNSTDBYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RUNSTDBYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RUNSTDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUNSTDBYSELECT_A {
        match self.bits {
            false => RUNSTDBYSELECT_A::DISABLE,
            true => RUNSTDBYSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RUNSTDBYSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RUNSTDBYSELECT_A::ENABLE
    }
}
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, RUNSTDBYSELECT_A, O>;
impl<'a, const O: u8> RUNSTDBY_W<'a, O> {
    #[doc = "Generic clock is not required in standby sleep mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RUNSTDBYSELECT_A::DISABLE)
    }
    #[doc = "Generic clock is required in standby sleep mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RUNSTDBYSELECT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
