#[doc = "Register `EPSTATUS` reader"]
pub type R = crate::R<EPSTATUS_SPEC>;
#[doc = "Field `DTGLOUT` reader - Data Toggle Out"]
pub type DTGLOUT_R = crate::BitReader;
#[doc = "Field `DTGLIN` reader - Data Toggle In"]
pub type DTGLIN_R = crate::BitReader;
#[doc = "Field `CURBK` reader - Current Bank"]
pub type CURBK_R = crate::BitReader;
#[doc = "Field `STALLRQ0` reader - Stall 0 Request"]
pub type STALLRQ0_R = crate::BitReader;
#[doc = "Field `STALLRQ1` reader - Stall 1 Request"]
pub type STALLRQ1_R = crate::BitReader;
#[doc = "Field `BK0RDY` reader - Bank 0 ready"]
pub type BK0RDY_R = crate::BitReader;
#[doc = "Field `BK1RDY` reader - Bank 1 ready"]
pub type BK1RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Toggle Out"]
    #[inline(always)]
    pub fn dtglout(&self) -> DTGLOUT_R {
        DTGLOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Toggle In"]
    #[inline(always)]
    pub fn dtglin(&self) -> DTGLIN_R {
        DTGLIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Current Bank"]
    #[inline(always)]
    pub fn curbk(&self) -> CURBK_R {
        CURBK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Stall 0 Request"]
    #[inline(always)]
    pub fn stallrq0(&self) -> STALLRQ0_R {
        STALLRQ0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall 1 Request"]
    #[inline(always)]
    pub fn stallrq1(&self) -> STALLRQ1_R {
        STALLRQ1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bank 0 ready"]
    #[inline(always)]
    pub fn bk0rdy(&self) -> BK0RDY_R {
        BK0RDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bank 1 ready"]
    #[inline(always)]
    pub fn bk1rdy(&self) -> BK1RDY_R {
        BK1RDY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DEVICE_ENDPOINT End Point Pipe Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPSTATUS_SPEC;
impl crate::RegisterSpec for EPSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`epstatus::R`](R) reader structure"]
impl crate::Readable for EPSTATUS_SPEC {}
#[doc = "`reset()` method sets EPSTATUS to value 0"]
impl crate::Resettable for EPSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
