#[doc = "Reader of register STATUS"]
pub type R = crate::R<u16, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u16, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUFOVF`"]
pub type BUFOVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFOVF`"]
pub struct BUFOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFOVF_W<'a> {
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
#[doc = "Reader of field `LENERR`"]
pub type LENERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LENERR`"]
pub struct LENERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LENERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&self) -> BUFOVF_R {
        BUFOVF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transaction Length Error"]
    #[inline(always)]
    pub fn lenerr(&self) -> LENERR_R {
        LENERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&mut self) -> BUFOVF_W {
        BUFOVF_W { w: self }
    }
    #[doc = "Bit 11 - Transaction Length Error"]
    #[inline(always)]
    pub fn lenerr(&mut self) -> LENERR_W {
        LENERR_W { w: self }
    }
}
