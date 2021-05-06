#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u32, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u32, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTINTEO0`"]
pub type EXTINTEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINTEO0`"]
pub struct EXTINTEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO0_W<'a> {
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
#[doc = "Reader of field `EXTINTEO1`"]
pub type EXTINTEO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINTEO1`"]
pub struct EXTINTEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO1_W<'a> {
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
#[doc = "Reader of field `EXTINTEO2`"]
pub type EXTINTEO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINTEO2`"]
pub struct EXTINTEO2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO2_W<'a> {
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
#[doc = "Reader of field `EXTINTEO3`"]
pub type EXTINTEO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINTEO3`"]
pub struct EXTINTEO3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO3_W<'a> {
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
#[doc = "Reader of field `EXTINTEO4`"]
pub type EXTINTEO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINTEO4`"]
pub struct EXTINTEO4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO4_W<'a> {
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
#[doc = "Reader of field `EXTINTEO5`"]
pub type EXTINTEO5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINTEO5`"]
pub struct EXTINTEO5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO5_W<'a> {
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
#[doc = "Reader of field `EXTINTEO6`"]
pub type EXTINTEO6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINTEO6`"]
pub struct EXTINTEO6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO6_W<'a> {
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
#[doc = "Reader of field `EXTINTEO7`"]
pub type EXTINTEO7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINTEO7`"]
pub struct EXTINTEO7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINTEO7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo0(&self) -> EXTINTEO0_R {
        EXTINTEO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo1(&self) -> EXTINTEO1_R {
        EXTINTEO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo2(&self) -> EXTINTEO2_R {
        EXTINTEO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo3(&self) -> EXTINTEO3_R {
        EXTINTEO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo4(&self) -> EXTINTEO4_R {
        EXTINTEO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo5(&self) -> EXTINTEO5_R {
        EXTINTEO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo6(&self) -> EXTINTEO6_R {
        EXTINTEO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo7(&self) -> EXTINTEO7_R {
        EXTINTEO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo0(&mut self) -> EXTINTEO0_W {
        EXTINTEO0_W { w: self }
    }
    #[doc = "Bit 1 - External Interrupt 1 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo1(&mut self) -> EXTINTEO1_W {
        EXTINTEO1_W { w: self }
    }
    #[doc = "Bit 2 - External Interrupt 2 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo2(&mut self) -> EXTINTEO2_W {
        EXTINTEO2_W { w: self }
    }
    #[doc = "Bit 3 - External Interrupt 3 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo3(&mut self) -> EXTINTEO3_W {
        EXTINTEO3_W { w: self }
    }
    #[doc = "Bit 4 - External Interrupt 4 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo4(&mut self) -> EXTINTEO4_W {
        EXTINTEO4_W { w: self }
    }
    #[doc = "Bit 5 - External Interrupt 5 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo5(&mut self) -> EXTINTEO5_W {
        EXTINTEO5_W { w: self }
    }
    #[doc = "Bit 6 - External Interrupt 6 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo6(&mut self) -> EXTINTEO6_W {
        EXTINTEO6_W { w: self }
    }
    #[doc = "Bit 7 - External Interrupt 7 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo7(&mut self) -> EXTINTEO7_W {
        EXTINTEO7_W { w: self }
    }
}
