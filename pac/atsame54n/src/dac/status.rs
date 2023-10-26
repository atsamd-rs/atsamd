#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `READY0` reader - DAC 0 Startup Ready"]
pub type READY0_R = crate::BitReader;
#[doc = "Field `READY1` reader - DAC 1 Startup Ready"]
pub type READY1_R = crate::BitReader;
#[doc = "Field `EOC0` reader - DAC 0 End of Conversion"]
pub type EOC0_R = crate::BitReader;
#[doc = "Field `EOC1` reader - DAC 1 End of Conversion"]
pub type EOC1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DAC 0 Startup Ready"]
    #[inline(always)]
    pub fn ready0(&self) -> READY0_R {
        READY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC 1 Startup Ready"]
    #[inline(always)]
    pub fn ready1(&self) -> READY1_R {
        READY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC 0 End of Conversion"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC 1 End of Conversion"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 3) & 1) != 0)
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
