#[doc = "Register `CC2R` reader"]
pub type R = crate::R<Cc2rSpec>;
#[doc = "Register `CC2R` writer"]
pub type W = crate::W<Cc2rSpec>;
#[doc = "Force SDCK Disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsdclkdselect {
    #[doc = "0: No effect"]
    Noeffect = 0,
    #[doc = "1: SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    Disable = 1,
}
impl From<Fsdclkdselect> for bool {
    #[inline(always)]
    fn from(variant: Fsdclkdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSDCLKD` reader - Force SDCK Disabled"]
pub type FsdclkdR = crate::BitReader<Fsdclkdselect>;
impl FsdclkdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsdclkdselect {
        match self.bits {
            false => Fsdclkdselect::Noeffect,
            true => Fsdclkdselect::Disable,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_noeffect(&self) -> bool {
        *self == Fsdclkdselect::Noeffect
    }
    #[doc = "SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fsdclkdselect::Disable
    }
}
#[doc = "Field `FSDCLKD` writer - Force SDCK Disabled"]
pub type FsdclkdW<'a, REG> = crate::BitWriter<'a, REG, Fsdclkdselect>;
impl<'a, REG> FsdclkdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut crate::W<REG> {
        self.variant(Fsdclkdselect::Noeffect)
    }
    #[doc = "SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fsdclkdselect::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - Force SDCK Disabled"]
    #[inline(always)]
    pub fn fsdclkd(&self) -> FsdclkdR {
        FsdclkdR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force SDCK Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn fsdclkd(&mut self) -> FsdclkdW<Cc2rSpec> {
        FsdclkdW::new(self, 0)
    }
}
#[doc = "Clock Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2rSpec;
impl crate::RegisterSpec for Cc2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2r::R`](R) reader structure"]
impl crate::Readable for Cc2rSpec {}
#[doc = "`write(|w| ..)` method takes [`cc2r::W`](W) writer structure"]
impl crate::Writable for Cc2rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC2R to value 0"]
impl crate::Resettable for Cc2rSpec {
    const RESET_VALUE: u32 = 0;
}
