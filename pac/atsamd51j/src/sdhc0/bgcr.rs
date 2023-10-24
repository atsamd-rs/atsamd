#[doc = "Register `BGCR` reader"]
pub type R = crate::R<BGCR_SPEC>;
#[doc = "Register `BGCR` writer"]
pub type W = crate::W<BGCR_SPEC>;
#[doc = "Field `STPBGR` reader - Stop at Block Gap Request"]
pub type STPBGR_R = crate::BitReader<STPBGRSELECT_A>;
#[doc = "Stop at Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STPBGRSELECT_A {
    #[doc = "0: Transfer"]
    TRANSFER = 0,
    #[doc = "1: Stop"]
    STOP = 1,
}
impl From<STPBGRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: STPBGRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl STPBGR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STPBGRSELECT_A {
        match self.bits {
            false => STPBGRSELECT_A::TRANSFER,
            true => STPBGRSELECT_A::STOP,
        }
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == STPBGRSELECT_A::TRANSFER
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STPBGRSELECT_A::STOP
    }
}
#[doc = "Field `STPBGR` writer - Stop at Block Gap Request"]
pub type STPBGR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STPBGRSELECT_A>;
impl<'a, REG, const O: u8> STPBGR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(STPBGRSELECT_A::TRANSFER)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(STPBGRSELECT_A::STOP)
    }
}
#[doc = "Field `CONTR` reader - Continue Request"]
pub type CONTR_R = crate::BitReader<CONTRSELECT_A>;
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONTRSELECT_A {
    #[doc = "0: Not affected"]
    GO_ON = 0,
    #[doc = "1: Restart"]
    RESTART = 1,
}
impl From<CONTRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CONTRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONTRSELECT_A {
        match self.bits {
            false => CONTRSELECT_A::GO_ON,
            true => CONTRSELECT_A::RESTART,
        }
    }
    #[doc = "Not affected"]
    #[inline(always)]
    pub fn is_go_on(&self) -> bool {
        *self == CONTRSELECT_A::GO_ON
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == CONTRSELECT_A::RESTART
    }
}
#[doc = "Field `CONTR` writer - Continue Request"]
pub type CONTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CONTRSELECT_A>;
impl<'a, REG, const O: u8> CONTR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not affected"]
    #[inline(always)]
    pub fn go_on(self) -> &'a mut crate::W<REG> {
        self.variant(CONTRSELECT_A::GO_ON)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut crate::W<REG> {
        self.variant(CONTRSELECT_A::RESTART)
    }
}
#[doc = "Field `RWCTRL` reader - Read Wait Control"]
pub type RWCTRL_R = crate::BitReader<RWCTRLSELECT_A>;
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWCTRLSELECT_A {
    #[doc = "0: Disable Read Wait Control"]
    DISABLE = 0,
    #[doc = "1: Enable Read Wait Control"]
    ENABLE = 1,
}
impl From<RWCTRLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RWCTRLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RWCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RWCTRLSELECT_A {
        match self.bits {
            false => RWCTRLSELECT_A::DISABLE,
            true => RWCTRLSELECT_A::ENABLE,
        }
    }
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RWCTRLSELECT_A::DISABLE
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RWCTRLSELECT_A::ENABLE
    }
}
#[doc = "Field `RWCTRL` writer - Read Wait Control"]
pub type RWCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RWCTRLSELECT_A>;
impl<'a, REG, const O: u8> RWCTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RWCTRLSELECT_A::DISABLE)
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RWCTRLSELECT_A::ENABLE)
    }
}
#[doc = "Field `INTBG` reader - Interrupt at Block Gap"]
pub type INTBG_R = crate::BitReader<INTBGSELECT_A>;
#[doc = "Interrupt at Block Gap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTBGSELECT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<INTBGSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INTBGSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INTBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTBGSELECT_A {
        match self.bits {
            false => INTBGSELECT_A::DISABLED,
            true => INTBGSELECT_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTBGSELECT_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTBGSELECT_A::ENABLED
    }
}
#[doc = "Field `INTBG` writer - Interrupt at Block Gap"]
pub type INTBG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INTBGSELECT_A>;
impl<'a, REG, const O: u8> INTBG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(INTBGSELECT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(INTBGSELECT_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stpbgr(&self) -> STPBGR_R {
        STPBGR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn contr(&self) -> CONTR_R {
        CONTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctrl(&self) -> RWCTRL_R {
        RWCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intbg(&self) -> INTBG_R {
        INTBG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline(always)]
    #[must_use]
    pub fn stpbgr(&mut self) -> STPBGR_W<BGCR_SPEC, 0> {
        STPBGR_W::new(self)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    #[must_use]
    pub fn contr(&mut self) -> CONTR_W<BGCR_SPEC, 1> {
        CONTR_W::new(self)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    #[must_use]
    pub fn rwctrl(&mut self) -> RWCTRL_W<BGCR_SPEC, 2> {
        RWCTRL_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt at Block Gap"]
    #[inline(always)]
    #[must_use]
    pub fn intbg(&mut self) -> INTBG_W<BGCR_SPEC, 3> {
        INTBG_W::new(self)
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
#[doc = "Block Gap Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BGCR_SPEC;
impl crate::RegisterSpec for BGCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bgcr::R`](R) reader structure"]
impl crate::Readable for BGCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bgcr::W`](W) writer structure"]
impl crate::Writable for BGCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BGCR to value 0"]
impl crate::Resettable for BGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
