#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0x0411"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0411
    }
}
#[doc = "Reader of field `ETMPD`"]
pub type ETMPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETMPD`"]
pub struct ETMPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMPD_W<'a> {
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
#[doc = "Reader of field `PORTSIZE`"]
pub type PORTSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PORTSIZE`"]
pub struct PORTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `BROUT`"]
pub type BROUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BROUT`"]
pub struct BROUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BROUT_W<'a> {
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
#[doc = "Reader of field `DBGRQ`"]
pub type DBGRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGRQ`"]
pub struct DBGRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRQ_W<'a> {
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
#[doc = "Reader of field `PROG`"]
pub type PROG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG`"]
pub struct PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_W<'a> {
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
#[doc = "Reader of field `PORTSEL`"]
pub type PORTSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTSEL`"]
pub struct PORTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTSEL_W<'a> {
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
#[doc = "Reader of field `PORTMODE2`"]
pub type PORTMODE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTMODE2`"]
pub struct PORTMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTMODE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PORTMODE`"]
pub type PORTMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PORTMODE`"]
pub struct PORTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PORTSIZE3`"]
pub type PORTSIZE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTSIZE3`"]
pub struct PORTSIZE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTSIZE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TSEN`"]
pub type TSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSEN`"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ETM Power Down"]
    #[inline(always)]
    pub fn etmpd(&self) -> ETMPD_R {
        ETMPD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Port Size bits 2:0"]
    #[inline(always)]
    pub fn portsize(&self) -> PORTSIZE_R {
        PORTSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn brout(&self) -> BROUT_R {
        BROUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgrq(&self) -> DBGRQ_R {
        DBGRQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ETM Port Select"]
    #[inline(always)]
    pub fn portsel(&self) -> PORTSEL_R {
        PORTSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Mode bit 2"]
    #[inline(always)]
    pub fn portmode2(&self) -> PORTMODE2_R {
        PORTMODE2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Port Mode bits 1:0"]
    #[inline(always)]
    pub fn portmode(&self) -> PORTMODE_R {
        PORTMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Port Size bit 3"]
    #[inline(always)]
    pub fn portsize3(&self) -> PORTSIZE3_R {
        PORTSIZE3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TimeStamp Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETM Power Down"]
    #[inline(always)]
    pub fn etmpd(&mut self) -> ETMPD_W {
        ETMPD_W { w: self }
    }
    #[doc = "Bits 4:6 - Port Size bits 2:0"]
    #[inline(always)]
    pub fn portsize(&mut self) -> PORTSIZE_W {
        PORTSIZE_W { w: self }
    }
    #[doc = "Bit 7 - Stall Processor"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 8 - Branch Output"]
    #[inline(always)]
    pub fn brout(&mut self) -> BROUT_W {
        BROUT_W { w: self }
    }
    #[doc = "Bit 9 - Debug Request Control"]
    #[inline(always)]
    pub fn dbgrq(&mut self) -> DBGRQ_W {
        DBGRQ_W { w: self }
    }
    #[doc = "Bit 10 - ETM Programming"]
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W {
        PROG_W { w: self }
    }
    #[doc = "Bit 11 - ETM Port Select"]
    #[inline(always)]
    pub fn portsel(&mut self) -> PORTSEL_W {
        PORTSEL_W { w: self }
    }
    #[doc = "Bit 13 - Port Mode bit 2"]
    #[inline(always)]
    pub fn portmode2(&mut self) -> PORTMODE2_W {
        PORTMODE2_W { w: self }
    }
    #[doc = "Bits 16:17 - Port Mode bits 1:0"]
    #[inline(always)]
    pub fn portmode(&mut self) -> PORTMODE_W {
        PORTMODE_W { w: self }
    }
    #[doc = "Bit 21 - Port Size bit 3"]
    #[inline(always)]
    pub fn portsize3(&mut self) -> PORTSIZE3_W {
        PORTSIZE3_W { w: self }
    }
    #[doc = "Bit 28 - TimeStamp Enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
}
