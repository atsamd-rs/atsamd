#[doc = "Writer for register PSTATUSSET"]
pub type W = crate::W<u8, super::PSTATUSSET>;
#[doc = "Register PSTATUSSET `reset()`'s with value 0"]
impl crate::ResetValue for super::PSTATUSSET {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DTGL`"]
pub struct DTGL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGL_W<'a> {
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
#[doc = "Write proxy for field `CURBK`"]
pub struct CURBK_W<'a> {
    w: &'a mut W,
}
impl<'a> CURBK_W<'a> {
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
#[doc = "Write proxy for field `PFREEZE`"]
pub struct PFREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PFREEZE_W<'a> {
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
#[doc = "Write proxy for field `BK0RDY`"]
pub struct BK0RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BK0RDY_W<'a> {
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
#[doc = "Write proxy for field `BK1RDY`"]
pub struct BK1RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BK1RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Data Toggle Set"]
    #[inline(always)]
    pub fn dtgl(&mut self) -> DTGL_W {
        DTGL_W { w: self }
    }
    #[doc = "Bit 2 - Current Bank Set"]
    #[inline(always)]
    pub fn curbk(&mut self) -> CURBK_W {
        CURBK_W { w: self }
    }
    #[doc = "Bit 4 - Pipe Freeze Set"]
    #[inline(always)]
    pub fn pfreeze(&mut self) -> PFREEZE_W {
        PFREEZE_W { w: self }
    }
    #[doc = "Bit 6 - Bank 0 Ready Set"]
    #[inline(always)]
    pub fn bk0rdy(&mut self) -> BK0RDY_W {
        BK0RDY_W { w: self }
    }
    #[doc = "Bit 7 - Bank 1 Ready Set"]
    #[inline(always)]
    pub fn bk1rdy(&mut self) -> BK1RDY_W {
        BK1RDY_W { w: self }
    }
}
