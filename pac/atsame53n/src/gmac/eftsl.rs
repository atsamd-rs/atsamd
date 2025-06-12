#[doc = "Register `EFTSL` reader"]
pub type R = crate::R<EftslSpec>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RudR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new(self.bits)
    }
}
#[doc = "PTP Event Frame Transmitted Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eftsl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EftslSpec;
impl crate::RegisterSpec for EftslSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eftsl::R`](R) reader structure"]
impl crate::Readable for EftslSpec {}
#[doc = "`reset()` method sets EFTSL to value 0"]
impl crate::Resettable for EftslSpec {}
