#[doc = "Register `CRCSTATUS` reader"]
pub type R = crate::R<CrcstatusSpec>;
#[doc = "Register `CRCSTATUS` writer"]
pub type W = crate::W<CrcstatusSpec>;
#[doc = "Field `CRCBUSY` reader - CRC Module Busy"]
pub type CrcbusyR = crate::BitReader;
#[doc = "Field `CRCBUSY` writer - CRC Module Busy"]
pub type CrcbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCZERO` reader - CRC Zero"]
pub type CrczeroR = crate::BitReader;
#[doc = "Field `CRCZERO` writer - CRC Zero"]
pub type CrczeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERR` reader - CRC Error"]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - CRC Error"]
pub type CrcerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CRC Module Busy"]
    #[inline(always)]
    pub fn crcbusy(&self) -> CrcbusyR {
        CrcbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Zero"]
    #[inline(always)]
    pub fn crczero(&self) -> CrczeroR {
        CrczeroR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC Error"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Module Busy"]
    #[inline(always)]
    pub fn crcbusy(&mut self) -> CrcbusyW<CrcstatusSpec> {
        CrcbusyW::new(self, 0)
    }
    #[doc = "Bit 1 - CRC Zero"]
    #[inline(always)]
    pub fn crczero(&mut self) -> CrczeroW<CrcstatusSpec> {
        CrczeroW::new(self, 1)
    }
    #[doc = "Bit 2 - CRC Error"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CrcerrW<CrcstatusSpec> {
        CrcerrW::new(self, 2)
    }
}
#[doc = "CRC Status\n\nYou can [`read`](crate::Reg::read) this register and get [`crcstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcstatusSpec;
impl crate::RegisterSpec for CrcstatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crcstatus::R`](R) reader structure"]
impl crate::Readable for CrcstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`crcstatus::W`](W) writer structure"]
impl crate::Writable for CrcstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCSTATUS to value 0"]
impl crate::Resettable for CrcstatusSpec {}
