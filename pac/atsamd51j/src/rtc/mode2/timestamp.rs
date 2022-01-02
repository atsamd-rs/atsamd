#[doc = "Register `TIMESTAMP` reader"]
pub struct R(crate::R<TIMESTAMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMESTAMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMESTAMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMESTAMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SECOND` reader - Second Timestamp Value"]
pub struct SECOND_R(crate::FieldReader<u8, u8>);
impl SECOND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SECOND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECOND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINUTE` reader - Minute Timestamp Value"]
pub struct MINUTE_R(crate::FieldReader<u8, u8>);
impl MINUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MINUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `HOUR` reader - Hour Timestamp Value"]
pub struct HOUR_R(crate::FieldReader<u8, HOUR_A>);
impl HOUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOUR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HOUR_A> {
        match self.bits {
            0 => Some(HOUR_A::AM),
            16 => Some(HOUR_A::PM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AM`"]
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        **self == HOUR_A::AM
    }
    #[doc = "Checks if the value of the field is `PM`"]
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        **self == HOUR_A::PM
    }
}
impl core::ops::Deref for HOUR_R {
    type Target = crate::FieldReader<u8, HOUR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY` reader - Day Timestamp Value"]
pub struct DAY_R(crate::FieldReader<u8, u8>);
impl DAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH` reader - Month Timestamp Value"]
pub struct MONTH_R(crate::FieldReader<u8, u8>);
impl MONTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MONTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YEAR` reader - Year Timestamp Value"]
pub struct YEAR_R(crate::FieldReader<u8, u8>);
impl YEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        YEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "MODE2 Timestamp\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timestamp](index.html) module"]
pub struct TIMESTAMP_SPEC;
impl crate::RegisterSpec for TIMESTAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timestamp::R](R) reader structure"]
impl crate::Readable for TIMESTAMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMESTAMP to value 0"]
impl crate::Resettable for TIMESTAMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
