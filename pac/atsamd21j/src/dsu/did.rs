#[doc = "Register `DID` reader"]
pub type R = crate::R<DidSpec>;
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DevselR = crate::FieldReader;
#[doc = "Field `REVISION` reader - Revision"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `DIE` reader - Die Identification"]
pub type DieR = crate::FieldReader;
#[doc = "Field `SERIES` reader - Product Series"]
pub type SeriesR = crate::FieldReader;
#[doc = "Field `FAMILY` reader - Product Family"]
pub type FamilyR = crate::FieldReader;
#[doc = "Field `PROCESSOR` reader - Processor"]
pub type ProcessorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Device Select"]
    #[inline(always)]
    pub fn devsel(&self) -> DevselR {
        DevselR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Die Identification"]
    #[inline(always)]
    pub fn die(&self) -> DieR {
        DieR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Product Series"]
    #[inline(always)]
    pub fn series(&self) -> SeriesR {
        SeriesR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 23:27 - Product Family"]
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
#[doc = "`reset()` method sets DID to value 0x1001_0300"]
impl crate::Resettable for DidSpec {
    const RESET_VALUE: u32 = 0x1001_0300;
}
