#[doc = "Reader of register STATUS"]
pub type R = crate::R<u16, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u16, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0x40"]
impl crate::ResetValue for super::STATUS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `QERR`"]
pub type QERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QERR`"]
pub struct QERR_W<'a> {
    w: &'a mut W,
}
impl<'a> QERR_W<'a> {
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
#[doc = "Reader of field `IDXERR`"]
pub type IDXERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDXERR`"]
pub struct IDXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IDXERR_W<'a> {
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
#[doc = "Reader of field `MPERR`"]
pub type MPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPERR`"]
pub struct MPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MPERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `WINERR`"]
pub type WINERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WINERR`"]
pub struct WINERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WINERR_W<'a> {
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
#[doc = "Reader of field `HERR`"]
pub type HERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HERR`"]
pub struct HERR_W<'a> {
    w: &'a mut W,
}
impl<'a> HERR_W<'a> {
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
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PRESCBUFV`"]
pub type PRESCBUFV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRESCBUFV`"]
pub struct PRESCBUFV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCBUFV_W<'a> {
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
#[doc = "Reader of field `FILTERBUFV`"]
pub type FILTERBUFV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTERBUFV`"]
pub struct FILTERBUFV_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTERBUFV_W<'a> {
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
#[doc = "Reader of field `CCBUFV0`"]
pub type CCBUFV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCBUFV0`"]
pub struct CCBUFV0_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV0_W<'a> {
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
#[doc = "Reader of field `CCBUFV1`"]
pub type CCBUFV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCBUFV1`"]
pub struct CCBUFV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV1_W<'a> {
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
    #[doc = "Bit 0 - Quadrature Error Flag"]
    #[inline(always)]
    pub fn qerr(&self) -> QERR_R {
        QERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Index Error Flag"]
    #[inline(always)]
    pub fn idxerr(&self) -> IDXERR_R {
        IDXERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Missing Pulse Error flag"]
    #[inline(always)]
    pub fn mperr(&self) -> MPERR_R {
        MPERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Window Error Flag"]
    #[inline(always)]
    pub fn winerr(&self) -> WINERR_R {
        WINERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hall Error Flag"]
    #[inline(always)]
    pub fn herr(&self) -> HERR_R {
        HERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Direction Status Flag"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Prescaler Buffer Valid"]
    #[inline(always)]
    pub fn prescbufv(&self) -> PRESCBUFV_R {
        PRESCBUFV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter Buffer Valid"]
    #[inline(always)]
    pub fn filterbufv(&self) -> FILTERBUFV_R {
        FILTERBUFV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv0(&self) -> CCBUFV0_R {
        CCBUFV0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv1(&self) -> CCBUFV1_R {
        CCBUFV1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quadrature Error Flag"]
    #[inline(always)]
    pub fn qerr(&mut self) -> QERR_W {
        QERR_W { w: self }
    }
    #[doc = "Bit 1 - Index Error Flag"]
    #[inline(always)]
    pub fn idxerr(&mut self) -> IDXERR_W {
        IDXERR_W { w: self }
    }
    #[doc = "Bit 2 - Missing Pulse Error flag"]
    #[inline(always)]
    pub fn mperr(&mut self) -> MPERR_W {
        MPERR_W { w: self }
    }
    #[doc = "Bit 4 - Window Error Flag"]
    #[inline(always)]
    pub fn winerr(&mut self) -> WINERR_W {
        WINERR_W { w: self }
    }
    #[doc = "Bit 5 - Hall Error Flag"]
    #[inline(always)]
    pub fn herr(&mut self) -> HERR_W {
        HERR_W { w: self }
    }
    #[doc = "Bit 6 - Stop"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 7 - Direction Status Flag"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 8 - Prescaler Buffer Valid"]
    #[inline(always)]
    pub fn prescbufv(&mut self) -> PRESCBUFV_W {
        PRESCBUFV_W { w: self }
    }
    #[doc = "Bit 9 - Filter Buffer Valid"]
    #[inline(always)]
    pub fn filterbufv(&mut self) -> FILTERBUFV_W {
        FILTERBUFV_W { w: self }
    }
    #[doc = "Bit 12 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv0(&mut self) -> CCBUFV0_W {
        CCBUFV0_W { w: self }
    }
    #[doc = "Bit 13 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv1(&mut self) -> CCBUFV1_W {
        CCBUFV1_W { w: self }
    }
}
