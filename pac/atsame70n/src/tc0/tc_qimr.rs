#[doc = "Register `TC_QIMR` reader"]
pub struct R(crate::R<TC_QIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_QIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_QIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_QIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDX` reader - Index"]
pub struct IDX_R(crate::FieldReader<bool, bool>);
impl IDX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRCHG` reader - Direction Change"]
pub struct DIRCHG_R(crate::FieldReader<bool, bool>);
impl DIRCHG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIRCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QERR` reader - Quadrature Error"]
pub struct QERR_R(crate::FieldReader<bool, bool>);
impl QERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPE` reader - Consecutive Missing Pulse Error"]
pub struct MPE_R(crate::FieldReader<bool, bool>);
impl MPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    pub fn qerr(&self) -> QERR_R {
        QERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Consecutive Missing Pulse Error"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "QDEC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_qimr](index.html) module"]
pub struct TC_QIMR_SPEC;
impl crate::RegisterSpec for TC_QIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc_qimr::R](R) reader structure"]
impl crate::Readable for TC_QIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TC_QIMR to value 0"]
impl crate::Resettable for TC_QIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
