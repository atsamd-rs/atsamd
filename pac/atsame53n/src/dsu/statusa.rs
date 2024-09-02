#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<StatusaSpec>;
#[doc = "Register `STATUSA` writer"]
pub type W = crate::W<StatusaSpec>;
#[doc = "Field `DONE` reader - Done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Done"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSTEXT` reader - CPU Reset Phase Extension"]
pub type CrstextR = crate::BitReader;
#[doc = "Field `CRSTEXT` writer - CPU Reset Phase Extension"]
pub type CrstextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - Bus Error"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - Bus Error"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAIL` reader - Failure"]
pub type FailR = crate::BitReader;
#[doc = "Field `FAIL` writer - Failure"]
pub type FailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Protection Error"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Protection Error"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Reset Phase Extension"]
    #[inline(always)]
    pub fn crstext(&self) -> CrstextR {
        CrstextR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    pub fn fail(&self) -> FailR {
        FailR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection Error"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<StatusaSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - CPU Reset Phase Extension"]
    #[inline(always)]
    #[must_use]
    pub fn crstext(&mut self) -> CrstextW<StatusaSpec> {
        CrstextW::new(self, 1)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BerrW<StatusaSpec> {
        BerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    #[must_use]
    pub fn fail(&mut self) -> FailW<StatusaSpec> {
        FailW::new(self, 3)
    }
    #[doc = "Bit 4 - Protection Error"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PerrW<StatusaSpec> {
        PerrW::new(self, 4)
    }
}
#[doc = "Status A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusaSpec;
impl crate::RegisterSpec for StatusaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for StatusaSpec {}
#[doc = "`write(|w| ..)` method takes [`statusa::W`](W) writer structure"]
impl crate::Writable for StatusaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for StatusaSpec {
    const RESET_VALUE: u8 = 0;
}
