#[doc = "Register `PSTATUS%s` reader"]
pub type R = crate::R<PSTATUS_SPEC>;
#[doc = "Field `DTGL` reader - Data Toggle"]
pub type DTGL_R = crate::BitReader;
#[doc = "Field `CURBK` reader - Current Bank"]
pub type CURBK_R = crate::BitReader;
#[doc = "Field `PFREEZE` reader - Pipe Freeze"]
pub type PFREEZE_R = crate::BitReader;
#[doc = "Field `BK0RDY` reader - Bank 0 ready"]
pub type BK0RDY_R = crate::BitReader;
#[doc = "Field `BK1RDY` reader - Bank 1 ready"]
pub type BK1RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Toggle"]
    #[inline(always)]
    pub fn dtgl(&self) -> DTGL_R {
        DTGL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Current Bank"]
    #[inline(always)]
    pub fn curbk(&self) -> CURBK_R {
        CURBK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pipe Freeze"]
    #[inline(always)]
    pub fn pfreeze(&self) -> PFREEZE_R {
        PFREEZE_R::new(((self.bits >> 4) & 1) != 0)
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
#[doc = "HOST End Point Pipe Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSTATUS_SPEC;
impl crate::RegisterSpec for PSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pstatus::R`](R) reader structure"]
impl crate::Readable for PSTATUS_SPEC {}
#[doc = "`reset()` method sets PSTATUS%s to value 0"]
impl crate::Resettable for PSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
