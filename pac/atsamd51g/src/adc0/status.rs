#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `ADCBUSY` reader - ADC Busy Status"]
pub type ADCBUSY_R = crate::BitReader;
#[doc = "Field `WCC` reader - Window Comparator Counter"]
pub type WCC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - ADC Busy Status"]
    #[inline(always)]
    pub fn adcbusy(&self) -> ADCBUSY_R {
        ADCBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:7 - Window Comparator Counter"]
    #[inline(always)]
    pub fn wcc(&self) -> WCC_R {
        WCC_R::new((self.bits >> 2) & 0x3f)
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
