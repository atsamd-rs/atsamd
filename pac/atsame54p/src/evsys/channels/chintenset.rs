#[doc = "Register `CHINTENSET` reader"]
pub type R = crate::R<ChintensetSpec>;
#[doc = "Register `CHINTENSET` writer"]
pub type W = crate::W<ChintensetSpec>;
#[doc = "Field `OVR` reader - Channel Overrun Interrupt Enable"]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - Channel Overrun Interrupt Enable"]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD` reader - Channel Event Detected Interrupt Enable"]
pub type EvdR = crate::BitReader;
#[doc = "Field `EVD` writer - Channel Event Detected Interrupt Enable"]
pub type EvdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Event Detected Interrupt Enable"]
    #[inline(always)]
    pub fn evd(&self) -> EvdR {
        EvdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OvrW<ChintensetSpec> {
        OvrW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Event Detected Interrupt Enable"]
    #[inline(always)]
    pub fn evd(&mut self) -> EvdW<ChintensetSpec> {
        EvdW::new(self, 1)
    }
}
#[doc = "Channel n Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChintensetSpec;
impl crate::RegisterSpec for ChintensetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chintenset::R`](R) reader structure"]
impl crate::Readable for ChintensetSpec {}
#[doc = "`write(|w| ..)` method takes [`chintenset::W`](W) writer structure"]
impl crate::Writable for ChintensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHINTENSET to value 0"]
impl crate::Resettable for ChintensetSpec {}
