#[doc = "Reader of register SHPR1"]
pub type R = crate::R<u32, super::SHPR1>;
#[doc = "Writer for register SHPR1"]
pub type W = crate::W<u32, super::SHPR1>;
#[doc = "Register SHPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_4`"]
pub type PRI_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_4`"]
pub struct PRI_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PRI_5`"]
pub type PRI_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_5`"]
pub struct PRI_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRI_6`"]
pub type PRI_6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_6`"]
pub struct PRI_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of system handler 4, MemManage"]
    #[inline(always)]
    pub fn pri_4(&self) -> PRI_4_R {
        PRI_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority of system handler 5, BusFault"]
    #[inline(always)]
    pub fn pri_5(&self) -> PRI_5_R {
        PRI_5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priority of system handler 6, UsageFault"]
    #[inline(always)]
    pub fn pri_6(&self) -> PRI_6_R {
        PRI_6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of system handler 4, MemManage"]
    #[inline(always)]
    pub fn pri_4(&mut self) -> PRI_4_W {
        PRI_4_W { w: self }
    }
    #[doc = "Bits 8:15 - Priority of system handler 5, BusFault"]
    #[inline(always)]
    pub fn pri_5(&mut self) -> PRI_5_W {
        PRI_5_W { w: self }
    }
    #[doc = "Bits 16:23 - Priority of system handler 6, UsageFault"]
    #[inline(always)]
    pub fn pri_6(&mut self) -> PRI_6_W {
        PRI_6_W { w: self }
    }
}
