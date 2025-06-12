#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Register `INTPEND` writer"]
pub type W = crate::W<IntpendSpec>;
#[doc = "Field `ID` reader - Channel ID"]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - Channel ID"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OVR` reader - Channel Overrun"]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - Channel Overrun"]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD` reader - Channel Event Detected"]
pub type EvdR = crate::BitReader;
#[doc = "Field `EVD` writer - Channel Event Detected"]
pub type EvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READY` reader - Ready"]
pub type ReadyR = crate::BitReader;
#[doc = "Field `READY` writer - Ready"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Event Detected"]
    #[inline(always)]
    pub fn evd(&self) -> EvdR {
        EvdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Ready"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<IntpendSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bit 8 - Channel Overrun"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OvrW<IntpendSpec> {
        OvrW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel Event Detected"]
    #[inline(always)]
    pub fn evd(&mut self) -> EvdW<IntpendSpec> {
        EvdW::new(self, 9)
    }
    #[doc = "Bit 14 - Ready"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<IntpendSpec> {
        ReadyW::new(self, 14)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<IntpendSpec> {
        BusyW::new(self, 15)
    }
}
#[doc = "Channel Pending Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpendSpec;
impl crate::RegisterSpec for IntpendSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intpend::R`](R) reader structure"]
impl crate::Readable for IntpendSpec {}
#[doc = "`write(|w| ..)` method takes [`intpend::W`](W) writer structure"]
impl crate::Writable for IntpendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTPEND to value 0x4000"]
impl crate::Resettable for IntpendSpec {
    const RESET_VALUE: u16 = 0x4000;
}
