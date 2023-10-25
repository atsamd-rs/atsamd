#[doc = "Register `BGCR_EMMC_MODE` reader"]
pub type R = crate::R<BGCR_EMMC_MODE_SPEC>;
#[doc = "Register `BGCR_EMMC_MODE` writer"]
pub type W = crate::W<BGCR_EMMC_MODE_SPEC>;
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
}
impl W {
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline(always)]
    #[must_use]
    pub fn stpbgr(&mut self) -> STPBGR_W<BGCR_EMMC_MODE_SPEC, 0> {
        STPBGR_W::new(self)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    #[must_use]
    pub fn contr(&mut self) -> CONTR_W<BGCR_EMMC_MODE_SPEC, 1> {
        CONTR_W::new(self)
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
#[doc = "Block Gap Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcr_emmc_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcr_emmc_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BGCR_EMMC_MODE_SPEC;
impl crate::RegisterSpec for BGCR_EMMC_MODE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bgcr_emmc_mode::R`](R) reader structure"]
impl crate::Readable for BGCR_EMMC_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bgcr_emmc_mode::W`](W) writer structure"]
impl crate::Writable for BGCR_EMMC_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BGCR_EMMC_MODE to value 0"]
impl crate::Resettable for BGCR_EMMC_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
