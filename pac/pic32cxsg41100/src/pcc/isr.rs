#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `DRDY` reader - Data Ready Interrupt Status"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Status"]
pub type OvreR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready Interrupt Status"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error Interrupt Status"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
