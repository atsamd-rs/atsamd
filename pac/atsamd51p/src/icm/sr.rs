#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `ENABLE` reader - ICM Controller Enable Register"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `RAWRMDIS` reader - RAW Region Monitoring Disabled Status"]
pub type RAWRMDIS_R = crate::FieldReader;
#[doc = "Field `RMDIS` reader - Region Monitoring Disabled Status"]
pub type RMDIS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - ICM Controller Enable Register"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - RAW Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rawrmdis(&self) -> RAWRMDIS_R {
        RAWRMDIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rmdis(&self) -> RMDIS_R {
        RMDIS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
