#[doc = "Reader of register IRQTRIG"]
pub type R = crate::R<u32, super::IRQTRIG>;
#[doc = "Writer for register IRQTRIG"]
pub type W = crate::W<u32, super::IRQTRIG>;
#[doc = "Register IRQTRIG `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQTRIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Reader of field `IRQNUM`"]
pub type IRQNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQNUM`"]
pub struct IRQNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `OVERRIDE`"]
pub type OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OVERRIDE`"]
pub struct OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Interrupt Request Number"]
    #[inline(always)]
    pub fn irqnum(&self) -> IRQNUM_R {
        IRQNUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt Request Override Value"]
    #[inline(always)]
    pub fn override_(&self) -> OVERRIDE_R {
        OVERRIDE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 8:15 - Interrupt Request Number"]
    #[inline(always)]
    pub fn irqnum(&mut self) -> IRQNUM_W {
        IRQNUM_W { w: self }
    }
    #[doc = "Bits 16:23 - Interrupt Request Override Value"]
    #[inline(always)]
    pub fn override_(&mut self) -> OVERRIDE_W {
        OVERRIDE_W { w: self }
    }
}
