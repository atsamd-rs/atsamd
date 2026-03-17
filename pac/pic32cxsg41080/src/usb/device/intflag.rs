#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `SUSPEND` reader - Suspend"]
pub type SuspendR = crate::BitReader;
#[doc = "Field `SUSPEND` writer - Suspend"]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - Start Of Frame"]
pub type SofR = crate::BitReader;
#[doc = "Field `SOF` writer - Start Of Frame"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORST` reader - End of Reset"]
pub type EorstR = crate::BitReader;
#[doc = "Field `EORST` writer - End of Reset"]
pub type EorstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - Wake Up"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Wake Up"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSM` reader - End Of Resume"]
pub type EorsmR = crate::BitReader;
#[doc = "Field `EORSM` writer - End Of Resume"]
pub type EorsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSM` reader - Upstream Resume"]
pub type UprsmR = crate::BitReader;
#[doc = "Field `UPRSM` writer - Upstream Resume"]
pub type UprsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMACER` reader - Ram Access"]
pub type RamacerR = crate::BitReader;
#[doc = "Field `RAMACER` writer - Ram Access"]
pub type RamacerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMNYET` reader - Link Power Management Not Yet"]
pub type LpmnyetR = crate::BitReader;
#[doc = "Field `LPMNYET` writer - Link Power Management Not Yet"]
pub type LpmnyetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMSUSP` reader - Link Power Management Suspend"]
pub type LpmsuspR = crate::BitReader;
#[doc = "Field `LPMSUSP` writer - Link Power Management Suspend"]
pub type LpmsuspW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Suspend"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Reset"]
    #[inline(always)]
    pub fn eorst(&self) -> EorstR {
        EorstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake Up"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Of Resume"]
    #[inline(always)]
    pub fn eorsm(&self) -> EorsmR {
        EorsmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume"]
    #[inline(always)]
    pub fn uprsm(&self) -> UprsmR {
        UprsmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ram Access"]
    #[inline(always)]
    pub fn ramacer(&self) -> RamacerR {
        RamacerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Link Power Management Not Yet"]
    #[inline(always)]
    pub fn lpmnyet(&self) -> LpmnyetR {
        LpmnyetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Link Power Management Suspend"]
    #[inline(always)]
    pub fn lpmsusp(&self) -> LpmsuspR {
        LpmsuspR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SuspendW<IntflagSpec> {
        SuspendW::new(self, 0)
    }
    #[doc = "Bit 2 - Start Of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SofW<IntflagSpec> {
        SofW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorst(&mut self) -> EorstW<IntflagSpec> {
        EorstW::new(self, 3)
    }
    #[doc = "Bit 4 - Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<IntflagSpec> {
        WakeupW::new(self, 4)
    }
    #[doc = "Bit 5 - End Of Resume"]
    #[inline(always)]
    #[must_use]
    pub fn eorsm(&mut self) -> EorsmW<IntflagSpec> {
        EorsmW::new(self, 5)
    }
    #[doc = "Bit 6 - Upstream Resume"]
    #[inline(always)]
    #[must_use]
    pub fn uprsm(&mut self) -> UprsmW<IntflagSpec> {
        UprsmW::new(self, 6)
    }
    #[doc = "Bit 7 - Ram Access"]
    #[inline(always)]
    #[must_use]
    pub fn ramacer(&mut self) -> RamacerW<IntflagSpec> {
        RamacerW::new(self, 7)
    }
    #[doc = "Bit 8 - Link Power Management Not Yet"]
    #[inline(always)]
    #[must_use]
    pub fn lpmnyet(&mut self) -> LpmnyetW<IntflagSpec> {
        LpmnyetW::new(self, 8)
    }
    #[doc = "Bit 9 - Link Power Management Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn lpmsusp(&mut self) -> LpmsuspW<IntflagSpec> {
        LpmsuspW::new(self, 9)
    }
}
#[doc = "DEVICE Device Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u16 = 0;
}
