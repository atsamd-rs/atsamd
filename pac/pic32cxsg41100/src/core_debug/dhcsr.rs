#[doc = "Register `DHCSR` reader"]
pub type R = crate::R<DhcsrSpec>;
#[doc = "Register `DHCSR` writer"]
pub type W = crate::W<DhcsrSpec>;
#[doc = "Field `C_DEBUGEN` reader - "]
pub type CDebugenR = crate::BitReader;
#[doc = "Field `C_DEBUGEN` writer - "]
pub type CDebugenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_HALT` reader - "]
pub type CHaltR = crate::BitReader;
#[doc = "Field `C_HALT` writer - "]
pub type CHaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_STEP` reader - "]
pub type CStepR = crate::BitReader;
#[doc = "Field `C_STEP` writer - "]
pub type CStepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_MASKINTS` reader - "]
pub type CMaskintsR = crate::BitReader;
#[doc = "Field `C_MASKINTS` writer - "]
pub type CMaskintsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_SNAPSTALL` reader - "]
pub type CSnapstallR = crate::BitReader;
#[doc = "Field `C_SNAPSTALL` writer - "]
pub type CSnapstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_REGRDY` reader - "]
pub type SRegrdyR = crate::BitReader;
#[doc = "Field `DBGKEY` writer - "]
pub type DbgkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `S_HALT` reader - "]
pub type SHaltR = crate::BitReader;
#[doc = "Field `S_SLEEP` reader - "]
pub type SSleepR = crate::BitReader;
#[doc = "Field `S_LOCKUP` reader - "]
pub type SLockupR = crate::BitReader;
#[doc = "Field `S_RETIRE_ST` reader - "]
pub type SRetireStR = crate::BitReader;
#[doc = "Field `S_RESET_ST` reader - "]
pub type SResetStR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c_debugen(&self) -> CDebugenR {
        CDebugenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c_halt(&self) -> CHaltR {
        CHaltR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c_step(&self) -> CStepR {
        CStepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn c_maskints(&self) -> CMaskintsR {
        CMaskintsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn c_snapstall(&self) -> CSnapstallR {
        CSnapstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn s_regrdy(&self) -> SRegrdyR {
        SRegrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn s_halt(&self) -> SHaltR {
        SHaltR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn s_sleep(&self) -> SSleepR {
        SSleepR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn s_lockup(&self) -> SLockupR {
        SLockupR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn s_retire_st(&self) -> SRetireStR {
        SRetireStR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn s_reset_st(&self) -> SResetStR {
        SResetStR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn c_debugen(&mut self) -> CDebugenW<DhcsrSpec> {
        CDebugenW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn c_halt(&mut self) -> CHaltW<DhcsrSpec> {
        CHaltW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn c_step(&mut self) -> CStepW<DhcsrSpec> {
        CStepW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn c_maskints(&mut self) -> CMaskintsW<DhcsrSpec> {
        CMaskintsW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn c_snapstall(&mut self) -> CSnapstallW<DhcsrSpec> {
        CSnapstallW::new(self, 5)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn dbgkey(&mut self) -> DbgkeyW<DhcsrSpec> {
        DbgkeyW::new(self, 16)
    }
}
#[doc = "Debug Halting Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhcsrSpec;
impl crate::RegisterSpec for DhcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhcsr::R`](R) reader structure"]
impl crate::Readable for DhcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dhcsr::W`](W) writer structure"]
impl crate::Writable for DhcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DHCSR to value 0"]
impl crate::Resettable for DhcsrSpec {
    const RESET_VALUE: u32 = 0;
}
