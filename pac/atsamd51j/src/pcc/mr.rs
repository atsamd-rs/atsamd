#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `PCEN` reader - Parallel Capture Enable"]
pub type PcenR = crate::BitReader;
#[doc = "Field `PCEN` writer - Parallel Capture Enable"]
pub type PcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsizeselect {
    #[doc = "0: 1 data is read in the PCC_RHR"]
    _1data = 0,
    #[doc = "1: 2 data is read in the PCC_RHR"]
    _2data = 1,
    #[doc = "2: 4 data are read in the PCC_RHR (only for 8 bits data size, ISIZE = 0)"]
    _4data = 2,
}
impl From<Dsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Dsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Dsizeselect {}
#[doc = "Field `DSIZE` reader - Data size"]
pub type DsizeR = crate::FieldReader<Dsizeselect>;
impl DsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsizeselect> {
        match self.bits {
            0 => Some(Dsizeselect::_1data),
            1 => Some(Dsizeselect::_2data),
            2 => Some(Dsizeselect::_4data),
            _ => None,
        }
    }
    #[doc = "1 data is read in the PCC_RHR"]
    #[inline(always)]
    pub fn is_1data(&self) -> bool {
        *self == Dsizeselect::_1data
    }
    #[doc = "2 data is read in the PCC_RHR"]
    #[inline(always)]
    pub fn is_2data(&self) -> bool {
        *self == Dsizeselect::_2data
    }
    #[doc = "4 data are read in the PCC_RHR (only for 8 bits data size, ISIZE = 0)"]
    #[inline(always)]
    pub fn is_4data(&self) -> bool {
        *self == Dsizeselect::_4data
    }
}
#[doc = "Field `DSIZE` writer - Data size"]
pub type DsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsizeselect>;
impl<'a, REG> DsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data is read in the PCC_RHR"]
    #[inline(always)]
    pub fn _1data(self) -> &'a mut crate::W<REG> {
        self.variant(Dsizeselect::_1data)
    }
    #[doc = "2 data is read in the PCC_RHR"]
    #[inline(always)]
    pub fn _2data(self) -> &'a mut crate::W<REG> {
        self.variant(Dsizeselect::_2data)
    }
    #[doc = "4 data are read in the PCC_RHR (only for 8 bits data size, ISIZE = 0)"]
    #[inline(always)]
    pub fn _4data(self) -> &'a mut crate::W<REG> {
        self.variant(Dsizeselect::_4data)
    }
}
#[doc = "Field `SCALE` reader - Scale data"]
pub type ScaleR = crate::BitReader;
#[doc = "Field `SCALE` writer - Scale data"]
pub type ScaleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALWYS` reader - Always Sampling"]
pub type AlwysR = crate::BitReader;
#[doc = "Field `ALWYS` writer - Always Sampling"]
pub type AlwysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALFS` reader - Half Sampling"]
pub type HalfsR = crate::BitReader;
#[doc = "Field `HALFS` writer - Half Sampling"]
pub type HalfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRSTS` reader - First sample"]
pub type FrstsR = crate::BitReader;
#[doc = "Field `FRSTS` writer - First sample"]
pub type FrstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Isizeselect {
    #[doc = "0: Input data bus size is 8 bits"]
    _8bits = 0,
    #[doc = "1: Input data bus size is 10 bits"]
    _10bits = 1,
    #[doc = "2: Input data bus size is 12 bits"]
    _12bits = 2,
    #[doc = "3: Input data bus size is 14 bits"]
    _14bits = 3,
}
impl From<Isizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Isizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Isizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Isizeselect {}
#[doc = "Field `ISIZE` reader - Input Data Size"]
pub type IsizeR = crate::FieldReader<Isizeselect>;
impl IsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Isizeselect> {
        match self.bits {
            0 => Some(Isizeselect::_8bits),
            1 => Some(Isizeselect::_10bits),
            2 => Some(Isizeselect::_12bits),
            3 => Some(Isizeselect::_14bits),
            _ => None,
        }
    }
    #[doc = "Input data bus size is 8 bits"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == Isizeselect::_8bits
    }
    #[doc = "Input data bus size is 10 bits"]
    #[inline(always)]
    pub fn is_10bits(&self) -> bool {
        *self == Isizeselect::_10bits
    }
    #[doc = "Input data bus size is 12 bits"]
    #[inline(always)]
    pub fn is_12bits(&self) -> bool {
        *self == Isizeselect::_12bits
    }
    #[doc = "Input data bus size is 14 bits"]
    #[inline(always)]
    pub fn is_14bits(&self) -> bool {
        *self == Isizeselect::_14bits
    }
}
#[doc = "Field `ISIZE` writer - Input Data Size"]
pub type IsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Isizeselect>;
impl<'a, REG> IsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input data bus size is 8 bits"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(Isizeselect::_8bits)
    }
    #[doc = "Input data bus size is 10 bits"]
    #[inline(always)]
    pub fn _10bits(self) -> &'a mut crate::W<REG> {
        self.variant(Isizeselect::_10bits)
    }
    #[doc = "Input data bus size is 12 bits"]
    #[inline(always)]
    pub fn _12bits(self) -> &'a mut crate::W<REG> {
        self.variant(Isizeselect::_12bits)
    }
    #[doc = "Input data bus size is 14 bits"]
    #[inline(always)]
    pub fn _14bits(self) -> &'a mut crate::W<REG> {
        self.variant(Isizeselect::_14bits)
    }
}
#[doc = "Field `CID` reader - Clear If Disabled"]
pub type CidR = crate::FieldReader;
#[doc = "Field `CID` writer - Clear If Disabled"]
pub type CidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Parallel Capture Enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data size"]
    #[inline(always)]
    pub fn dsize(&self) -> DsizeR {
        DsizeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Scale data"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Always Sampling"]
    #[inline(always)]
    pub fn alwys(&self) -> AlwysR {
        AlwysR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half Sampling"]
    #[inline(always)]
    pub fn halfs(&self) -> HalfsR {
        HalfsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - First sample"]
    #[inline(always)]
    pub fn frsts(&self) -> FrstsR {
        FrstsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Input Data Size"]
    #[inline(always)]
    pub fn isize(&self) -> IsizeR {
        IsizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Clear If Disabled"]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Parallel Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PcenW<MrSpec> {
        PcenW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Data size"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DsizeW<MrSpec> {
        DsizeW::new(self, 4)
    }
    #[doc = "Bit 8 - Scale data"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> ScaleW<MrSpec> {
        ScaleW::new(self, 8)
    }
    #[doc = "Bit 9 - Always Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn alwys(&mut self) -> AlwysW<MrSpec> {
        AlwysW::new(self, 9)
    }
    #[doc = "Bit 10 - Half Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn halfs(&mut self) -> HalfsW<MrSpec> {
        HalfsW::new(self, 10)
    }
    #[doc = "Bit 11 - First sample"]
    #[inline(always)]
    #[must_use]
    pub fn frsts(&mut self) -> FrstsW<MrSpec> {
        FrstsW::new(self, 11)
    }
    #[doc = "Bits 16:18 - Input Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn isize(&mut self) -> IsizeW<MrSpec> {
        IsizeW::new(self, 16)
    }
    #[doc = "Bits 30:31 - Clear If Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cid(&mut self) -> CidW<MrSpec> {
        CidW::new(self, 30)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
