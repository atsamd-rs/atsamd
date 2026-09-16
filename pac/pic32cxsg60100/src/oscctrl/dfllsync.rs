#[doc = "Register `DFLLSYNC` reader"]
pub type R = crate::R<DfllsyncSpec>;
#[doc = "Register `DFLLSYNC` writer"]
pub type W = crate::W<DfllsyncSpec>;
#[doc = "Field `ENABLE` reader - ENABLE Synchronization Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - ENABLE Synchronization Busy"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLCTRLB` reader - DFLLCTRLB Synchronization Busy"]
pub type DfllctrlbR = crate::BitReader;
#[doc = "Field `DFLLCTRLB` writer - DFLLCTRLB Synchronization Busy"]
pub type DfllctrlbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLVAL` reader - DFLLVAL Synchronization Busy"]
pub type DfllvalR = crate::BitReader;
#[doc = "Field `DFLLVAL` writer - DFLLVAL Synchronization Busy"]
pub type DfllvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLMUL` reader - DFLLMUL Synchronization Busy"]
pub type DfllmulR = crate::BitReader;
#[doc = "Field `DFLLMUL` writer - DFLLMUL Synchronization Busy"]
pub type DfllmulW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn dfllctrlb(&self) -> DfllctrlbR {
        DfllctrlbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllval(&self) -> DfllvalR {
        DfllvalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllmul(&self) -> DfllmulR {
        DfllmulR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<DfllsyncSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn dfllctrlb(&mut self) -> DfllctrlbW<DfllsyncSpec> {
        DfllctrlbW::new(self, 2)
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn dfllval(&mut self) -> DfllvalW<DfllsyncSpec> {
        DfllvalW::new(self, 3)
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn dfllmul(&mut self) -> DfllmulW<DfllsyncSpec> {
        DfllmulW::new(self, 4)
    }
}
#[doc = "DFLL48M Synchronization\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllsync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllsync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfllsyncSpec;
impl crate::RegisterSpec for DfllsyncSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dfllsync::R`](R) reader structure"]
impl crate::Readable for DfllsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`dfllsync::W`](W) writer structure"]
impl crate::Writable for DfllsyncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DFLLSYNC to value 0"]
impl crate::Resettable for DfllsyncSpec {
    const RESET_VALUE: u8 = 0;
}
