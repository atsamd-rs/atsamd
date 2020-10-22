#[doc = "Reader of register TSH"]
pub type R = crate::R<u32, super::TSH>;
#[doc = "Writer for register TSH"]
pub type W = crate::W<u32, super::TSH>;
#[doc = "Register TSH `reset()`'s with value 0"]
impl crate::ResetValue for super::TSH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCS`"]
pub type TCS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TCS`"]
pub struct TCS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Count in Seconds"]
    #[inline(always)]
    pub fn tcs(&self) -> TCS_R {
        TCS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Count in Seconds"]
    #[inline(always)]
    pub fn tcs(&mut self) -> TCS_W {
        TCS_W { w: self }
    }
}
