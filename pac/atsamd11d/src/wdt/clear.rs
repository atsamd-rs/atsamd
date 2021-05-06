#[doc = "Writer for register CLEAR"]
pub type W = crate::W<u8, super::CLEAR>;
#[doc = "Register CLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLEAR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Watchdog Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLEAR_AW {
    #[doc = "165: Clear Key"]
    KEY = 165,
}
impl From<CLEAR_AW> for u8 {
    #[inline(always)]
    fn from(variant: CLEAR_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `CLEAR`"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLEAR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clear Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CLEAR_AW::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog Clear"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
}
