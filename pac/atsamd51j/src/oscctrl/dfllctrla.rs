#[doc = "Register `DFLLCTRLA` reader"]
pub type R = crate::R<DfllctrlaSpec>;
#[doc = "Register `DFLLCTRLA` writer"]
pub type W = crate::W<DfllctrlaSpec>;
#[doc = "Field `ENABLE` reader - DFLL Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - DFLL Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - DFLL Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DFLL Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<DfllctrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<DfllctrlaSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<DfllctrlaSpec> {
        OndemandW::new(self, 7)
    }
}
#[doc = "DFLL48M Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfllctrlaSpec;
impl crate::RegisterSpec for DfllctrlaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dfllctrla::R`](R) reader structure"]
impl crate::Readable for DfllctrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`dfllctrla::W`](W) writer structure"]
impl crate::Writable for DfllctrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFLLCTRLA to value 0x82"]
impl crate::Resettable for DfllctrlaSpec {
    const RESET_VALUE: u8 = 0x82;
}
