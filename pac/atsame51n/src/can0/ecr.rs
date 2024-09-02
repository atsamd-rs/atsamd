#[doc = "Register `ECR` reader"]
pub type R = crate::R<EcrSpec>;
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TecR = crate::FieldReader;
#[doc = "Field `REC` reader - Receive Error Counter"]
pub type RecR = crate::FieldReader;
#[doc = "Field `RP` reader - Receive Error Passive"]
pub type RpR = crate::BitReader;
#[doc = "Field `CEL` reader - CAN Error Logging"]
pub type CelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn rp(&self) -> RpR {
        RpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CAN Error Logging"]
    #[inline(always)]
    pub fn cel(&self) -> CelR {
        CelR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Error Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrSpec;
impl crate::RegisterSpec for EcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for EcrSpec {}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for EcrSpec {
    const RESET_VALUE: u32 = 0;
}
