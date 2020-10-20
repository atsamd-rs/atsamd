#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u8, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLB {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `START0`"]
pub struct START0_W<'a> {
    w: &'a mut W,
}
impl<'a> START0_W<'a> {
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
#[doc = "Write proxy for field `START1`"]
pub struct START1_W<'a> {
    w: &'a mut W,
}
impl<'a> START1_W<'a> {
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
    #[doc = "Bit 0 - Comparator 0 Start Comparison"]
    #[inline(always)]
    pub fn start0(&mut self) -> START0_W {
        START0_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 Start Comparison"]
    #[inline(always)]
    pub fn start1(&mut self) -> START1_W {
        START1_W { w: self }
    }
}
