#[doc = "Register `EPINTSMRY` reader"]
pub struct R(crate::R<EPINTSMRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPINTSMRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPINTSMRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPINTSMRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPINT0` reader - End Point 0 Interrupt"]
pub type EPINT0_R = crate::BitReader<bool>;
#[doc = "Field `EPINT1` reader - End Point 1 Interrupt"]
pub type EPINT1_R = crate::BitReader<bool>;
#[doc = "Field `EPINT2` reader - End Point 2 Interrupt"]
pub type EPINT2_R = crate::BitReader<bool>;
#[doc = "Field `EPINT3` reader - End Point 3 Interrupt"]
pub type EPINT3_R = crate::BitReader<bool>;
#[doc = "Field `EPINT4` reader - End Point 4 Interrupt"]
pub type EPINT4_R = crate::BitReader<bool>;
#[doc = "Field `EPINT5` reader - End Point 5 Interrupt"]
pub type EPINT5_R = crate::BitReader<bool>;
#[doc = "Field `EPINT6` reader - End Point 6 Interrupt"]
pub type EPINT6_R = crate::BitReader<bool>;
#[doc = "Field `EPINT7` reader - End Point 7 Interrupt"]
pub type EPINT7_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - End Point 0 Interrupt"]
    #[inline(always)]
    pub fn epint0(&self) -> EPINT0_R {
        EPINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End Point 1 Interrupt"]
    #[inline(always)]
    pub fn epint1(&self) -> EPINT1_R {
        EPINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End Point 2 Interrupt"]
    #[inline(always)]
    pub fn epint2(&self) -> EPINT2_R {
        EPINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End Point 3 Interrupt"]
    #[inline(always)]
    pub fn epint3(&self) -> EPINT3_R {
        EPINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End Point 4 Interrupt"]
    #[inline(always)]
    pub fn epint4(&self) -> EPINT4_R {
        EPINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Point 5 Interrupt"]
    #[inline(always)]
    pub fn epint5(&self) -> EPINT5_R {
        EPINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End Point 6 Interrupt"]
    #[inline(always)]
    pub fn epint6(&self) -> EPINT6_R {
        EPINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End Point 7 Interrupt"]
    #[inline(always)]
    pub fn epint7(&self) -> EPINT7_R {
        EPINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DEVICE End Point Interrupt Summary\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintsmry](index.html) module"]
pub struct EPINTSMRY_SPEC;
impl crate::RegisterSpec for EPINTSMRY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [epintsmry::R](R) reader structure"]
impl crate::Readable for EPINTSMRY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPINTSMRY to value 0"]
impl crate::Resettable for EPINTSMRY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
