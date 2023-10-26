#[doc = "Register `STATUSD` reader"]
pub type R = crate::R<STATUSD_SPEC>;
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Protect Enable"]
pub type SERCOM4__R = crate::BitReader;
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Protect Enable"]
pub type SERCOM5__R = crate::BitReader;
#[doc = "Field `ADC0_` reader - ADC0 APB Protect Enable"]
pub type ADC0__R = crate::BitReader;
#[doc = "Field `ADC1_` reader - ADC1 APB Protect Enable"]
pub type ADC1__R = crate::BitReader;
#[doc = "Field `DAC_` reader - DAC APB Protect Enable"]
pub type DAC__R = crate::BitReader;
#[doc = "Field `PCC_` reader - PCC APB Protect Enable"]
pub type PCC__R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SERCOM4 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Protect Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 APB Protect Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> ADC1__R {
        ADC1__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC APB Protect Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - PCC APB Protect Enable"]
    #[inline(always)]
    pub fn pcc_(&self) -> PCC__R {
        PCC__R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSD_SPEC;
impl crate::RegisterSpec for STATUSD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusd::R`](R) reader structure"]
impl crate::Readable for STATUSD_SPEC {}
#[doc = "`reset()` method sets STATUSD to value 0"]
impl crate::Resettable for STATUSD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
