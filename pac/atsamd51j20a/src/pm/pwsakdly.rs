#[doc = "Reader of register PWSAKDLY"]
pub type R = crate::R<u8, super::PWSAKDLY>;
#[doc = "Writer for register PWSAKDLY"]
pub type W = crate::W<u8, super::PWSAKDLY>;
#[doc = "Register PWSAKDLY `reset()`'s with value 0"]
impl crate::ResetValue for super::PWSAKDLY {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLYVAL`"]
pub type DLYVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYVAL`"]
pub struct DLYVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u8) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `IGNACK`"]
pub type IGNACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGNACK`"]
pub struct IGNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNACK_W<'a> {
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
    #[doc = "Bits 0:6 - Delay Value"]
    #[inline(always)]
    pub fn dlyval(&self) -> DLYVAL_R {
        DLYVAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Delay Value"]
    #[inline(always)]
    pub fn dlyval(&mut self) -> DLYVAL_W {
        DLYVAL_W { w: self }
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IGNACK_W {
        IGNACK_W { w: self }
    }
}
