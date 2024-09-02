#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `LQOS` reader - Latency Quality Of Service"]
pub type LqosR = crate::FieldReader;
#[doc = "Field `LQOS` writer - Latency Quality Of Service"]
pub type LqosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "DMA Trigger Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dccdmalevelselect {
    #[doc = "0: Trigger rises when DCC is empty"]
    Empty = 0,
    #[doc = "1: Trigger rises when DCC is full"]
    Full = 1,
}
impl From<Dccdmalevelselect> for u8 {
    #[inline(always)]
    fn from(variant: Dccdmalevelselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dccdmalevelselect {
    type Ux = u8;
}
impl crate::IsEnum for Dccdmalevelselect {}
#[doc = "Field `DCCDMALEVEL` reader - DMA Trigger Level"]
pub type DccdmalevelR = crate::FieldReader<Dccdmalevelselect>;
impl DccdmalevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dccdmalevelselect> {
        match self.bits {
            0 => Some(Dccdmalevelselect::Empty),
            1 => Some(Dccdmalevelselect::Full),
            _ => None,
        }
    }
    #[doc = "Trigger rises when DCC is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Dccdmalevelselect::Empty
    }
    #[doc = "Trigger rises when DCC is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Dccdmalevelselect::Full
    }
}
#[doc = "Field `DCCDMALEVEL` writer - DMA Trigger Level"]
pub type DccdmalevelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dccdmalevelselect>;
impl<'a, REG> DccdmalevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger rises when DCC is empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Dccdmalevelselect::Empty)
    }
    #[doc = "Trigger rises when DCC is full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Dccdmalevelselect::Full)
    }
}
#[doc = "Field `ETBRAMEN` reader - Trace Control"]
pub type EtbramenR = crate::BitReader;
#[doc = "Field `ETBRAMEN` writer - Trace Control"]
pub type EtbramenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline(always)]
    pub fn lqos(&self) -> LqosR {
        LqosR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline(always)]
    pub fn dccdmalevel(&self) -> DccdmalevelR {
        DccdmalevelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline(always)]
    pub fn etbramen(&self) -> EtbramenR {
        EtbramenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline(always)]
    #[must_use]
    pub fn lqos(&mut self) -> LqosW<CfgSpec> {
        LqosW::new(self, 0)
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn dccdmalevel(&mut self) -> DccdmalevelW<CfgSpec> {
        DccdmalevelW::new(self, 2)
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline(always)]
    #[must_use]
    pub fn etbramen(&mut self) -> EtbramenW<CfgSpec> {
        EtbramenW::new(self, 4)
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
#[doc = "`reset()` method sets CFG to value 0x02"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x02;
}
