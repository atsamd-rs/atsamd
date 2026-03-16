#[doc = "Register `SHCSR` reader"]
pub type R = crate::R<ShcsrSpec>;
#[doc = "Register `SHCSR` writer"]
pub type W = crate::W<ShcsrSpec>;
#[doc = "Field `MEMFAULTACT` reader - MemManage exception active bit"]
pub type MemfaultactR = crate::BitReader;
#[doc = "Field `MEMFAULTACT` writer - MemManage exception active bit"]
pub type MemfaultactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFAULTACT` reader - BusFault exception active bit"]
pub type BusfaultactR = crate::BitReader;
#[doc = "Field `BUSFAULTACT` writer - BusFault exception active bit"]
pub type BusfaultactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USGFAULTACT` reader - UsageFault exception active bit"]
pub type UsgfaultactR = crate::BitReader;
#[doc = "Field `USGFAULTACT` writer - UsageFault exception active bit"]
pub type UsgfaultactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVCALLACT` reader - SVCall active bit"]
pub type SvcallactR = crate::BitReader;
#[doc = "Field `SVCALLACT` writer - SVCall active bit"]
pub type SvcallactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONITORACT` reader - DebugMonitor exception active bit"]
pub type MonitoractR = crate::BitReader;
#[doc = "Field `MONITORACT` writer - DebugMonitor exception active bit"]
pub type MonitoractW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDSVACT` reader - PendSV exception active bit"]
pub type PendsvactR = crate::BitReader;
#[doc = "Field `PENDSVACT` writer - PendSV exception active bit"]
pub type PendsvactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTICKACT` reader - SysTick exception active bit"]
pub type SystickactR = crate::BitReader;
#[doc = "Field `SYSTICKACT` writer - SysTick exception active bit"]
pub type SystickactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USGFAULTPENDED` reader - UsageFault exception pending bit"]
pub type UsgfaultpendedR = crate::BitReader;
#[doc = "Field `USGFAULTPENDED` writer - UsageFault exception pending bit"]
pub type UsgfaultpendedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMFAULTPENDED` reader - MemManage exception pending bit"]
pub type MemfaultpendedR = crate::BitReader;
#[doc = "Field `MEMFAULTPENDED` writer - MemManage exception pending bit"]
pub type MemfaultpendedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFAULTPENDED` reader - BusFault exception pending bit"]
pub type BusfaultpendedR = crate::BitReader;
#[doc = "Field `BUSFAULTPENDED` writer - BusFault exception pending bit"]
pub type BusfaultpendedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVCALLPENDED` reader - SVCall pending bit"]
pub type SvcallpendedR = crate::BitReader;
#[doc = "Field `SVCALLPENDED` writer - SVCall pending bit"]
pub type SvcallpendedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMFAULTENA` reader - MemManage enable bit"]
pub type MemfaultenaR = crate::BitReader;
#[doc = "Field `MEMFAULTENA` writer - MemManage enable bit"]
pub type MemfaultenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFAULTENA` reader - BusFault enable bit"]
pub type BusfaultenaR = crate::BitReader;
#[doc = "Field `BUSFAULTENA` writer - BusFault enable bit"]
pub type BusfaultenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USGFAULTENA` reader - UsageFault enable bit"]
pub type UsgfaultenaR = crate::BitReader;
#[doc = "Field `USGFAULTENA` writer - UsageFault enable bit"]
pub type UsgfaultenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MemManage exception active bit"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MemfaultactR {
        MemfaultactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BusFault exception active bit"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BusfaultactR {
        BusfaultactR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - UsageFault exception active bit"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> UsgfaultactR {
        UsgfaultactR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - SVCall active bit"]
    #[inline(always)]
    pub fn svcallact(&self) -> SvcallactR {
        SvcallactR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DebugMonitor exception active bit"]
    #[inline(always)]
    pub fn monitoract(&self) -> MonitoractR {
        MonitoractR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - PendSV exception active bit"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PendsvactR {
        PendsvactR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SysTick exception active bit"]
    #[inline(always)]
    pub fn systickact(&self) -> SystickactR {
        SystickactR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UsageFault exception pending bit"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> UsgfaultpendedR {
        UsgfaultpendedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MemManage exception pending bit"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MemfaultpendedR {
        MemfaultpendedR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BusFault exception pending bit"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BusfaultpendedR {
        BusfaultpendedR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SVCall pending bit"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SvcallpendedR {
        SvcallpendedR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MemManage enable bit"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MemfaultenaR {
        MemfaultenaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BusFault enable bit"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BusfaultenaR {
        BusfaultenaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UsageFault enable bit"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> UsgfaultenaR {
        UsgfaultenaR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MemManage exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultact(&mut self) -> MemfaultactW<ShcsrSpec> {
        MemfaultactW::new(self, 0)
    }
    #[doc = "Bit 1 - BusFault exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultact(&mut self) -> BusfaultactW<ShcsrSpec> {
        BusfaultactW::new(self, 1)
    }
    #[doc = "Bit 3 - UsageFault exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultact(&mut self) -> UsgfaultactW<ShcsrSpec> {
        UsgfaultactW::new(self, 3)
    }
    #[doc = "Bit 7 - SVCall active bit"]
    #[inline(always)]
    #[must_use]
    pub fn svcallact(&mut self) -> SvcallactW<ShcsrSpec> {
        SvcallactW::new(self, 7)
    }
    #[doc = "Bit 8 - DebugMonitor exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn monitoract(&mut self) -> MonitoractW<ShcsrSpec> {
        MonitoractW::new(self, 8)
    }
    #[doc = "Bit 10 - PendSV exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvact(&mut self) -> PendsvactW<ShcsrSpec> {
        PendsvactW::new(self, 10)
    }
    #[doc = "Bit 11 - SysTick exception active bit"]
    #[inline(always)]
    #[must_use]
    pub fn systickact(&mut self) -> SystickactW<ShcsrSpec> {
        SystickactW::new(self, 11)
    }
    #[doc = "Bit 12 - UsageFault exception pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultpended(&mut self) -> UsgfaultpendedW<ShcsrSpec> {
        UsgfaultpendedW::new(self, 12)
    }
    #[doc = "Bit 13 - MemManage exception pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultpended(&mut self) -> MemfaultpendedW<ShcsrSpec> {
        MemfaultpendedW::new(self, 13)
    }
    #[doc = "Bit 14 - BusFault exception pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultpended(&mut self) -> BusfaultpendedW<ShcsrSpec> {
        BusfaultpendedW::new(self, 14)
    }
    #[doc = "Bit 15 - SVCall pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn svcallpended(&mut self) -> SvcallpendedW<ShcsrSpec> {
        SvcallpendedW::new(self, 15)
    }
    #[doc = "Bit 16 - MemManage enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultena(&mut self) -> MemfaultenaW<ShcsrSpec> {
        MemfaultenaW::new(self, 16)
    }
    #[doc = "Bit 17 - BusFault enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultena(&mut self) -> BusfaultenaW<ShcsrSpec> {
        BusfaultenaW::new(self, 17)
    }
    #[doc = "Bit 18 - UsageFault enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultena(&mut self) -> UsgfaultenaW<ShcsrSpec> {
        UsgfaultenaW::new(self, 18)
    }
}
#[doc = "System Handler Control and State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShcsrSpec;
impl crate::RegisterSpec for ShcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shcsr::R`](R) reader structure"]
impl crate::Readable for ShcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`shcsr::W`](W) writer structure"]
impl crate::Writable for ShcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for ShcsrSpec {
    const RESET_VALUE: u32 = 0;
}
