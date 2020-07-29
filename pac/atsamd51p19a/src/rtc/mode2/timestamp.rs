#[doc = "Reader of register TIMESTAMP"]
pub type R = crate::R<u32, super::TIMESTAMP>;
#[doc = "Reader of field `SECOND`"]
pub type SECOND_R = crate::R<u8, u8>;
#[doc = "Reader of field `MINUTE`"]
pub type MINUTE_R = crate::R<u8, u8>;
#[doc = "Hour Timestamp Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HOUR_A {
    #[doc = "0: AM when CLKREP in 12-hour"]
    AM = 0,
    #[doc = "16: PM when CLKREP in 12-hour"]
    PM = 16,
}
impl From<HOUR_A> for u8 {
    #[inline(always)]
    fn from(variant: HOUR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HOUR`"]
pub type HOUR_R = crate::R<u8, HOUR_A>;
impl HOUR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HOUR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HOUR_A::AM),
            16 => Val(HOUR_A::PM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AM`"]
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == HOUR_A::AM
    }
    #[doc = "Checks if the value of the field is `PM`"]
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == HOUR_A::PM
    }
}
#[doc = "Reader of field `DAY`"]
pub type DAY_R = crate::R<u8, u8>;
#[doc = "Reader of field `MONTH`"]
pub type MONTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `YEAR`"]
pub type YEAR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Second Timestamp Value"]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Minute Timestamp Value"]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:16 - Hour Timestamp Value"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Day Timestamp Value"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:25 - Month Timestamp Value"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:31 - Year Timestamp Value"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
