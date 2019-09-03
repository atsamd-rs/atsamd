#[doc = "Reader of register FLOW"]
pub type R = crate::R<u32, super::FLOW>;
#[doc = "Writer for register FLOW"]
pub type W = crate::W<u32, super::FLOW>;
#[doc = "Register FLOW `reset()`'s with value 0"]
impl crate::ResetValue for super::FLOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUTOSTOP`"]
pub type AUTOSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSTOP`"]
pub struct AUTOSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTOP_W<'a> {
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
#[doc = "Reader of field `AUTOHALT`"]
pub type AUTOHALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOHALT`"]
pub struct AUTOHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOHALT_W<'a> {
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
#[doc = "Reader of field `WATERMARK`"]
pub type WATERMARK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WATERMARK`"]
pub struct WATERMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> WATERMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Auto Stop Tracing"]
    #[inline(always)]
    pub fn autostop(&self) -> AUTOSTOP_R {
        AUTOSTOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto Halt Request"]
    #[inline(always)]
    pub fn autohalt(&self) -> AUTOHALT_R {
        AUTOHALT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 3:31 - Watermark value"]
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Stop Tracing"]
    #[inline(always)]
    pub fn autostop(&mut self) -> AUTOSTOP_W {
        AUTOSTOP_W { w: self }
    }
    #[doc = "Bit 1 - Auto Halt Request"]
    #[inline(always)]
    pub fn autohalt(&mut self) -> AUTOHALT_W {
        AUTOHALT_W { w: self }
    }
    #[doc = "Bits 3:31 - Watermark value"]
    #[inline(always)]
    pub fn watermark(&mut self) -> WATERMARK_W {
        WATERMARK_W { w: self }
    }
}
