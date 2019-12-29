#[doc = "Writer for register RESTART"]
pub type W = crate::W<u8, super::RESTART>;
#[doc = "Register RESTART `reset()`'s with value 0"]
impl crate::ResetValue for super::RESTART {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CPU0`"]
pub struct CPU0_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0_W<'a> {
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
#[doc = "Write proxy for field `CPU1`"]
pub struct CPU1_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1_W<'a> {
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
#[doc = "Write proxy for field `EXTBRK`"]
pub struct EXTBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTBRK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - CPU 0 Break Master"]
    #[inline(always)]
    pub fn cpu0(&mut self) -> CPU0_W {
        CPU0_W { w: self }
    }
    #[doc = "Bit 1 - CPU 1 Break Master"]
    #[inline(always)]
    pub fn cpu1(&mut self) -> CPU1_W {
        CPU1_W { w: self }
    }
    #[doc = "Bit 7 - External Break Master"]
    #[inline(always)]
    pub fn extbrk(&mut self) -> EXTBRK_W {
        EXTBRK_W { w: self }
    }
}
