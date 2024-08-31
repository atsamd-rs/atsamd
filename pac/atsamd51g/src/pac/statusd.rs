#[doc = "Register `STATUSD` reader"]
pub type R = crate::R<StatusdSpec>;
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Protect Enable"]
pub type Sercom4_R = crate::BitReader;
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Protect Enable"]
pub type Sercom5_R = crate::BitReader;
#[doc = "Field `ADC0_` reader - ADC0 APB Protect Enable"]
pub type Adc0_R = crate::BitReader;
#[doc = "Field `ADC1_` reader - ADC1 APB Protect Enable"]
pub type Adc1_R = crate::BitReader;
#[doc = "Field `DAC_` reader - DAC APB Protect Enable"]
pub type Dac_R = crate::BitReader;
#[doc = "Field `PCC_` reader - PCC APB Protect Enable"]
pub type Pcc_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SERCOM4 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> Sercom4_R {
        Sercom4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> Sercom5_R {
        Sercom5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Protect Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> Adc0_R {
        Adc0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 APB Protect Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> Adc1_R {
        Adc1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC APB Protect Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> Dac_R {
        Dac_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - PCC APB Protect Enable"]
    #[inline(always)]
    pub fn pcc_(&self) -> Pcc_R {
        Pcc_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge D\n\nYou can [`read`](crate::Reg::read) this register and get [`statusd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusdSpec;
impl crate::RegisterSpec for StatusdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusd::R`](R) reader structure"]
impl crate::Readable for StatusdSpec {}
#[doc = "`reset()` method sets STATUSD to value 0"]
impl crate::Resettable for StatusdSpec {
    const RESET_VALUE: u32 = 0;
}
