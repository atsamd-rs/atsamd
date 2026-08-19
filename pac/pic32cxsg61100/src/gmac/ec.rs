#[doc = "Register `EC` reader"]
pub type R = crate::R<EcSpec>;
#[doc = "Field `XCOL` reader - Excessive Collisions"]
pub type XcolR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Excessive Collisions"]
    #[inline(always)]
    pub fn xcol(&self) -> XcolR {
        XcolR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Excessive Collisions Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcSpec;
impl crate::RegisterSpec for EcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ec::R`](R) reader structure"]
impl crate::Readable for EcSpec {}
#[doc = "`reset()` method sets EC to value 0"]
impl crate::Resettable for EcSpec {
    const RESET_VALUE: u32 = 0;
}
