#[doc = "Register `UCE` reader"]
pub type R = crate::R<UceSpec>;
#[doc = "Field `UCKER` reader - UDP Checksum Errors"]
pub type UckerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - UDP Checksum Errors"]
    #[inline(always)]
    pub fn ucker(&self) -> UckerR {
        UckerR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UDP Checksum Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uce::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UceSpec;
impl crate::RegisterSpec for UceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uce::R`](R) reader structure"]
impl crate::Readable for UceSpec {}
#[doc = "`reset()` method sets UCE to value 0"]
impl crate::Resettable for UceSpec {
    const RESET_VALUE: u32 = 0;
}
