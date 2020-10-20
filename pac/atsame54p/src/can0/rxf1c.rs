#[doc = "Reader of register RXF1C"]
pub type R = crate::R<u32, super::RXF1C>;
#[doc = "Writer for register RXF1C"]
pub type W = crate::W<u32, super::RXF1C>;
#[doc = "Register RXF1C `reset()`'s with value 0"]
impl crate::ResetValue for super::RXF1C {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F1SA`"]
pub type F1SA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `F1SA`"]
pub struct F1SA_W<'a> {
    w: &'a mut W,
}
impl<'a> F1SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `F1S`"]
pub type F1S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F1S`"]
pub struct F1S_W<'a> {
    w: &'a mut W,
}
impl<'a> F1S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `F1WM`"]
pub type F1WM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F1WM`"]
pub struct F1WM_W<'a> {
    w: &'a mut W,
}
impl<'a> F1WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `F1OM`"]
pub type F1OM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `F1OM`"]
pub struct F1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> F1OM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Rx FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1sa(&self) -> F1SA_R {
        F1SA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size"]
    #[inline(always)]
    pub fn f1s(&self) -> F1S_R {
        F1S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark"]
    #[inline(always)]
    pub fn f1wm(&self) -> F1WM_R {
        F1WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 1 Operation Mode"]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Rx FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1sa(&mut self) -> F1SA_W {
        F1SA_W { w: self }
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size"]
    #[inline(always)]
    pub fn f1s(&mut self) -> F1S_W {
        F1S_W { w: self }
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark"]
    #[inline(always)]
    pub fn f1wm(&mut self) -> F1WM_W {
        F1WM_W { w: self }
    }
    #[doc = "Bit 31 - FIFO 1 Operation Mode"]
    #[inline(always)]
    pub fn f1om(&mut self) -> F1OM_W {
        F1OM_W { w: self }
    }
}
