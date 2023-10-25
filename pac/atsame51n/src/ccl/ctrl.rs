#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
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
    pub const fn variant(&self) -> SWRSTSELECT_A {
        match self.bits {
            false => SWRSTSELECT_A::DISABLE,
            true => SWRSTSELECT_A::ENABLE,
        }
    }
    #[doc = "The peripheral is not reset"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SWRSTSELECT_A::DISABLE
    }
    #[doc = "The peripheral is reset"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SWRSTSELECT_A::ENABLE
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWRSTSELECT_A>;
impl<'a, REG, const O: u8> SWRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The peripheral is not reset"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SWRSTSELECT_A::DISABLE)
    }
    #[doc = "The peripheral is reset"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> ENABLESELECT_A {
        match self.bits {
            false => ENABLESELECT_A::DISABLE,
            true => ENABLESELECT_A::ENABLE,
        }
    }
    #[doc = "The peripheral is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLESELECT_A::DISABLE
    }
    #[doc = "The peripheral is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLESELECT_A::ENABLE
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENABLESELECT_A>;
impl<'a, REG, const O: u8> ENABLE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The peripheral is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLESELECT_A::DISABLE)
    }
    #[doc = "The peripheral is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> RUNSTDBYSELECT_A {
        match self.bits {
            false => RUNSTDBYSELECT_A::DISABLE,
            true => RUNSTDBYSELECT_A::ENABLE,
        }
    }
    #[doc = "Generic clock is not required in standby sleep mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RUNSTDBYSELECT_A::DISABLE
    }
    #[doc = "Generic clock is required in standby sleep mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RUNSTDBYSELECT_A::ENABLE
    }
}
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RUNSTDBYSELECT_A>;
impl<'a, REG, const O: u8> RUNSTDBY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generic clock is not required in standby sleep mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RUNSTDBYSELECT_A::DISABLE)
    }
    #[doc = "Generic clock is required in standby sleep mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub fn swrst(&mut self) -> SWRST_W<CTRL_SPEC, 0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRL_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRL_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
