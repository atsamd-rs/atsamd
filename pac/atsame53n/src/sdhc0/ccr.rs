#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `INTCLKEN` reader - Internal Clock Enable"]
pub type INTCLKEN_R = crate::BitReader<INTCLKENSELECT_A>;
#[doc = "Internal Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTCLKENSELECT_A {
    #[doc = "0: Stop"]
    OFF = 0,
    #[doc = "1: Oscillate"]
    ON = 1,
}
impl From<INTCLKENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INTCLKENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INTCLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTCLKENSELECT_A {
        match self.bits {
            false => INTCLKENSELECT_A::OFF,
            true => INTCLKENSELECT_A::ON,
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == INTCLKENSELECT_A::OFF
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == INTCLKENSELECT_A::ON
    }
}
#[doc = "Field `INTCLKEN` writer - Internal Clock Enable"]
pub type INTCLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INTCLKENSELECT_A>;
impl<'a, REG, const O: u8> INTCLKEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(INTCLKENSELECT_A::OFF)
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(INTCLKENSELECT_A::ON)
    }
}
#[doc = "Field `INTCLKS` reader - Internal Clock Stable"]
pub type INTCLKS_R = crate::BitReader<INTCLKSSELECT_A>;
#[doc = "Internal Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTCLKSSELECT_A {
    #[doc = "0: Not Ready"]
    NOT_READY = 0,
    #[doc = "1: Ready"]
    READY = 1,
}
impl From<INTCLKSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INTCLKSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INTCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTCLKSSELECT_A {
        match self.bits {
            false => INTCLKSSELECT_A::NOT_READY,
            true => INTCLKSSELECT_A::READY,
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == INTCLKSSELECT_A::NOT_READY
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == INTCLKSSELECT_A::READY
    }
}
#[doc = "Field `INTCLKS` writer - Internal Clock Stable"]
pub type INTCLKS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INTCLKSSELECT_A>;
impl<'a, REG, const O: u8> INTCLKS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(INTCLKSSELECT_A::NOT_READY)
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(INTCLKSSELECT_A::READY)
    }
}
#[doc = "Field `SDCLKEN` reader - SD Clock Enable"]
pub type SDCLKEN_R = crate::BitReader<SDCLKENSELECT_A>;
#[doc = "SD Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCLKENSELECT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<SDCLKENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SDCLKENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SDCLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDCLKENSELECT_A {
        match self.bits {
            false => SDCLKENSELECT_A::DISABLE,
            true => SDCLKENSELECT_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDCLKENSELECT_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDCLKENSELECT_A::ENABLE
    }
}
#[doc = "Field `SDCLKEN` writer - SD Clock Enable"]
pub type SDCLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SDCLKENSELECT_A>;
impl<'a, REG, const O: u8> SDCLKEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLKENSELECT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLKENSELECT_A::ENABLE)
    }
}
#[doc = "Field `CLKGSEL` reader - Clock Generator Select"]
pub type CLKGSEL_R = crate::BitReader<CLKGSELSELECT_A>;
#[doc = "Clock Generator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKGSELSELECT_A {
    #[doc = "0: Divided Clock Mode"]
    DIV = 0,
    #[doc = "1: Programmable Clock Mode"]
    PROG = 1,
}
impl From<CLKGSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CLKGSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKGSELSELECT_A {
        match self.bits {
            false => CLKGSELSELECT_A::DIV,
            true => CLKGSELSELECT_A::PROG,
        }
    }
    #[doc = "Divided Clock Mode"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == CLKGSELSELECT_A::DIV
    }
    #[doc = "Programmable Clock Mode"]
    #[inline(always)]
    pub fn is_prog(&self) -> bool {
        *self == CLKGSELSELECT_A::PROG
    }
}
#[doc = "Field `CLKGSEL` writer - Clock Generator Select"]
pub type CLKGSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLKGSELSELECT_A>;
impl<'a, REG, const O: u8> CLKGSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Divided Clock Mode"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(CLKGSELSELECT_A::DIV)
    }
    #[doc = "Programmable Clock Mode"]
    #[inline(always)]
    pub fn prog(self) -> &'a mut crate::W<REG> {
        self.variant(CLKGSELSELECT_A::PROG)
    }
}
#[doc = "Field `USDCLKFSEL` reader - Upper Bits of SDCLK Frequency Select"]
pub type USDCLKFSEL_R = crate::FieldReader;
#[doc = "Field `USDCLKFSEL` writer - Upper Bits of SDCLK Frequency Select"]
pub type USDCLKFSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SDCLKFSEL` reader - SDCLK Frequency Select"]
pub type SDCLKFSEL_R = crate::FieldReader;
#[doc = "Field `SDCLKFSEL` writer - SDCLK Frequency Select"]
pub type SDCLKFSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&self) -> INTCLKEN_R {
        INTCLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclks(&self) -> INTCLKS_R {
        INTCLKS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&self) -> SDCLKEN_R {
        SDCLKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgsel(&self) -> CLKGSEL_R {
        CLKGSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select"]
    #[inline(always)]
    pub fn usdclkfsel(&self) -> USDCLKFSEL_R {
        USDCLKFSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfsel(&self) -> SDCLKFSEL_R {
        SDCLKFSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn intclken(&mut self) -> INTCLKEN_W<CCR_SPEC, 0> {
        INTCLKEN_W::new(self)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    #[must_use]
    pub fn intclks(&mut self) -> INTCLKS_W<CCR_SPEC, 1> {
        INTCLKS_W::new(self)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdclken(&mut self) -> SDCLKEN_W<CCR_SPEC, 2> {
        SDCLKEN_W::new(self)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    #[must_use]
    pub fn clkgsel(&mut self) -> CLKGSEL_W<CCR_SPEC, 5> {
        CLKGSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn usdclkfsel(&mut self) -> USDCLKFSEL_W<CCR_SPEC, 6> {
        USDCLKFSEL_W::new(self)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfsel(&mut self) -> SDCLKFSEL_W<CCR_SPEC, 8> {
        SDCLKFSEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
