#[doc = "Reader of register INSTRCTRL"]
pub type R = crate::R<u32, super::INSTRCTRL>;
#[doc = "Writer for register INSTRCTRL"]
pub type W = crate::W<u32, super::INSTRCTRL>;
#[doc = "Register INSTRCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INSTRCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSTR`"]
pub type INSTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR`"]
pub struct INSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `OPTCODE`"]
pub type OPTCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPTCODE`"]
pub struct OPTCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn instr(&self) -> INSTR_R {
        INSTR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn optcode(&self) -> OPTCODE_R {
        OPTCODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn instr(&mut self) -> INSTR_W {
        INSTR_W { w: self }
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn optcode(&mut self) -> OPTCODE_W {
        OPTCODE_W { w: self }
    }
}
