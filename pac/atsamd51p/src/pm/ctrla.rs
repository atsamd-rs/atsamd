#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u8, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u8, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IORET`"]
pub type IORET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IORET`"]
pub struct IORET_W<'a> {
    w: &'a mut W,
}
impl<'a> IORET_W<'a> {
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
impl R {
    #[doc = "Bit 2 - I/O Retention"]
    #[inline(always)]
    pub fn ioret(&self) -> IORET_R {
        IORET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - I/O Retention"]
    #[inline(always)]
    pub fn ioret(&mut self) -> IORET_W {
        IORET_W { w: self }
    }
}
