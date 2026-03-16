#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `ICDIS` reader - Instruction Cache Disable"]
pub type IcdisR = crate::BitReader;
#[doc = "Field `ICDIS` writer - Instruction Cache Disable"]
pub type IcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDIS` reader - Data Cache Disable"]
pub type DcdisR = crate::BitReader;
#[doc = "Field `DCDIS` writer - Data Cache Disable"]
pub type DcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Cache size configured by software\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csizeswselect {
    #[doc = "0: The Cache Size is configured to 1KB"]
    ConfCsize1kb = 0,
    #[doc = "1: The Cache Size is configured to 2KB"]
    ConfCsize2kb = 1,
    #[doc = "2: The Cache Size is configured to 4KB"]
    ConfCsize4kb = 2,
}
impl From<Csizeswselect> for u8 {
    #[inline(always)]
    fn from(variant: Csizeswselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csizeswselect {
    type Ux = u8;
}
impl crate::IsEnum for Csizeswselect {}
#[doc = "Field `CSIZESW` reader - Cache size configured by software"]
pub type CsizeswR = crate::FieldReader<Csizeswselect>;
impl CsizeswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csizeswselect> {
        match self.bits {
            0 => Some(Csizeswselect::ConfCsize1kb),
            1 => Some(Csizeswselect::ConfCsize2kb),
            2 => Some(Csizeswselect::ConfCsize4kb),
            _ => None,
        }
    }
    #[doc = "The Cache Size is configured to 1KB"]
    #[inline(always)]
    pub fn is_conf_csize_1kb(&self) -> bool {
        *self == Csizeswselect::ConfCsize1kb
    }
    #[doc = "The Cache Size is configured to 2KB"]
    #[inline(always)]
    pub fn is_conf_csize_2kb(&self) -> bool {
        *self == Csizeswselect::ConfCsize2kb
    }
    #[doc = "The Cache Size is configured to 4KB"]
    #[inline(always)]
    pub fn is_conf_csize_4kb(&self) -> bool {
        *self == Csizeswselect::ConfCsize4kb
    }
}
#[doc = "Field `CSIZESW` writer - Cache size configured by software"]
pub type CsizeswW<'a, REG> = crate::FieldWriter<'a, REG, 3, Csizeswselect>;
impl<'a, REG> CsizeswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The Cache Size is configured to 1KB"]
    #[inline(always)]
    pub fn conf_csize_1kb(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeswselect::ConfCsize1kb)
    }
    #[doc = "The Cache Size is configured to 2KB"]
    #[inline(always)]
    pub fn conf_csize_2kb(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeswselect::ConfCsize2kb)
    }
    #[doc = "The Cache Size is configured to 4KB"]
    #[inline(always)]
    pub fn conf_csize_4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeswselect::ConfCsize4kb)
    }
}
impl R {
    #[doc = "Bit 1 - Instruction Cache Disable"]
    #[inline(always)]
    pub fn icdis(&self) -> IcdisR {
        IcdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Cache Disable"]
    #[inline(always)]
    pub fn dcdis(&self) -> DcdisR {
        DcdisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Cache size configured by software"]
    #[inline(always)]
    pub fn csizesw(&self) -> CsizeswR {
        CsizeswR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Instruction Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn icdis(&mut self) -> IcdisW<CfgSpec> {
        IcdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Data Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdis(&mut self) -> DcdisW<CfgSpec> {
        DcdisW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Cache size configured by software"]
    #[inline(always)]
    #[must_use]
    pub fn csizesw(&mut self) -> CsizeswW<CfgSpec> {
        CsizeswW::new(self, 4)
    }
}
#[doc = "Cache Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CFG to value 0x20"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x20;
}
