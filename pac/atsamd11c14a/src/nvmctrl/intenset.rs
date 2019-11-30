#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u8, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u8, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READY`"]
pub struct READY_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_W<'a> {
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
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - NVM Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVM Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W { w: self }
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
}
