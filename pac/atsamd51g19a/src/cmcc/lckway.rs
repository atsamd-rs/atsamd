#[doc = "Reader of register LCKWAY"]
pub type R = crate::R<u32, super::LCKWAY>;
#[doc = "Writer for register LCKWAY"]
pub type W = crate::W<u32, super::LCKWAY>;
#[doc = "Register LCKWAY `reset()`'s with value 0"]
impl crate::ResetValue for super::LCKWAY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCKWAY`"]
pub type LCKWAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LCKWAY`"]
pub struct LCKWAY_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKWAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Lockdown way Register"]
    #[inline(always)]
    pub fn lckway(&self) -> LCKWAY_R {
        LCKWAY_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Lockdown way Register"]
    #[inline(always)]
    pub fn lckway(&mut self) -> LCKWAY_W {
        LCKWAY_W { w: self }
    }
}
