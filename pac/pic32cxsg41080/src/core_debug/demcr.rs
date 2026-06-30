#[doc = "Register `DEMCR` reader"]
pub type R = crate::R<DemcrSpec>;
#[doc = "Register `DEMCR` writer"]
pub type W = crate::W<DemcrSpec>;
#[doc = "Field `VC_CORERESET` reader - "]
pub type VcCoreresetR = crate::BitReader;
#[doc = "Field `VC_CORERESET` writer - "]
pub type VcCoreresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_MMERR` reader - "]
pub type VcMmerrR = crate::BitReader;
#[doc = "Field `VC_MMERR` writer - "]
pub type VcMmerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_NOCPERR` reader - "]
pub type VcNocperrR = crate::BitReader;
#[doc = "Field `VC_NOCPERR` writer - "]
pub type VcNocperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_CHKERR` reader - "]
pub type VcChkerrR = crate::BitReader;
#[doc = "Field `VC_CHKERR` writer - "]
pub type VcChkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_STATERR` reader - "]
pub type VcStaterrR = crate::BitReader;
#[doc = "Field `VC_STATERR` writer - "]
pub type VcStaterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_BUSERR` reader - "]
pub type VcBuserrR = crate::BitReader;
#[doc = "Field `VC_BUSERR` writer - "]
pub type VcBuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_INTERR` reader - "]
pub type VcInterrR = crate::BitReader;
#[doc = "Field `VC_INTERR` writer - "]
pub type VcInterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_HARDERR` reader - "]
pub type VcHarderrR = crate::BitReader;
#[doc = "Field `VC_HARDERR` writer - "]
pub type VcHarderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_EN` reader - "]
pub type MonEnR = crate::BitReader;
#[doc = "Field `MON_EN` writer - "]
pub type MonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_PEND` reader - "]
pub type MonPendR = crate::BitReader;
#[doc = "Field `MON_PEND` writer - "]
pub type MonPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_STEP` reader - "]
pub type MonStepR = crate::BitReader;
#[doc = "Field `MON_STEP` writer - "]
pub type MonStepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_REQ` reader - "]
pub type MonReqR = crate::BitReader;
#[doc = "Field `MON_REQ` writer - "]
pub type MonReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCENA` reader - "]
pub type TrcenaR = crate::BitReader;
#[doc = "Field `TRCENA` writer - "]
pub type TrcenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vc_corereset(&self) -> VcCoreresetR {
        VcCoreresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VcMmerrR {
        VcMmerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VcNocperrR {
        VcNocperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VcChkerrR {
        VcChkerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VcStaterrR {
        VcStaterrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VcBuserrR {
        VcBuserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vc_interr(&self) -> VcInterrR {
        VcInterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VcHarderrR {
        VcHarderrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mon_en(&self) -> MonEnR {
        MonEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mon_pend(&self) -> MonPendR {
        MonPendR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mon_step(&self) -> MonStepR {
        MonStepR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mon_req(&self) -> MonReqR {
        MonReqR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn trcena(&self) -> TrcenaR {
        TrcenaR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn vc_corereset(&mut self) -> VcCoreresetW<DemcrSpec> {
        VcCoreresetW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn vc_mmerr(&mut self) -> VcMmerrW<DemcrSpec> {
        VcMmerrW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn vc_nocperr(&mut self) -> VcNocperrW<DemcrSpec> {
        VcNocperrW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn vc_chkerr(&mut self) -> VcChkerrW<DemcrSpec> {
        VcChkerrW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn vc_staterr(&mut self) -> VcStaterrW<DemcrSpec> {
        VcStaterrW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn vc_buserr(&mut self) -> VcBuserrW<DemcrSpec> {
        VcBuserrW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn vc_interr(&mut self) -> VcInterrW<DemcrSpec> {
        VcInterrW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn vc_harderr(&mut self) -> VcHarderrW<DemcrSpec> {
        VcHarderrW::new(self, 10)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn mon_en(&mut self) -> MonEnW<DemcrSpec> {
        MonEnW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn mon_pend(&mut self) -> MonPendW<DemcrSpec> {
        MonPendW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn mon_step(&mut self) -> MonStepW<DemcrSpec> {
        MonStepW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn mon_req(&mut self) -> MonReqW<DemcrSpec> {
        MonReqW::new(self, 19)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn trcena(&mut self) -> TrcenaW<DemcrSpec> {
        TrcenaW::new(self, 24)
    }
}
#[doc = "Debug Exception and Monitor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`demcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DemcrSpec;
impl crate::RegisterSpec for DemcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`demcr::R`](R) reader structure"]
impl crate::Readable for DemcrSpec {}
#[doc = "`write(|w| ..)` method takes [`demcr::W`](W) writer structure"]
impl crate::Writable for DemcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEMCR to value 0"]
impl crate::Resettable for DemcrSpec {
    const RESET_VALUE: u32 = 0;
}
