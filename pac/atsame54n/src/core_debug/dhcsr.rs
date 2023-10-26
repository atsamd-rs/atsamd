#[doc = "Register `DHCSR` reader"]
pub type R = crate::R<DHCSR_SPEC>;
#[doc = "Register `DHCSR` writer"]
pub type W = crate::W<DHCSR_SPEC>;
#[doc = "Field `C_DEBUGEN` reader - "]
pub type C_DEBUGEN_R = crate::BitReader;
#[doc = "Field `C_DEBUGEN` writer - "]
pub type C_DEBUGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C_HALT` reader - "]
pub type C_HALT_R = crate::BitReader;
#[doc = "Field `C_HALT` writer - "]
pub type C_HALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C_STEP` reader - "]
pub type C_STEP_R = crate::BitReader;
#[doc = "Field `C_STEP` writer - "]
pub type C_STEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C_MASKINTS` reader - "]
pub type C_MASKINTS_R = crate::BitReader;
#[doc = "Field `C_MASKINTS` writer - "]
pub type C_MASKINTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C_SNAPSTALL` reader - "]
pub type C_SNAPSTALL_R = crate::BitReader;
#[doc = "Field `C_SNAPSTALL` writer - "]
pub type C_SNAPSTALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `S_REGRDY` reader - "]
pub type S_REGRDY_R = crate::BitReader;
#[doc = "Field `DBGKEY` writer - "]
pub type DBGKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `S_HALT` reader - "]
pub type S_HALT_R = crate::BitReader;
#[doc = "Field `S_SLEEP` reader - "]
pub type S_SLEEP_R = crate::BitReader;
#[doc = "Field `S_LOCKUP` reader - "]
pub type S_LOCKUP_R = crate::BitReader;
#[doc = "Field `S_RETIRE_ST` reader - "]
pub type S_RETIRE_ST_R = crate::BitReader;
#[doc = "Field `S_RESET_ST` reader - "]
pub type S_RESET_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c_debugen(&self) -> C_DEBUGEN_R {
        C_DEBUGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c_halt(&self) -> C_HALT_R {
        C_HALT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c_step(&self) -> C_STEP_R {
        C_STEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn c_maskints(&self) -> C_MASKINTS_R {
        C_MASKINTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn c_snapstall(&self) -> C_SNAPSTALL_R {
        C_SNAPSTALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn s_regrdy(&self) -> S_REGRDY_R {
        S_REGRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn s_halt(&self) -> S_HALT_R {
        S_HALT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn s_sleep(&self) -> S_SLEEP_R {
        S_SLEEP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn s_lockup(&self) -> S_LOCKUP_R {
        S_LOCKUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn s_retire_st(&self) -> S_RETIRE_ST_R {
        S_RETIRE_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn s_reset_st(&self) -> S_RESET_ST_R {
        S_RESET_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn c_debugen(&mut self) -> C_DEBUGEN_W<DHCSR_SPEC, 0> {
        C_DEBUGEN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn c_halt(&mut self) -> C_HALT_W<DHCSR_SPEC, 1> {
        C_HALT_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn c_step(&mut self) -> C_STEP_W<DHCSR_SPEC, 2> {
        C_STEP_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn c_maskints(&mut self) -> C_MASKINTS_W<DHCSR_SPEC, 3> {
        C_MASKINTS_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn c_snapstall(&mut self) -> C_SNAPSTALL_W<DHCSR_SPEC, 5> {
        C_SNAPSTALL_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn dbgkey(&mut self) -> DBGKEY_W<DHCSR_SPEC, 16> {
        DBGKEY_W::new(self)
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
#[doc = "Debug Halting Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHCSR_SPEC;
impl crate::RegisterSpec for DHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhcsr::R`](R) reader structure"]
impl crate::Readable for DHCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dhcsr::W`](W) writer structure"]
impl crate::Writable for DHCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DHCSR to value 0"]
impl crate::Resettable for DHCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
