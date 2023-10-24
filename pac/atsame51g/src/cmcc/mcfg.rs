#[doc = "Register `MCFG` reader"]
pub type R = crate::R<MCFG_SPEC>;
#[doc = "Register `MCFG` writer"]
pub type W = crate::W<MCFG_SPEC>;
#[doc = "Field `MODE` reader - Cache Controller Monitor Counter Mode"]
pub type MODE_R = crate::FieldReader<MODESELECT_A>;
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
impl crate::FieldSpec for MODESELECT_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::CYCLE_COUNT),
            1 => Some(MODESELECT_A::IHIT_COUNT),
            2 => Some(MODESELECT_A::DHIT_COUNT),
            _ => None,
        }
    }
    #[doc = "Cycle counter"]
    #[inline(always)]
    pub fn is_cycle_count(&self) -> bool {
        *self == MODESELECT_A::CYCLE_COUNT
    }
    #[doc = "Instruction hit counter"]
    #[inline(always)]
    pub fn is_ihit_count(&self) -> bool {
        *self == MODESELECT_A::IHIT_COUNT
    }
    #[doc = "Data hit counter"]
    #[inline(always)]
    pub fn is_dhit_count(&self) -> bool {
        *self == MODESELECT_A::DHIT_COUNT
    }
}
#[doc = "Field `MODE` writer - Cache Controller Monitor Counter Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, MODESELECT_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Cycle counter"]
    #[inline(always)]
    pub fn cycle_count(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::CYCLE_COUNT)
    }
    #[doc = "Instruction hit counter"]
    #[inline(always)]
    pub fn ihit_count(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::IHIT_COUNT)
    }
    #[doc = "Data hit counter"]
    #[inline(always)]
    pub fn dhit_count(self) -> &'a mut crate::W<REG> {
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
    pub fn mode(&mut self) -> MODE_W<MCFG_SPEC, 0> {
        MODE_W::new(self)
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
#[doc = "Cache Monitor Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCFG_SPEC;
impl crate::RegisterSpec for MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcfg::R`](R) reader structure"]
impl crate::Readable for MCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcfg::W`](W) writer structure"]
impl crate::Writable for MCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCFG to value 0"]
impl crate::Resettable for MCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
