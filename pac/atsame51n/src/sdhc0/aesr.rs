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
#[doc = "ADMA Error State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERRST_A {
    #[doc = "0: ST_STOP (Stop DMA)"]
    STOP = 0,
    #[doc = "1: ST_FDS (Fetch Descriptor)"]
    FDS = 1,
    #[doc = "3: ST_TFR (Transfer Data)"]
    TFR = 3,
}
impl From<ERRST_A> for u8 {
    #[inline(always)]
    fn from(variant: ERRST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ERRST` reader - ADMA Error State"]
pub struct ERRST_R(crate::FieldReader<u8, ERRST_A>);
impl ERRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERRST_A> {
        match self.bits {
            0 => Some(ERRST_A::STOP),
            1 => Some(ERRST_A::FDS),
            3 => Some(ERRST_A::TFR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == ERRST_A::STOP
    }
    #[doc = "Checks if the value of the field is `FDS`"]
    #[inline(always)]
    pub fn is_fds(&self) -> bool {
        **self == ERRST_A::FDS
    }
    #[doc = "Checks if the value of the field is `TFR`"]
    #[inline(always)]
    pub fn is_tfr(&self) -> bool {
        **self == ERRST_A::TFR
    }
}
impl core::ops::Deref for ERRST_R {
    type Target = crate::FieldReader<u8, ERRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADMA Length Mismatch Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMIS_A {
    #[doc = "0: No Error"]
    NO = 0,
    #[doc = "1: Error"]
    YES = 1,
}
impl From<LMIS_A> for bool {
    #[inline(always)]
    fn from(variant: LMIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LMIS` reader - ADMA Length Mismatch Error"]
pub struct LMIS_R(crate::FieldReader<bool, LMIS_A>);
impl LMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LMIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMIS_A {
        match self.bits {
            false => LMIS_A::NO,
            true => LMIS_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == LMIS_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == LMIS_A::YES
    }
}
impl core::ops::Deref for LMIS_R {
    type Target = crate::FieldReader<bool, LMIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - ADMA Error State"]
    #[inline(always)]
    pub fn errst(&self) -> ERRST_R {
        ERRST_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn lmis(&self) -> LMIS_R {
        LMIS_R::new(((self.bits >> 2) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
