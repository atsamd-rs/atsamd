#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `BUSY` reader - FREQM Status"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - FREQM Status"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - Sticky Count Value Overflow"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Sticky Count Value Overflow"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FREQM Status"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sticky Count Value Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FREQM Status"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<StatusSpec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Sticky Count Value Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<StatusSpec> {
        OvfW::new(self, 1)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u8 = 0;
}
