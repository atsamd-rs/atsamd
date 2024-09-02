#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub type Bod33rdyR = crate::BitReader;
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub type Bod33detR = crate::BitReader;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub type B33srdyR = crate::BitReader;
#[doc = "Field `VREGRDY` reader - Voltage Regulator Ready"]
pub type VregrdyR = crate::BitReader;
#[doc = "Field `VCORERDY` reader - VDDCORE Ready"]
pub type VcorerdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> Bod33rdyR {
        Bod33rdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> Bod33detR {
        Bod33detR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33srdyR {
        B33srdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    pub fn vregrdy(&self) -> VregrdyR {
        VregrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    pub fn vcorerdy(&self) -> VcorerdyR {
        VcorerdyR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Power and Clocks Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
