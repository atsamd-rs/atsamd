#[doc = "Register `FFSR` reader"]
pub type R = crate::R<FfsrSpec>;
#[doc = "Field `FlInProg` reader - "]
pub type FlInProgR = crate::BitReader;
#[doc = "Field `FtStopped` reader - "]
pub type FtStoppedR = crate::BitReader;
#[doc = "Field `TCPresent` reader - "]
pub type TcpresentR = crate::BitReader;
#[doc = "Field `FtNonStop` reader - "]
pub type FtNonStopR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fl_in_prog(&self) -> FlInProgR {
        FlInProgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ft_stopped(&self) -> FtStoppedR {
        FtStoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tcpresent(&self) -> TcpresentR {
        TcpresentR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ft_non_stop(&self) -> FtNonStopR {
        FtNonStopR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Formatter and Flush Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfsrSpec;
impl crate::RegisterSpec for FfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffsr::R`](R) reader structure"]
impl crate::Readable for FfsrSpec {}
#[doc = "`reset()` method sets FFSR to value 0"]
impl crate::Resettable for FfsrSpec {
    const RESET_VALUE: u32 = 0;
}
