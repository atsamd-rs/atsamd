#[doc = "Reader of register FFCR"]
pub type R = crate::R<u32, super::FFCR>;
#[doc = "Writer for register FFCR"]
pub type W = crate::W<u32, super::FFCR>;
#[doc = "Register FFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EnFCont`"]
pub type ENFCONT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EnFCont`"]
pub struct ENFCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENFCONT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TrigIn`"]
pub type TRIGIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TrigIn`"]
pub struct TRIGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_fcont(&self) -> ENFCONT_R {
        ENFCONT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trig_in(&self) -> TRIGIN_R {
        TRIGIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_fcont(&mut self) -> ENFCONT_W {
        ENFCONT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trig_in(&mut self) -> TRIGIN_W {
        TRIGIN_W { w: self }
    }
}
