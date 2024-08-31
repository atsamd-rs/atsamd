#[doc = "Register `DEVTYPE` reader"]
pub type R = crate::R<DevtypeSpec>;
#[doc = "Field `SubType` reader - "]
pub type SubTypeR = crate::FieldReader;
#[doc = "Field `MajorType` reader - "]
pub type MajorTypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sub_type(&self) -> SubTypeR {
        SubTypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn major_type(&self) -> MajorTypeR {
        MajorTypeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "TPIU_DEVTYPE\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevtypeSpec;
impl crate::RegisterSpec for DevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devtype::R`](R) reader structure"]
impl crate::Readable for DevtypeSpec {}
#[doc = "`reset()` method sets DEVTYPE to value 0"]
impl crate::Resettable for DevtypeSpec {
    const RESET_VALUE: u32 = 0;
}
