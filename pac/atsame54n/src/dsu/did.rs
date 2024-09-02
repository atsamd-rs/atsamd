#[doc = "Register `DID` reader"]
pub type R = crate::R<DidSpec>;
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DevselR = crate::FieldReader;
#[doc = "Field `REVISION` reader - Revision Number"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `DIE` reader - Die Number"]
pub type DieR = crate::FieldReader;
#[doc = "Series\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seriesselect {
    #[doc = "1: SAM E51"]
    Same51 = 1,
    #[doc = "3: SAM E53"]
    Same53 = 3,
    #[doc = "4: SAM E54"]
    Same54 = 4,
    #[doc = "6: SAM D51"]
    Samd51 = 6,
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
            1 => Some(Seriesselect::Same51),
            3 => Some(Seriesselect::Same53),
            4 => Some(Seriesselect::Same54),
            6 => Some(Seriesselect::Samd51),
            _ => None,
        }
    }
    #[doc = "SAM E51"]
    #[inline(always)]
    pub fn is_same51(&self) -> bool {
        *self == Seriesselect::Same51
    }
    #[doc = "SAM E53"]
    #[inline(always)]
    pub fn is_same53(&self) -> bool {
        *self == Seriesselect::Same53
    }
    #[doc = "SAM E54"]
    #[inline(always)]
    pub fn is_same54(&self) -> bool {
        *self == Seriesselect::Same54
    }
    #[doc = "SAM D51"]
    #[inline(always)]
    pub fn is_samd51(&self) -> bool {
        *self == Seriesselect::Samd51
    }
}
#[doc = "Family\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Familyselect {
    #[doc = "0: General purpose microcontroller"]
    Samd5x = 0,
    #[doc = "3: PicoPower"]
    Same5x = 3,
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
            0 => Some(Familyselect::Samd5x),
            3 => Some(Familyselect::Same5x),
            _ => None,
        }
    }
    #[doc = "General purpose microcontroller"]
    #[inline(always)]
    pub fn is_samd5x(&self) -> bool {
        *self == Familyselect::Samd5x
    }
    #[doc = "PicoPower"]
    #[inline(always)]
    pub fn is_same5x(&self) -> bool {
        *self == Familyselect::Same5x
    }
}
#[doc = "Processor\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Processorselect {
    #[doc = "6: Cortex-M4 with FPU"]
    Cm4f = 6,
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
            6 => Some(Processorselect::Cm4f),
            _ => None,
        }
    }
    #[doc = "Cortex-M4 with FPU"]
    #[inline(always)]
    pub fn is_cm4f(&self) -> bool {
        *self == Processorselect::Cm4f
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
#[doc = "`reset()` method sets DID to value 0x6184_0203"]
impl crate::Resettable for DidSpec {
    const RESET_VALUE: u32 = 0x6184_0203;
}
