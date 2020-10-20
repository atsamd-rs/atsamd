#[doc = "Reader of register CPACR"]
pub type R = crate::R<u32, super::CPACR>;
#[doc = "Writer for register CPACR"]
pub type W = crate::W<u32, super::CPACR>;
#[doc = "Register CPACR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Access privileges for coprocessor 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP10_A {
    #[doc = "0: Access denied"]
    DENIED = 0,
    #[doc = "1: Privileged access only"]
    PRIV = 1,
    #[doc = "3: Full access"]
    FULL = 3,
}
impl From<CP10_A> for u8 {
    #[inline(always)]
    fn from(variant: CP10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP10`"]
pub type CP10_R = crate::R<u8, CP10_A>;
impl CP10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP10_A::DENIED),
            1 => Val(CP10_A::PRIV),
            3 => Val(CP10_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DENIED`"]
    #[inline(always)]
    pub fn is_denied(&self) -> bool {
        *self == CP10_A::DENIED
    }
    #[doc = "Checks if the value of the field is `PRIV`"]
    #[inline(always)]
    pub fn is_priv_(&self) -> bool {
        *self == CP10_A::PRIV
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == CP10_A::FULL
    }
}
#[doc = "Write proxy for field `CP10`"]
pub struct CP10_W<'a> {
    w: &'a mut W,
}
impl<'a> CP10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn denied(self) -> &'a mut W {
        self.variant(CP10_A::DENIED)
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn priv_(self) -> &'a mut W {
        self.variant(CP10_A::PRIV)
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(CP10_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP11_A {
    #[doc = "0: Access denied"]
    DENIED = 0,
    #[doc = "1: Privileged access only"]
    PRIV = 1,
    #[doc = "3: Full access"]
    FULL = 3,
}
impl From<CP11_A> for u8 {
    #[inline(always)]
    fn from(variant: CP11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP11`"]
pub type CP11_R = crate::R<u8, CP11_A>;
impl CP11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP11_A::DENIED),
            1 => Val(CP11_A::PRIV),
            3 => Val(CP11_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DENIED`"]
    #[inline(always)]
    pub fn is_denied(&self) -> bool {
        *self == CP11_A::DENIED
    }
    #[doc = "Checks if the value of the field is `PRIV`"]
    #[inline(always)]
    pub fn is_priv_(&self) -> bool {
        *self == CP11_A::PRIV
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == CP11_A::FULL
    }
}
#[doc = "Write proxy for field `CP11`"]
pub struct CP11_W<'a> {
    w: &'a mut W,
}
impl<'a> CP11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn denied(self) -> &'a mut W {
        self.variant(CP11_A::DENIED)
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn priv_(self) -> &'a mut W {
        self.variant(CP11_A::PRIV)
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(CP11_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    pub fn cp10(&mut self) -> CP10_W {
        CP10_W { w: self }
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    pub fn cp11(&mut self) -> CP11_W {
        CP11_W { w: self }
    }
}
