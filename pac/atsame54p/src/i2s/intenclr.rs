#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u16, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u16, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXRDY0`"]
pub type RXRDY0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXRDY0`"]
pub struct RXRDY0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRDY0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RXRDY1`"]
pub type RXRDY1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXRDY1`"]
pub struct RXRDY1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRDY1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RXOR0`"]
pub type RXOR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOR0`"]
pub struct RXOR0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RXOR1`"]
pub type RXOR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOR1`"]
pub struct RXOR1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TXRDY0`"]
pub type TXRDY0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRDY0`"]
pub struct TXRDY0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXRDY1`"]
pub type TXRDY1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRDY1`"]
pub struct TXRDY1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TXUR0`"]
pub type TXUR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUR0`"]
pub struct TXUR0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TXUR1`"]
pub type TXUR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUR1`"]
pub struct TXUR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy0(&self) -> RXRDY0_R {
        RXRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy1(&self) -> RXRDY1_R {
        RXRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor0(&self) -> RXOR0_R {
        RXOR0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor1(&self) -> RXOR1_R {
        RXOR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy0(&self) -> TXRDY0_R {
        TXRDY0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy1(&self) -> TXRDY1_R {
        TXRDY1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txur0(&self) -> TXUR0_R {
        TXUR0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txur1(&self) -> TXUR1_R {
        TXUR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy0(&mut self) -> RXRDY0_W {
        RXRDY0_W { w: self }
    }
    #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy1(&mut self) -> RXRDY1_W {
        RXRDY1_W { w: self }
    }
    #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor0(&mut self) -> RXOR0_W {
        RXOR0_W { w: self }
    }
    #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor1(&mut self) -> RXOR1_W {
        RXOR1_W { w: self }
    }
    #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy0(&mut self) -> TXRDY0_W {
        TXRDY0_W { w: self }
    }
    #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy1(&mut self) -> TXRDY1_W {
        TXRDY1_W { w: self }
    }
    #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txur0(&mut self) -> TXUR0_W {
        TXUR0_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txur1(&mut self) -> TXUR1_W {
        TXUR1_W { w: self }
    }
}
