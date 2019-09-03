#[doc = "Writer for register MC2R"]
pub type W = crate::W<u8, super::MC2R>;
#[doc = "Register MC2R `reset()`'s with value 0"]
impl crate::ResetValue for super::MC2R {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SRESP`"]
pub struct SRESP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRESP_W<'a> {
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
#[doc = "Write proxy for field `ABOOT`"]
pub struct ABOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABOOT_W<'a> {
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
impl W {
    #[doc = "Bit 0 - e.MMC Abort Wait IRQ"]
    #[inline(always)]
    pub fn sresp(&mut self) -> SRESP_W {
        SRESP_W { w: self }
    }
    #[doc = "Bit 1 - e.MMC Abort Boot"]
    #[inline(always)]
    pub fn aboot(&mut self) -> ABOOT_W {
        ABOOT_W { w: self }
    }
}
