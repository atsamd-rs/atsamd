#[doc = "Register `PID2` reader"]
pub type R = crate::R<PID2_SPEC>;
#[doc = "Field `JEPIDCH` reader - JEP-106 Identity Code High"]
pub type JEPIDCH_R = crate::FieldReader;
#[doc = "Field `JEPU` reader - JEP-106 Identity Code is used"]
pub type JEPU_R = crate::BitReader;
#[doc = "Field `REVISION` reader - Revision Number"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - JEP-106 Identity Code High"]
    #[inline(always)]
    pub fn jepidch(&self) -> JEPIDCH_R {
        JEPIDCH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEP-106 Identity Code is used"]
    #[inline(always)]
    pub fn jepu(&self) -> JEPU_R {
        JEPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Revision Number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID2_SPEC;
impl crate::RegisterSpec for PID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid2::R`](R) reader structure"]
impl crate::Readable for PID2_SPEC {}
#[doc = "`reset()` method sets PID2 to value 0x09"]
impl crate::Resettable for PID2_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
