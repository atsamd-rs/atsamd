#[doc = "Register `CC2R` reader"]
pub type R = crate::R<CC2R_SPEC>;
#[doc = "Register `CC2R` writer"]
pub type W = crate::W<CC2R_SPEC>;
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
    pub const fn variant(&self) -> FSDCLKDSELECT_A {
        match self.bits {
            false => FSDCLKDSELECT_A::NOEFFECT,
            true => FSDCLKDSELECT_A::DISABLE,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_noeffect(&self) -> bool {
        *self == FSDCLKDSELECT_A::NOEFFECT
    }
    #[doc = "SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FSDCLKDSELECT_A::DISABLE
    }
}
#[doc = "Field `FSDCLKD` writer - Force SDCK Disabled"]
pub type FSDCLKD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FSDCLKDSELECT_A>;
impl<'a, REG, const O: u8> FSDCLKD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut crate::W<REG> {
        self.variant(FSDCLKDSELECT_A::NOEFFECT)
    }
    #[doc = "SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
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
    pub fn fsdclkd(&mut self) -> FSDCLKD_W<CC2R_SPEC, 0> {
        FSDCLKD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC2R_SPEC;
impl crate::RegisterSpec for CC2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2r::R`](R) reader structure"]
impl crate::Readable for CC2R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc2r::W`](W) writer structure"]
impl crate::Writable for CC2R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC2R to value 0"]
impl crate::Resettable for CC2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
