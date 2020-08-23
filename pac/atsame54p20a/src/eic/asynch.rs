#[doc = "Reader of register ASYNCH"]
pub type R = crate::R<u32, super::ASYNCH>;
#[doc = "Writer for register ASYNCH"]
pub type W = crate::W<u32, super::ASYNCH>;
#[doc = "Register ASYNCH `reset()`'s with value 0"]
impl crate::ResetValue for super::ASYNCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Asynchronous Edge Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ASYNCH_A {
    #[doc = "0: Edge detection is clock synchronously operated"]
    SYNC = 0,
    #[doc = "1: Edge detection is clock asynchronously operated"]
    ASYNC = 1,
}
impl From<ASYNCH_A> for u16 {
    #[inline(always)]
    fn from(variant: ASYNCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ASYNCH`"]
pub type ASYNCH_R = crate::R<u16, ASYNCH_A>;
impl ASYNCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ASYNCH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ASYNCH_A::SYNC),
            1 => Val(ASYNCH_A::ASYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == ASYNCH_A::SYNC
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async_(&self) -> bool {
        *self == ASYNCH_A::ASYNC
    }
}
#[doc = "Write proxy for field `ASYNCH`"]
pub struct ASYNCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASYNCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(ASYNCH_A::SYNC)
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(ASYNCH_A::ASYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn asynch(&self) -> ASYNCH_R {
        ASYNCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn asynch(&mut self) -> ASYNCH_W {
        ASYNCH_W { w: self }
    }
}
