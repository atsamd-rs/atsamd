#[doc = "Reader of register BAUD_FRAC_MODE"]
pub type R = crate::R<u16, super::BAUD_FRAC_MODE>;
#[doc = "Writer for register BAUD_FRAC_MODE"]
pub type W = crate::W<u16, super::BAUD_FRAC_MODE>;
#[doc = "Register BAUD_FRAC_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::BAUD_FRAC_MODE {
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
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u16) & 0x1fff);
        self.w
    }
}
#[doc = "Reader of field `FP`"]
pub type FP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FP`"]
pub struct FP_W<'a> {
    w: &'a mut W,
}
impl<'a> FP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u16) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&mut self) -> BAUD_W {
        BAUD_W { w: self }
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&mut self) -> FP_W {
        FP_W { w: self }
    }
}
