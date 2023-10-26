#[doc = "Register `PID0` reader"]
pub type R = crate::R<PID0_SPEC>;
#[doc = "Field `PARTNBL` reader - Part Number Low"]
pub type PARTNBL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Part Number Low"]
    #[inline(always)]
    pub fn partnbl(&self) -> PARTNBL_R {
        PARTNBL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral Identification 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID0_SPEC;
impl crate::RegisterSpec for PID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid0::R`](R) reader structure"]
impl crate::Readable for PID0_SPEC {}
#[doc = "`reset()` method sets PID0 to value 0xd0"]
impl crate::Resettable for PID0_SPEC {
    const RESET_VALUE: Self::Ux = 0xd0;
}
