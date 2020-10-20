#[doc = "Reader of register STATUSD"]
pub type R = crate::R<u32, super::STATUSD>;
#[doc = "Reader of field `SERCOM4_`"]
pub type SERCOM4__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM5_`"]
pub type SERCOM5__R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0_`"]
pub type ADC0__R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1_`"]
pub type ADC1__R = crate::R<bool, bool>;
#[doc = "Reader of field `DAC_`"]
pub type DAC__R = crate::R<bool, bool>;
#[doc = "Reader of field `PCC_`"]
pub type PCC__R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SERCOM4 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Protect Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC1 APB Protect Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> ADC1__R {
        ADC1__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DAC APB Protect Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PCC APB Protect Enable"]
    #[inline(always)]
    pub fn pcc_(&self) -> PCC__R {
        PCC__R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
