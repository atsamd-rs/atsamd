#[doc = "Reader of register BDPR"]
pub type R = crate::R<u32, super::BDPR>;
#[doc = "Writer for register BDPR"]
pub type W = crate::W<u32, super::BDPR>;
#[doc = "Register BDPR `reset()`'s with value 0"]
impl crate::ResetValue for super::BDPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUFDATA`"]
pub type BUFDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUFDATA`"]
pub struct BUFDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdata(&self) -> BUFDATA_R {
        BUFDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdata(&mut self) -> BUFDATA_W {
        BUFDATA_W { w: self }
    }
}
