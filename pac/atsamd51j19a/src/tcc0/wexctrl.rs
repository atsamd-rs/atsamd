#[doc = "Reader of register WEXCTRL"]
pub type R = crate::R<u32, super::WEXCTRL>;
#[doc = "Writer for register WEXCTRL"]
pub type W = crate::W<u32, super::WEXCTRL>;
#[doc = "Register WEXCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WEXCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OTMX`"]
pub type OTMX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OTMX`"]
pub struct OTMX_W<'a> {
    w: &'a mut W,
}
impl<'a> OTMX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DTIEN0`"]
pub type DTIEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTIEN0`"]
pub struct DTIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIEN0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DTIEN1`"]
pub type DTIEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTIEN1`"]
pub struct DTIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIEN1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DTIEN2`"]
pub type DTIEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTIEN2`"]
pub struct DTIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIEN2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DTIEN3`"]
pub type DTIEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTIEN3`"]
pub struct DTIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIEN3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DTLS`"]
pub type DTLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTLS`"]
pub struct DTLS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DTHS`"]
pub type DTHS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTHS`"]
pub struct DTHS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline(always)]
    pub fn otmx(&self) -> OTMX_R {
        OTMX_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
    #[inline(always)]
    pub fn dtien0(&self) -> DTIEN0_R {
        DTIEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
    #[inline(always)]
    pub fn dtien1(&self) -> DTIEN1_R {
        DTIEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
    #[inline(always)]
    pub fn dtien2(&self) -> DTIEN2_R {
        DTIEN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
    #[inline(always)]
    pub fn dtien3(&self) -> DTIEN3_R {
        DTIEN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
    #[inline(always)]
    pub fn dtls(&self) -> DTLS_R {
        DTLS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
    #[inline(always)]
    pub fn dths(&self) -> DTHS_R {
        DTHS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline(always)]
    pub fn otmx(&mut self) -> OTMX_W {
        OTMX_W { w: self }
    }
    #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
    #[inline(always)]
    pub fn dtien0(&mut self) -> DTIEN0_W {
        DTIEN0_W { w: self }
    }
    #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
    #[inline(always)]
    pub fn dtien1(&mut self) -> DTIEN1_W {
        DTIEN1_W { w: self }
    }
    #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
    #[inline(always)]
    pub fn dtien2(&mut self) -> DTIEN2_W {
        DTIEN2_W { w: self }
    }
    #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
    #[inline(always)]
    pub fn dtien3(&mut self) -> DTIEN3_W {
        DTIEN3_W { w: self }
    }
    #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
    #[inline(always)]
    pub fn dtls(&mut self) -> DTLS_W {
        DTLS_W { w: self }
    }
    #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
    #[inline(always)]
    pub fn dths(&mut self) -> DTHS_W {
        DTHS_W { w: self }
    }
}
