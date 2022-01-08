#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Speed Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: Full-speed mode"]
    FS = 0,
    #[doc = "1: High-speed mode"]
    HS = 1,
    #[doc = "2: Low-speed mode"]
    LS = 2,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPEED` reader - Speed Status"]
pub struct SPEED_R(crate::FieldReader<u8, SPEED_A>);
impl SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPEED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEED_A> {
        match self.bits {
            0 => Some(SPEED_A::FS),
            1 => Some(SPEED_A::HS),
            2 => Some(SPEED_A::LS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        **self == SPEED_A::FS
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        **self == SPEED_A::HS
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        **self == SPEED_A::LS
    }
}
impl core::ops::Deref for SPEED_R {
    type Target = crate::FieldReader<u8, SPEED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB Line State Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINESTATE_A {
    #[doc = "0: SE0/RESET"]
    _0 = 0,
    #[doc = "1: FS-J or LS-K State"]
    _1 = 1,
    #[doc = "2: FS-K or LS-J State"]
    _2 = 2,
}
impl From<LINESTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: LINESTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LINESTATE` reader - USB Line State Status"]
pub struct LINESTATE_R(crate::FieldReader<u8, LINESTATE_A>);
impl LINESTATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LINESTATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINESTATE_A> {
        match self.bits {
            0 => Some(LINESTATE_A::_0),
            1 => Some(LINESTATE_A::_1),
            2 => Some(LINESTATE_A::_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LINESTATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LINESTATE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == LINESTATE_A::_2
    }
}
impl core::ops::Deref for LINESTATE_R {
    type Target = crate::FieldReader<u8, LINESTATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 2:3 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - USB Line State Status"]
    #[inline(always)]
    pub fn linestate(&self) -> LINESTATE_R {
        LINESTATE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "DEVICE Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x40"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
