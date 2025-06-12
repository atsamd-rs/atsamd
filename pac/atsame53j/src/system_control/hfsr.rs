#[doc = "Register `HFSR` reader"]
pub type R = crate::R<HfsrSpec>;
#[doc = "Register `HFSR` writer"]
pub type W = crate::W<HfsrSpec>;
#[doc = "Field `VECTTBL` reader - BusFault on a Vector Table read during exception processing"]
pub type VecttblR = crate::BitReader;
#[doc = "Field `VECTTBL` writer - BusFault on a Vector Table read during exception processing"]
pub type VecttblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCED` reader - Forced Hard Fault"]
pub type ForcedR = crate::BitReader;
#[doc = "Field `FORCED` writer - Forced Hard Fault"]
pub type ForcedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGEVT` reader - Debug: always write 0"]
pub type DebugevtR = crate::BitReader;
#[doc = "Field `DEBUGEVT` writer - Debug: always write 0"]
pub type DebugevtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - BusFault on a Vector Table read during exception processing"]
    #[inline(always)]
    pub fn vecttbl(&self) -> VecttblR {
        VecttblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 30 - Forced Hard Fault"]
    #[inline(always)]
    pub fn forced(&self) -> ForcedR {
        ForcedR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Debug: always write 0"]
    #[inline(always)]
    pub fn debugevt(&self) -> DebugevtR {
        DebugevtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BusFault on a Vector Table read during exception processing"]
    #[inline(always)]
    pub fn vecttbl(&mut self) -> VecttblW<HfsrSpec> {
        VecttblW::new(self, 1)
    }
    #[doc = "Bit 30 - Forced Hard Fault"]
    #[inline(always)]
    pub fn forced(&mut self) -> ForcedW<HfsrSpec> {
        ForcedW::new(self, 30)
    }
    #[doc = "Bit 31 - Debug: always write 0"]
    #[inline(always)]
    pub fn debugevt(&mut self) -> DebugevtW<HfsrSpec> {
        DebugevtW::new(self, 31)
    }
}
#[doc = "HardFault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfsrSpec;
impl crate::RegisterSpec for HfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfsr::R`](R) reader structure"]
impl crate::Readable for HfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`hfsr::W`](W) writer structure"]
impl crate::Writable for HfsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFSR to value 0"]
impl crate::Resettable for HfsrSpec {}
