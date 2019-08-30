#[doc = "Reader of register FNUM"]
pub type R = crate::R<u16, super::FNUM>;
#[doc = "Writer for register FNUM"]
pub type W = crate::W<u16, super::FNUM>;
#[doc = "Register FNUM `reset()`'s with value 0"]
impl crate::ResetValue for super::FNUM {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MFNUM`"]
pub type MFNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MFNUM`"]
pub struct MFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> MFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `FNUM`"]
pub type FNUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FNUM`"]
pub struct FNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> FNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 3)) | (((value as u16) & 0x07ff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&mut self) -> MFNUM_W {
        MFNUM_W { w: self }
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&mut self) -> FNUM_W {
        FNUM_W { w: self }
    }
}
