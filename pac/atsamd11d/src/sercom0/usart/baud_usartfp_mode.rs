#[doc = "Reader of register BAUD_USARTFP_MODE"]
pub type R = crate::R<u16, super::BAUD_USARTFP_MODE>;
#[doc = "Writer for register BAUD_USARTFP_MODE"]
pub type W = crate::W<u16, super::BAUD_USARTFP_MODE>;
#[doc = "Register BAUD_USARTFP_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::BAUD_USARTFP_MODE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BAUD`"]
pub type BAUD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BAUD`"]
pub struct BAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&mut self) -> BAUD_W {
        BAUD_W { w: self }
    }
}
