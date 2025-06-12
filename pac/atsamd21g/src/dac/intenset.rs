#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `UNDERRUN` reader - Underrun Interrupt Enable"]
pub type UnderrunR = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - Underrun Interrupt Enable"]
pub type UnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - Data Buffer Empty Interrupt Enable"]
pub type EmptyR = crate::BitReader;
#[doc = "Field `EMPTY` writer - Data Buffer Empty Interrupt Enable"]
pub type EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready Interrupt Enable"]
pub type SyncrdyR = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready Interrupt Enable"]
pub type SyncrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn underrun(&self) -> UnderrunR {
        UnderrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SyncrdyR {
        SyncrdyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UnderrunW<IntensetSpec> {
        UnderrunW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty(&mut self) -> EmptyW<IntensetSpec> {
        EmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&mut self) -> SyncrdyW<IntensetSpec> {
        SyncrdyW::new(self, 2)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
