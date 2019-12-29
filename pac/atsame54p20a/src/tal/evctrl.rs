#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u16, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u16, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRKEI`"]
pub type BRKEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRKEI`"]
pub struct BRKEI_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEI_W<'a> {
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
#[doc = "Reader of field `BRKEO`"]
pub type BRKEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRKEO`"]
pub struct BRKEO_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEO_W<'a> {
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
#[doc = "Reader of field `IRQMONEO0`"]
pub type IRQMONEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQMONEO0`"]
pub struct IRQMONEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQMONEO0_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Break Input Event Enable"]
    #[inline(always)]
    pub fn brkei(&self) -> BRKEI_R {
        BRKEI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Break Output Event Enable"]
    #[inline(always)]
    pub fn brkeo(&self) -> BRKEO_R {
        BRKEO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt Request Monitor 0 Output Event Enable"]
    #[inline(always)]
    pub fn irqmoneo0(&self) -> IRQMONEO0_R {
        IRQMONEO0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Break Input Event Enable"]
    #[inline(always)]
    pub fn brkei(&mut self) -> BRKEI_W {
        BRKEI_W { w: self }
    }
    #[doc = "Bit 1 - Break Output Event Enable"]
    #[inline(always)]
    pub fn brkeo(&mut self) -> BRKEO_W {
        BRKEO_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt Request Monitor 0 Output Event Enable"]
    #[inline(always)]
    pub fn irqmoneo0(&mut self) -> IRQMONEO0_W {
        IRQMONEO0_W { w: self }
    }
}
