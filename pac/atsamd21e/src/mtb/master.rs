#[doc = "Register `MASTER` reader"]
pub type R = crate::R<MasterSpec>;
#[doc = "Register `MASTER` writer"]
pub type W = crate::W<MasterSpec>;
#[doc = "Field `MASK` reader - Maximum Value of the Trace Buffer in SRAM"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Maximum Value of the Trace Buffer in SRAM"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TSTARTEN` reader - Trace Start Input Enable"]
pub type TstartenR = crate::BitReader;
#[doc = "Field `TSTARTEN` writer - Trace Start Input Enable"]
pub type TstartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTOPEN` reader - Trace Stop Input Enable"]
pub type TstopenR = crate::BitReader;
#[doc = "Field `TSTOPEN` writer - Trace Stop Input Enable"]
pub type TstopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFRWPRIV` reader - Special Function Register Write Privilege"]
pub type SfrwprivR = crate::BitReader;
#[doc = "Field `SFRWPRIV` writer - Special Function Register Write Privilege"]
pub type SfrwprivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPRIV` reader - SRAM Privilege"]
pub type RamprivR = crate::BitReader;
#[doc = "Field `RAMPRIV` writer - SRAM Privilege"]
pub type RamprivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALTREQ` reader - Halt Request"]
pub type HaltreqR = crate::BitReader;
#[doc = "Field `HALTREQ` writer - Halt Request"]
pub type HaltreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Main Trace Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Main Trace Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Maximum Value of the Trace Buffer in SRAM"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Trace Start Input Enable"]
    #[inline(always)]
    pub fn tstarten(&self) -> TstartenR {
        TstartenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trace Stop Input Enable"]
    #[inline(always)]
    pub fn tstopen(&self) -> TstopenR {
        TstopenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege"]
    #[inline(always)]
    pub fn sfrwpriv(&self) -> SfrwprivR {
        SfrwprivR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM Privilege"]
    #[inline(always)]
    pub fn rampriv(&self) -> RamprivR {
        RamprivR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Halt Request"]
    #[inline(always)]
    pub fn haltreq(&self) -> HaltreqR {
        HaltreqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Main Trace Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Maximum Value of the Trace Buffer in SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<MasterSpec> {
        MaskW::new(self, 0)
    }
    #[doc = "Bit 5 - Trace Start Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstarten(&mut self) -> TstartenW<MasterSpec> {
        TstartenW::new(self, 5)
    }
    #[doc = "Bit 6 - Trace Stop Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstopen(&mut self) -> TstopenW<MasterSpec> {
        TstopenW::new(self, 6)
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege"]
    #[inline(always)]
    #[must_use]
    pub fn sfrwpriv(&mut self) -> SfrwprivW<MasterSpec> {
        SfrwprivW::new(self, 7)
    }
    #[doc = "Bit 8 - SRAM Privilege"]
    #[inline(always)]
    #[must_use]
    pub fn rampriv(&mut self) -> RamprivW<MasterSpec> {
        RamprivW::new(self, 8)
    }
    #[doc = "Bit 9 - Halt Request"]
    #[inline(always)]
    #[must_use]
    pub fn haltreq(&mut self) -> HaltreqW<MasterSpec> {
        HaltreqW::new(self, 9)
    }
    #[doc = "Bit 31 - Main Trace Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<MasterSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "MTB Master\n\nYou can [`read`](crate::Reg::read) this register and get [`master::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasterSpec;
impl crate::RegisterSpec for MasterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`master::R`](R) reader structure"]
impl crate::Readable for MasterSpec {}
#[doc = "`write(|w| ..)` method takes [`master::W`](W) writer structure"]
impl crate::Writable for MasterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASTER to value 0"]
impl crate::Resettable for MasterSpec {
    const RESET_VALUE: u32 = 0;
}
