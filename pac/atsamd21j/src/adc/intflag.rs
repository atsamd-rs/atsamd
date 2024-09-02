#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `RESRDY` reader - Result Ready"]
pub type ResrdyR = crate::BitReader;
#[doc = "Field `RESRDY` writer - Result Ready"]
pub type ResrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN` reader - Overrun"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - Overrun"]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINMON` reader - Window Monitor"]
pub type WinmonR = crate::BitReader;
#[doc = "Field `WINMON` writer - Window Monitor"]
pub type WinmonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready"]
pub type SyncrdyR = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready"]
pub type SyncrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Result Ready"]
    #[inline(always)]
    pub fn resrdy(&self) -> ResrdyR {
        ResrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Monitor"]
    #[inline(always)]
    pub fn winmon(&self) -> WinmonR {
        WinmonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Ready"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SyncrdyR {
        SyncrdyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Ready"]
    #[inline(always)]
    #[must_use]
    pub fn resrdy(&mut self) -> ResrdyW<IntflagSpec> {
        ResrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OverrunW<IntflagSpec> {
        OverrunW::new(self, 1)
    }
    #[doc = "Bit 2 - Window Monitor"]
    #[inline(always)]
    #[must_use]
    pub fn winmon(&mut self) -> WinmonW<IntflagSpec> {
        WinmonW::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn syncrdy(&mut self) -> SyncrdyW<IntflagSpec> {
        SyncrdyW::new(self, 3)
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
