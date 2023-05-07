#[doc = "Register `AESR` reader"]
pub struct R(crate::R<AESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERRST` reader - ADMA Error State"]
pub type ERRST_R = crate::FieldReader<u8, ERRSTSELECT_A>;
#[doc = "ADMA Error State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERRSTSELECT_A {
    #[doc = "0: ST_STOP (Stop DMA)"]
    STOP = 0,
    #[doc = "1: ST_FDS (Fetch Descriptor)"]
    FDS = 1,
    #[doc = "3: ST_TFR (Transfer Data)"]
    TFR = 3,
}
impl From<ERRSTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ERRSTSELECT_A) -> Self {
        variant as _
    }
}
impl ERRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERRSTSELECT_A> {
        match self.bits {
            0 => Some(ERRSTSELECT_A::STOP),
            1 => Some(ERRSTSELECT_A::FDS),
            3 => Some(ERRSTSELECT_A::TFR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == ERRSTSELECT_A::STOP
    }
    #[doc = "Checks if the value of the field is `FDS`"]
    #[inline(always)]
    pub fn is_fds(&self) -> bool {
        *self == ERRSTSELECT_A::FDS
    }
    #[doc = "Checks if the value of the field is `TFR`"]
    #[inline(always)]
    pub fn is_tfr(&self) -> bool {
        *self == ERRSTSELECT_A::TFR
    }
}
#[doc = "Field `LMIS` reader - ADMA Length Mismatch Error"]
pub type LMIS_R = crate::BitReader<LMISSELECT_A>;
#[doc = "ADMA Length Mismatch Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LMISSELECT_A {
    #[doc = "0: No Error"]
    NO = 0,
    #[doc = "1: Error"]
    YES = 1,
}
impl From<LMISSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LMISSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LMIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMISSELECT_A {
        match self.bits {
            false => LMISSELECT_A::NO,
            true => LMISSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == LMISSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == LMISSELECT_A::YES
    }
}
impl R {
    #[doc = "Bits 0:1 - ADMA Error State"]
    #[inline(always)]
    pub fn errst(&self) -> ERRST_R {
        ERRST_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn lmis(&self) -> LMIS_R {
        LMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ADMA Error Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesr](index.html) module"]
pub struct AESR_SPEC;
impl crate::RegisterSpec for AESR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [aesr::R](R) reader structure"]
impl crate::Readable for AESR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AESR to value 0"]
impl crate::Resettable for AESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
