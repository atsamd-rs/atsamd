#[doc = "Reader of register BAUD"]
pub type R = crate::R<u32, super::BAUD>;
#[doc = "Writer for register BAUD"]
pub type W = crate::W<u32, super::BAUD>;
#[doc = "Register BAUD `reset()`'s with value 0"]
impl crate::ResetValue for super::BAUD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BAUD`"]
pub type BAUD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BAUD`"]
pub struct BAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BAUDLOW`"]
pub type BAUDLOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BAUDLOW`"]
pub struct BAUDLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUDLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HSBAUD`"]
pub type HSBAUD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSBAUD`"]
pub struct HSBAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> HSBAUD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HSBAUDLOW`"]
pub type HSBAUDLOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSBAUDLOW`"]
pub struct HSBAUDLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSBAUDLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Baud Rate Value Low"]
    #[inline(always)]
    pub fn baudlow(&self) -> BAUDLOW_R {
        BAUDLOW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - High Speed Baud Rate Value"]
    #[inline(always)]
    pub fn hsbaud(&self) -> HSBAUD_R {
        HSBAUD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - High Speed Baud Rate Value Low"]
    #[inline(always)]
    pub fn hsbaudlow(&self) -> HSBAUDLOW_R {
        HSBAUDLOW_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&mut self) -> BAUD_W {
        BAUD_W { w: self }
    }
    #[doc = "Bits 8:15 - Baud Rate Value Low"]
    #[inline(always)]
    pub fn baudlow(&mut self) -> BAUDLOW_W {
        BAUDLOW_W { w: self }
    }
    #[doc = "Bits 16:23 - High Speed Baud Rate Value"]
    #[inline(always)]
    pub fn hsbaud(&mut self) -> HSBAUD_W {
        HSBAUD_W { w: self }
    }
    #[doc = "Bits 24:31 - High Speed Baud Rate Value Low"]
    #[inline(always)]
    pub fn hsbaudlow(&mut self) -> HSBAUDLOW_W {
        HSBAUDLOW_W { w: self }
    }
}
