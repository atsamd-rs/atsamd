#[doc = "Reader of register CTRLC"]
pub type R = crate::R<u8, super::CTRLC>;
#[doc = "Writer for register CTRLC"]
pub type W = crate::W<u8, super::CTRLC>;
#[doc = "Register CTRLC `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INVEN0`"]
pub type INVEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEN0`"]
pub struct INVEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN0_W<'a> {
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
#[doc = "Reader of field `INVEN1`"]
pub type INVEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEN1`"]
pub struct INVEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN1_W<'a> {
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
#[doc = "Reader of field `CPTEN0`"]
pub type CPTEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPTEN0`"]
pub struct CPTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTEN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CPTEN1`"]
pub type CPTEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPTEN1`"]
pub struct CPTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Output Waveform 0 Invert Enable"]
    #[inline(always)]
    pub fn inven0(&self) -> INVEN0_R {
        INVEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Waveform 1 Invert Enable"]
    #[inline(always)]
    pub fn inven1(&self) -> INVEN1_R {
        INVEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&self) -> CPTEN0_R {
        CPTEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&self) -> CPTEN1_R {
        CPTEN1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Waveform 0 Invert Enable"]
    #[inline(always)]
    pub fn inven0(&mut self) -> INVEN0_W {
        INVEN0_W { w: self }
    }
    #[doc = "Bit 1 - Output Waveform 1 Invert Enable"]
    #[inline(always)]
    pub fn inven1(&mut self) -> INVEN1_W {
        INVEN1_W { w: self }
    }
    #[doc = "Bit 4 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&mut self) -> CPTEN0_W {
        CPTEN0_W { w: self }
    }
    #[doc = "Bit 5 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&mut self) -> CPTEN1_W {
        CPTEN1_W { w: self }
    }
}
