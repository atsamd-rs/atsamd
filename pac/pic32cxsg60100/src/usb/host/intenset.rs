#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `HSOF` reader - Host Start Of Frame Interrupt Enable"]
pub type HsofR = crate::BitReader;
#[doc = "Field `HSOF` writer - Host Start Of Frame Interrupt Enable"]
pub type HsofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Bus Reset Interrupt Enable"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Bus Reset Interrupt Enable"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - Wake Up Interrupt Enable"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Wake Up Interrupt Enable"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNRSM` reader - DownStream to the Device Interrupt Enable"]
pub type DnrsmR = crate::BitReader;
#[doc = "Field `DNRSM` writer - DownStream to the Device Interrupt Enable"]
pub type DnrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSM` reader - Upstream Resume fromthe device Interrupt Enable"]
pub type UprsmR = crate::BitReader;
#[doc = "Field `UPRSM` writer - Upstream Resume fromthe device Interrupt Enable"]
pub type UprsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMACER` reader - Ram Access Interrupt Enable"]
pub type RamacerR = crate::BitReader;
#[doc = "Field `RAMACER` writer - Ram Access Interrupt Enable"]
pub type RamacerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCONN` reader - Link Power Management Interrupt Enable"]
pub type DconnR = crate::BitReader;
#[doc = "Field `DCONN` writer - Link Power Management Interrupt Enable"]
pub type DconnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISC` reader - Device Disconnection Interrupt Enable"]
pub type DdiscR = crate::BitReader;
#[doc = "Field `DDISC` writer - Device Disconnection Interrupt Enable"]
pub type DdiscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Host Start Of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn hsof(&self) -> HsofR {
        HsofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Reset Interrupt Enable"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake Up Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DownStream to the Device Interrupt Enable"]
    #[inline(always)]
    pub fn dnrsm(&self) -> DnrsmR {
        DnrsmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume fromthe device Interrupt Enable"]
    #[inline(always)]
    pub fn uprsm(&self) -> UprsmR {
        UprsmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ram Access Interrupt Enable"]
    #[inline(always)]
    pub fn ramacer(&self) -> RamacerR {
        RamacerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Link Power Management Interrupt Enable"]
    #[inline(always)]
    pub fn dconn(&self) -> DconnR {
        DconnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Device Disconnection Interrupt Enable"]
    #[inline(always)]
    pub fn ddisc(&self) -> DdiscR {
        DdiscR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Host Start Of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsof(&mut self) -> HsofW<IntensetSpec> {
        HsofW::new(self, 2)
    }
    #[doc = "Bit 3 - Bus Reset Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<IntensetSpec> {
        RstW::new(self, 3)
    }
    #[doc = "Bit 4 - Wake Up Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<IntensetSpec> {
        WakeupW::new(self, 4)
    }
    #[doc = "Bit 5 - DownStream to the Device Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dnrsm(&mut self) -> DnrsmW<IntensetSpec> {
        DnrsmW::new(self, 5)
    }
    #[doc = "Bit 6 - Upstream Resume fromthe device Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uprsm(&mut self) -> UprsmW<IntensetSpec> {
        UprsmW::new(self, 6)
    }
    #[doc = "Bit 7 - Ram Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramacer(&mut self) -> RamacerW<IntensetSpec> {
        RamacerW::new(self, 7)
    }
    #[doc = "Bit 8 - Link Power Management Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dconn(&mut self) -> DconnW<IntensetSpec> {
        DconnW::new(self, 8)
    }
    #[doc = "Bit 9 - Device Disconnection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddisc(&mut self) -> DdiscW<IntensetSpec> {
        DdiscW::new(self, 9)
    }
}
#[doc = "HOST Host Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u16 = 0;
}
