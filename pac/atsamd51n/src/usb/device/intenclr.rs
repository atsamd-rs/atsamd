#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `SUSPEND` reader - Suspend Interrupt Enable"]
pub type SuspendR = crate::BitReader;
#[doc = "Field `SUSPEND` writer - Suspend Interrupt Enable"]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - Start Of Frame Interrupt Enable"]
pub type SofR = crate::BitReader;
#[doc = "Field `SOF` writer - Start Of Frame Interrupt Enable"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORST` reader - End of Reset Interrupt Enable"]
pub type EorstR = crate::BitReader;
#[doc = "Field `EORST` writer - End of Reset Interrupt Enable"]
pub type EorstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - Wake Up Interrupt Enable"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Wake Up Interrupt Enable"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSM` reader - End Of Resume Interrupt Enable"]
pub type EorsmR = crate::BitReader;
#[doc = "Field `EORSM` writer - End Of Resume Interrupt Enable"]
pub type EorsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSM` reader - Upstream Resume Interrupt Enable"]
pub type UprsmR = crate::BitReader;
#[doc = "Field `UPRSM` writer - Upstream Resume Interrupt Enable"]
pub type UprsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMACER` reader - Ram Access Interrupt Enable"]
pub type RamacerR = crate::BitReader;
#[doc = "Field `RAMACER` writer - Ram Access Interrupt Enable"]
pub type RamacerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMNYET` reader - Link Power Management Not Yet Interrupt Enable"]
pub type LpmnyetR = crate::BitReader;
#[doc = "Field `LPMNYET` writer - Link Power Management Not Yet Interrupt Enable"]
pub type LpmnyetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMSUSP` reader - Link Power Management Suspend Interrupt Enable"]
pub type LpmsuspR = crate::BitReader;
#[doc = "Field `LPMSUSP` writer - Link Power Management Suspend Interrupt Enable"]
pub type LpmsuspW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn eorst(&self) -> EorstR {
        EorstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake Up Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn eorsm(&self) -> EorsmR {
        EorsmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn uprsm(&self) -> UprsmR {
        UprsmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ram Access Interrupt Enable"]
    #[inline(always)]
    pub fn ramacer(&self) -> RamacerR {
        RamacerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Link Power Management Not Yet Interrupt Enable"]
    #[inline(always)]
    pub fn lpmnyet(&self) -> LpmnyetR {
        LpmnyetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Link Power Management Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn lpmsusp(&self) -> LpmsuspR {
        LpmsuspR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SuspendW<IntenclrSpec> {
        SuspendW::new(self, 0)
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn sof(&mut self) -> SofW<IntenclrSpec> {
        SofW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn eorst(&mut self) -> EorstW<IntenclrSpec> {
        EorstW::new(self, 3)
    }
    #[doc = "Bit 4 - Wake Up Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<IntenclrSpec> {
        WakeupW::new(self, 4)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn eorsm(&mut self) -> EorsmW<IntenclrSpec> {
        EorsmW::new(self, 5)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn uprsm(&mut self) -> UprsmW<IntenclrSpec> {
        UprsmW::new(self, 6)
    }
    #[doc = "Bit 7 - Ram Access Interrupt Enable"]
    #[inline(always)]
    pub fn ramacer(&mut self) -> RamacerW<IntenclrSpec> {
        RamacerW::new(self, 7)
    }
    #[doc = "Bit 8 - Link Power Management Not Yet Interrupt Enable"]
    #[inline(always)]
    pub fn lpmnyet(&mut self) -> LpmnyetW<IntenclrSpec> {
        LpmnyetW::new(self, 8)
    }
    #[doc = "Bit 9 - Link Power Management Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn lpmsusp(&mut self) -> LpmsuspW<IntenclrSpec> {
        LpmsuspW::new(self, 9)
    }
}
#[doc = "DEVICE Device Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
