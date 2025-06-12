#[doc = "Register `IHCE` reader"]
pub type R = crate::R<IhceSpec>;
#[doc = "Field `HCKER` reader - IP Header Checksum Errors"]
pub type HckerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - IP Header Checksum Errors"]
    #[inline(always)]
    pub fn hcker(&self) -> HckerR {
        HckerR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IP Header Checksum Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ihce::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhceSpec;
impl crate::RegisterSpec for IhceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ihce::R`](R) reader structure"]
impl crate::Readable for IhceSpec {}
#[doc = "`reset()` method sets IHCE to value 0"]
impl crate::Resettable for IhceSpec {}
