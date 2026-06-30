#[doc = "Register `RWD` reader"]
pub type R = crate::R<RwdSpec>;
#[doc = "Register `RWD` writer"]
pub type W = crate::W<RwdSpec>;
#[doc = "Field `WDC` reader - Watchdog Configuration"]
pub type WdcR = crate::FieldReader;
#[doc = "Field `WDC` writer - Watchdog Configuration"]
pub type WdcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDV` reader - Watchdog Value"]
pub type WdvR = crate::FieldReader;
#[doc = "Field `WDV` writer - Watchdog Value"]
pub type WdvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Watchdog Configuration"]
    #[inline(always)]
    pub fn wdc(&self) -> WdcR {
        WdcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog Value"]
    #[inline(always)]
    pub fn wdv(&self) -> WdvR {
        WdvR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WdcW<RwdSpec> {
        WdcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Watchdog Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdv(&mut self) -> WdvW<RwdSpec> {
        WdvW::new(self, 8)
    }
}
#[doc = "RAM Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`rwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RwdSpec;
impl crate::RegisterSpec for RwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rwd::R`](R) reader structure"]
impl crate::Readable for RwdSpec {}
#[doc = "`write(|w| ..)` method takes [`rwd::W`](W) writer structure"]
impl crate::Writable for RwdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RWD to value 0"]
impl crate::Resettable for RwdSpec {
    const RESET_VALUE: u32 = 0;
}
