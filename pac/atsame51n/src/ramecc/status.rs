#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `ECCDIS` reader - ECC Disable"]
pub type ECCDIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ECC Disable"]
    #[inline(always)]
    pub fn eccdis(&self) -> ECCDIS_R {
        ECCDIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
