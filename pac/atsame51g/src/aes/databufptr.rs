#[doc = "Reader of register DATABUFPTR"]
pub type R = crate::R<u8, super::DATABUFPTR>;
#[doc = "Writer for register DATABUFPTR"]
pub type W = crate::W<u8, super::DATABUFPTR>;
#[doc = "Register DATABUFPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::DATABUFPTR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INDATAPTR`"]
pub type INDATAPTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INDATAPTR`"]
pub struct INDATAPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INDATAPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Data Pointer"]
    #[inline(always)]
    pub fn indataptr(&self) -> INDATAPTR_R {
        INDATAPTR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Data Pointer"]
    #[inline(always)]
    pub fn indataptr(&mut self) -> INDATAPTR_W {
        INDATAPTR_W { w: self }
    }
}
