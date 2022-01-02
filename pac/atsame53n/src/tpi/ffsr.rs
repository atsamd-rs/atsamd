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
pub struct FLINPROG_R(crate::FieldReader<bool, bool>);
impl FLINPROG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLINPROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLINPROG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FtStopped` reader - "]
pub struct FTSTOPPED_R(crate::FieldReader<bool, bool>);
impl FTSTOPPED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTSTOPPED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTSTOPPED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCPresent` reader - "]
pub struct TCPRESENT_R(crate::FieldReader<bool, bool>);
impl TCPRESENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCPRESENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCPRESENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FtNonStop` reader - "]
pub struct FTNONSTOP_R(crate::FieldReader<bool, bool>);
impl FTNONSTOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTNONSTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTNONSTOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fl_in_prog(&self) -> FLINPROG_R {
        FLINPROG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ft_stopped(&self) -> FTSTOPPED_R {
        FTSTOPPED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tcpresent(&self) -> TCPRESENT_R {
        TCPRESENT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ft_non_stop(&self) -> FTNONSTOP_R {
        FTNONSTOP_R::new(((self.bits >> 3) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
