#[doc = "Reader of register HIBCFG"]
pub type R = crate::R<u8, super::HIBCFG>;
#[doc = "Writer for register HIBCFG"]
pub type W = crate::W<u8, super::HIBCFG>;
#[doc = "Register HIBCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::HIBCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RAMCFG`"]
pub type RAMCFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RAMCFG`"]
pub struct RAMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `BRAMCFG`"]
pub type BRAMCFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRAMCFG`"]
pub struct BRAMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> BRAMCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u8) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&self) -> RAMCFG_R {
        RAMCFG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Backup Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&self) -> BRAMCFG_R {
        BRAMCFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&mut self) -> RAMCFG_W {
        RAMCFG_W { w: self }
    }
    #[doc = "Bits 2:3 - Backup Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&mut self) -> BRAMCFG_W {
        BRAMCFG_W { w: self }
    }
}
