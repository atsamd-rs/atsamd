#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready"]
pub type Xosc32krdyR = crate::BitReader;
#[doc = "Field `XOSC32KFAIL` reader - XOSC32K Clock Failure Detector"]
pub type Xosc32kfailR = crate::BitReader;
#[doc = "Field `XOSC32KSW` reader - XOSC32K Clock switch"]
pub type Xosc32kswR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> Xosc32krdyR {
        Xosc32krdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector"]
    #[inline(always)]
    pub fn xosc32kfail(&self) -> Xosc32kfailR {
        Xosc32kfailR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XOSC32K Clock switch"]
    #[inline(always)]
    pub fn xosc32ksw(&self) -> Xosc32kswR {
        Xosc32kswR::new(((self.bits >> 3) & 1) != 0)
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
impl crate::Resettable for StatusSpec {}
