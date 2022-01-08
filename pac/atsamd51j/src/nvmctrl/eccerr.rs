#[doc = "Register `ECCERR` reader"]
pub struct R(crate::R<ECCERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Error Address"]
pub struct ADDR_R(crate::FieldReader<u32, u32>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Low Double-Word Error Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPEL_A {
    #[doc = "0: No Error Detected Since Last Read"]
    NONE = 0,
    #[doc = "1: At Least One Single Error Detected Since last Read"]
    SINGLE = 1,
    #[doc = "2: At Least One Dual Error Detected Since Last Read"]
    DUAL = 2,
}
impl From<TYPEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TYPEL` reader - Low Double-Word Error Type"]
pub struct TYPEL_R(crate::FieldReader<u8, TYPEL_A>);
impl TYPEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TYPEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TYPEL_A> {
        match self.bits {
            0 => Some(TYPEL_A::NONE),
            1 => Some(TYPEL_A::SINGLE),
            2 => Some(TYPEL_A::DUAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == TYPEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == TYPEL_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        **self == TYPEL_A::DUAL
    }
}
impl core::ops::Deref for TYPEL_R {
    type Target = crate::FieldReader<u8, TYPEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "High Double-Word Error Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPEH_A {
    #[doc = "0: No Error Detected Since Last Read"]
    NONE = 0,
    #[doc = "1: At Least One Single Error Detected Since last Read"]
    SINGLE = 1,
    #[doc = "2: At Least One Dual Error Detected Since Last Read"]
    DUAL = 2,
}
impl From<TYPEH_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPEH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TYPEH` reader - High Double-Word Error Type"]
pub struct TYPEH_R(crate::FieldReader<u8, TYPEH_A>);
impl TYPEH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TYPEH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TYPEH_A> {
        match self.bits {
            0 => Some(TYPEH_A::NONE),
            1 => Some(TYPEH_A::SINGLE),
            2 => Some(TYPEH_A::DUAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == TYPEH_A::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == TYPEH_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        **self == TYPEH_A::DUAL
    }
}
impl core::ops::Deref for TYPEH_R {
    type Target = crate::FieldReader<u8, TYPEH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Error Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 28:29 - Low Double-Word Error Type"]
    #[inline(always)]
    pub fn typel(&self) -> TYPEL_R {
        TYPEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - High Double-Word Error Type"]
    #[inline(always)]
    pub fn typeh(&self) -> TYPEH_R {
        TYPEH_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
#[doc = "ECC Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccerr](index.html) module"]
pub struct ECCERR_SPEC;
impl crate::RegisterSpec for ECCERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccerr::R](R) reader structure"]
impl crate::Readable for ECCERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECCERR to value 0"]
impl crate::Resettable for ECCERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
