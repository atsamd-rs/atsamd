#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u32, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u32, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTINTEO`"]
pub type EXTINTEO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EXTINTEO`"]
pub struct EXTINTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - External Interrupt Event Output Enable"]
    #[inline(always)]
    pub fn extinteo(&self) -> EXTINTEO_R {
        EXTINTEO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt Event Output Enable"]
    #[inline(always)]
    pub fn extinteo(&mut self) -> EXTINTEO_W {
        EXTINTEO_W { w: self }
    }
}
