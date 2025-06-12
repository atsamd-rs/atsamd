#[doc = "Register `VREG` reader"]
pub type R = crate::R<VregSpec>;
#[doc = "Register `VREG` writer"]
pub type W = crate::W<VregSpec>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCELDO` reader - Force LDO Voltage Regulator"]
pub type ForceldoR = crate::BitReader;
#[doc = "Field `FORCELDO` writer - Force LDO Voltage Regulator"]
pub type ForceldoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - Force LDO Voltage Regulator"]
    #[inline(always)]
    pub fn forceldo(&self) -> ForceldoR {
        ForceldoR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<VregSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 13 - Force LDO Voltage Regulator"]
    #[inline(always)]
    pub fn forceldo(&mut self) -> ForceldoW<VregSpec> {
        ForceldoW::new(self, 13)
    }
}
#[doc = "Voltage Regulator System (VREG) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VregSpec;
impl crate::RegisterSpec for VregSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`vreg::R`](R) reader structure"]
impl crate::Readable for VregSpec {}
#[doc = "`write(|w| ..)` method takes [`vreg::W`](W) writer structure"]
impl crate::Writable for VregSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREG to value 0"]
impl crate::Resettable for VregSpec {}
