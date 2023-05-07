#[doc = "Register `MCCAR` reader"]
pub struct R(crate::R<MCCAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCCAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCCAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCCAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAXCUR33V` reader - Maximum Current for 3.3V"]
pub type MAXCUR33V_R = crate::FieldReader<u8, MAXCUR33VSELECT_A>;
#[doc = "Maximum Current for 3.3V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAXCUR33VSELECT_A {
    #[doc = "0: Get information via another method"]
    OTHER = 0,
    #[doc = "1: 4mA"]
    _4MA = 1,
    #[doc = "2: 8mA"]
    _8MA = 2,
    #[doc = "3: 12mA"]
    _12MA = 3,
}
impl From<MAXCUR33VSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXCUR33VSELECT_A) -> Self {
        variant as _
    }
}
impl MAXCUR33V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAXCUR33VSELECT_A> {
        match self.bits {
            0 => Some(MAXCUR33VSELECT_A::OTHER),
            1 => Some(MAXCUR33VSELECT_A::_4MA),
            2 => Some(MAXCUR33VSELECT_A::_8MA),
            3 => Some(MAXCUR33VSELECT_A::_12MA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == MAXCUR33VSELECT_A::OTHER
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == MAXCUR33VSELECT_A::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline(always)]
    pub fn is_8ma(&self) -> bool {
        *self == MAXCUR33VSELECT_A::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline(always)]
    pub fn is_12ma(&self) -> bool {
        *self == MAXCUR33VSELECT_A::_12MA
    }
}
#[doc = "Field `MAXCUR30V` reader - Maximum Current for 3.0V"]
pub type MAXCUR30V_R = crate::FieldReader<u8, MAXCUR30VSELECT_A>;
#[doc = "Maximum Current for 3.0V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAXCUR30VSELECT_A {
    #[doc = "0: Get information via another method"]
    OTHER = 0,
    #[doc = "1: 4mA"]
    _4MA = 1,
    #[doc = "2: 8mA"]
    _8MA = 2,
    #[doc = "3: 12mA"]
    _12MA = 3,
}
impl From<MAXCUR30VSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXCUR30VSELECT_A) -> Self {
        variant as _
    }
}
impl MAXCUR30V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAXCUR30VSELECT_A> {
        match self.bits {
            0 => Some(MAXCUR30VSELECT_A::OTHER),
            1 => Some(MAXCUR30VSELECT_A::_4MA),
            2 => Some(MAXCUR30VSELECT_A::_8MA),
            3 => Some(MAXCUR30VSELECT_A::_12MA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == MAXCUR30VSELECT_A::OTHER
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == MAXCUR30VSELECT_A::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline(always)]
    pub fn is_8ma(&self) -> bool {
        *self == MAXCUR30VSELECT_A::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline(always)]
    pub fn is_12ma(&self) -> bool {
        *self == MAXCUR30VSELECT_A::_12MA
    }
}
#[doc = "Field `MAXCUR18V` reader - Maximum Current for 1.8V"]
pub type MAXCUR18V_R = crate::FieldReader<u8, MAXCUR18VSELECT_A>;
#[doc = "Maximum Current for 1.8V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAXCUR18VSELECT_A {
    #[doc = "0: Get information via another method"]
    OTHER = 0,
    #[doc = "1: 4mA"]
    _4MA = 1,
    #[doc = "2: 8mA"]
    _8MA = 2,
    #[doc = "3: 12mA"]
    _12MA = 3,
}
impl From<MAXCUR18VSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXCUR18VSELECT_A) -> Self {
        variant as _
    }
}
impl MAXCUR18V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAXCUR18VSELECT_A> {
        match self.bits {
            0 => Some(MAXCUR18VSELECT_A::OTHER),
            1 => Some(MAXCUR18VSELECT_A::_4MA),
            2 => Some(MAXCUR18VSELECT_A::_8MA),
            3 => Some(MAXCUR18VSELECT_A::_12MA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == MAXCUR18VSELECT_A::OTHER
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == MAXCUR18VSELECT_A::_4MA
    }
    #[doc = "Checks if the value of the field is `_8MA`"]
    #[inline(always)]
    pub fn is_8ma(&self) -> bool {
        *self == MAXCUR18VSELECT_A::_8MA
    }
    #[doc = "Checks if the value of the field is `_12MA`"]
    #[inline(always)]
    pub fn is_12ma(&self) -> bool {
        *self == MAXCUR18VSELECT_A::_12MA
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline(always)]
    pub fn maxcur33v(&self) -> MAXCUR33V_R {
        MAXCUR33V_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V"]
    #[inline(always)]
    pub fn maxcur30v(&self) -> MAXCUR30V_R {
        MAXCUR30V_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V"]
    #[inline(always)]
    pub fn maxcur18v(&self) -> MAXCUR18V_R {
        MAXCUR18V_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Maximum Current Capabilities\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mccar](index.html) module"]
pub struct MCCAR_SPEC;
impl crate::RegisterSpec for MCCAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mccar::R](R) reader structure"]
impl crate::Readable for MCCAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCCAR to value 0"]
impl crate::Resettable for MCCAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
