#[doc = "Reader of register DFLLMUL"]
pub type R = crate::R<u32, super::DFLLMUL>;
#[doc = "Writer for register DFLLMUL"]
pub type W = crate::W<u32, super::DFLLMUL>;
#[doc = "Register DFLLMUL `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLLMUL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MUL`"]
pub type MUL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MUL`"]
pub struct MUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `FSTEP`"]
pub type FSTEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSTEP`"]
pub struct FSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CSTEP`"]
pub type CSTEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSTEP`"]
pub struct CSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&self) -> MUL_R {
        MUL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Fine Maximum Step"]
    #[inline(always)]
    pub fn fstep(&self) -> FSTEP_R {
        FSTEP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 26:31 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&self) -> CSTEP_R {
        CSTEP_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&mut self) -> MUL_W {
        MUL_W { w: self }
    }
    #[doc = "Bits 16:23 - Fine Maximum Step"]
    #[inline(always)]
    pub fn fstep(&mut self) -> FSTEP_W {
        FSTEP_W { w: self }
    }
    #[doc = "Bits 26:31 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&mut self) -> CSTEP_W {
        CSTEP_W { w: self }
    }
}
