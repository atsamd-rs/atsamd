#[doc = "Reader of register AESR"]
pub type R = crate::R<u8, super::AESR>;
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
#[doc = "Reader of field `ERRST`"]
pub type ERRST_R = crate::R<u8, ERRST_A>;
impl ERRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ERRST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ERRST_A::STOP),
            1 => Val(ERRST_A::FDS),
            3 => Val(ERRST_A::TFR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == ERRST_A::STOP
    }
    #[doc = "Checks if the value of the field is `FDS`"]
    #[inline(always)]
    pub fn is_fds(&self) -> bool {
        *self == ERRST_A::FDS
    }
    #[doc = "Checks if the value of the field is `TFR`"]
    #[inline(always)]
    pub fn is_tfr(&self) -> bool {
        *self == ERRST_A::TFR
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
#[doc = "Reader of field `LMIS`"]
pub type LMIS_R = crate::R<bool, LMIS_A>;
impl LMIS_R {
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
        *self == LMIS_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == LMIS_A::YES
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
