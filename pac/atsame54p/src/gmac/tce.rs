#[doc = "Register `TCE` reader"]
pub type R = crate::R<TceSpec>;
#[doc = "Field `TCKER` reader - TCP Checksum Errors"]
pub type TckerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - TCP Checksum Errors"]
    #[inline(always)]
    pub fn tcker(&self) -> TckerR {
        TckerR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TCP Checksum Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tce::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TceSpec;
impl crate::RegisterSpec for TceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tce::R`](R) reader structure"]
impl crate::Readable for TceSpec {}
#[doc = "`reset()` method sets TCE to value 0"]
impl crate::Resettable for TceSpec {}
