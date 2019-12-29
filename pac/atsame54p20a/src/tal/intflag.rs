#[doc = "Reader of register INTFLAG"]
pub type R = crate::R<u8, super::INTFLAG>;
#[doc = "Writer for register INTFLAG"]
pub type W = crate::W<u8, super::INTFLAG>;
#[doc = "Register INTFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRK`"]
pub type BRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRK`"]
pub struct BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_W<'a> {
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
#[doc = "Reader of field `IPS0`"]
pub type IPS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS0`"]
pub struct IPS0_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS0_W<'a> {
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
#[doc = "Reader of field `IPS1`"]
pub type IPS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS1`"]
pub struct IPS1_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS1_W<'a> {
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
    #[doc = "Bit 0 - Break"]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Inter-Processor Signal for CPU 0"]
    #[inline(always)]
    pub fn ips0(&self) -> IPS0_R {
        IPS0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Inter-Processor Signal for CPU 1"]
    #[inline(always)]
    pub fn ips1(&self) -> IPS1_R {
        IPS1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Break"]
    #[inline(always)]
    pub fn brk(&mut self) -> BRK_W {
        BRK_W { w: self }
    }
    #[doc = "Bit 1 - Inter-Processor Signal for CPU 0"]
    #[inline(always)]
    pub fn ips0(&mut self) -> IPS0_W {
        IPS0_W { w: self }
    }
    #[doc = "Bit 2 - Inter-Processor Signal for CPU 1"]
    #[inline(always)]
    pub fn ips1(&mut self) -> IPS1_W {
        IPS1_W { w: self }
    }
}
