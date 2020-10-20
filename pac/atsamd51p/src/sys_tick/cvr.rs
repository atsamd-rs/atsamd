#[doc = "Reader of register CVR"]
pub type R = crate::R<u32, super::CVR>;
#[doc = "Writer for register CVR"]
pub type W = crate::W<u32, super::CVR>;
#[doc = "Register CVR `reset()`'s with value 0"]
impl crate::ResetValue for super::CVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CURRENT`"]
pub type CURRENT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CURRENT`"]
pub struct CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Current value at the time the register is accessed"]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Current value at the time the register is accessed"]
    #[inline(always)]
    pub fn current(&mut self) -> CURRENT_W {
        CURRENT_W { w: self }
    }
}
