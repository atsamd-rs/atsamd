#[doc = "Register `ECCERR` reader"]
pub type R = crate::R<ECCERR_SPEC>;
#[doc = "Field `ADDR` reader - Error Address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `TYPEL` reader - Low Double-Word Error Type"]
pub type TYPEL_R = crate::FieldReader<TYPELSELECT_A>;
#[doc = "Low Double-Word Error Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPELSELECT_A {
    #[doc = "0: No Error Detected Since Last Read"]
    NONE = 0,
    #[doc = "1: At Least One Single Error Detected Since last Read"]
    SINGLE = 1,
    #[doc = "2: At Least One Dual Error Detected Since Last Read"]
    DUAL = 2,
}
impl From<TYPELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TYPELSELECT_A {
    type Ux = u8;
}
impl TYPEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TYPELSELECT_A> {
        match self.bits {
            0 => Some(TYPELSELECT_A::NONE),
            1 => Some(TYPELSELECT_A::SINGLE),
            2 => Some(TYPELSELECT_A::DUAL),
            _ => None,
        }
    }
    #[doc = "No Error Detected Since Last Read"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TYPELSELECT_A::NONE
    }
    #[doc = "At Least One Single Error Detected Since last Read"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TYPELSELECT_A::SINGLE
    }
    #[doc = "At Least One Dual Error Detected Since Last Read"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == TYPELSELECT_A::DUAL
    }
}
#[doc = "Field `TYPEH` reader - High Double-Word Error Type"]
pub type TYPEH_R = crate::FieldReader<TYPEHSELECT_A>;
#[doc = "High Double-Word Error Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPEHSELECT_A {
    #[doc = "0: No Error Detected Since Last Read"]
    NONE = 0,
    #[doc = "1: At Least One Single Error Detected Since last Read"]
    SINGLE = 1,
    #[doc = "2: At Least One Dual Error Detected Since Last Read"]
    DUAL = 2,
}
impl From<TYPEHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPEHSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TYPEHSELECT_A {
    type Ux = u8;
}
impl TYPEH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TYPEHSELECT_A> {
        match self.bits {
            0 => Some(TYPEHSELECT_A::NONE),
            1 => Some(TYPEHSELECT_A::SINGLE),
            2 => Some(TYPEHSELECT_A::DUAL),
            _ => None,
        }
    }
    #[doc = "No Error Detected Since Last Read"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TYPEHSELECT_A::NONE
    }
    #[doc = "At Least One Single Error Detected Since last Read"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TYPEHSELECT_A::SINGLE
    }
    #[doc = "At Least One Dual Error Detected Since Last Read"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == TYPEHSELECT_A::DUAL
    }
}
impl R {
    #[doc = "Bits 0:23 - Error Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 28:29 - Low Double-Word Error Type"]
    #[inline(always)]
    pub fn typel(&self) -> TYPEL_R {
        TYPEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - High Double-Word Error Type"]
    #[inline(always)]
    pub fn typeh(&self) -> TYPEH_R {
        TYPEH_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "ECC Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccerr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCERR_SPEC;
impl crate::RegisterSpec for ECCERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccerr::R`](R) reader structure"]
impl crate::Readable for ECCERR_SPEC {}
#[doc = "`reset()` method sets ECCERR to value 0"]
impl crate::Resettable for ECCERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
