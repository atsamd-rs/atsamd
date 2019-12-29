#[doc = "Reader of register INTCPUSEL8"]
pub type R = crate::R<u32, super::INTCPUSEL8>;
#[doc = "Writer for register INTCPUSEL8"]
pub type W = crate::W<u32, super::INTCPUSEL8>;
#[doc = "Register INTCPUSEL8 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCPUSEL8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDHC0`"]
pub type SDHC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDHC0`"]
pub struct SDHC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDHC0_W<'a> {
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
#[doc = "Reader of field `SDHC1`"]
pub type SDHC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDHC1`"]
pub struct SDHC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDHC1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SDHC0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sdhc0(&self) -> SDHC0_R {
        SDHC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - SDHC1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sdhc1(&self) -> SDHC1_R {
        SDHC1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDHC0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sdhc0(&mut self) -> SDHC0_W {
        SDHC0_W { w: self }
    }
    #[doc = "Bit 2 - SDHC1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sdhc1(&mut self) -> SDHC1_W {
        SDHC1_W { w: self }
    }
}
