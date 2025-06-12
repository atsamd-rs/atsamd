#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `DONE` reader - Command Done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Command Done"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRE` reader - Address Error"]
pub type AddreR = crate::BitReader;
#[doc = "Field `ADDRE` writer - Address Error"]
pub type AddreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGE` reader - Programming Error"]
pub type ProgeR = crate::BitReader;
#[doc = "Field `PROGE` writer - Programming Error"]
pub type ProgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKE` reader - Lock Error"]
pub type LockeR = crate::BitReader;
#[doc = "Field `LOCKE` writer - Lock Error"]
pub type LockeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCSE` reader - ECC Single Error"]
pub type EccseR = crate::BitReader;
#[doc = "Field `ECCSE` writer - ECC Single Error"]
pub type EccseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCDE` reader - ECC Dual Error"]
pub type EccdeR = crate::BitReader;
#[doc = "Field `ECCDE` writer - ECC Dual Error"]
pub type EccdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVME` reader - NVM Error"]
pub type NvmeR = crate::BitReader;
#[doc = "Field `NVME` writer - NVM Error"]
pub type NvmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - Suspended Write Or Erase Operation"]
pub type SuspR = crate::BitReader;
#[doc = "Field `SUSP` writer - Suspended Write Or Erase Operation"]
pub type SuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEESFULL` reader - Active SEES Full"]
pub type SeesfullR = crate::BitReader;
#[doc = "Field `SEESFULL` writer - Active SEES Full"]
pub type SeesfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEESOVF` reader - Active SEES Overflow"]
pub type SeesovfR = crate::BitReader;
#[doc = "Field `SEESOVF` writer - Active SEES Overflow"]
pub type SeesovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEEWRC` reader - SEE Write Completed"]
pub type SeewrcR = crate::BitReader;
#[doc = "Field `SEEWRC` writer - SEE Write Completed"]
pub type SeewrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Error"]
    #[inline(always)]
    pub fn addre(&self) -> AddreR {
        AddreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Programming Error"]
    #[inline(always)]
    pub fn proge(&self) -> ProgeR {
        ProgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock Error"]
    #[inline(always)]
    pub fn locke(&self) -> LockeR {
        LockeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ECC Single Error"]
    #[inline(always)]
    pub fn eccse(&self) -> EccseR {
        EccseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ECC Dual Error"]
    #[inline(always)]
    pub fn eccde(&self) -> EccdeR {
        EccdeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NVM Error"]
    #[inline(always)]
    pub fn nvme(&self) -> NvmeR {
        NvmeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspended Write Or Erase Operation"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Active SEES Full"]
    #[inline(always)]
    pub fn seesfull(&self) -> SeesfullR {
        SeesfullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Active SEES Overflow"]
    #[inline(always)]
    pub fn seesovf(&self) -> SeesovfR {
        SeesovfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SEE Write Completed"]
    #[inline(always)]
    pub fn seewrc(&self) -> SeewrcR {
        SeewrcR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Done"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<IntflagSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Address Error"]
    #[inline(always)]
    pub fn addre(&mut self) -> AddreW<IntflagSpec> {
        AddreW::new(self, 1)
    }
    #[doc = "Bit 2 - Programming Error"]
    #[inline(always)]
    pub fn proge(&mut self) -> ProgeW<IntflagSpec> {
        ProgeW::new(self, 2)
    }
    #[doc = "Bit 3 - Lock Error"]
    #[inline(always)]
    pub fn locke(&mut self) -> LockeW<IntflagSpec> {
        LockeW::new(self, 3)
    }
    #[doc = "Bit 4 - ECC Single Error"]
    #[inline(always)]
    pub fn eccse(&mut self) -> EccseW<IntflagSpec> {
        EccseW::new(self, 4)
    }
    #[doc = "Bit 5 - ECC Dual Error"]
    #[inline(always)]
    pub fn eccde(&mut self) -> EccdeW<IntflagSpec> {
        EccdeW::new(self, 5)
    }
    #[doc = "Bit 6 - NVM Error"]
    #[inline(always)]
    pub fn nvme(&mut self) -> NvmeW<IntflagSpec> {
        NvmeW::new(self, 6)
    }
    #[doc = "Bit 7 - Suspended Write Or Erase Operation"]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<IntflagSpec> {
        SuspW::new(self, 7)
    }
    #[doc = "Bit 8 - Active SEES Full"]
    #[inline(always)]
    pub fn seesfull(&mut self) -> SeesfullW<IntflagSpec> {
        SeesfullW::new(self, 8)
    }
    #[doc = "Bit 9 - Active SEES Overflow"]
    #[inline(always)]
    pub fn seesovf(&mut self) -> SeesovfW<IntflagSpec> {
        SeesovfW::new(self, 9)
    }
    #[doc = "Bit 10 - SEE Write Completed"]
    #[inline(always)]
    pub fn seewrc(&mut self) -> SeewrcW<IntflagSpec> {
        SeewrcW::new(self, 10)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
