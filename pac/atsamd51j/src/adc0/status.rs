#[doc = "Reader of register STATUS"]
pub type R = crate::R<u8, super::STATUS>;
#[doc = "Reader of field `ADCBUSY`"]
pub type ADCBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `WCC`"]
pub type WCC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - ADC Busy Status"]
    #[inline(always)]
    pub fn adcbusy(&self) -> ADCBUSY_R {
        ADCBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - Window Comparator Counter"]
    #[inline(always)]
    pub fn wcc(&self) -> WCC_R {
        WCC_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
