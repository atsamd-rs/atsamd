#[doc = "Register `RVR` reader"]
pub type R = crate::R<RvrSpec>;
#[doc = "Register `RVR` writer"]
pub type W = crate::W<RvrSpec>;
#[doc = "Field `RELOAD` reader - Value to load into the SysTick Current Value Register when the counter reaches 0"]
pub type ReloadR = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - Value to load into the SysTick Current Value Register when the counter reaches 0"]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0"]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> ReloadW<RvrSpec> {
        ReloadW::new(self, 0)
    }
}
#[doc = "SysTick Reload Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RvrSpec;
impl crate::RegisterSpec for RvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rvr::R`](R) reader structure"]
impl crate::Readable for RvrSpec {}
#[doc = "`write(|w| ..)` method takes [`rvr::W`](W) writer structure"]
impl crate::Writable for RvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RVR to value 0"]
impl crate::Resettable for RvrSpec {
    const RESET_VALUE: u32 = 0;
}
