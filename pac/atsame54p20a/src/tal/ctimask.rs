#[doc = "Reader of register CTIMASK%s"]
pub type R = crate::R<u8, super::CTIMASK>;
#[doc = "Writer for register CTIMASK%s"]
pub type W = crate::W<u8, super::CTIMASK>;
#[doc = "Register CTIMASK%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CTIMASK {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPU0`"]
pub type CPU0_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `CPU1`"]
pub type CPU1_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `EVBRK`"]
pub type EVBRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVBRK`"]
pub struct EVBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> EVBRK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EXTBRK`"]
pub type EXTBRK_R = crate::R<bool, bool>;
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
impl R {
    #[doc = "Bit 0 - CPU 0 Break Master"]
    #[inline(always)]
    pub fn cpu0(&self) -> CPU0_R {
        CPU0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU 1 Break Master"]
    #[inline(always)]
    pub fn cpu1(&self) -> CPU1_R {
        CPU1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Event Break Master"]
    #[inline(always)]
    pub fn evbrk(&self) -> EVBRK_R {
        EVBRK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Break Master"]
    #[inline(always)]
    pub fn extbrk(&self) -> EXTBRK_R {
        EXTBRK_R::new(((self.bits >> 7) & 0x01) != 0)
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
    #[doc = "Bit 6 - Event Break Master"]
    #[inline(always)]
    pub fn evbrk(&mut self) -> EVBRK_W {
        EVBRK_W { w: self }
    }
    #[doc = "Bit 7 - External Break Master"]
    #[inline(always)]
    pub fn extbrk(&mut self) -> EXTBRK_W {
        EXTBRK_W { w: self }
    }
}
