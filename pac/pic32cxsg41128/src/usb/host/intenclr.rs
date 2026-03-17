#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `HSOF` reader - Host Start Of Frame Interrupt Disable"]
pub type HsofR = crate::BitReader;
#[doc = "Field `HSOF` writer - Host Start Of Frame Interrupt Disable"]
pub type HsofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - BUS Reset Interrupt Disable"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - BUS Reset Interrupt Disable"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - Wake Up Interrupt Disable"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Wake Up Interrupt Disable"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNRSM` reader - DownStream to Device Interrupt Disable"]
pub type DnrsmR = crate::BitReader;
#[doc = "Field `DNRSM` writer - DownStream to Device Interrupt Disable"]
pub type DnrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSM` reader - Upstream Resume from Device Interrupt Disable"]
pub type UprsmR = crate::BitReader;
#[doc = "Field `UPRSM` writer - Upstream Resume from Device Interrupt Disable"]
pub type UprsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMACER` reader - Ram Access Interrupt Disable"]
pub type RamacerR = crate::BitReader;
#[doc = "Field `RAMACER` writer - Ram Access Interrupt Disable"]
pub type RamacerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCONN` reader - Device Connection Interrupt Disable"]
pub type DconnR = crate::BitReader;
#[doc = "Field `DCONN` writer - Device Connection Interrupt Disable"]
pub type DconnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISC` reader - Device Disconnection Interrupt Disable"]
pub type DdiscR = crate::BitReader;
#[doc = "Field `DDISC` writer - Device Disconnection Interrupt Disable"]
pub type DdiscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Host Start Of Frame Interrupt Disable"]
    #[inline(always)]
    pub fn hsof(&self) -> HsofR {
        HsofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUS Reset Interrupt Disable"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake Up Interrupt Disable"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DownStream to Device Interrupt Disable"]
    #[inline(always)]
    pub fn dnrsm(&self) -> DnrsmR {
        DnrsmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume from Device Interrupt Disable"]
    #[inline(always)]
    pub fn uprsm(&self) -> UprsmR {
        UprsmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ram Access Interrupt Disable"]
    #[inline(always)]
    pub fn ramacer(&self) -> RamacerR {
        RamacerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Device Connection Interrupt Disable"]
    #[inline(always)]
    pub fn dconn(&self) -> DconnR {
        DconnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Device Disconnection Interrupt Disable"]
    #[inline(always)]
    pub fn ddisc(&self) -> DdiscR {
        DdiscR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Host Start Of Frame Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hsof(&mut self) -> HsofW<IntenclrSpec> {
        HsofW::new(self, 2)
    }
    #[doc = "Bit 3 - BUS Reset Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<IntenclrSpec> {
        RstW::new(self, 3)
    }
    #[doc = "Bit 4 - Wake Up Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<IntenclrSpec> {
        WakeupW::new(self, 4)
    }
    #[doc = "Bit 5 - DownStream to Device Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dnrsm(&mut self) -> DnrsmW<IntenclrSpec> {
        DnrsmW::new(self, 5)
    }
    #[doc = "Bit 6 - Upstream Resume from Device Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn uprsm(&mut self) -> UprsmW<IntenclrSpec> {
        UprsmW::new(self, 6)
    }
    #[doc = "Bit 7 - Ram Access Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ramacer(&mut self) -> RamacerW<IntenclrSpec> {
        RamacerW::new(self, 7)
    }
    #[doc = "Bit 8 - Device Connection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dconn(&mut self) -> DconnW<IntenclrSpec> {
        DconnW::new(self, 8)
    }
    #[doc = "Bit 9 - Device Disconnection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ddisc(&mut self) -> DdiscW<IntenclrSpec> {
        DdiscW::new(self, 9)
    }
}
#[doc = "HOST Host Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u16 = 0;
}
