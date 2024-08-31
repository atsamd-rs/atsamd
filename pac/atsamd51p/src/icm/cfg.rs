#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `WBDIS` reader - Write Back Disable"]
pub type WbdisR = crate::BitReader;
#[doc = "Field `WBDIS` writer - Write Back Disable"]
pub type WbdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOMDIS` reader - End of Monitoring Disable"]
pub type EomdisR = crate::BitReader;
#[doc = "Field `EOMDIS` writer - End of Monitoring Disable"]
pub type EomdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLBDIS` reader - Secondary List Branching Disable"]
pub type SlbdisR = crate::BitReader;
#[doc = "Field `SLBDIS` writer - Secondary List Branching Disable"]
pub type SlbdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBC` reader - Bus Burden Control"]
pub type BbcR = crate::FieldReader;
#[doc = "Field `BBC` writer - Bus Burden Control"]
pub type BbcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASCD` reader - Automatic Switch To Compare Digest"]
pub type AscdR = crate::BitReader;
#[doc = "Field `ASCD` writer - Automatic Switch To Compare Digest"]
pub type AscdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALBUFF` reader - Dual Input Buffer"]
pub type DualbuffR = crate::BitReader;
#[doc = "Field `DUALBUFF` writer - Dual Input Buffer"]
pub type DualbuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIHASH` reader - User Initial Hash Value"]
pub type UihashR = crate::BitReader;
#[doc = "Field `UIHASH` writer - User Initial Hash Value"]
pub type UihashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "User SHA Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ualgoselect {
    #[doc = "0: SHA1 Algorithm"]
    Sha1 = 0,
    #[doc = "1: SHA256 Algorithm"]
    Sha256 = 1,
    #[doc = "4: SHA224 Algorithm"]
    Sha224 = 4,
}
impl From<Ualgoselect> for u8 {
    #[inline(always)]
    fn from(variant: Ualgoselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ualgoselect {
    type Ux = u8;
}
impl crate::IsEnum for Ualgoselect {}
#[doc = "Field `UALGO` reader - User SHA Algorithm"]
pub type UalgoR = crate::FieldReader<Ualgoselect>;
impl UalgoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ualgoselect> {
        match self.bits {
            0 => Some(Ualgoselect::Sha1),
            1 => Some(Ualgoselect::Sha256),
            4 => Some(Ualgoselect::Sha224),
            _ => None,
        }
    }
    #[doc = "SHA1 Algorithm"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == Ualgoselect::Sha1
    }
    #[doc = "SHA256 Algorithm"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == Ualgoselect::Sha256
    }
    #[doc = "SHA224 Algorithm"]
    #[inline(always)]
    pub fn is_sha224(&self) -> bool {
        *self == Ualgoselect::Sha224
    }
}
#[doc = "Field `UALGO` writer - User SHA Algorithm"]
pub type UalgoW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ualgoselect>;
impl<'a, REG> UalgoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SHA1 Algorithm"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut crate::W<REG> {
        self.variant(Ualgoselect::Sha1)
    }
    #[doc = "SHA256 Algorithm"]
    #[inline(always)]
    pub fn sha256(self) -> &'a mut crate::W<REG> {
        self.variant(Ualgoselect::Sha256)
    }
    #[doc = "SHA224 Algorithm"]
    #[inline(always)]
    pub fn sha224(self) -> &'a mut crate::W<REG> {
        self.variant(Ualgoselect::Sha224)
    }
}
#[doc = "Field `HAPROT` reader - Region Hash Area Protection"]
pub type HaprotR = crate::FieldReader;
#[doc = "Field `HAPROT` writer - Region Hash Area Protection"]
pub type HaprotW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DAPROT` reader - Region Descriptor Area Protection"]
pub type DaprotR = crate::FieldReader;
#[doc = "Field `DAPROT` writer - Region Descriptor Area Protection"]
pub type DaprotW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline(always)]
    pub fn wbdis(&self) -> WbdisR {
        WbdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline(always)]
    pub fn eomdis(&self) -> EomdisR {
        EomdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline(always)]
    pub fn slbdis(&self) -> SlbdisR {
        SlbdisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline(always)]
    pub fn bbc(&self) -> BbcR {
        BbcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline(always)]
    pub fn ascd(&self) -> AscdR {
        AscdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DualbuffR {
        DualbuffR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline(always)]
    pub fn uihash(&self) -> UihashR {
        UihashR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline(always)]
    pub fn ualgo(&self) -> UalgoR {
        UalgoR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - Region Hash Area Protection"]
    #[inline(always)]
    pub fn haprot(&self) -> HaprotR {
        HaprotR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Region Descriptor Area Protection"]
    #[inline(always)]
    pub fn daprot(&self) -> DaprotR {
        DaprotR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wbdis(&mut self) -> WbdisW<CfgSpec> {
        WbdisW::new(self, 0)
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eomdis(&mut self) -> EomdisW<CfgSpec> {
        EomdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline(always)]
    #[must_use]
    pub fn slbdis(&mut self) -> SlbdisW<CfgSpec> {
        SlbdisW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline(always)]
    #[must_use]
    pub fn bbc(&mut self) -> BbcW<CfgSpec> {
        BbcW::new(self, 4)
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline(always)]
    #[must_use]
    pub fn ascd(&mut self) -> AscdW<CfgSpec> {
        AscdW::new(self, 8)
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn dualbuff(&mut self) -> DualbuffW<CfgSpec> {
        DualbuffW::new(self, 9)
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline(always)]
    #[must_use]
    pub fn uihash(&mut self) -> UihashW<CfgSpec> {
        UihashW::new(self, 12)
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline(always)]
    #[must_use]
    pub fn ualgo(&mut self) -> UalgoW<CfgSpec> {
        UalgoW::new(self, 13)
    }
    #[doc = "Bits 16:21 - Region Hash Area Protection"]
    #[inline(always)]
    #[must_use]
    pub fn haprot(&mut self) -> HaprotW<CfgSpec> {
        HaprotW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Region Descriptor Area Protection"]
    #[inline(always)]
    #[must_use]
    pub fn daprot(&mut self) -> DaprotW<CfgSpec> {
        DaprotW::new(self, 24)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
