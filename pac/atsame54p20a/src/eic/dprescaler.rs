#[doc = "Reader of register DPRESCALER"]
pub type R = crate::R<u32, super::DPRESCALER>;
#[doc = "Writer for register DPRESCALER"]
pub type W = crate::W<u32, super::DPRESCALER>;
#[doc = "Register DPRESCALER `reset()`'s with value 0"]
impl crate::ResetValue for super::DPRESCALER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRESCALER0`"]
pub type PRESCALER0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESCALER0`"]
pub struct PRESCALER0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `PRESCALER1`"]
pub type PRESCALER1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESCALER1`"]
pub struct PRESCALER1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `STATES0`"]
pub type STATES0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATES0`"]
pub struct STATES0_W<'a> {
    w: &'a mut W,
}
impl<'a> STATES0_W<'a> {
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
#[doc = "Reader of field `STATES1`"]
pub type STATES1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATES1`"]
pub struct STATES1_W<'a> {
    w: &'a mut W,
}
impl<'a> STATES1_W<'a> {
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
#[doc = "Reader of field `TICKON`"]
pub type TICKON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TICKON`"]
pub struct TICKON_W<'a> {
    w: &'a mut W,
}
impl<'a> TICKON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler0(&self) -> PRESCALER0_R {
        PRESCALER0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler1(&self) -> PRESCALER1_R {
        PRESCALER1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    pub fn states0(&self) -> STATES0_R {
        STATES0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline(always)]
    pub fn states1(&self) -> STATES1_R {
        STATES1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    pub fn tickon(&self) -> TICKON_R {
        TICKON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler0(&mut self) -> PRESCALER0_W {
        PRESCALER0_W { w: self }
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler1(&mut self) -> PRESCALER1_W {
        PRESCALER1_W { w: self }
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    pub fn states0(&mut self) -> STATES0_W {
        STATES0_W { w: self }
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline(always)]
    pub fn states1(&mut self) -> STATES1_W {
        STATES1_W { w: self }
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    pub fn tickon(&mut self) -> TICKON_W {
        TICKON_W { w: self }
    }
}
