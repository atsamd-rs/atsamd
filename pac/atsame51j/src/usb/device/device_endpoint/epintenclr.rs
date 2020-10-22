#[doc = "Reader of register EPINTENCLR"]
pub type R = crate::R<u8, super::EPINTENCLR>;
#[doc = "Writer for register EPINTENCLR"]
pub type W = crate::W<u8, super::EPINTENCLR>;
#[doc = "Register EPINTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::EPINTENCLR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRCPT0`"]
pub type TRCPT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCPT0`"]
pub struct TRCPT0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCPT0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TRCPT1`"]
pub type TRCPT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCPT1`"]
pub struct TRCPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCPT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TRFAIL0`"]
pub type TRFAIL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRFAIL0`"]
pub struct TRFAIL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRFAIL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TRFAIL1`"]
pub type TRFAIL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRFAIL1`"]
pub struct TRFAIL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRFAIL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RXSTP`"]
pub type RXSTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXSTP`"]
pub struct RXSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `STALL0`"]
pub type STALL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL0`"]
pub struct STALL0_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `STALL1`"]
pub type STALL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL1`"]
pub struct STALL1_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt0(&self) -> TRCPT0_R {
        TRCPT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt1(&self) -> TRCPT1_R {
        TRCPT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail0(&self) -> TRFAIL0_R {
        TRFAIL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail1(&self) -> TRFAIL1_R {
        TRFAIL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Received Setup Interrupt Disable"]
    #[inline(always)]
    pub fn rxstp(&self) -> RXSTP_R {
        RXSTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stall 0 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall0(&self) -> STALL0_R {
        STALL0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stall 1 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall1(&self) -> STALL1_R {
        STALL1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt0(&mut self) -> TRCPT0_W {
        TRCPT0_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt1(&mut self) -> TRCPT1_W {
        TRCPT1_W { w: self }
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail0(&mut self) -> TRFAIL0_W {
        TRFAIL0_W { w: self }
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail1(&mut self) -> TRFAIL1_W {
        TRFAIL1_W { w: self }
    }
    #[doc = "Bit 4 - Received Setup Interrupt Disable"]
    #[inline(always)]
    pub fn rxstp(&mut self) -> RXSTP_W {
        RXSTP_W { w: self }
    }
    #[doc = "Bit 5 - Stall 0 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall0(&mut self) -> STALL0_W {
        STALL0_W { w: self }
    }
    #[doc = "Bit 6 - Stall 1 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall1(&mut self) -> STALL1_W {
        STALL1_W { w: self }
    }
}
