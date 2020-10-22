#[doc = "Reader of register TIDM[%s]"]
pub type R = crate::R<u32, super::TIDM>;
#[doc = "Writer for register TIDM[%s]"]
pub type W = crate::W<u32, super::TIDM>;
#[doc = "Register TIDM[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::TIDM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TID`"]
pub type TID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TID`"]
pub struct TID_W<'a> {
    w: &'a mut W,
}
impl<'a> TID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Type ID Match 1"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 1"]
    #[inline(always)]
    pub fn tid(&mut self) -> TID_W {
        TID_W { w: self }
    }
}
