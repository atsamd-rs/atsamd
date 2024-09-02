#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `UNDERRUN` reader - Underrun"]
pub type UnderrunR = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - Underrun"]
pub type UnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - Data Buffer Empty"]
pub type EmptyR = crate::BitReader;
#[doc = "Field `EMPTY` writer - Data Buffer Empty"]
pub type EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready"]
pub type SyncrdyR = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready"]
pub type SyncrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Underrun"]
    #[inline(always)]
    pub fn underrun(&self) -> UnderrunR {
        UnderrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization Ready"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SyncrdyR {
        SyncrdyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn underrun(&mut self) -> UnderrunW<IntflagSpec> {
        UnderrunW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<IntflagSpec> {
        EmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn syncrdy(&mut self) -> SyncrdyW<IntflagSpec> {
        SyncrdyW::new(self, 2)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u8 = 0;
}
