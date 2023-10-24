#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `DRDY` reader - Data Ready Interrupt Status"]
pub type DRDY_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Status"]
pub type OVRE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready Interrupt Status"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error Interrupt Status"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
