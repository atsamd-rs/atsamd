#[doc = "Register `FFSR` reader"]
pub struct R(crate::R<FFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FlInProg` reader - "]
pub type FL_IN_PROG_R = crate::BitReader<bool>;
#[doc = "Field `FtStopped` reader - "]
pub type FT_STOPPED_R = crate::BitReader<bool>;
#[doc = "Field `TCPresent` reader - "]
pub type TCPRESENT_R = crate::BitReader<bool>;
#[doc = "Field `FtNonStop` reader - "]
pub type FT_NON_STOP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fl_in_prog(&self) -> FL_IN_PROG_R {
        FL_IN_PROG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ft_stopped(&self) -> FT_STOPPED_R {
        FT_STOPPED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tcpresent(&self) -> TCPRESENT_R {
        TCPRESENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ft_non_stop(&self) -> FT_NON_STOP_R {
        FT_NON_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Formatter and Flush Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffsr](index.html) module"]
pub struct FFSR_SPEC;
impl crate::RegisterSpec for FFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffsr::R](R) reader structure"]
impl crate::Readable for FFSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FFSR to value 0"]
impl crate::Resettable for FFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
