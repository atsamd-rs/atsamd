#[doc = "Register `CPUID` reader"]
pub type R = crate::R<CpuidSpec>;
#[doc = "Field `REVISION` reader - Processor revision number"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `PARTNO` reader - Process Part Number, 0xC24=Cortex-M4"]
pub type PartnoR = crate::FieldReader<u16>;
#[doc = "Field `CONSTANT` reader - Constant"]
pub type ConstantR = crate::FieldReader;
#[doc = "Field `VARIANT` reader - Variant number"]
pub type VariantR = crate::FieldReader;
#[doc = "Field `IMPLEMENTER` reader - Implementer code, 0x41=ARM"]
pub type ImplementerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Processor revision number"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Process Part Number, 0xC24=Cortex-M4"]
    #[inline(always)]
    pub fn partno(&self) -> PartnoR {
        PartnoR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Constant"]
    #[inline(always)]
    pub fn constant(&self) -> ConstantR {
        ConstantR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Variant number"]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementer code, 0x41=ARM"]
    #[inline(always)]
    pub fn implementer(&self) -> ImplementerR {
        ImplementerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CPUID Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuidSpec;
impl crate::RegisterSpec for CpuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuid::R`](R) reader structure"]
impl crate::Readable for CpuidSpec {}
#[doc = "`reset()` method sets CPUID to value 0x410f_c240"]
impl crate::Resettable for CpuidSpec {
    const RESET_VALUE: u32 = 0x410f_c240;
}
