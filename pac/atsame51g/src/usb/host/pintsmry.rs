#[doc = "Register `PINTSMRY` reader"]
pub struct R(crate::R<PINTSMRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINTSMRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINTSMRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINTSMRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPINT0` reader - Pipe 0 Interrupt"]
pub type EPINT0_R = crate::BitReader<bool>;
#[doc = "Field `EPINT1` reader - Pipe 1 Interrupt"]
pub type EPINT1_R = crate::BitReader<bool>;
#[doc = "Field `EPINT2` reader - Pipe 2 Interrupt"]
pub type EPINT2_R = crate::BitReader<bool>;
#[doc = "Field `EPINT3` reader - Pipe 3 Interrupt"]
pub type EPINT3_R = crate::BitReader<bool>;
#[doc = "Field `EPINT4` reader - Pipe 4 Interrupt"]
pub type EPINT4_R = crate::BitReader<bool>;
#[doc = "Field `EPINT5` reader - Pipe 5 Interrupt"]
pub type EPINT5_R = crate::BitReader<bool>;
#[doc = "Field `EPINT6` reader - Pipe 6 Interrupt"]
pub type EPINT6_R = crate::BitReader<bool>;
#[doc = "Field `EPINT7` reader - Pipe 7 Interrupt"]
pub type EPINT7_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Pipe 0 Interrupt"]
    #[inline(always)]
    pub fn epint0(&self) -> EPINT0_R {
        EPINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pipe 1 Interrupt"]
    #[inline(always)]
    pub fn epint1(&self) -> EPINT1_R {
        EPINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pipe 2 Interrupt"]
    #[inline(always)]
    pub fn epint2(&self) -> EPINT2_R {
        EPINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe 3 Interrupt"]
    #[inline(always)]
    pub fn epint3(&self) -> EPINT3_R {
        EPINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pipe 4 Interrupt"]
    #[inline(always)]
    pub fn epint4(&self) -> EPINT4_R {
        EPINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pipe 5 Interrupt"]
    #[inline(always)]
    pub fn epint5(&self) -> EPINT5_R {
        EPINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pipe 6 Interrupt"]
    #[inline(always)]
    pub fn epint6(&self) -> EPINT6_R {
        EPINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pipe 7 Interrupt"]
    #[inline(always)]
    pub fn epint7(&self) -> EPINT7_R {
        EPINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "HOST Pipe Interrupt Summary\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintsmry](index.html) module"]
pub struct PINTSMRY_SPEC;
impl crate::RegisterSpec for PINTSMRY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pintsmry::R](R) reader structure"]
impl crate::Readable for PINTSMRY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PINTSMRY to value 0"]
impl crate::Resettable for PINTSMRY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
