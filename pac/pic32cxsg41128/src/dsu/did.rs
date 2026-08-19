#[doc = "Register `DID` reader"]
pub type R = crate::R<DidSpec>;
#[doc = "Device Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Devselselect {
    #[doc = "0: 1 MB Flash / 256 KB SRAM / 128-pin"]
    _1mb256kb128pin = 0,
    #[doc = "1: 1 MB Flash / 256 KB SRAM / 100-pin"]
    _1mb256kb100pin = 1,
}
impl From<Devselselect> for u8 {
    #[inline(always)]
    fn from(variant: Devselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Devselselect {
    type Ux = u8;
}
impl crate::IsEnum for Devselselect {}
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DevselR = crate::FieldReader<Devselselect>;
impl DevselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Devselselect> {
        match self.bits {
            0 => Some(Devselselect::_1mb256kb128pin),
            1 => Some(Devselselect::_1mb256kb100pin),
            _ => None,
        }
    }
    #[doc = "1 MB Flash / 256 KB SRAM / 128-pin"]
    #[inline(always)]
    pub fn is_1mb_256kb_128pin(&self) -> bool {
        *self == Devselselect::_1mb256kb128pin
    }
    #[doc = "1 MB Flash / 256 KB SRAM / 100-pin"]
    #[inline(always)]
    pub fn is_1mb_256kb_100pin(&self) -> bool {
        *self == Devselselect::_1mb256kb100pin
    }
}
#[doc = "Field `REVISION` reader - Revision Number"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `DIE` reader - Die Number"]
pub type DieR = crate::FieldReader;
#[doc = "Series\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seriesselect {
    #[doc = "0: PIC32CX SG Without Immutable Boot, With HSM"]
    Pic32cxSg60 = 0,
    #[doc = "2: PIC32CX SG With Immutable Boot and HSM"]
    Pic32cxSg61 = 2,
    #[doc = "7: PIC32CX SG With Immutable Boot, No HSM"]
    Pic32cxSg41 = 7,
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
            0 => Some(Seriesselect::Pic32cxSg60),
            2 => Some(Seriesselect::Pic32cxSg61),
            7 => Some(Seriesselect::Pic32cxSg41),
            _ => None,
        }
    }
    #[doc = "PIC32CX SG Without Immutable Boot, With HSM"]
    #[inline(always)]
    pub fn is_pic32cx_sg60(&self) -> bool {
        *self == Seriesselect::Pic32cxSg60
    }
    #[doc = "PIC32CX SG With Immutable Boot and HSM"]
    #[inline(always)]
    pub fn is_pic32cx_sg61(&self) -> bool {
        *self == Seriesselect::Pic32cxSg61
    }
    #[doc = "PIC32CX SG With Immutable Boot, No HSM"]
    #[inline(always)]
    pub fn is_pic32cx_sg41(&self) -> bool {
        *self == Seriesselect::Pic32cxSg41
    }
}
#[doc = "Family\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Familyselect {
    #[doc = "3: PIC32CX Advanced Feature Set Microcontrollers"]
    _1 = 3,
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
            3 => Some(Familyselect::_1),
            _ => None,
        }
    }
    #[doc = "PIC32CX Advanced Feature Set Microcontrollers"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Familyselect::_1
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
#[doc = "`reset()` method sets DID to value 0x6187_0500"]
impl crate::Resettable for DidSpec {
    const RESET_VALUE: u32 = 0x6187_0500;
}
