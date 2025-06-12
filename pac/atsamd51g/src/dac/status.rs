#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `READY0` reader - DAC 0 Startup Ready"]
pub type Ready0R = crate::BitReader;
#[doc = "Field `READY1` reader - DAC 1 Startup Ready"]
pub type Ready1R = crate::BitReader;
#[doc = "Field `EOC0` reader - DAC 0 End of Conversion"]
pub type Eoc0R = crate::BitReader;
#[doc = "Field `EOC1` reader - DAC 1 End of Conversion"]
pub type Eoc1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DAC 0 Startup Ready"]
    #[inline(always)]
    pub fn ready0(&self) -> Ready0R {
        Ready0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC 1 Startup Ready"]
    #[inline(always)]
    pub fn ready1(&self) -> Ready1R {
        Ready1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC 0 End of Conversion"]
    #[inline(always)]
    pub fn eoc0(&self) -> Eoc0R {
        Eoc0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC 1 End of Conversion"]
    #[inline(always)]
    pub fn eoc1(&self) -> Eoc1R {
        Eoc1R::new(((self.bits >> 3) & 1) != 0)
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
impl crate::Resettable for StatusSpec {}
