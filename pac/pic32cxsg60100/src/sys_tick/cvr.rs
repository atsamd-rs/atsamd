#[doc = "Register `CVR` reader"]
pub type R = crate::R<CvrSpec>;
#[doc = "Register `CVR` writer"]
pub type W = crate::W<CvrSpec>;
#[doc = "Field `CURRENT` reader - Current value at the time the register is accessed"]
pub type CurrentR = crate::FieldReader<u32>;
#[doc = "Field `CURRENT` writer - Current value at the time the register is accessed"]
pub type CurrentW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Current value at the time the register is accessed"]
    #[inline(always)]
    pub fn current(&self) -> CurrentR {
        CurrentR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Current value at the time the register is accessed"]
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CurrentW<CvrSpec> {
        CurrentW::new(self, 0)
    }
}
#[doc = "SysTick Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvrSpec;
impl crate::RegisterSpec for CvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvr::R`](R) reader structure"]
impl crate::Readable for CvrSpec {}
#[doc = "`write(|w| ..)` method takes [`cvr::W`](W) writer structure"]
impl crate::Writable for CvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CVR to value 0"]
impl crate::Resettable for CvrSpec {
    const RESET_VALUE: u32 = 0;
}
