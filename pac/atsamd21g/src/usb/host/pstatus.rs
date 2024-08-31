#[doc = "Register `PSTATUS%s` reader"]
pub type R = crate::R<PstatusSpec>;
#[doc = "Field `DTGL` reader - Data Toggle"]
pub type DtglR = crate::BitReader;
#[doc = "Field `CURBK` reader - Current Bank"]
pub type CurbkR = crate::BitReader;
#[doc = "Field `PFREEZE` reader - Pipe Freeze"]
pub type PfreezeR = crate::BitReader;
#[doc = "Field `BK0RDY` reader - Bank 0 ready"]
pub type Bk0rdyR = crate::BitReader;
#[doc = "Field `BK1RDY` reader - Bank 1 ready"]
pub type Bk1rdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Toggle"]
    #[inline(always)]
    pub fn dtgl(&self) -> DtglR {
        DtglR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Current Bank"]
    #[inline(always)]
    pub fn curbk(&self) -> CurbkR {
        CurbkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pipe Freeze"]
    #[inline(always)]
    pub fn pfreeze(&self) -> PfreezeR {
        PfreezeR::new(((self.bits >> 4) & 1) != 0)
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
#[doc = "HOST End Point Pipe Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PstatusSpec;
impl crate::RegisterSpec for PstatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pstatus::R`](R) reader structure"]
impl crate::Readable for PstatusSpec {}
#[doc = "`reset()` method sets PSTATUS%s to value 0"]
impl crate::Resettable for PstatusSpec {
    const RESET_VALUE: u8 = 0;
}
