#[doc = "Reader of register MASTER"]
pub type R = crate::R<u32, super::MASTER>;
#[doc = "Writer for register MASTER"]
pub type W = crate::W<u32, super::MASTER>;
#[doc = "Register MASTER `reset()`'s with value 0"]
impl crate::ResetValue for super::MASTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `TSTARTEN`"]
pub type TSTARTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTARTEN`"]
pub struct TSTARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTARTEN_W<'a> {
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
#[doc = "Reader of field `TSTOPEN`"]
pub type TSTOPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTOPEN`"]
pub struct TSTOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTOPEN_W<'a> {
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
#[doc = "Reader of field `SFRWPRIV`"]
pub type SFRWPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFRWPRIV`"]
pub struct SFRWPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SFRWPRIV_W<'a> {
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
#[doc = "Reader of field `RAMPRIV`"]
pub type RAMPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMPRIV`"]
pub struct RAMPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPRIV_W<'a> {
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
#[doc = "Reader of field `HALTREQ`"]
pub type HALTREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALTREQ`"]
pub struct HALTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTREQ_W<'a> {
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
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bits 0:4 - Maximum Value of the Trace Buffer in SRAM"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Trace Start Input Enable"]
    #[inline(always)]
    pub fn tstarten(&self) -> TSTARTEN_R {
        TSTARTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Trace Stop Input Enable"]
    #[inline(always)]
    pub fn tstopen(&self) -> TSTOPEN_R {
        TSTOPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege"]
    #[inline(always)]
    pub fn sfrwpriv(&self) -> SFRWPRIV_R {
        SFRWPRIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SRAM Privilege"]
    #[inline(always)]
    pub fn rampriv(&self) -> RAMPRIV_R {
        RAMPRIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Halt Request"]
    #[inline(always)]
    pub fn haltreq(&self) -> HALTREQ_R {
        HALTREQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Main Trace Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Maximum Value of the Trace Buffer in SRAM"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bit 5 - Trace Start Input Enable"]
    #[inline(always)]
    pub fn tstarten(&mut self) -> TSTARTEN_W {
        TSTARTEN_W { w: self }
    }
    #[doc = "Bit 6 - Trace Stop Input Enable"]
    #[inline(always)]
    pub fn tstopen(&mut self) -> TSTOPEN_W {
        TSTOPEN_W { w: self }
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege"]
    #[inline(always)]
    pub fn sfrwpriv(&mut self) -> SFRWPRIV_W {
        SFRWPRIV_W { w: self }
    }
    #[doc = "Bit 8 - SRAM Privilege"]
    #[inline(always)]
    pub fn rampriv(&mut self) -> RAMPRIV_W {
        RAMPRIV_W { w: self }
    }
    #[doc = "Bit 9 - Halt Request"]
    #[inline(always)]
    pub fn haltreq(&mut self) -> HALTREQ_W {
        HALTREQ_W { w: self }
    }
    #[doc = "Bit 31 - Main Trace Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
