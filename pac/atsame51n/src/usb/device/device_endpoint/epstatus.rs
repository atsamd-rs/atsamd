#[doc = "Register `EPSTATUS` reader"]
pub type R = crate::R<EpstatusSpec>;
#[doc = "Field `DTGLOUT` reader - Data Toggle Out"]
pub type DtgloutR = crate::BitReader;
#[doc = "Field `DTGLIN` reader - Data Toggle In"]
pub type DtglinR = crate::BitReader;
#[doc = "Field `CURBK` reader - Current Bank"]
pub type CurbkR = crate::BitReader;
#[doc = "Field `STALLRQ0` reader - Stall 0 Request"]
pub type Stallrq0R = crate::BitReader;
#[doc = "Field `STALLRQ1` reader - Stall 1 Request"]
pub type Stallrq1R = crate::BitReader;
#[doc = "Field `BK0RDY` reader - Bank 0 ready"]
pub type Bk0rdyR = crate::BitReader;
#[doc = "Field `BK1RDY` reader - Bank 1 ready"]
pub type Bk1rdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Toggle Out"]
    #[inline(always)]
    pub fn dtglout(&self) -> DtgloutR {
        DtgloutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Toggle In"]
    #[inline(always)]
    pub fn dtglin(&self) -> DtglinR {
        DtglinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Current Bank"]
    #[inline(always)]
    pub fn curbk(&self) -> CurbkR {
        CurbkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Stall 0 Request"]
    #[inline(always)]
    pub fn stallrq0(&self) -> Stallrq0R {
        Stallrq0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall 1 Request"]
    #[inline(always)]
    pub fn stallrq1(&self) -> Stallrq1R {
        Stallrq1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bank 0 ready"]
    #[inline(always)]
    pub fn bk0rdy(&self) -> Bk0rdyR {
        Bk0rdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bank 1 ready"]
    #[inline(always)]
    pub fn bk1rdy(&self) -> Bk1rdyR {
        Bk1rdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DEVICE_ENDPOINT End Point Pipe Status\n\nYou can [`read`](crate::Reg::read) this register and get [`epstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpstatusSpec;
impl crate::RegisterSpec for EpstatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`epstatus::R`](R) reader structure"]
impl crate::Readable for EpstatusSpec {}
#[doc = "`reset()` method sets EPSTATUS to value 0"]
impl crate::Resettable for EpstatusSpec {}
