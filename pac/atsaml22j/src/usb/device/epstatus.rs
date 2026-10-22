#[doc = "Register `EPSTATUS%s` reader"]
pub struct R(crate::R<EPSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DTGLOUT` reader - Data Toggle Out"]
pub type DTGLOUT_R = crate::BitReader<bool>;
#[doc = "Field `DTGLIN` reader - Data Toggle In"]
pub type DTGLIN_R = crate::BitReader<bool>;
#[doc = "Field `CURBK` reader - Current Bank"]
pub type CURBK_R = crate::BitReader<bool>;
#[doc = "Field `STALLRQ0` reader - Stall 0 Request"]
pub type STALLRQ0_R = crate::BitReader<bool>;
#[doc = "Field `STALLRQ1` reader - Stall 1 Request"]
pub type STALLRQ1_R = crate::BitReader<bool>;
#[doc = "Field `BK0RDY` reader - Bank 0 ready"]
pub type BK0RDY_R = crate::BitReader<bool>;
#[doc = "Field `BK1RDY` reader - Bank 1 ready"]
pub type BK1RDY_R = crate::BitReader<bool>;
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
#[doc = "DEVICE End Point Pipe Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatus](index.html) module"]
pub struct EPSTATUS_SPEC;
impl crate::RegisterSpec for EPSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [epstatus::R](R) reader structure"]
impl crate::Readable for EPSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPSTATUS%s to value 0"]
impl crate::Resettable for EPSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
