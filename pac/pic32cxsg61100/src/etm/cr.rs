#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ETMPD` reader - ETM Power Down"]
pub type EtmpdR = crate::BitReader;
#[doc = "Field `ETMPD` writer - ETM Power Down"]
pub type EtmpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTSIZE` reader - Port Size bits 2:0"]
pub type PortsizeR = crate::FieldReader;
#[doc = "Field `PORTSIZE` writer - Port Size bits 2:0"]
pub type PortsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STALL` reader - Stall Processor"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Stall Processor"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROUT` reader - Branch Output"]
pub type BroutR = crate::BitReader;
#[doc = "Field `BROUT` writer - Branch Output"]
pub type BroutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRQ` reader - Debug Request Control"]
pub type DbgrqR = crate::BitReader;
#[doc = "Field `DBGRQ` writer - Debug Request Control"]
pub type DbgrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG` reader - ETM Programming"]
pub type ProgR = crate::BitReader;
#[doc = "Field `PROG` writer - ETM Programming"]
pub type ProgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTSEL` reader - ETM Port Select"]
pub type PortselR = crate::BitReader;
#[doc = "Field `PORTSEL` writer - ETM Port Select"]
pub type PortselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTMODE2` reader - Port Mode bit 2"]
pub type Portmode2R = crate::BitReader;
#[doc = "Field `PORTMODE2` writer - Port Mode bit 2"]
pub type Portmode2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTMODE` reader - Port Mode bits 1:0"]
pub type PortmodeR = crate::FieldReader;
#[doc = "Field `PORTMODE` writer - Port Mode bits 1:0"]
pub type PortmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PORTSIZE3` reader - Port Size bit 3"]
pub type Portsize3R = crate::BitReader;
#[doc = "Field `PORTSIZE3` writer - Port Size bit 3"]
pub type Portsize3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - TimeStamp Enable"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - TimeStamp Enable"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ETM Power Down"]
    #[inline(always)]
    pub fn etmpd(&self) -> EtmpdR {
        EtmpdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Port Size bits 2:0"]
    #[inline(always)]
    pub fn portsize(&self) -> PortsizeR {
        PortsizeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn brout(&self) -> BroutR {
        BroutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgrq(&self) -> DbgrqR {
        DbgrqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn prog(&self) -> ProgR {
        ProgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ETM Port Select"]
    #[inline(always)]
    pub fn portsel(&self) -> PortselR {
        PortselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Mode bit 2"]
    #[inline(always)]
    pub fn portmode2(&self) -> Portmode2R {
        Portmode2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Port Mode bits 1:0"]
    #[inline(always)]
    pub fn portmode(&self) -> PortmodeR {
        PortmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 21 - Port Size bit 3"]
    #[inline(always)]
    pub fn portsize3(&self) -> Portsize3R {
        Portsize3R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - TimeStamp Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETM Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn etmpd(&mut self) -> EtmpdW<CrSpec> {
        EtmpdW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Port Size bits 2:0"]
    #[inline(always)]
    #[must_use]
    pub fn portsize(&mut self) -> PortsizeW<CrSpec> {
        PortsizeW::new(self, 4)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<CrSpec> {
        StallW::new(self, 7)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    #[must_use]
    pub fn brout(&mut self) -> BroutW<CrSpec> {
        BroutW::new(self, 8)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrq(&mut self) -> DbgrqW<CrSpec> {
        DbgrqW::new(self, 9)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    #[must_use]
    pub fn prog(&mut self) -> ProgW<CrSpec> {
        ProgW::new(self, 10)
    }
    #[doc = "Bit 11 - ETM Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn portsel(&mut self) -> PortselW<CrSpec> {
        PortselW::new(self, 11)
    }
    #[doc = "Bit 13 - Port Mode bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn portmode2(&mut self) -> Portmode2W<CrSpec> {
        Portmode2W::new(self, 13)
    }
    #[doc = "Bits 16:17 - Port Mode bits 1:0"]
    #[inline(always)]
    #[must_use]
    pub fn portmode(&mut self) -> PortmodeW<CrSpec> {
        PortmodeW::new(self, 16)
    }
    #[doc = "Bit 21 - Port Size bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn portsize3(&mut self) -> Portsize3W<CrSpec> {
        Portsize3W::new(self, 21)
    }
    #[doc = "Bit 28 - TimeStamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TsenW<CrSpec> {
        TsenW::new(self, 28)
    }
}
#[doc = "ETM Main Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x0411"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x0411;
}
