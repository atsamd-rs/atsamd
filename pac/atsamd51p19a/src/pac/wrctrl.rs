#[doc = "Reader of register WRCTRL"]
pub type R = crate::R<u32, super::WRCTRL>;
#[doc = "Writer for register WRCTRL"]
pub type W = crate::W<u32, super::WRCTRL>;
#[doc = "Register WRCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WRCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERID`"]
pub type PERID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PERID`"]
pub struct PERID_W<'a> {
    w: &'a mut W,
}
impl<'a> PERID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Peripheral access control key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "0: No action"]
    OFF = 0,
    #[doc = "1: Clear protection"]
    CLR = 1,
    #[doc = "2: Set protection"]
    SET = 2,
    #[doc = "3: Set and lock protection"]
    SETLCK = 3,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u8, KEY_A>;
impl KEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KEY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KEY_A::OFF),
            1 => Val(KEY_A::CLR),
            2 => Val(KEY_A::SET),
            3 => Val(KEY_A::SETLCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == KEY_A::OFF
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == KEY_A::CLR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == KEY_A::SET
    }
    #[doc = "Checks if the value of the field is `SETLCK`"]
    #[inline(always)]
    pub fn is_setlck(&self) -> bool {
        *self == KEY_A::SETLCK
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(KEY_A::OFF)
    }
    #[doc = "Clear protection"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(KEY_A::CLR)
    }
    #[doc = "Set protection"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(KEY_A::SET)
    }
    #[doc = "Set and lock protection"]
    #[inline(always)]
    pub fn setlck(self) -> &'a mut W {
        self.variant(KEY_A::SETLCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PERID_R {
        PERID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline(always)]
    pub fn perid(&mut self) -> PERID_W {
        PERID_W { w: self }
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
