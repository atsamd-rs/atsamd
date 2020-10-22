#[doc = "Reader of register MAN"]
pub type R = crate::R<u32, super::MAN>;
#[doc = "Writer for register MAN"]
pub type W = crate::W<u32, super::MAN>;
#[doc = "Register MAN `reset()`'s with value 0"]
impl crate::ResetValue for super::MAN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `WTN`"]
pub type WTN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WTN`"]
pub struct WTN_W<'a> {
    w: &'a mut W,
}
impl<'a> WTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `REGA`"]
pub type REGA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGA`"]
pub struct REGA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `PHYA`"]
pub type PHYA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHYA`"]
pub struct PHYA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 23)) | (((value as u32) & 0x1f) << 23);
        self.w
    }
}
#[doc = "Reader of field `OP`"]
pub type OP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OP`"]
pub struct OP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `CLTTO`"]
pub type CLTTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLTTO`"]
pub struct CLTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `WZO`"]
pub type WZO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WZO`"]
pub struct WZO_W<'a> {
    w: &'a mut W,
}
impl<'a> WZO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PHY Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Write Ten"]
    #[inline(always)]
    pub fn wtn(&self) -> WTN_R {
        WTN_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn rega(&self) -> REGA_R {
        REGA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn phya(&self) -> PHYA_R {
        PHYA_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation"]
    #[inline(always)]
    pub fn op(&self) -> OP_R {
        OP_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Clause 22 Operation"]
    #[inline(always)]
    pub fn cltto(&self) -> CLTTO_R {
        CLTTO_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write ZERO"]
    #[inline(always)]
    pub fn wzo(&self) -> WZO_R {
        WZO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 16:17 - Write Ten"]
    #[inline(always)]
    pub fn wtn(&mut self) -> WTN_W {
        WTN_W { w: self }
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn rega(&mut self) -> REGA_W {
        REGA_W { w: self }
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn phya(&mut self) -> PHYA_W {
        PHYA_W { w: self }
    }
    #[doc = "Bits 28:29 - Operation"]
    #[inline(always)]
    pub fn op(&mut self) -> OP_W {
        OP_W { w: self }
    }
    #[doc = "Bit 30 - Clause 22 Operation"]
    #[inline(always)]
    pub fn cltto(&mut self) -> CLTTO_W {
        CLTTO_W { w: self }
    }
    #[doc = "Bit 31 - Write ZERO"]
    #[inline(always)]
    pub fn wzo(&mut self) -> WZO_W {
        WZO_W { w: self }
    }
}
