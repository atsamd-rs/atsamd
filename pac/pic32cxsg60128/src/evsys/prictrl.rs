#[doc = "Register `PRICTRL` reader"]
pub type R = crate::R<PrictrlSpec>;
#[doc = "Register `PRICTRL` writer"]
pub type W = crate::W<PrictrlSpec>;
#[doc = "Field `PRI` reader - Channel Priority Number"]
pub type PriR = crate::FieldReader;
#[doc = "Field `PRI` writer - Channel Priority Number"]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RREN` reader - Round-Robin Scheduling Enable"]
pub type RrenR = crate::BitReader;
#[doc = "Field `RREN` writer - Round-Robin Scheduling Enable"]
pub type RrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Channel Priority Number"]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rren(&self) -> RrenR {
        RrenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PriW<PrictrlSpec> {
        PriW::new(self, 0)
    }
    #[doc = "Bit 7 - Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rren(&mut self) -> RrenW<PrictrlSpec> {
        RrenW::new(self, 7)
    }
}
#[doc = "Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`prictrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prictrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrictrlSpec;
impl crate::RegisterSpec for PrictrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prictrl::R`](R) reader structure"]
impl crate::Readable for PrictrlSpec {}
#[doc = "`write(|w| ..)` method takes [`prictrl::W`](W) writer structure"]
impl crate::Writable for PrictrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PRICTRL to value 0"]
impl crate::Resettable for PrictrlSpec {
    const RESET_VALUE: u8 = 0;
}
