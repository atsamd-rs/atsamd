#[doc = "Register `TIMESTAMP` reader"]
pub type R = crate::R<TIMESTAMP_SPEC>;
#[doc = "Field `SECOND` reader - Second Timestamp Value"]
pub type SECOND_R = crate::FieldReader;
#[doc = "Field `MINUTE` reader - Minute Timestamp Value"]
pub type MINUTE_R = crate::FieldReader;
#[doc = "Field `HOUR` reader - Hour Timestamp Value"]
pub type HOUR_R = crate::FieldReader<HOURSELECT_A>;
#[doc = "Hour Timestamp Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HOURSELECT_A {
    #[doc = "0: AM when CLKREP in 12-hour"]
    AM = 0,
    #[doc = "16: PM when CLKREP in 12-hour"]
    PM = 16,
}
impl From<HOURSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: HOURSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HOURSELECT_A {
    type Ux = u8;
}
impl HOUR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HOURSELECT_A> {
        match self.bits {
            0 => Some(HOURSELECT_A::AM),
            16 => Some(HOURSELECT_A::PM),
            _ => None,
        }
    }
    #[doc = "AM when CLKREP in 12-hour"]
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == HOURSELECT_A::AM
    }
    #[doc = "PM when CLKREP in 12-hour"]
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == HOURSELECT_A::PM
    }
}
#[doc = "Field `DAY` reader - Day Timestamp Value"]
pub type DAY_R = crate::FieldReader;
#[doc = "Field `MONTH` reader - Month Timestamp Value"]
pub type MONTH_R = crate::FieldReader;
#[doc = "Field `YEAR` reader - Year Timestamp Value"]
pub type YEAR_R = crate::FieldReader;
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
#[doc = "MODE2 Timestamp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMESTAMP_SPEC;
impl crate::RegisterSpec for TIMESTAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp::R`](R) reader structure"]
impl crate::Readable for TIMESTAMP_SPEC {}
#[doc = "`reset()` method sets TIMESTAMP to value 0"]
impl crate::Resettable for TIMESTAMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
