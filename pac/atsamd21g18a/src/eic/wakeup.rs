#[doc = "Reader of register WAKEUP"]
pub type R = crate::R<u32, super::WAKEUP>;
#[doc = "Writer for register WAKEUP"]
pub type W = crate::W<u32, super::WAKEUP>;
#[doc = "Register WAKEUP `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKEUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKEUPEN0`"]
pub type WAKEUPEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN0`"]
pub struct WAKEUPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN0_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN1`"]
pub type WAKEUPEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN1`"]
pub struct WAKEUPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN1_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN2`"]
pub type WAKEUPEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN2`"]
pub struct WAKEUPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN2_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN3`"]
pub type WAKEUPEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN3`"]
pub struct WAKEUPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN3_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN4`"]
pub type WAKEUPEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN4`"]
pub struct WAKEUPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN4_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN5`"]
pub type WAKEUPEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN5`"]
pub struct WAKEUPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN5_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN6`"]
pub type WAKEUPEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN6`"]
pub struct WAKEUPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN6_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN7`"]
pub type WAKEUPEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN7`"]
pub struct WAKEUPEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN7_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN8`"]
pub type WAKEUPEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN8`"]
pub struct WAKEUPEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN8_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN9`"]
pub type WAKEUPEN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN9`"]
pub struct WAKEUPEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN9_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN10`"]
pub type WAKEUPEN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN10`"]
pub struct WAKEUPEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN10_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN11`"]
pub type WAKEUPEN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN11`"]
pub struct WAKEUPEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN11_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN12`"]
pub type WAKEUPEN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN12`"]
pub struct WAKEUPEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `WAKEUPEN13`"]
pub type WAKEUPEN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN13`"]
pub struct WAKEUPEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN13_W<'a> {
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
#[doc = "Reader of field `WAKEUPEN14`"]
pub type WAKEUPEN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN14`"]
pub struct WAKEUPEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `WAKEUPEN15`"]
pub type WAKEUPEN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPEN15`"]
pub struct WAKEUPEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEN15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen0(&self) -> WAKEUPEN0_R {
        WAKEUPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen1(&self) -> WAKEUPEN1_R {
        WAKEUPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen2(&self) -> WAKEUPEN2_R {
        WAKEUPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen3(&self) -> WAKEUPEN3_R {
        WAKEUPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen4(&self) -> WAKEUPEN4_R {
        WAKEUPEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen5(&self) -> WAKEUPEN5_R {
        WAKEUPEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen6(&self) -> WAKEUPEN6_R {
        WAKEUPEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen7(&self) -> WAKEUPEN7_R {
        WAKEUPEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen8(&self) -> WAKEUPEN8_R {
        WAKEUPEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen9(&self) -> WAKEUPEN9_R {
        WAKEUPEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen10(&self) -> WAKEUPEN10_R {
        WAKEUPEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen11(&self) -> WAKEUPEN11_R {
        WAKEUPEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen12(&self) -> WAKEUPEN12_R {
        WAKEUPEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen13(&self) -> WAKEUPEN13_R {
        WAKEUPEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen14(&self) -> WAKEUPEN14_R {
        WAKEUPEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen15(&self) -> WAKEUPEN15_R {
        WAKEUPEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen0(&mut self) -> WAKEUPEN0_W {
        WAKEUPEN0_W { w: self }
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen1(&mut self) -> WAKEUPEN1_W {
        WAKEUPEN1_W { w: self }
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen2(&mut self) -> WAKEUPEN2_W {
        WAKEUPEN2_W { w: self }
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen3(&mut self) -> WAKEUPEN3_W {
        WAKEUPEN3_W { w: self }
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen4(&mut self) -> WAKEUPEN4_W {
        WAKEUPEN4_W { w: self }
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen5(&mut self) -> WAKEUPEN5_W {
        WAKEUPEN5_W { w: self }
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen6(&mut self) -> WAKEUPEN6_W {
        WAKEUPEN6_W { w: self }
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen7(&mut self) -> WAKEUPEN7_W {
        WAKEUPEN7_W { w: self }
    }
    #[doc = "Bit 8 - External Interrupt 8 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen8(&mut self) -> WAKEUPEN8_W {
        WAKEUPEN8_W { w: self }
    }
    #[doc = "Bit 9 - External Interrupt 9 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen9(&mut self) -> WAKEUPEN9_W {
        WAKEUPEN9_W { w: self }
    }
    #[doc = "Bit 10 - External Interrupt 10 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen10(&mut self) -> WAKEUPEN10_W {
        WAKEUPEN10_W { w: self }
    }
    #[doc = "Bit 11 - External Interrupt 11 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen11(&mut self) -> WAKEUPEN11_W {
        WAKEUPEN11_W { w: self }
    }
    #[doc = "Bit 12 - External Interrupt 12 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen12(&mut self) -> WAKEUPEN12_W {
        WAKEUPEN12_W { w: self }
    }
    #[doc = "Bit 13 - External Interrupt 13 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen13(&mut self) -> WAKEUPEN13_W {
        WAKEUPEN13_W { w: self }
    }
    #[doc = "Bit 14 - External Interrupt 14 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen14(&mut self) -> WAKEUPEN14_W {
        WAKEUPEN14_W { w: self }
    }
    #[doc = "Bit 15 - External Interrupt 15 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen15(&mut self) -> WAKEUPEN15_W {
        WAKEUPEN15_W { w: self }
    }
}
