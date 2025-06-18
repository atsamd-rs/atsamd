#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `HSOF` reader - Host Start Of Frame"]
pub type HsofR = crate::BitReader;
#[doc = "Field `HSOF` writer - Host Start Of Frame"]
pub type HsofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Bus Reset"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Bus Reset"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - Wake Up"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Wake Up"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNRSM` reader - Downstream"]
pub type DnrsmR = crate::BitReader;
#[doc = "Field `DNRSM` writer - Downstream"]
pub type DnrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSM` reader - Upstream Resume from the Device"]
pub type UprsmR = crate::BitReader;
#[doc = "Field `UPRSM` writer - Upstream Resume from the Device"]
pub type UprsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMACER` reader - Ram Access"]
pub type RamacerR = crate::BitReader;
#[doc = "Field `RAMACER` writer - Ram Access"]
pub type RamacerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCONN` reader - Device Connection"]
pub type DconnR = crate::BitReader;
#[doc = "Field `DCONN` writer - Device Connection"]
pub type DconnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISC` reader - Device Disconnection"]
pub type DdiscR = crate::BitReader;
#[doc = "Field `DDISC` writer - Device Disconnection"]
pub type DdiscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Host Start Of Frame"]
    #[inline(always)]
    pub fn hsof(&self) -> HsofR {
        HsofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake Up"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Downstream"]
    #[inline(always)]
    pub fn dnrsm(&self) -> DnrsmR {
        DnrsmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume from the Device"]
    #[inline(always)]
    pub fn uprsm(&self) -> UprsmR {
        UprsmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ram Access"]
    #[inline(always)]
    pub fn ramacer(&self) -> RamacerR {
        RamacerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Device Connection"]
    #[inline(always)]
    pub fn dconn(&self) -> DconnR {
        DconnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Device Disconnection"]
    #[inline(always)]
    pub fn ddisc(&self) -> DdiscR {
        DdiscR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Host Start Of Frame"]
    #[inline(always)]
    pub fn hsof(&mut self) -> HsofW<IntflagSpec> {
        HsofW::new(self, 2)
    }
    #[doc = "Bit 3 - Bus Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<IntflagSpec> {
        RstW::new(self, 3)
    }
    #[doc = "Bit 4 - Wake Up"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<IntflagSpec> {
        WakeupW::new(self, 4)
    }
    #[doc = "Bit 5 - Downstream"]
    #[inline(always)]
    pub fn dnrsm(&mut self) -> DnrsmW<IntflagSpec> {
        DnrsmW::new(self, 5)
    }
    #[doc = "Bit 6 - Upstream Resume from the Device"]
    #[inline(always)]
    pub fn uprsm(&mut self) -> UprsmW<IntflagSpec> {
        UprsmW::new(self, 6)
    }
    #[doc = "Bit 7 - Ram Access"]
    #[inline(always)]
    pub fn ramacer(&mut self) -> RamacerW<IntflagSpec> {
        RamacerW::new(self, 7)
    }
    #[doc = "Bit 8 - Device Connection"]
    #[inline(always)]
    pub fn dconn(&mut self) -> DconnW<IntflagSpec> {
        DconnW::new(self, 8)
    }
    #[doc = "Bit 9 - Device Disconnection"]
    #[inline(always)]
    pub fn ddisc(&mut self) -> DdiscW<IntflagSpec> {
        DdiscW::new(self, 9)
    }
}
#[doc = "HOST Host Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
