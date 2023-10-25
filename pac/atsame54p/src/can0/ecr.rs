#[doc = "Register `ECR` reader"]
pub type R = crate::R<ECR_SPEC>;
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - Receive Error Counter"]
pub type REC_R = crate::FieldReader;
#[doc = "Field `RP` reader - Receive Error Passive"]
pub type RP_R = crate::BitReader;
#[doc = "Field `CEL` reader - CAN Error Logging"]
pub type CEL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CAN Error Logging"]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for ECR_SPEC {}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
