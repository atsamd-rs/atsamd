#[doc = "Register `CHSTATUS` reader"]
pub type R = crate::R<ChstatusSpec>;
#[doc = "Register `CHSTATUS` writer"]
pub type W = crate::W<ChstatusSpec>;
#[doc = "Field `PEND` reader - Channel Pending"]
pub type PendR = crate::BitReader;
#[doc = "Field `PEND` writer - Channel Pending"]
pub type PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Channel Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - Channel Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` reader - Channel Fetch Error"]
pub type FerrR = crate::BitReader;
#[doc = "Field `FERR` writer - Channel Fetch Error"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERR` reader - Channel CRC Error"]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - Channel CRC Error"]
pub type CrcerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Pending"]
    #[inline(always)]
    pub fn pend(&self) -> PendR {
        PendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Fetch Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel CRC Error"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Pending"]
    #[inline(always)]
    pub fn pend(&mut self) -> PendW<ChstatusSpec> {
        PendW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<ChstatusSpec> {
        BusyW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Fetch Error"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<ChstatusSpec> {
        FerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel CRC Error"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CrcerrW<ChstatusSpec> {
        CrcerrW::new(self, 3)
    }
}
#[doc = "Channel n Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatusSpec;
impl crate::RegisterSpec for ChstatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for ChstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`chstatus::W`](W) writer structure"]
impl crate::Writable for ChstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSTATUS to value 0"]
impl crate::Resettable for ChstatusSpec {}
