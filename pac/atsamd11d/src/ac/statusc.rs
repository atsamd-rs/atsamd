#[doc = "Register `STATUSC` reader"]
pub struct R(crate::R<STATUSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATE0` reader - Comparator 0 Current State"]
pub struct STATE0_R(crate::FieldReader<bool, bool>);
impl STATE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE1` reader - Comparator 1 Current State"]
pub struct STATE1_R(crate::FieldReader<bool, bool>);
impl STATE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Window 0 Current State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WSTATE0_A {
    #[doc = "0: Signal is above window"]
    ABOVE = 0,
    #[doc = "1: Signal is inside window"]
    INSIDE = 1,
    #[doc = "2: Signal is below window"]
    BELOW = 2,
}
impl From<WSTATE0_A> for u8 {
    #[inline(always)]
    fn from(variant: WSTATE0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WSTATE0` reader - Window 0 Current State"]
pub struct WSTATE0_R(crate::FieldReader<u8, WSTATE0_A>);
impl WSTATE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WSTATE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WSTATE0_A> {
        match self.bits {
            0 => Some(WSTATE0_A::ABOVE),
            1 => Some(WSTATE0_A::INSIDE),
            2 => Some(WSTATE0_A::BELOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        **self == WSTATE0_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        **self == WSTATE0_A::INSIDE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        **self == WSTATE0_A::BELOW
    }
}
impl core::ops::Deref for WSTATE0_R {
    type Target = crate::FieldReader<u8, WSTATE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 0 Current State"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Current State"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Window 0 Current State"]
    #[inline(always)]
    pub fn wstate0(&self) -> WSTATE0_R {
        WSTATE0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
#[doc = "Status C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusc](index.html) module"]
pub struct STATUSC_SPEC;
impl crate::RegisterSpec for STATUSC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [statusc::R](R) reader structure"]
impl crate::Readable for STATUSC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for STATUSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
