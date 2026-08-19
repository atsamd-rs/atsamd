#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `ADCBUSY` reader - ADC Busy Status"]
pub type AdcbusyR = crate::BitReader;
#[doc = "Field `WCC` reader - Window Comparator Counter"]
pub type WccR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - ADC Busy Status"]
    #[inline(always)]
    pub fn adcbusy(&self) -> AdcbusyR {
        AdcbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:7 - Window Comparator Counter"]
    #[inline(always)]
    pub fn wcc(&self) -> WccR {
        WccR::new((self.bits >> 2) & 0x3f)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u8 = 0;
}
