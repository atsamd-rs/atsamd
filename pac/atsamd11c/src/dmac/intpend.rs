#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Register `INTPEND` writer"]
pub type W = crate::W<IntpendSpec>;
#[doc = "Field `ID` reader - Channel ID"]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - Channel ID"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TERR` reader - Transfer Error"]
pub type TerrR = crate::BitReader;
#[doc = "Field `TERR` writer - Transfer Error"]
pub type TerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMPL` reader - Transfer Complete"]
pub type TcmplR = crate::BitReader;
#[doc = "Field `TCMPL` writer - Transfer Complete"]
pub type TcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - Channel Suspend"]
pub type SuspR = crate::BitReader;
#[doc = "Field `SUSP` writer - Channel Suspend"]
pub type SuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` reader - Fetch Error"]
pub type FerrR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `PEND` reader - Pending"]
pub type PendR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Transfer Error"]
    #[inline(always)]
    pub fn terr(&self) -> TerrR {
        TerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TcmplR {
        TcmplR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fetch Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending"]
    #[inline(always)]
    pub fn pend(&self) -> PendR {
        PendR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel ID"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<IntpendSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bit 8 - Transfer Error"]
    #[inline(always)]
    pub fn terr(&mut self) -> TerrW<IntpendSpec> {
        TerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&mut self) -> TcmplW<IntpendSpec> {
        TcmplW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<IntpendSpec> {
        SuspW::new(self, 10)
    }
}
#[doc = "Interrupt Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for IntpendSpec {}
