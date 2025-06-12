#[doc = "Register `CHINTFLAG` reader"]
pub type R = crate::R<ChintflagSpec>;
#[doc = "Register `CHINTFLAG` writer"]
pub type W = crate::W<ChintflagSpec>;
#[doc = "Field `OVR` reader - Channel Overrun"]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - Channel Overrun"]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD` reader - Channel Event Detected"]
pub type EvdR = crate::BitReader;
#[doc = "Field `EVD` writer - Channel Event Detected"]
pub type EvdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Event Detected"]
    #[inline(always)]
    pub fn evd(&self) -> EvdR {
        EvdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Overrun"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OvrW<ChintflagSpec> {
        OvrW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Event Detected"]
    #[inline(always)]
    pub fn evd(&mut self) -> EvdW<ChintflagSpec> {
        EvdW::new(self, 1)
    }
}
#[doc = "Channel n Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chintflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChintflagSpec;
impl crate::RegisterSpec for ChintflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chintflag::R`](R) reader structure"]
impl crate::Readable for ChintflagSpec {}
#[doc = "`write(|w| ..)` method takes [`chintflag::W`](W) writer structure"]
impl crate::Writable for ChintflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHINTFLAG to value 0"]
impl crate::Resettable for ChintflagSpec {}
