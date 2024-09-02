#[doc = "Register `DID` reader"]
pub type R = crate::R<DidSpec>;
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DevselR = crate::FieldReader;
#[doc = "Field `REVISION` reader - Revision Number"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `DIE` reader - Die Number"]
pub type DieR = crate::FieldReader;
#[doc = "Series\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seriesselect {
    #[doc = "0: Cortex-M0+ processor, basic feature set"]
    _0 = 0,
    #[doc = "1: Cortex-M0+ processor, USB"]
    _1 = 1,
}
impl From<Seriesselect> for u8 {
    #[inline(always)]
    fn from(variant: Seriesselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Seriesselect {
    type Ux = u8;
}
impl crate::IsEnum for Seriesselect {}
#[doc = "Field `SERIES` reader - Series"]
pub type SeriesR = crate::FieldReader<Seriesselect>;
impl SeriesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Seriesselect> {
        match self.bits {
            0 => Some(Seriesselect::_0),
            1 => Some(Seriesselect::_1),
            _ => None,
        }
    }
    #[doc = "Cortex-M0+ processor, basic feature set"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Seriesselect::_0
    }
    #[doc = "Cortex-M0+ processor, USB"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Seriesselect::_1
    }
}
#[doc = "Family\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Familyselect {
    #[doc = "0: General purpose microcontroller"]
    _0 = 0,
    #[doc = "1: PicoPower"]
    _1 = 1,
}
impl From<Familyselect> for u8 {
    #[inline(always)]
    fn from(variant: Familyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Familyselect {
    type Ux = u8;
}
impl crate::IsEnum for Familyselect {}
#[doc = "Field `FAMILY` reader - Family"]
pub type FamilyR = crate::FieldReader<Familyselect>;
impl FamilyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Familyselect> {
        match self.bits {
            0 => Some(Familyselect::_0),
            1 => Some(Familyselect::_1),
            _ => None,
        }
    }
    #[doc = "General purpose microcontroller"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Familyselect::_0
    }
    #[doc = "PicoPower"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Familyselect::_1
    }
}
#[doc = "Processor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Processorselect {
    #[doc = "0: Cortex-M0"]
    _0 = 0,
    #[doc = "1: Cortex-M0+"]
    _1 = 1,
    #[doc = "2: Cortex-M3"]
    _2 = 2,
    #[doc = "3: Cortex-M4"]
    _3 = 3,
}
impl From<Processorselect> for u8 {
    #[inline(always)]
    fn from(variant: Processorselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Processorselect {
    type Ux = u8;
}
impl crate::IsEnum for Processorselect {}
#[doc = "Field `PROCESSOR` reader - Processor"]
pub type ProcessorR = crate::FieldReader<Processorselect>;
impl ProcessorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Processorselect> {
        match self.bits {
            0 => Some(Processorselect::_0),
            1 => Some(Processorselect::_1),
            2 => Some(Processorselect::_2),
            3 => Some(Processorselect::_3),
            _ => None,
        }
    }
    #[doc = "Cortex-M0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Processorselect::_0
    }
    #[doc = "Cortex-M0+"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Processorselect::_1
    }
    #[doc = "Cortex-M3"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Processorselect::_2
    }
    #[doc = "Cortex-M4"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == Processorselect::_3
    }
}
impl R {
    #[doc = "Bits 0:7 - Device Select"]
    #[inline(always)]
    pub fn devsel(&self) -> DevselR {
        DevselR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Revision Number"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Die Number"]
    #[inline(always)]
    pub fn die(&self) -> DieR {
        DieR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Series"]
    #[inline(always)]
    pub fn series(&self) -> SeriesR {
        SeriesR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 23:27 - Family"]
    #[inline(always)]
    pub fn family(&self) -> FamilyR {
        FamilyR::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - Processor"]
    #[inline(always)]
    pub fn processor(&self) -> ProcessorR {
        ProcessorR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Device Identification\n\nYou can [`read`](crate::Reg::read) this register and get [`did::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DidSpec;
impl crate::RegisterSpec for DidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`did::R`](R) reader structure"]
impl crate::Readable for DidSpec {}
#[doc = "`reset()` method sets DID to value 0x1003_0109"]
impl crate::Resettable for DidSpec {
    const RESET_VALUE: u32 = 0x1003_0109;
}
