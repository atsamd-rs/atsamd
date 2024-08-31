#[doc = "Register `PMUX[%s]` reader"]
pub type R = crate::R<PmuxSpec>;
#[doc = "Register `PMUX[%s]` writer"]
pub type W = crate::W<PmuxSpec>;
#[doc = "Field `PMUXE` reader - Peripheral Multiplexing for Even-Numbered Pin"]
pub type PmuxeR = crate::FieldReader;
#[doc = "Field `PMUXE` writer - Peripheral Multiplexing for Even-Numbered Pin"]
pub type PmuxeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PMUXO` reader - Peripheral Multiplexing for Odd-Numbered Pin"]
pub type PmuxoR = crate::FieldReader;
#[doc = "Field `PMUXO` writer - Peripheral Multiplexing for Odd-Numbered Pin"]
pub type PmuxoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&self) -> PmuxeR {
        PmuxeR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&self) -> PmuxoR {
        PmuxoR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxe(&mut self) -> PmuxeW<PmuxSpec> {
        PmuxeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxo(&mut self) -> PmuxoW<PmuxSpec> {
        PmuxoW::new(self, 4)
    }
}
#[doc = "Peripheral Multiplexing\n\nYou can [`read`](crate::Reg::read) this register and get [`pmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuxSpec;
impl crate::RegisterSpec for PmuxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmux::R`](R) reader structure"]
impl crate::Readable for PmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`pmux::W`](W) writer structure"]
impl crate::Writable for PmuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PMUX[%s]
to value 0"]
impl crate::Resettable for PmuxSpec {
    const RESET_VALUE: u8 = 0;
}
