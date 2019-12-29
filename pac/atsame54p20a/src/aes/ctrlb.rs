#[doc = "Reader of register CTRLB"]
pub type R = crate::R<u8, super::CTRLB>;
#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u8, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLB {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Reader of field `NEWMSG`"]
pub type NEWMSG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NEWMSG`"]
pub struct NEWMSG_W<'a> {
    w: &'a mut W,
}
impl<'a> NEWMSG_W<'a> {
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
#[doc = "Reader of field `EOM`"]
pub type EOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOM`"]
pub struct EOM_W<'a> {
    w: &'a mut W,
}
impl<'a> EOM_W<'a> {
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
#[doc = "Reader of field `GFMUL`"]
pub type GFMUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GFMUL`"]
pub struct GFMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> GFMUL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Start Encryption/Decryption"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - New message"]
    #[inline(always)]
    pub fn newmsg(&self) -> NEWMSG_R {
        NEWMSG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of message"]
    #[inline(always)]
    pub fn eom(&self) -> EOM_R {
        EOM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GF Multiplication"]
    #[inline(always)]
    pub fn gfmul(&self) -> GFMUL_R {
        GFMUL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Encryption/Decryption"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - New message"]
    #[inline(always)]
    pub fn newmsg(&mut self) -> NEWMSG_W {
        NEWMSG_W { w: self }
    }
    #[doc = "Bit 2 - End of message"]
    #[inline(always)]
    pub fn eom(&mut self) -> EOM_W {
        EOM_W { w: self }
    }
    #[doc = "Bit 3 - GF Multiplication"]
    #[inline(always)]
    pub fn gfmul(&mut self) -> GFMUL_W {
        GFMUL_W { w: self }
    }
}
