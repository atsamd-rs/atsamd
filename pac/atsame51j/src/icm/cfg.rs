#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `WBDIS` reader - Write Back Disable"]
pub type WBDIS_R = crate::BitReader;
#[doc = "Field `WBDIS` writer - Write Back Disable"]
pub type WBDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOMDIS` reader - End of Monitoring Disable"]
pub type EOMDIS_R = crate::BitReader;
#[doc = "Field `EOMDIS` writer - End of Monitoring Disable"]
pub type EOMDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLBDIS` reader - Secondary List Branching Disable"]
pub type SLBDIS_R = crate::BitReader;
#[doc = "Field `SLBDIS` writer - Secondary List Branching Disable"]
pub type SLBDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BBC` reader - Bus Burden Control"]
pub type BBC_R = crate::FieldReader;
#[doc = "Field `BBC` writer - Bus Burden Control"]
pub type BBC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ASCD` reader - Automatic Switch To Compare Digest"]
pub type ASCD_R = crate::BitReader;
#[doc = "Field `ASCD` writer - Automatic Switch To Compare Digest"]
pub type ASCD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUALBUFF` reader - Dual Input Buffer"]
pub type DUALBUFF_R = crate::BitReader;
#[doc = "Field `DUALBUFF` writer - Dual Input Buffer"]
pub type DUALBUFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UIHASH` reader - User Initial Hash Value"]
pub type UIHASH_R = crate::BitReader;
#[doc = "Field `UIHASH` writer - User Initial Hash Value"]
pub type UIHASH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UALGO` reader - User SHA Algorithm"]
pub type UALGO_R = crate::FieldReader<UALGOSELECT_A>;
#[doc = "User SHA Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UALGOSELECT_A {
    #[doc = "0: SHA1 Algorithm"]
    SHA1 = 0,
    #[doc = "1: SHA256 Algorithm"]
    SHA256 = 1,
    #[doc = "4: SHA224 Algorithm"]
    SHA224 = 4,
}
impl From<UALGOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: UALGOSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UALGOSELECT_A {
    type Ux = u8;
}
impl UALGO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UALGOSELECT_A> {
        match self.bits {
            0 => Some(UALGOSELECT_A::SHA1),
            1 => Some(UALGOSELECT_A::SHA256),
            4 => Some(UALGOSELECT_A::SHA224),
            _ => None,
        }
    }
    #[doc = "SHA1 Algorithm"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == UALGOSELECT_A::SHA1
    }
    #[doc = "SHA256 Algorithm"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == UALGOSELECT_A::SHA256
    }
    #[doc = "SHA224 Algorithm"]
    #[inline(always)]
    pub fn is_sha224(&self) -> bool {
        *self == UALGOSELECT_A::SHA224
    }
}
#[doc = "Field `UALGO` writer - User SHA Algorithm"]
pub type UALGO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, UALGOSELECT_A>;
impl<'a, REG, const O: u8> UALGO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SHA1 Algorithm"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut crate::W<REG> {
        self.variant(UALGOSELECT_A::SHA1)
    }
    #[doc = "SHA256 Algorithm"]
    #[inline(always)]
    pub fn sha256(self) -> &'a mut crate::W<REG> {
        self.variant(UALGOSELECT_A::SHA256)
    }
    #[doc = "SHA224 Algorithm"]
    #[inline(always)]
    pub fn sha224(self) -> &'a mut crate::W<REG> {
        self.variant(UALGOSELECT_A::SHA224)
    }
}
#[doc = "Field `HAPROT` reader - Region Hash Area Protection"]
pub type HAPROT_R = crate::FieldReader;
#[doc = "Field `HAPROT` writer - Region Hash Area Protection"]
pub type HAPROT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `DAPROT` reader - Region Descriptor Area Protection"]
pub type DAPROT_R = crate::FieldReader;
#[doc = "Field `DAPROT` writer - Region Descriptor Area Protection"]
pub type DAPROT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline(always)]
    pub fn wbdis(&self) -> WBDIS_R {
        WBDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline(always)]
    pub fn eomdis(&self) -> EOMDIS_R {
        EOMDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline(always)]
    pub fn slbdis(&self) -> SLBDIS_R {
        SLBDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline(always)]
    pub fn bbc(&self) -> BBC_R {
        BBC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline(always)]
    pub fn ascd(&self) -> ASCD_R {
        ASCD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DUALBUFF_R {
        DUALBUFF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline(always)]
    pub fn uihash(&self) -> UIHASH_R {
        UIHASH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline(always)]
    pub fn ualgo(&self) -> UALGO_R {
        UALGO_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - Region Hash Area Protection"]
    #[inline(always)]
    pub fn haprot(&self) -> HAPROT_R {
        HAPROT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Region Descriptor Area Protection"]
    #[inline(always)]
    pub fn daprot(&self) -> DAPROT_R {
        DAPROT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wbdis(&mut self) -> WBDIS_W<CFG_SPEC, 0> {
        WBDIS_W::new(self)
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eomdis(&mut self) -> EOMDIS_W<CFG_SPEC, 1> {
        EOMDIS_W::new(self)
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline(always)]
    #[must_use]
    pub fn slbdis(&mut self) -> SLBDIS_W<CFG_SPEC, 2> {
        SLBDIS_W::new(self)
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline(always)]
    #[must_use]
    pub fn bbc(&mut self) -> BBC_W<CFG_SPEC, 4> {
        BBC_W::new(self)
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline(always)]
    #[must_use]
    pub fn ascd(&mut self) -> ASCD_W<CFG_SPEC, 8> {
        ASCD_W::new(self)
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn dualbuff(&mut self) -> DUALBUFF_W<CFG_SPEC, 9> {
        DUALBUFF_W::new(self)
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline(always)]
    #[must_use]
    pub fn uihash(&mut self) -> UIHASH_W<CFG_SPEC, 12> {
        UIHASH_W::new(self)
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline(always)]
    #[must_use]
    pub fn ualgo(&mut self) -> UALGO_W<CFG_SPEC, 13> {
        UALGO_W::new(self)
    }
    #[doc = "Bits 16:21 - Region Hash Area Protection"]
    #[inline(always)]
    #[must_use]
    pub fn haprot(&mut self) -> HAPROT_W<CFG_SPEC, 16> {
        HAPROT_W::new(self)
    }
    #[doc = "Bits 24:29 - Region Descriptor Area Protection"]
    #[inline(always)]
    #[must_use]
    pub fn daprot(&mut self) -> DAPROT_W<CFG_SPEC, 24> {
        DAPROT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
