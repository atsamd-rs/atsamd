#[doc = "Reader of register TSR"]
pub type R = crate::R<u32, super::TSR>;
#[doc = "Writer for register TSR"]
pub type W = crate::W<u32, super::TSR>;
#[doc = "Register TSR `reset()`'s with value 0"]
impl crate::ResetValue for super::TSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UBR`"]
pub type UBR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UBR`"]
pub struct UBR_W<'a> {
    w: &'a mut W,
}
impl<'a> UBR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `COL`"]
pub type COL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COL`"]
pub struct COL_W<'a> {
    w: &'a mut W,
}
impl<'a> COL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RLE`"]
pub type RLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RLE`"]
pub struct RLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TXGO`"]
pub type TXGO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXGO`"]
pub struct TXGO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TFC`"]
pub type TFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFC`"]
pub struct TFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TXCOMP`"]
pub type TXCOMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCOMP`"]
pub struct TXCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCOMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `UND`"]
pub type UND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UND`"]
pub struct UND_W<'a> {
    w: &'a mut W,
}
impl<'a> UND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `HRESP`"]
pub type HRESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HRESP`"]
pub struct HRESP_W<'a> {
    w: &'a mut W,
}
impl<'a> HRESP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    pub fn ubr(&self) -> UBR_R {
        UBR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rle(&self) -> RLE_R {
        RLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    pub fn txgo(&self) -> TXGO_R {
        TXGO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn und(&self) -> UND_R {
        UND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    pub fn ubr(&mut self) -> UBR_W {
        UBR_W { w: self }
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    pub fn col(&mut self) -> COL_W {
        COL_W { w: self }
    }
    #[doc = "Bit 2 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rle(&mut self) -> RLE_W {
        RLE_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    pub fn txgo(&mut self) -> TXGO_W {
        TXGO_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&mut self) -> TFC_W {
        TFC_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    pub fn txcomp(&mut self) -> TXCOMP_W {
        TXCOMP_W { w: self }
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn und(&mut self) -> UND_W {
        UND_W { w: self }
    }
    #[doc = "Bit 8 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&mut self) -> HRESP_W {
        HRESP_W { w: self }
    }
}
