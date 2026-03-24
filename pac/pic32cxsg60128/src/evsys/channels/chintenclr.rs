#[doc = "Register `CHINTENCLR` reader"]
pub type R = crate::R<ChintenclrSpec>;
#[doc = "Register `CHINTENCLR` writer"]
pub type W = crate::W<ChintenclrSpec>;
#[doc = "Field `OVR` reader - Channel Overrun Interrupt Disable"]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - Channel Overrun Interrupt Disable"]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD` reader - Channel Event Detected Interrupt Disable"]
pub type EvdR = crate::BitReader;
#[doc = "Field `EVD` writer - Channel Event Detected Interrupt Disable"]
pub type EvdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Overrun Interrupt Disable"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Event Detected Interrupt Disable"]
    #[inline(always)]
    pub fn evd(&self) -> EvdR {
        EvdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OvrW<ChintenclrSpec> {
        OvrW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Event Detected Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn evd(&mut self) -> EvdW<ChintenclrSpec> {
        EvdW::new(self, 1)
    }
}
#[doc = "Channel n Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChintenclrSpec;
impl crate::RegisterSpec for ChintenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chintenclr::R`](R) reader structure"]
impl crate::Readable for ChintenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`chintenclr::W`](W) writer structure"]
impl crate::Writable for ChintenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CHINTENCLR to value 0"]
impl crate::Resettable for ChintenclrSpec {
    const RESET_VALUE: u8 = 0;
}
